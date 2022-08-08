#pragma once
#include <wx/wx.h>
//#include <wx/activityindicator.h>
#include <wx/artprov.h>
#include <wx/bookctrl.h>
#include <wx/clrpicker.h>
#include <wx/datectrl.h>
#include <wx/dirctrl.h>
#include <wx/editlbox.h>
#include <wx/filectrl.h>
#include <wx/filepicker.h>
#include <wx/fontpicker.h>
#include <wx/headerctrl.h>
#include <wx/hyperlink.h>
#include <wx/spinbutt.h>
#include <wx/spinctrl.h>
#include <wx/srchctrl.h>
#include <wx/tglbtn.h>
#include <wx/timectrl.h>
#include <wx/wrapsizer.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

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
wxAcceleratorTable *wxAcceleratorTable_new2(const wxString * resource);
bool wxAcceleratorTable_IsOk(const wxAcceleratorTable * self);

// CLASS: wxAccessible
wxClassInfo *wxAccessible_CLASSINFO();
wxAccessible *wxAccessible_new(wxWindow * win);
wxWindow * wxAccessible_GetWindow(wxAccessible * self);
void wxAccessible_SetWindow(wxAccessible * self, wxWindow * window);

// CLASS: wxActivateEvent
wxClassInfo *wxActivateEvent_CLASSINFO();
bool wxActivateEvent_GetActive(const wxActivateEvent * self);

// CLASS: wxActiveXContainer
wxClassInfo *wxActiveXContainer_CLASSINFO();
// Mix-in(s) to wxActiveXContainer
wxTrackable *wxActiveXContainer_AsTrackable(wxActiveXContainer* obj);

// CLASS: wxActiveXEvent
wxClassInfo *wxActiveXEvent_CLASSINFO();
size_t wxActiveXEvent_ParamCount(const wxActiveXEvent * self);
wxString *wxActiveXEvent_ParamName(const wxActiveXEvent * self, size_t idx);
wxString *wxActiveXEvent_ParamType(const wxActiveXEvent * self, size_t idx);
wxActiveXEventNativeMSW * wxActiveXEvent_GetNativeParameters(const wxActiveXEvent * self);

// CLASS: wxActivityIndicator
wxClassInfo *wxActivityIndicator_CLASSINFO();
wxActivityIndicator *wxActivityIndicator_new();
wxActivityIndicator *wxActivityIndicator_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxActivityIndicator_Create(wxActivityIndicator * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxActivityIndicator_Start(wxActivityIndicator * self);
void wxActivityIndicator_Stop(wxActivityIndicator * self);
bool wxActivityIndicator_IsRunning(const wxActivityIndicator * self);
// Mix-in(s) to wxActivityIndicator
wxTrackable *wxActivityIndicator_AsTrackable(wxActivityIndicator* obj);

// CLASS: wxAddRemoveAdaptor
void wxAddRemoveAdaptor_delete(wxAddRemoveAdaptor *self);
wxAddRemoveAdaptor *wxAddRemoveAdaptor_new();
wxWindow * wxAddRemoveAdaptor_GetItemsCtrl(const wxAddRemoveAdaptor * self);
bool wxAddRemoveAdaptor_CanAdd(const wxAddRemoveAdaptor * self);
bool wxAddRemoveAdaptor_CanRemove(const wxAddRemoveAdaptor * self);
void wxAddRemoveAdaptor_OnAdd(wxAddRemoveAdaptor * self);
void wxAddRemoveAdaptor_OnRemove(wxAddRemoveAdaptor * self);

// CLASS: wxAddRemoveCtrl
wxClassInfo *wxAddRemoveCtrl_CLASSINFO();
wxAddRemoveCtrl *wxAddRemoveCtrl_new();
wxAddRemoveCtrl *wxAddRemoveCtrl_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxAddRemoveCtrl_Create(wxAddRemoveCtrl * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxAddRemoveCtrl_SetAdaptor(wxAddRemoveCtrl * self, wxAddRemoveAdaptor * adaptor);
void wxAddRemoveCtrl_SetButtonsToolTips(wxAddRemoveCtrl * self, const wxString * addtip, const wxString * removetip);
// Mix-in(s) to wxAddRemoveCtrl
wxTrackable *wxAddRemoveCtrl_AsTrackable(wxAddRemoveCtrl* obj);

// CLASS: wxAffineMatrix2D
void wxAffineMatrix2D_delete(wxAffineMatrix2D *self);
wxAffineMatrix2D *wxAffineMatrix2D_new();
void wxAffineMatrix2D_IsEqual(wxAffineMatrix2D * self, const wxAffineMatrix2DBase * t);
void wxAffineMatrix2D_Mirror(wxAffineMatrix2D * self, int direction);
wxPoint2DDouble *wxAffineMatrix2D_TransformPoint(const wxAffineMatrix2D * self, const wxPoint2DDouble * p);
void wxAffineMatrix2D_TransformPoint1(const wxAffineMatrix2D * self, wxDouble * x, wxDouble * y);
wxPoint2DDouble *wxAffineMatrix2D_TransformDistance(const wxAffineMatrix2D * self, const wxPoint2DDouble * p);
void wxAffineMatrix2D_TransformDistance1(const wxAffineMatrix2D * self, wxDouble * dx, wxDouble * dy);

// CLASS: wxAffineMatrix2DBase
void wxAffineMatrix2DBase_delete(wxAffineMatrix2DBase *self);
wxAffineMatrix2DBase *wxAffineMatrix2DBase_new();
void wxAffineMatrix2DBase_Set(wxAffineMatrix2DBase * self, const wxMatrix2D * mat2_d, const wxPoint2DDouble * tr);
void wxAffineMatrix2DBase_Get(const wxAffineMatrix2DBase * self, wxMatrix2D * mat2_d, wxPoint2DDouble * tr);
void wxAffineMatrix2DBase_Concat(wxAffineMatrix2DBase * self, const wxAffineMatrix2DBase * t);
bool wxAffineMatrix2DBase_Invert(wxAffineMatrix2DBase * self);
bool wxAffineMatrix2DBase_IsIdentity(const wxAffineMatrix2DBase * self);
bool wxAffineMatrix2DBase_IsEqual(const wxAffineMatrix2DBase * self, const wxAffineMatrix2DBase * t);
void wxAffineMatrix2DBase_Mirror(wxAffineMatrix2DBase * self, int direction);
wxPoint2DDouble *wxAffineMatrix2DBase_TransformPoint(const wxAffineMatrix2DBase * self, const wxPoint2DDouble * p);
void wxAffineMatrix2DBase_TransformPoint1(const wxAffineMatrix2DBase * self, wxDouble * x, wxDouble * y);
wxPoint2DDouble *wxAffineMatrix2DBase_TransformDistance(const wxAffineMatrix2DBase * self, const wxPoint2DDouble * p);
void wxAffineMatrix2DBase_TransformDistance1(const wxAffineMatrix2DBase * self, wxDouble * dx, wxDouble * dy);

// CLASS: wxAnimationCtrl
wxClassInfo *wxAnimationCtrl_CLASSINFO();
wxAnimationCtrl *wxAnimationCtrl_new(wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxAnimationCtrl_Create(wxAnimationCtrl * self, wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxAnimation *wxAnimationCtrl_CreateAnimation(const wxAnimationCtrl * self);
wxAnimation *wxAnimationCtrl_GetAnimation(const wxAnimationCtrl * self);
wxBitmap *wxAnimationCtrl_GetInactiveBitmap(const wxAnimationCtrl * self);
bool wxAnimationCtrl_IsPlaying(const wxAnimationCtrl * self);
bool wxAnimationCtrl_Play(wxAnimationCtrl * self);
void wxAnimationCtrl_SetAnimation(wxAnimationCtrl * self, const wxAnimation * anim);
void wxAnimationCtrl_SetInactiveBitmap(wxAnimationCtrl * self, const wxBitmapBundle * bmp);
void wxAnimationCtrl_Stop(wxAnimationCtrl * self);
wxAnimation *wxAnimationCtrl_CreateCompatibleAnimation();
// Mix-in(s) to wxAnimationCtrl
wxTrackable *wxAnimationCtrl_AsTrackable(wxAnimationCtrl* obj);

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
// Mix-in(s) to wxAnyButton
wxTrackable *wxAnyButton_AsTrackable(wxAnyButton* obj);

// CLASS: wxAppProgressIndicator
void wxAppProgressIndicator_delete(wxAppProgressIndicator *self);
wxAppProgressIndicator *wxAppProgressIndicator_new(wxWindow * parent, int max_value);
bool wxAppProgressIndicator_IsAvailable(const wxAppProgressIndicator * self);
void wxAppProgressIndicator_SetValue(wxAppProgressIndicator * self, int value);
void wxAppProgressIndicator_SetRange(wxAppProgressIndicator * self, int range);
bool wxAppProgressIndicator_Pulse(wxAppProgressIndicator * self);

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
wxIconBundle *wxArtProvider_GetIconBundle(const wxArtID * id, const wxArtClient * client);
bool wxArtProvider_HasNativeProvider();
bool wxArtProvider_Pop();
void wxArtProvider_Push(wxArtProvider * provider);
void wxArtProvider_PushBack(wxArtProvider * provider);
bool wxArtProvider_Remove(wxArtProvider * provider);
wxArtID *wxArtProvider_GetMessageBoxIconId(int flags);
wxIcon *wxArtProvider_GetMessageBoxIcon(int flags);

// CLASS: wxAutoBufferedPaintDC
wxClassInfo *wxAutoBufferedPaintDC_CLASSINFO();
wxAutoBufferedPaintDC *wxAutoBufferedPaintDC_new(wxWindow * window);

// CLASS: wxAutomationObject
wxClassInfo *wxAutomationObject_CLASSINFO();
wxAutomationObject *wxAutomationObject_new(WXIDISPATCH * dispatch_ptr);
bool wxAutomationObject_CreateInstance(const wxAutomationObject * self, const wxString * prog_id);
bool wxAutomationObject_IsOk(const wxAutomationObject * self);
WXIDISPATCH * wxAutomationObject_GetDispatchPtr(const wxAutomationObject * self);
bool wxAutomationObject_GetInstance(const wxAutomationObject * self, const wxString * prog_id, int flags);
void wxAutomationObject_SetDispatchPtr(wxAutomationObject * self, WXIDISPATCH * dispatch_ptr);
long wxAutomationObject_GetConvertVariantFlags(const wxAutomationObject * self);
void wxAutomationObject_SetConvertVariantFlags(wxAutomationObject * self, long flags);

// CLASS: wxBannerWindow
wxClassInfo *wxBannerWindow_CLASSINFO();
wxBannerWindow *wxBannerWindow_new();
wxBannerWindow *wxBannerWindow_new1(wxWindow * parent, wxDirection dir);
wxBannerWindow *wxBannerWindow_new2(wxWindow * parent, wxWindowID winid, wxDirection dir, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxBannerWindow_Create(wxBannerWindow * self, wxWindow * parent, wxWindowID winid, wxDirection dir, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxBannerWindow_SetBitmap(wxBannerWindow * self, const wxBitmapBundle * bmp);
void wxBannerWindow_SetText(wxBannerWindow * self, const wxString * title, const wxString * message);
void wxBannerWindow_SetGradient(wxBannerWindow * self, const wxColour * start, const wxColour * end);
// Mix-in(s) to wxBannerWindow
wxTrackable *wxBannerWindow_AsTrackable(wxBannerWindow* obj);

// CLASS: wxBitmap
wxClassInfo *wxBitmap_CLASSINFO();
wxBitmap *wxBitmap_new();
wxBitmap *wxBitmap_new1(const wxBitmap * bitmap);
wxBitmap *wxBitmap_new3(int width, int height, int depth);
wxBitmap *wxBitmap_new4(const wxSize * sz, int depth);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new5(int width, int height, const wxDC * dc);
#endif
wxBitmap *wxBitmap_new6(const char *const * bits);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new8(const wxImage * img, int depth);
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxBitmap *wxBitmap_new9(const wxImage * img, const wxDC * dc);
#endif
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new10(const wxCursor * cursor);
#endif
wxImage *wxBitmap_ConvertToImage(const wxBitmap * self);
bool wxBitmap_CopyFromIcon(wxBitmap * self, const wxIcon * icon);
bool wxBitmap_Create(wxBitmap * self, int width, int height, int depth);
bool wxBitmap_Create1(wxBitmap * self, const wxSize * sz, int depth);
bool wxBitmap_Create2(wxBitmap * self, int width, int height, const wxDC * dc);
#if wxCHECK_VERSION(3, 1, 0)
bool wxBitmap_CreateWithDIPSize(wxBitmap * self, const wxSize * size, double scale, int depth);
bool wxBitmap_CreateWithDIPSize1(wxBitmap * self, int width, int height, double scale, int depth);
#endif
bool wxBitmap_CreateScaled(wxBitmap * self, int width, int height, int depth, double logical_scale);
int wxBitmap_GetDepth(const wxBitmap * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxBitmap_GetDIPSize(const wxBitmap * self);
#endif
int wxBitmap_GetHeight(const wxBitmap * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxBitmap_GetLogicalHeight(const wxBitmap * self);
wxSize *wxBitmap_GetLogicalSize(const wxBitmap * self);
double wxBitmap_GetLogicalWidth(const wxBitmap * self);
#endif
wxMask * wxBitmap_GetMask(const wxBitmap * self);
wxPalette * wxBitmap_GetPalette(const wxBitmap * self);
wxBitmap *wxBitmap_GetSubBitmap(const wxBitmap * self, const wxRect * rect);
double wxBitmap_GetScaleFactor(const wxBitmap * self);
double wxBitmap_GetScaledHeight(const wxBitmap * self);
wxSize *wxBitmap_GetScaledSize(const wxBitmap * self);
double wxBitmap_GetScaledWidth(const wxBitmap * self);
wxSize *wxBitmap_GetSize(const wxBitmap * self);
int wxBitmap_GetWidth(const wxBitmap * self);
bool wxBitmap_HasAlpha(const wxBitmap * self);
bool wxBitmap_IsOk(const wxBitmap * self);
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmap_SetDepth(wxBitmap * self, int depth);
void wxBitmap_SetHeight(wxBitmap * self, int height);
#endif
#if wxCHECK_VERSION(3, 1, 0)
void wxBitmap_SetScaleFactor(wxBitmap * self, double scale);
#endif
void wxBitmap_SetMask(wxBitmap * self, wxMask * mask);
void wxBitmap_SetPalette(wxBitmap * self, const wxPalette * palette);
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmap_SetWidth(wxBitmap * self, int width);
#endif
void wxBitmap_AddHandler(wxBitmapHandler * handler);
void wxBitmap_CleanUpHandlers();
#ifndef __WXMSW__
wxBitmapHandler * wxBitmap_FindHandler(const wxString * name);
#endif
void wxBitmap_InitStandardHandlers();
void wxBitmap_InsertHandler(wxBitmapHandler * handler);
wxBitmap *wxBitmap_NewFromPNGData(const void * data, size_t size);
bool wxBitmap_RemoveHandler(const wxString * name);
#if wxCHECK_VERSION(3, 1, 0)
void wxBitmap_Rescale(wxBitmap * bmp, const wxSize * size_needed);
#endif

// CLASS: wxBitmapBundle
void wxBitmapBundle_delete(wxBitmapBundle *self);
wxBitmapBundle *wxBitmapBundle_new();
wxBitmapBundle *wxBitmapBundle_new1(const wxBitmap * bitmap);
wxBitmapBundle *wxBitmapBundle_new2(const wxIcon * icon);
wxBitmapBundle *wxBitmapBundle_new3(const wxImage * image);
#if wxCHECK_VERSION(3, 2, 0)
wxBitmapBundle *wxBitmapBundle_new4(const char *const * xpm);
#endif
wxBitmapBundle *wxBitmapBundle_new5(const wxBitmapBundle * other);
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmapBundle_Clear(wxBitmapBundle * self);
#endif
bool wxBitmapBundle_IsOk(const wxBitmapBundle * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxBitmapBundle_GetDefaultSize(const wxBitmapBundle * self);
wxSize *wxBitmapBundle_GetPreferredBitmapSizeAtScale(const wxBitmapBundle * self, double scale);
wxSize *wxBitmapBundle_GetPreferredBitmapSizeFor(const wxBitmapBundle * self, const wxWindow * window);
wxSize *wxBitmapBundle_GetPreferredLogicalSizeFor(const wxBitmapBundle * self, const wxWindow * window);
wxBitmap *wxBitmapBundle_GetBitmap(const wxBitmapBundle * self, const wxSize * size);
wxBitmap *wxBitmapBundle_GetBitmapFor(const wxBitmapBundle * self, const wxWindow * window);
wxIcon *wxBitmapBundle_GetIcon(const wxBitmapBundle * self, const wxSize * size);
wxIcon *wxBitmapBundle_GetIconFor(const wxBitmapBundle * self, const wxWindow * window);
#endif
bool wxBitmapBundle_IsSameAs(const wxBitmapBundle * self, const wxBitmapBundle * other);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxBitmapBundle_FromBitmaps1(const wxBitmap * bitmap1, const wxBitmap * bitmap2);
wxBitmapBundle *wxBitmapBundle_FromBitmap(const wxBitmap * bitmap);
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxBitmapBundle *wxBitmapBundle_FromIconBundle(const wxIconBundle * icon_bundle);
#endif
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxBitmapBundle_FromImage(const wxImage * image);
wxBitmapBundle *wxBitmapBundle_FromImpl(wxBitmapBundleImpl * impl_);
wxBitmapBundle *wxBitmapBundle_FromResources(const wxString * name);
wxBitmapBundle *wxBitmapBundle_FromFiles(const wxString * path, const wxString * filename, const wxString * extension);
wxBitmapBundle *wxBitmapBundle_FromFiles1(const wxString * fullpathname);
wxBitmapBundle *wxBitmapBundle_FromSVG1(const char * data, const wxSize * size_def);
wxBitmapBundle *wxBitmapBundle_FromSVGFile(const wxString * path, const wxSize * size_def);
wxBitmapBundle *wxBitmapBundle_FromSVGResource(const wxString * name, const wxSize * size_def);
#endif

// CLASS: wxBitmapBundleImpl
void wxBitmapBundleImpl_delete(wxBitmapBundleImpl *self);
wxSize *wxBitmapBundleImpl_GetDefaultSize(const wxBitmapBundleImpl * self);
wxSize *wxBitmapBundleImpl_GetPreferredBitmapSizeAtScale(const wxBitmapBundleImpl * self, double scale);
wxBitmap *wxBitmapBundleImpl_GetBitmap(wxBitmapBundleImpl * self, const wxSize * size);

// CLASS: wxBitmapButton
wxClassInfo *wxBitmapButton_CLASSINFO();
wxBitmapButton *wxBitmapButton_new();
wxBitmapButton *wxBitmapButton_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * bitmap, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxBitmapButton_Create(wxBitmapButton * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * bitmap, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
#if wxCHECK_VERSION(3, 1, 0)
bool wxBitmapButton_CreateCloseButton(wxBitmapButton * self, wxWindow * parent, wxWindowID winid, const wxString * name);
wxBitmapButton * wxBitmapButton_NewCloseButton(wxWindow * parent, wxWindowID winid, const wxString * name);
#endif
// Mix-in(s) to wxBitmapButton
wxTrackable *wxBitmapButton_AsTrackable(wxBitmapButton* obj);

// CLASS: wxBitmapComboBox
wxClassInfo *wxBitmapComboBox_CLASSINFO();
wxBitmapComboBox *wxBitmapComboBox_new();
wxBitmapComboBox *wxBitmapComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
int wxBitmapComboBox_Append(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap);
int wxBitmapComboBox_Append1(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, void * client_data);
int wxBitmapComboBox_Append2(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, wxClientData * client_data);
bool wxBitmapComboBox_Create1(wxBitmapComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
wxSize *wxBitmapComboBox_GetBitmapSize(const wxBitmapComboBox * self);
wxBitmap *wxBitmapComboBox_GetItemBitmap(const wxBitmapComboBox * self, unsigned int n);
int wxBitmapComboBox_Insert(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, unsigned int pos);
int wxBitmapComboBox_Insert1(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, unsigned int pos, void * client_data);
int wxBitmapComboBox_Insert2(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, unsigned int pos, wxClientData * client_data);
void wxBitmapComboBox_SetItemBitmap(wxBitmapComboBox * self, unsigned int n, const wxBitmapBundle * bitmap);
// Mix-in(s) to wxBitmapComboBox
wxItemContainer *wxBitmapComboBox_AsItemContainer(wxBitmapComboBox* obj);
wxTextEntryBase *wxBitmapComboBox_AsTextEntry(wxBitmapComboBox* obj);
wxTrackable *wxBitmapComboBox_AsTrackable(wxBitmapComboBox* obj);

// CLASS: wxBitmapDataObject
void wxBitmapDataObject_delete(wxBitmapDataObject *self);
wxBitmapDataObject *wxBitmapDataObject_new(const wxBitmap * bitmap);
wxBitmap *wxBitmapDataObject_GetBitmap(const wxBitmapDataObject * self);
void wxBitmapDataObject_SetBitmap(wxBitmapDataObject * self, const wxBitmap * bitmap);

// CLASS: wxBitmapHandler
wxClassInfo *wxBitmapHandler_CLASSINFO();
wxBitmapHandler *wxBitmapHandler_new();
wxString *wxBitmapHandler_GetExtension(const wxBitmapHandler * self);
wxString *wxBitmapHandler_GetName(const wxBitmapHandler * self);
void wxBitmapHandler_SetExtension(wxBitmapHandler * self, const wxString * extension);
void wxBitmapHandler_SetName(wxBitmapHandler * self, const wxString * name);

// CLASS: wxBitmapToggleButton
wxClassInfo *wxBitmapToggleButton_CLASSINFO();
wxBitmapToggleButton *wxBitmapToggleButton_new();
wxBitmapToggleButton *wxBitmapToggleButton_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name);
bool wxBitmapToggleButton_Create(wxBitmapToggleButton * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name);
// Mix-in(s) to wxBitmapToggleButton
wxTrackable *wxBitmapToggleButton_AsTrackable(wxBitmapToggleButton* obj);

// CLASS: wxBookCtrlBase
wxClassInfo *wxBookCtrlBase_CLASSINFO();
int wxBookCtrlBase_GetPageImage(const wxBookCtrlBase * self, size_t n_page);
bool wxBookCtrlBase_SetPageImage(wxBookCtrlBase * self, size_t page, int image);
wxString *wxBookCtrlBase_GetPageText(const wxBookCtrlBase * self, size_t n_page);
bool wxBookCtrlBase_SetPageText(wxBookCtrlBase * self, size_t page, const wxString * text);
int wxBookCtrlBase_GetSelection(const wxBookCtrlBase * self);
wxWindow * wxBookCtrlBase_GetCurrentPage(const wxBookCtrlBase * self);
int wxBookCtrlBase_SetSelection(wxBookCtrlBase * self, size_t page);
void wxBookCtrlBase_AdvanceSelection(wxBookCtrlBase * self, bool forward);
int wxBookCtrlBase_ChangeSelection(wxBookCtrlBase * self, size_t page);
int wxBookCtrlBase_FindPage(const wxBookCtrlBase * self, const wxWindow * page);
void wxBookCtrlBase_SetPageSize(wxBookCtrlBase * self, const wxSize * size);
int wxBookCtrlBase_HitTest(const wxBookCtrlBase * self, const wxPoint * pt, long * flags);
bool wxBookCtrlBase_AddPage(wxBookCtrlBase * self, wxWindow * page, const wxString * text, bool select, int image_id);
bool wxBookCtrlBase_DeleteAllPages(wxBookCtrlBase * self);
bool wxBookCtrlBase_DeletePage(wxBookCtrlBase * self, size_t page);
bool wxBookCtrlBase_InsertPage(wxBookCtrlBase * self, size_t index, wxWindow * page, const wxString * text, bool select, int image_id);
bool wxBookCtrlBase_RemovePage(wxBookCtrlBase * self, size_t page);
size_t wxBookCtrlBase_GetPageCount(const wxBookCtrlBase * self);
wxWindow * wxBookCtrlBase_GetPage(const wxBookCtrlBase * self, size_t page);
bool wxBookCtrlBase_Create(wxBookCtrlBase * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxBookCtrlBase
wxWithImages *wxBookCtrlBase_AsWithImages(wxBookCtrlBase* obj);
wxTrackable *wxBookCtrlBase_AsTrackable(wxBookCtrlBase* obj);

// CLASS: wxBookCtrlEvent
wxClassInfo *wxBookCtrlEvent_CLASSINFO();
int wxBookCtrlEvent_GetOldSelection(const wxBookCtrlEvent * self);
int wxBookCtrlEvent_GetSelection(const wxBookCtrlEvent * self);
void wxBookCtrlEvent_SetOldSelection(wxBookCtrlEvent * self, int page);
void wxBookCtrlEvent_SetSelection(wxBookCtrlEvent * self, int page);

// CLASS: wxBoxSizer
wxClassInfo *wxBoxSizer_CLASSINFO();
wxBoxSizer *wxBoxSizer_new(int orient);
int wxBoxSizer_GetOrientation(const wxBoxSizer * self);
void wxBoxSizer_SetOrientation(wxBoxSizer * self, int orient);

// CLASS: wxBrush
wxClassInfo *wxBrush_CLASSINFO();
wxBrush *wxBrush_new();
wxBrush *wxBrush_new2(const wxBitmap * stipple_bitmap);
wxBrush *wxBrush_new3(const wxBrush * brush);
wxColour *wxBrush_GetColour(const wxBrush * self);
wxBitmap * wxBrush_GetStipple(const wxBrush * self);
bool wxBrush_IsHatch(const wxBrush * self);
bool wxBrush_IsOk(const wxBrush * self);
bool wxBrush_IsNonTransparent(const wxBrush * self);
bool wxBrush_IsTransparent(const wxBrush * self);
void wxBrush_SetColour(wxBrush * self, const wxColour * colour);
void wxBrush_SetStipple(wxBrush * self, const wxBitmap * bitmap);

// CLASS: wxBrushList
void wxBrushList_delete(wxBrushList *self);

// CLASS: wxBufferedDC
wxClassInfo *wxBufferedDC_CLASSINFO();
wxBufferedDC *wxBufferedDC_new();
wxBufferedDC *wxBufferedDC_new1(wxDC * dc, const wxSize * area, int style);
wxBufferedDC *wxBufferedDC_new2(wxDC * dc, wxBitmap * buffer, int style);
void wxBufferedDC_Init(wxBufferedDC * self, wxDC * dc, const wxSize * area, int style);
void wxBufferedDC_Init1(wxBufferedDC * self, wxDC * dc, wxBitmap * buffer, int style);
void wxBufferedDC_UnMask(wxBufferedDC * self);
void wxBufferedDC_SetStyle(wxBufferedDC * self, int style);
int wxBufferedDC_GetStyle(const wxBufferedDC * self);

// CLASS: wxBufferedPaintDC
wxClassInfo *wxBufferedPaintDC_CLASSINFO();
wxBufferedPaintDC *wxBufferedPaintDC_new(wxWindow * window, wxBitmap * buffer, int style);
wxBufferedPaintDC *wxBufferedPaintDC_new1(wxWindow * window, int style);

// CLASS: wxBusyCursor
void wxBusyCursor_delete(wxBusyCursor *self);
wxBusyCursor *wxBusyCursor_new(const wxCursor * cursor);

// CLASS: wxBusyInfo
void wxBusyInfo_delete(wxBusyInfo *self);
wxBusyInfo *wxBusyInfo_new(const wxBusyInfoFlags * flags);
wxBusyInfo *wxBusyInfo_new1(const wxString * msg, wxWindow * parent);
void wxBusyInfo_UpdateText(wxBusyInfo * self, const wxString * str);
void wxBusyInfo_UpdateLabel(wxBusyInfo * self, const wxString * str);

// CLASS: wxButton
wxClassInfo *wxButton_CLASSINFO();
wxButton *wxButton_new();
wxButton *wxButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxButton_Create(wxButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxButton_GetAuthNeeded(const wxButton * self);
void wxButton_SetAuthNeeded(wxButton * self, bool needed);
wxWindow * wxButton_SetDefault(wxButton * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxButton_GetDefaultSize(wxWindow * win);
#endif
// Mix-in(s) to wxButton
wxTrackable *wxButton_AsTrackable(wxButton* obj);

// CLASS: wxCalculateLayoutEvent
wxClassInfo *wxCalculateLayoutEvent_CLASSINFO();
wxCalculateLayoutEvent *wxCalculateLayoutEvent_new(wxWindowID id);
int wxCalculateLayoutEvent_GetFlags(const wxCalculateLayoutEvent * self);
wxRect *wxCalculateLayoutEvent_GetRect(const wxCalculateLayoutEvent * self);
void wxCalculateLayoutEvent_SetFlags(wxCalculateLayoutEvent * self, int flags);
void wxCalculateLayoutEvent_SetRect(wxCalculateLayoutEvent * self, const wxRect * rect);

// CLASS: wxCalendarCtrl
wxClassInfo *wxCalendarCtrl_CLASSINFO();
bool wxCalendarCtrl_SetDateRange(wxCalendarCtrl * self, const wxDateTime * lowerdate, const wxDateTime * upperdate);
bool wxCalendarCtrl_GetDateRange(const wxCalendarCtrl * self, wxDateTime * lowerdate, wxDateTime * upperdate);
wxCalendarCtrl *wxCalendarCtrl_new();
wxCalendarCtrl *wxCalendarCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * date, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxCalendarCtrl_Create(wxCalendarCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * date, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxCalendarCtrl_EnableHolidayDisplay(wxCalendarCtrl * self, bool display);
bool wxCalendarCtrl_EnableMonthChange(wxCalendarCtrl * self, bool enable);
void wxCalendarCtrl_EnableYearChange(wxCalendarCtrl * self, bool enable);
wxCalendarDateAttr * wxCalendarCtrl_GetAttr(const wxCalendarCtrl * self, size_t day);
wxDateTime *wxCalendarCtrl_GetDate(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHeaderColourBg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHeaderColourFg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHighlightColourBg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHighlightColourFg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHolidayColourBg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHolidayColourFg(const wxCalendarCtrl * self);
void wxCalendarCtrl_ResetAttr(wxCalendarCtrl * self, size_t day);
void wxCalendarCtrl_SetAttr(wxCalendarCtrl * self, size_t day, wxCalendarDateAttr * attr);
bool wxCalendarCtrl_SetDate(wxCalendarCtrl * self, const wxDateTime * date);
void wxCalendarCtrl_SetHeaderColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg);
void wxCalendarCtrl_SetHighlightColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg);
void wxCalendarCtrl_SetHoliday(wxCalendarCtrl * self, size_t day);
void wxCalendarCtrl_SetHolidayColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg);
void wxCalendarCtrl_Mark(wxCalendarCtrl * self, size_t day, bool mark);
// Mix-in(s) to wxCalendarCtrl
wxTrackable *wxCalendarCtrl_AsTrackable(wxCalendarCtrl* obj);

// CLASS: wxCalendarDateAttr
void wxCalendarDateAttr_delete(wxCalendarDateAttr *self);
wxColour *wxCalendarDateAttr_GetBackgroundColour(const wxCalendarDateAttr * self);
wxColour *wxCalendarDateAttr_GetBorderColour(const wxCalendarDateAttr * self);
wxFont *wxCalendarDateAttr_GetFont(const wxCalendarDateAttr * self);
wxColour *wxCalendarDateAttr_GetTextColour(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasBackgroundColour(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasBorder(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasBorderColour(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasFont(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasTextColour(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_IsHoliday(const wxCalendarDateAttr * self);
void wxCalendarDateAttr_SetBackgroundColour(wxCalendarDateAttr * self, const wxColour * col_back);
void wxCalendarDateAttr_SetBorderColour(wxCalendarDateAttr * self, const wxColour * col);
void wxCalendarDateAttr_SetFont(wxCalendarDateAttr * self, const wxFont * font);
void wxCalendarDateAttr_SetHoliday(wxCalendarDateAttr * self, bool holiday);
void wxCalendarDateAttr_SetTextColour(wxCalendarDateAttr * self, const wxColour * col_text);
wxCalendarDateAttr *wxCalendarDateAttr_GetMark();
void wxCalendarDateAttr_SetMark(const wxCalendarDateAttr * m);

// CLASS: wxCalendarEvent
wxClassInfo *wxCalendarEvent_CLASSINFO();
wxCalendarEvent *wxCalendarEvent_new();

// CLASS: wxCaret
void wxCaret_delete(wxCaret *self);
wxCaret *wxCaret_new();
wxCaret *wxCaret_new1(wxWindow * window, int width, int height);
wxCaret *wxCaret_new2(wxWindow * window, const wxSize * size);
bool wxCaret_Create(wxCaret * self, wxWindow * window, int width, int height);
bool wxCaret_Create1(wxCaret * self, wxWindow * window, const wxSize * size);
void wxCaret_GetPosition(const wxCaret * self, int * x, int * y);
wxPoint *wxCaret_GetPosition1(const wxCaret * self);
void wxCaret_GetSize(const wxCaret * self, int * width, int * height);
wxSize *wxCaret_GetSize1(const wxCaret * self);
wxWindow * wxCaret_GetWindow(const wxCaret * self);
void wxCaret_Hide(wxCaret * self);
bool wxCaret_IsOk(const wxCaret * self);
bool wxCaret_IsVisible(const wxCaret * self);
void wxCaret_Move(wxCaret * self, int x, int y);
void wxCaret_Move1(wxCaret * self, const wxPoint * pt);
void wxCaret_SetSize(wxCaret * self, int width, int height);
void wxCaret_SetSize1(wxCaret * self, const wxSize * size);
void wxCaret_Show(wxCaret * self, bool show);
int wxCaret_GetBlinkTime();
void wxCaret_SetBlinkTime(int milliseconds);

// CLASS: wxCheckBox
wxClassInfo *wxCheckBox_CLASSINFO();
wxCheckBox *wxCheckBox_new();
wxCheckBox *wxCheckBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCheckBox_Create(wxCheckBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCheckBox_GetValue(const wxCheckBox * self);
wxCheckBoxState wxCheckBox_Get3StateValue(const wxCheckBox * self);
bool wxCheckBox_Is3State(const wxCheckBox * self);
bool wxCheckBox_Is3rdStateAllowedForUser(const wxCheckBox * self);
bool wxCheckBox_IsChecked(const wxCheckBox * self);
void wxCheckBox_SetValue(wxCheckBox * self, bool state);
void wxCheckBox_Set3StateValue(wxCheckBox * self, wxCheckBoxState state);
// Mix-in(s) to wxCheckBox
wxTrackable *wxCheckBox_AsTrackable(wxCheckBox* obj);

// CLASS: wxCheckListBox
wxClassInfo *wxCheckListBox_CLASSINFO();
wxCheckListBox *wxCheckListBox_new();
wxCheckListBox *wxCheckListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxCheckListBox_Create1(wxCheckListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
void wxCheckListBox_Check(wxCheckListBox * self, unsigned int item, bool check);
bool wxCheckListBox_IsChecked(const wxCheckListBox * self, unsigned int item);
unsigned int wxCheckListBox_GetCheckedItems(const wxCheckListBox * self, wxArrayInt * checked_items);
// Mix-in(s) to wxCheckListBox
wxItemContainer *wxCheckListBox_AsItemContainer(wxCheckListBox* obj);
wxTrackable *wxCheckListBox_AsTrackable(wxCheckListBox* obj);

// CLASS: wxChildFocusEvent
wxClassInfo *wxChildFocusEvent_CLASSINFO();
wxChildFocusEvent *wxChildFocusEvent_new(wxWindow * win);
wxWindow * wxChildFocusEvent_GetWindow(const wxChildFocusEvent * self);

// CLASS: wxChoice
wxClassInfo *wxChoice_CLASSINFO();
wxChoice *wxChoice_new();
wxChoice *wxChoice_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxChoice_Create1(wxChoice * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
int wxChoice_GetColumns(const wxChoice * self);
int wxChoice_GetCurrentSelection(const wxChoice * self);
void wxChoice_SetColumns(wxChoice * self, int n);
bool wxChoice_IsSorted(const wxChoice * self);
// Mix-in(s) to wxChoice
wxItemContainer *wxChoice_AsItemContainer(wxChoice* obj);
wxTrackable *wxChoice_AsTrackable(wxChoice* obj);

// CLASS: wxChoicebook
wxClassInfo *wxChoicebook_CLASSINFO();
wxChoicebook *wxChoicebook_new();
wxChoicebook *wxChoicebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxChoicebook_Create(wxChoicebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxChoice * wxChoicebook_GetChoiceCtrl(const wxChoicebook * self);
// Mix-in(s) to wxChoicebook
wxWithImages *wxChoicebook_AsWithImages(wxChoicebook* obj);
wxTrackable *wxChoicebook_AsTrackable(wxChoicebook* obj);

// CLASS: wxClientDC
wxClassInfo *wxClientDC_CLASSINFO();
wxClientDC *wxClientDC_new(wxWindow * window);

// CLASS: wxClipboard
wxClassInfo *wxClipboard_CLASSINFO();
wxClipboard *wxClipboard_new();
bool wxClipboard_AddData(wxClipboard * self, wxDataObject * data);
void wxClipboard_Clear(wxClipboard * self);
void wxClipboard_Close(wxClipboard * self);
bool wxClipboard_Flush(wxClipboard * self);
bool wxClipboard_GetData(wxClipboard * self, wxDataObject * data);
bool wxClipboard_IsOpened(const wxClipboard * self);
bool wxClipboard_IsSupported(wxClipboard * self, const wxDataFormat * format);
bool wxClipboard_IsUsingPrimarySelection(const wxClipboard * self);
bool wxClipboard_Open(wxClipboard * self);
bool wxClipboard_SetData(wxClipboard * self, wxDataObject * data);
void wxClipboard_UsePrimarySelection(wxClipboard * self, bool primary);
wxClipboard * wxClipboard_Get();

// CLASS: wxClipboardTextEvent
wxClassInfo *wxClipboardTextEvent_CLASSINFO();

// CLASS: wxCloseEvent
wxClassInfo *wxCloseEvent_CLASSINFO();
bool wxCloseEvent_CanVeto(const wxCloseEvent * self);
bool wxCloseEvent_GetLoggingOff(const wxCloseEvent * self);
void wxCloseEvent_SetCanVeto(wxCloseEvent * self, bool can_veto);
void wxCloseEvent_SetLoggingOff(wxCloseEvent * self, bool logging_off);
void wxCloseEvent_Veto(wxCloseEvent * self, bool veto);
bool wxCloseEvent_GetVeto(const wxCloseEvent * self);

// CLASS: wxCollapsibleHeaderCtrl
wxClassInfo *wxCollapsibleHeaderCtrl_CLASSINFO();
wxCollapsibleHeaderCtrl *wxCollapsibleHeaderCtrl_new();
wxCollapsibleHeaderCtrl *wxCollapsibleHeaderCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCollapsibleHeaderCtrl_Create(wxCollapsibleHeaderCtrl * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxCollapsibleHeaderCtrl_SetCollapsed(wxCollapsibleHeaderCtrl * self, bool collapsed);
bool wxCollapsibleHeaderCtrl_IsCollapsed(const wxCollapsibleHeaderCtrl * self);
// Mix-in(s) to wxCollapsibleHeaderCtrl
wxTrackable *wxCollapsibleHeaderCtrl_AsTrackable(wxCollapsibleHeaderCtrl* obj);

// CLASS: wxCollapsiblePane
wxClassInfo *wxCollapsiblePane_CLASSINFO();
wxCollapsiblePane *wxCollapsiblePane_new();
wxCollapsiblePane *wxCollapsiblePane_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCollapsiblePane_Create(wxCollapsiblePane * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxCollapsiblePane_Collapse(wxCollapsiblePane * self, bool collapse);
void wxCollapsiblePane_Expand(wxCollapsiblePane * self);
wxWindow * wxCollapsiblePane_GetPane(const wxCollapsiblePane * self);
bool wxCollapsiblePane_IsCollapsed(const wxCollapsiblePane * self);
bool wxCollapsiblePane_IsExpanded(const wxCollapsiblePane * self);
// Mix-in(s) to wxCollapsiblePane
wxTrackable *wxCollapsiblePane_AsTrackable(wxCollapsiblePane* obj);

// CLASS: wxCollapsiblePaneEvent
wxClassInfo *wxCollapsiblePaneEvent_CLASSINFO();
wxCollapsiblePaneEvent *wxCollapsiblePaneEvent_new(wxObject * generator, int id, bool collapsed);
bool wxCollapsiblePaneEvent_GetCollapsed(const wxCollapsiblePaneEvent * self);
void wxCollapsiblePaneEvent_SetCollapsed(wxCollapsiblePaneEvent * self, bool collapsed);

// CLASS: wxColour
wxClassInfo *wxColour_CLASSINFO();
wxColour *wxColour_new();
wxColour *wxColour_new2(const wxString * colour_name);
wxColour *wxColour_new4(const wxColour * colour);
#if wxCHECK_VERSION(3, 1, 0)
unsigned int wxColour_GetAlpha(const wxColour * self);
unsigned int wxColour_GetBlue(const wxColour * self);
unsigned int wxColour_GetGreen(const wxColour * self);
unsigned int wxColour_GetRed(const wxColour * self);
#endif
wxString *wxColour_GetAsString(const wxColour * self, long flags);
#if wxCHECK_VERSION(3, 1, 0)
double wxColour_GetLuminance(const wxColour * self);
#endif
bool wxColour_IsOk(const wxColour * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxColour_IsSolid(const wxColour * self);
#endif
bool wxColour_Set2(wxColour * self, const wxString * str);
void wxColour_MakeMono(unsigned char * r, unsigned char * g, unsigned char * b, bool on);
void wxColour_MakeGrey(unsigned char * r, unsigned char * g, unsigned char * b);
void wxColour_MakeGrey1(unsigned char * r, unsigned char * g, unsigned char * b, double weight_r, double weight_g, double weight_b);
void wxColour_ChangeLightness1(unsigned char * r, unsigned char * g, unsigned char * b, int ialpha);

// CLASS: wxColourData
wxClassInfo *wxColourData_CLASSINFO();
wxColourData *wxColourData_new();
bool wxColourData_GetChooseFull(const wxColourData * self);
bool wxColourData_GetChooseAlpha(const wxColourData * self);
wxColour * wxColourData_GetColour(wxColourData * self);
wxColour *wxColourData_GetCustomColour(const wxColourData * self, int i);
void wxColourData_SetChooseFull(wxColourData * self, bool flag);
void wxColourData_SetChooseAlpha(wxColourData * self, bool flag);
void wxColourData_SetColour(wxColourData * self, const wxColour * colour);
void wxColourData_SetCustomColour(wxColourData * self, int i, const wxColour * colour);
wxString *wxColourData_ToString(const wxColourData * self);
bool wxColourData_FromString(wxColourData * self, const wxString * str);

// CLASS: wxColourDatabase
void wxColourDatabase_delete(wxColourDatabase *self);
wxColourDatabase *wxColourDatabase_new();
void wxColourDatabase_AddColour(wxColourDatabase * self, const wxString * colour_name, const wxColour * colour);
wxColour *wxColourDatabase_Find(const wxColourDatabase * self, const wxString * colour_name);
wxString *wxColourDatabase_FindName(const wxColourDatabase * self, const wxColour * colour);

// CLASS: wxColourDialog
wxClassInfo *wxColourDialog_CLASSINFO();
wxColourDialog *wxColourDialog_new(wxWindow * parent, const wxColourData * data);
bool wxColourDialog_Create(wxColourDialog * self, wxWindow * parent, const wxColourData * data);
wxColourData * wxColourDialog_GetColourData(wxColourDialog * self);
// Mix-in(s) to wxColourDialog
wxTrackable *wxColourDialog_AsTrackable(wxColourDialog* obj);

// CLASS: wxColourDialogEvent
wxClassInfo *wxColourDialogEvent_CLASSINFO();
wxColourDialogEvent *wxColourDialogEvent_new();
wxColour *wxColourDialogEvent_GetColour(const wxColourDialogEvent * self);
void wxColourDialogEvent_SetColour(wxColourDialogEvent * self, const wxColour * colour);

// CLASS: wxColourPickerCtrl
wxClassInfo *wxColourPickerCtrl_CLASSINFO();
wxColourPickerCtrl *wxColourPickerCtrl_new();
wxColourPickerCtrl *wxColourPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxColourPickerCtrl_Create(wxColourPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxColour *wxColourPickerCtrl_GetColour(const wxColourPickerCtrl * self);
void wxColourPickerCtrl_SetColour(wxColourPickerCtrl * self, const wxColour * col);
// Mix-in(s) to wxColourPickerCtrl
wxTrackable *wxColourPickerCtrl_AsTrackable(wxColourPickerCtrl* obj);

// CLASS: wxColourPickerEvent
wxClassInfo *wxColourPickerEvent_CLASSINFO();
wxColourPickerEvent *wxColourPickerEvent_new();
wxColourPickerEvent *wxColourPickerEvent_new1(wxObject * generator, int id, const wxColour * colour);
wxColour *wxColourPickerEvent_GetColour(const wxColourPickerEvent * self);
void wxColourPickerEvent_SetColour(wxColourPickerEvent * self, const wxColour * pos);

// CLASS: wxComboBox
wxClassInfo *wxComboBox_CLASSINFO();
wxComboBox *wxComboBox_new();
wxComboBox *wxComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxComboBox_Create1(wxComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
int wxComboBox_GetCurrentSelection(const wxComboBox * self);
bool wxComboBox_IsListEmpty(const wxComboBox * self);
bool wxComboBox_IsTextEmpty(const wxComboBox * self);
void wxComboBox_Popup(wxComboBox * self);
void wxComboBox_Dismiss(wxComboBox * self);
// Mix-in(s) to wxComboBox
wxItemContainer *wxComboBox_AsItemContainer(wxComboBox* obj);
wxTextEntryBase *wxComboBox_AsTextEntry(wxComboBox* obj);
wxTrackable *wxComboBox_AsTrackable(wxComboBox* obj);

// CLASS: wxComboCtrl
wxClassInfo *wxComboCtrl_CLASSINFO();
wxComboCtrl *wxComboCtrl_new();
wxComboCtrl *wxComboCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxComboCtrl_Create(wxComboCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxComboCtrl_Dismiss(wxComboCtrl * self);
void wxComboCtrl_EnablePopupAnimation(wxComboCtrl * self, bool enable);
bool wxComboCtrl_IsKeyPopupToggle(const wxComboCtrl * self, const wxKeyEvent * event);
void wxComboCtrl_PrepareBackground(const wxComboCtrl * self, wxDC * dc, const wxRect * rect, int flags);
bool wxComboCtrl_ShouldDrawFocus(const wxComboCtrl * self);
wxBitmap *wxComboCtrl_GetBitmapDisabled(const wxComboCtrl * self);
wxBitmap *wxComboCtrl_GetBitmapHover(const wxComboCtrl * self);
wxBitmap *wxComboCtrl_GetBitmapNormal(const wxComboCtrl * self);
wxBitmap *wxComboCtrl_GetBitmapPressed(const wxComboCtrl * self);
wxSize *wxComboCtrl_GetButtonSize(wxComboCtrl * self);
int wxComboCtrl_GetCustomPaintWidth(const wxComboCtrl * self);
wxPoint *wxComboCtrl_GetMargins(const wxComboCtrl * self);
wxComboPopup * wxComboCtrl_GetPopupControl(wxComboCtrl * self);
wxWindow * wxComboCtrl_GetPopupWindow(const wxComboCtrl * self);
wxTextCtrl * wxComboCtrl_GetTextCtrl(const wxComboCtrl * self);
wxCoord wxComboCtrl_GetTextIndent(const wxComboCtrl * self);
wxRect *wxComboCtrl_GetTextRect(const wxComboCtrl * self);
void wxComboCtrl_HidePopup(wxComboCtrl * self, bool generate_event);
bool wxComboCtrl_IsPopupShown(const wxComboCtrl * self);
bool wxComboCtrl_IsPopupWindowState(const wxComboCtrl * self, int state);
void wxComboCtrl_OnButtonClick(wxComboCtrl * self);
void wxComboCtrl_Popup(wxComboCtrl * self);
void wxComboCtrl_SetButtonBitmaps(wxComboCtrl * self, const wxBitmapBundle * bmp_normal, bool push_button_bg, const wxBitmapBundle * bmp_pressed, const wxBitmapBundle * bmp_hover, const wxBitmapBundle * bmp_disabled);
void wxComboCtrl_SetButtonPosition(wxComboCtrl * self, int width, int height, int side, int spacing_x);
void wxComboCtrl_SetCustomPaintWidth(wxComboCtrl * self, int width);
void wxComboCtrl_SetMainControl(wxComboCtrl * self, wxWindow * win);
bool wxComboCtrl_SetMargins(wxComboCtrl * self, const wxPoint * pt);
bool wxComboCtrl_SetMargins1(wxComboCtrl * self, wxCoord left, wxCoord top);
void wxComboCtrl_SetPopupAnchor(wxComboCtrl * self, int anchor_side);
void wxComboCtrl_SetPopupControl(wxComboCtrl * self, wxComboPopup * popup);
void wxComboCtrl_SetPopupExtents(wxComboCtrl * self, int ext_left, int ext_right);
void wxComboCtrl_SetPopupMaxHeight(wxComboCtrl * self, int height);
void wxComboCtrl_SetPopupMinWidth(wxComboCtrl * self, int width);
void wxComboCtrl_SetText(wxComboCtrl * self, const wxString * value);
void wxComboCtrl_SetTextCtrlStyle(wxComboCtrl * self, int style);
void wxComboCtrl_SetTextIndent(wxComboCtrl * self, int indent);
void wxComboCtrl_SetValueByUser(wxComboCtrl * self, const wxString * value);
void wxComboCtrl_ShowPopup(wxComboCtrl * self);
void wxComboCtrl_UseAltPopupWindow(wxComboCtrl * self, bool enable);
int wxComboCtrl_GetFeatures();
// Mix-in(s) to wxComboCtrl
wxTextEntryBase *wxComboCtrl_AsTextEntry(wxComboCtrl* obj);
wxTrackable *wxComboCtrl_AsTrackable(wxComboCtrl* obj);

// CLASS: wxComboPopup
void wxComboPopup_delete(wxComboPopup *self);
wxComboPopup *wxComboPopup_new();
bool wxComboPopup_Create(wxComboPopup * self, wxWindow * parent);
void wxComboPopup_DestroyPopup(wxComboPopup * self);
void wxComboPopup_Dismiss(wxComboPopup * self);
bool wxComboPopup_FindItem(wxComboPopup * self, const wxString * item, wxString * true_item);
wxSize *wxComboPopup_GetAdjustedSize(wxComboPopup * self, int min_width, int pref_height, int max_height);
wxComboCtrl * wxComboPopup_GetComboCtrl(const wxComboPopup * self);
wxWindow * wxComboPopup_GetControl(wxComboPopup * self);
wxString *wxComboPopup_GetStringValue(const wxComboPopup * self);
void wxComboPopup_Init(wxComboPopup * self);
bool wxComboPopup_IsCreated(const wxComboPopup * self);
bool wxComboPopup_LazyCreate(wxComboPopup * self);
void wxComboPopup_OnComboDoubleClick(wxComboPopup * self);
void wxComboPopup_OnComboKeyEvent(wxComboPopup * self, wxKeyEvent * event);
void wxComboPopup_OnDismiss(wxComboPopup * self);
void wxComboPopup_OnPopup(wxComboPopup * self);
void wxComboPopup_PaintComboControl(wxComboPopup * self, wxDC * dc, const wxRect * rect);
void wxComboPopup_SetStringValue(wxComboPopup * self, const wxString * value);

// CLASS: wxCommand
wxClassInfo *wxCommand_CLASSINFO();
wxCommand *wxCommand_new(bool can_undo, const wxString * name);
bool wxCommand_CanUndo(const wxCommand * self);
bool wxCommand_Do(wxCommand * self);
wxString *wxCommand_GetName(const wxCommand * self);
bool wxCommand_Undo(wxCommand * self);

// CLASS: wxCommandEvent
wxClassInfo *wxCommandEvent_CLASSINFO();
void * wxCommandEvent_GetClientData(const wxCommandEvent * self);
wxClientData * wxCommandEvent_GetClientObject(const wxCommandEvent * self);
long wxCommandEvent_GetExtraLong(const wxCommandEvent * self);
int wxCommandEvent_GetInt(const wxCommandEvent * self);
int wxCommandEvent_GetSelection(const wxCommandEvent * self);
wxString *wxCommandEvent_GetString(const wxCommandEvent * self);
bool wxCommandEvent_IsChecked(const wxCommandEvent * self);
bool wxCommandEvent_IsSelection(const wxCommandEvent * self);
void wxCommandEvent_SetClientData(wxCommandEvent * self, void * client_data);
void wxCommandEvent_SetClientObject(wxCommandEvent * self, wxClientData * client_object);
void wxCommandEvent_SetExtraLong(wxCommandEvent * self, long extra_long);
void wxCommandEvent_SetInt(wxCommandEvent * self, int int_command);
void wxCommandEvent_SetString(wxCommandEvent * self, const wxString * string);

// CLASS: wxCommandLinkButton
wxClassInfo *wxCommandLinkButton_CLASSINFO();
wxCommandLinkButton *wxCommandLinkButton_new();
wxCommandLinkButton *wxCommandLinkButton_new1(wxWindow * parent, wxWindowID id, const wxString * main_label, const wxString * note, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCommandLinkButton_Create(wxCommandLinkButton * self, wxWindow * parent, wxWindowID id, const wxString * main_label, const wxString * note, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxCommandLinkButton_SetMainLabelAndNote(wxCommandLinkButton * self, const wxString * main_label, const wxString * note);
void wxCommandLinkButton_SetMainLabel(wxCommandLinkButton * self, const wxString * main_label);
void wxCommandLinkButton_SetNote(wxCommandLinkButton * self, const wxString * note);
wxString *wxCommandLinkButton_GetMainLabel(const wxCommandLinkButton * self);
wxString *wxCommandLinkButton_GetNote(const wxCommandLinkButton * self);
// Mix-in(s) to wxCommandLinkButton
wxTrackable *wxCommandLinkButton_AsTrackable(wxCommandLinkButton* obj);

// CLASS: wxCommandProcessor
wxClassInfo *wxCommandProcessor_CLASSINFO();
wxCommandProcessor *wxCommandProcessor_new(int max_commands);
bool wxCommandProcessor_CanUndo(const wxCommandProcessor * self);
bool wxCommandProcessor_CanRedo(const wxCommandProcessor * self);
void wxCommandProcessor_ClearCommands(wxCommandProcessor * self);
wxList * wxCommandProcessor_GetCommands(wxCommandProcessor * self);
wxCommand * wxCommandProcessor_GetCurrentCommand(const wxCommandProcessor * self);
wxMenu * wxCommandProcessor_GetEditMenu(const wxCommandProcessor * self);
int wxCommandProcessor_GetMaxCommands(const wxCommandProcessor * self);
wxString *wxCommandProcessor_GetRedoAccelerator(const wxCommandProcessor * self);
wxString *wxCommandProcessor_GetRedoMenuLabel(const wxCommandProcessor * self);
wxString *wxCommandProcessor_GetUndoAccelerator(const wxCommandProcessor * self);
wxString *wxCommandProcessor_GetUndoMenuLabel(const wxCommandProcessor * self);
void wxCommandProcessor_Initialize(wxCommandProcessor * self);
bool wxCommandProcessor_IsDirty(const wxCommandProcessor * self);
void wxCommandProcessor_MarkAsSaved(wxCommandProcessor * self);
bool wxCommandProcessor_Redo(wxCommandProcessor * self);
void wxCommandProcessor_SetEditMenu(wxCommandProcessor * self, wxMenu * menu);
void wxCommandProcessor_SetMenuStrings(wxCommandProcessor * self);
void wxCommandProcessor_SetRedoAccelerator(wxCommandProcessor * self, const wxString * accel);
void wxCommandProcessor_SetUndoAccelerator(wxCommandProcessor * self, const wxString * accel);
bool wxCommandProcessor_Submit(wxCommandProcessor * self, wxCommand * command, bool store_it);
void wxCommandProcessor_Store(wxCommandProcessor * self, wxCommand * command);
bool wxCommandProcessor_Undo(wxCommandProcessor * self);

// CLASS: wxContextHelp
wxClassInfo *wxContextHelp_CLASSINFO();
wxContextHelp *wxContextHelp_new(wxWindow * window, bool do_now);
bool wxContextHelp_BeginContextHelp(wxContextHelp * self, wxWindow * window);
bool wxContextHelp_EndContextHelp(wxContextHelp * self);

// CLASS: wxContextHelpButton
wxClassInfo *wxContextHelpButton_CLASSINFO();
wxContextHelpButton *wxContextHelpButton_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style);
// Mix-in(s) to wxContextHelpButton
wxTrackable *wxContextHelpButton_AsTrackable(wxContextHelpButton* obj);

// CLASS: wxContextMenuEvent
wxClassInfo *wxContextMenuEvent_CLASSINFO();
wxPoint *wxContextMenuEvent_GetPosition(const wxContextMenuEvent * self);
void wxContextMenuEvent_SetPosition(wxContextMenuEvent * self, const wxPoint * point);

// CLASS: wxControl
wxClassInfo *wxControl_CLASSINFO();
wxControl *wxControl_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxControl *wxControl_new1();
bool wxControl_Create(wxControl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxControl_Command(wxControl * self, wxCommandEvent * event);
wxString *wxControl_GetLabelText(const wxControl * self);
wxSize *wxControl_GetSizeFromTextSize(const wxControl * self, int xlen, int ylen);
wxSize *wxControl_GetSizeFromTextSize1(const wxControl * self, const wxSize * tsize);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxControl_GetSizeFromText(const wxControl * self, const wxString * text);
#endif
void wxControl_SetLabelText(wxControl * self, const wxString * text);
bool wxControl_SetLabelMarkup(wxControl * self, const wxString * markup);
wxString *wxControl_GetLabelText1(const wxString * label);
wxString *wxControl_RemoveMnemonics(const wxString * str);
wxString *wxControl_EscapeMnemonics(const wxString * text);
wxString *wxControl_Ellipsize(const wxString * label, const wxDC * dc, wxEllipsizeMode mode, int max_width, int flags);
// Mix-in(s) to wxControl
wxTrackable *wxControl_AsTrackable(wxControl* obj);

// CLASS: wxControlWithItems
wxClassInfo *wxControlWithItems_CLASSINFO();
// Mix-in(s) to wxControlWithItems
wxItemContainer *wxControlWithItems_AsItemContainer(wxControlWithItems* obj);
wxTrackable *wxControlWithItems_AsTrackable(wxControlWithItems* obj);

// CLASS: wxCredentialEntryDialog
wxClassInfo *wxCredentialEntryDialog_CLASSINFO();
wxCredentialEntryDialog *wxCredentialEntryDialog_new();
wxCredentialEntryDialog *wxCredentialEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * title, const wxWebCredentials * cred);
bool wxCredentialEntryDialog_Create(wxCredentialEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * title, const wxWebCredentials * cred);
wxWebCredentials *wxCredentialEntryDialog_GetCredentials(const wxCredentialEntryDialog * self);
void wxCredentialEntryDialog_SetUser(wxCredentialEntryDialog * self, const wxString * user);
void wxCredentialEntryDialog_SetPassword(wxCredentialEntryDialog * self, const wxString * password);
// Mix-in(s) to wxCredentialEntryDialog
wxTrackable *wxCredentialEntryDialog_AsTrackable(wxCredentialEntryDialog* obj);

// CLASS: wxCursor
wxClassInfo *wxCursor_CLASSINFO();
wxCursor *wxCursor_new();
wxCursor *wxCursor_new4(const wxImage * image);
wxCursor *wxCursor_new5(const char *const * xpm_data);
wxCursor *wxCursor_new6(const wxCursor * cursor);
bool wxCursor_IsOk(const wxCursor * self);
wxPoint *wxCursor_GetHotSpot(const wxCursor * self);

// CLASS: wxCustomDataObject
void wxCustomDataObject_delete(wxCustomDataObject *self);
wxCustomDataObject *wxCustomDataObject_new(const wxDataFormat * format);
void * wxCustomDataObject_Alloc(wxCustomDataObject * self, size_t size);
void wxCustomDataObject_Free(wxCustomDataObject * self);
void * wxCustomDataObject_GetData(const wxCustomDataObject * self);
size_t wxCustomDataObject_GetSize(const wxCustomDataObject * self);
void wxCustomDataObject_TakeData(wxCustomDataObject * self, size_t size, void * data);

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
wxPen *wxDC_GetPen(const wxDC * self);
void wxDC_SetBackground(wxDC * self, const wxBrush * brush);
void wxDC_SetBrush(wxDC * self, const wxBrush * brush);
void wxDC_SetPen(wxDC * self, const wxPen * pen);
void wxDC_CopyAttributes(wxDC * self, const wxDC * dc);
double wxDC_GetContentScaleFactor(const wxDC * self);
int wxDC_GetDepth(const wxDC * self);
wxPoint *wxDC_GetDeviceOrigin(const wxDC * self);
bool wxDC_GetPixel(const wxDC * self, wxCoord x, wxCoord y, wxColour * colour);
wxSize *wxDC_GetPPI(const wxDC * self);
wxSize *wxDC_FromDIP(const wxDC * self, const wxSize * sz);
wxPoint *wxDC_FromDIP1(const wxDC * self, const wxPoint * pt);
int wxDC_FromDIP2(const wxDC * self, int d);
wxSize *wxDC_ToDIP(const wxDC * self, const wxSize * sz);
wxPoint *wxDC_ToDIP1(const wxDC * self, const wxPoint * pt);
int wxDC_ToDIP2(const wxDC * self, int d);
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
wxSize *wxDPIChangedEvent_Scale(const wxDPIChangedEvent * self, wxSize sz);
int wxDPIChangedEvent_ScaleX(const wxDPIChangedEvent * self, int x);
int wxDPIChangedEvent_ScaleY(const wxDPIChangedEvent * self, int y);

// CLASS: wxDataObject
void wxDataObject_delete(wxDataObject *self);
wxDataObject *wxDataObject_new();
bool wxDataObject_GetDataHere(const wxDataObject * self, const wxDataFormat * format, void * buf);
size_t wxDataObject_GetDataSize(const wxDataObject * self, const wxDataFormat * format);
bool wxDataObject_SetData(wxDataObject * self, const wxDataFormat * format, size_t len, const void * buf);

// CLASS: wxDataObjectComposite
void wxDataObjectComposite_delete(wxDataObjectComposite *self);
wxDataObjectComposite *wxDataObjectComposite_new();
void wxDataObjectComposite_Add(wxDataObjectComposite * self, wxDataObjectSimple * data_object, bool preferred);

// CLASS: wxDataObjectSimple
void wxDataObjectSimple_delete(wxDataObjectSimple *self);
wxDataObjectSimple *wxDataObjectSimple_new(const wxDataFormat * format);
bool wxDataObjectSimple_GetDataHere(const wxDataObjectSimple * self, void * buf);
size_t wxDataObjectSimple_GetDataSize(const wxDataObjectSimple * self);
const wxDataFormat * wxDataObjectSimple_GetFormat(const wxDataObjectSimple * self);
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
wxVector< wxDataViewColumn * > wxDataViewCtrl_GetSortingColumns(const wxDataViewCtrl * self);
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
// Mix-in(s) to wxDataViewCtrl
wxTrackable *wxDataViewCtrl_AsTrackable(wxDataViewCtrl* obj);

// CLASS: wxDataViewCustomRenderer
wxClassInfo *wxDataViewCustomRenderer_CLASSINFO();
wxString *wxDataViewCustomRenderer_GetDefaultType();
bool wxDataViewCustomRenderer_ActivateCell(wxDataViewCustomRenderer * self, const wxRect * cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col, const wxMouseEvent * mouse_event);
wxDataViewItemAttr *wxDataViewCustomRenderer_GetAttr(const wxDataViewCustomRenderer * self);
wxSize *wxDataViewCustomRenderer_GetSize(const wxDataViewCustomRenderer * self);
bool wxDataViewCustomRenderer_LeftClick(wxDataViewCustomRenderer * self, wxPoint cursor, wxRect cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col);
bool wxDataViewCustomRenderer_Activate(wxDataViewCustomRenderer * self, wxRect cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col);
bool wxDataViewCustomRenderer_Render(wxDataViewCustomRenderer * self, wxRect cell, wxDC * dc, int state);
void wxDataViewCustomRenderer_RenderText(wxDataViewCustomRenderer * self, const wxString * text, int xoffset, wxRect cell, wxDC * dc, int state);
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
const wxVariant * wxDataViewEvent_GetValue(const wxDataViewEvent * self);
bool wxDataViewEvent_IsEditCancelled(const wxDataViewEvent * self);
void wxDataViewEvent_SetColumn(wxDataViewEvent * self, int col);
void wxDataViewEvent_SetDataViewColumn(wxDataViewEvent * self, wxDataViewColumn * col);
void wxDataViewEvent_SetModel(wxDataViewEvent * self, wxDataViewModel * model);
void wxDataViewEvent_SetValue(wxDataViewEvent * self, const wxVariant * value);
void wxDataViewEvent_SetDataObject(wxDataViewEvent * self, wxDataObject * obj);
size_t wxDataViewEvent_GetDataSize(const wxDataViewEvent * self);
void * wxDataViewEvent_GetDataBuffer(const wxDataViewEvent * self);
void wxDataViewEvent_SetDragFlags(wxDataViewEvent * self, int flags);
int wxDataViewEvent_GetCacheFrom(const wxDataViewEvent * self);
int wxDataViewEvent_GetCacheTo(const wxDataViewEvent * self);
int wxDataViewEvent_GetProposedDropIndex(const wxDataViewEvent * self);
wxDataViewItem *wxDataViewEvent_GetItem(const wxDataViewEvent * self);
void wxDataViewEvent_SetItem(wxDataViewEvent * self, const wxDataViewItem * item);
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
void wxDataViewIndexListModel_delete(wxDataViewIndexListModel *self);
wxDataViewIndexListModel *wxDataViewIndexListModel_new(unsigned int initial_size);
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
void wxDataViewListCtrl_AppendColumn(wxDataViewListCtrl * self, wxDataViewColumn * column, const wxString * varianttype);
void wxDataViewListCtrl_InsertColumn(wxDataViewListCtrl * self, unsigned int pos, wxDataViewColumn * column, const wxString * varianttype);
void wxDataViewListCtrl_PrependColumn(wxDataViewListCtrl * self, wxDataViewColumn * column, const wxString * varianttype);
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
wxDataViewListStore * wxDataViewListCtrl_GetStore(wxDataViewListCtrl * self);
const wxDataViewListStore * wxDataViewListCtrl_GetStore1(const wxDataViewListCtrl * self);
int wxDataViewListCtrl_ItemToRow(const wxDataViewListCtrl * self, const wxDataViewItem * item);
wxDataViewItem *wxDataViewListCtrl_RowToItem(const wxDataViewListCtrl * self, int row);
// Mix-in(s) to wxDataViewListCtrl
wxTrackable *wxDataViewListCtrl_AsTrackable(wxDataViewListCtrl* obj);

// CLASS: wxDataViewListModel
void wxDataViewListModel_delete(wxDataViewListModel *self);
bool wxDataViewListModel_GetAttrByRow(const wxDataViewListModel * self, unsigned int row, unsigned int col, wxDataViewItemAttr * attr);
bool wxDataViewListModel_IsEnabledByRow(const wxDataViewListModel * self, unsigned int row, unsigned int col);
unsigned int wxDataViewListModel_GetCount(const wxDataViewListModel * self);
unsigned int wxDataViewListModel_GetRow(const wxDataViewListModel * self, const wxDataViewItem * item);
void wxDataViewListModel_GetValueByRow(const wxDataViewListModel * self, wxVariant * variant, unsigned int row, unsigned int col);
bool wxDataViewListModel_SetValueByRow(wxDataViewListModel * self, const wxVariant * variant, unsigned int row, unsigned int col);

// CLASS: wxDataViewListStore
void wxDataViewListStore_delete(wxDataViewListStore *self);
wxDataViewListStore *wxDataViewListStore_new();
void wxDataViewListStore_PrependColumn(wxDataViewListStore * self, const wxString * varianttype);
void wxDataViewListStore_InsertColumn(wxDataViewListStore * self, unsigned int pos, const wxString * varianttype);
void wxDataViewListStore_AppendColumn(wxDataViewListStore * self, const wxString * varianttype);
void wxDataViewListStore_DeleteAllItems(wxDataViewListStore * self);
unsigned int wxDataViewListStore_GetItemCount(const wxDataViewListStore * self);

// CLASS: wxDataViewModel
void wxDataViewModel_delete(wxDataViewModel *self);
wxDataViewModel *wxDataViewModel_new();
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
wxDataViewModelNotifier *wxDataViewModelNotifier_new();
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
wxString *wxDataViewRenderer_GetAccessibleDescription(const wxDataViewRenderer * self);
int wxDataViewRenderer_GetAlignment(const wxDataViewRenderer * self);
wxEllipsizeMode wxDataViewRenderer_GetEllipsizeMode(const wxDataViewRenderer * self);
wxDataViewColumn * wxDataViewRenderer_GetOwner(const wxDataViewRenderer * self);
bool wxDataViewRenderer_GetValue(const wxDataViewRenderer * self, wxVariant * value);
wxString *wxDataViewRenderer_GetVariantType(const wxDataViewRenderer * self);
bool wxDataViewRenderer_IsCompatibleVariantType(const wxDataViewRenderer * self, const wxString * variant_type);
void wxDataViewRenderer_SetAlignment(wxDataViewRenderer * self, int align);
void wxDataViewRenderer_SetOwner(wxDataViewRenderer * self, wxDataViewColumn * owner);
bool wxDataViewRenderer_SetValue(wxDataViewRenderer * self, const wxVariant * value);
void wxDataViewRenderer_SetValueAdjuster(wxDataViewRenderer * self, wxDataViewValueAdjuster * transformer);
bool wxDataViewRenderer_Validate(wxDataViewRenderer * self, wxVariant * value);
bool wxDataViewRenderer_HasEditorCtrl(const wxDataViewRenderer * self);
wxWindow * wxDataViewRenderer_CreateEditorCtrl(wxDataViewRenderer * self, wxWindow * parent, wxRect label_rect, const wxVariant * value);
bool wxDataViewRenderer_GetValueFromEditorCtrl(wxDataViewRenderer * self, wxWindow * editor, wxVariant * value);
bool wxDataViewRenderer_StartEditing(wxDataViewRenderer * self, const wxDataViewItem * item, wxRect label_rect);
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
wxDataViewItem *wxDataViewTreeCtrl_GetItemParent(const wxDataViewTreeCtrl * self, wxDataViewItem item);
wxString *wxDataViewTreeCtrl_GetItemText(const wxDataViewTreeCtrl * self, const wxDataViewItem * item);
wxDataViewItem *wxDataViewTreeCtrl_GetNthChild(const wxDataViewTreeCtrl * self, const wxDataViewItem * parent, unsigned int pos);
wxDataViewTreeStore * wxDataViewTreeCtrl_GetStore(wxDataViewTreeCtrl * self);
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
// Mix-in(s) to wxDataViewTreeCtrl
wxTrackable *wxDataViewTreeCtrl_AsTrackable(wxDataViewTreeCtrl* obj);

// CLASS: wxDataViewTreeStore
void wxDataViewTreeStore_delete(wxDataViewTreeStore *self);
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
void wxDataViewVirtualListModel_delete(wxDataViewVirtualListModel *self);
wxDataViewVirtualListModel *wxDataViewVirtualListModel_new(unsigned int initial_size);
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
// Mix-in(s) to wxDatePickerCtrl
wxTrackable *wxDatePickerCtrl_AsTrackable(wxDatePickerCtrl* obj);

// CLASS: wxDelegateRendererNative
void wxDelegateRendererNative_delete(wxDelegateRendererNative *self);
wxDelegateRendererNative *wxDelegateRendererNative_new();
wxDelegateRendererNative *wxDelegateRendererNative_new1(wxRendererNative * renderer_native);

// CLASS: wxDialUpEvent
wxClassInfo *wxDialUpEvent_CLASSINFO();
wxDialUpEvent *wxDialUpEvent_new(bool is_connected, bool is_own_event);
bool wxDialUpEvent_IsConnectedEvent(const wxDialUpEvent * self);
bool wxDialUpEvent_IsOwnEvent(const wxDialUpEvent * self);

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
wxSizer * wxDialog_CreateTextSizer(wxDialog * self, const wxString * message, int width_max);
bool wxDialog_DoLayoutAdaptation(wxDialog * self);
void wxDialog_EndModal(wxDialog * self, int ret_code);
int wxDialog_GetAffirmativeId(const wxDialog * self);
wxWindow * wxDialog_GetContentWindow(const wxDialog * self);
int wxDialog_GetEscapeId(const wxDialog * self);
bool wxDialog_GetLayoutAdaptationDone(const wxDialog * self);
int wxDialog_GetLayoutAdaptationLevel(const wxDialog * self);
wxArrayInt * wxDialog_GetMainButtonIds(wxDialog * self);
int wxDialog_GetReturnCode(const wxDialog * self);
wxToolBar * wxDialog_GetToolBar(const wxDialog * self);
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
void wxDialog_ShowWindowModalThenDo(wxDialog * self, const Functor * on_end_modal);
void wxDialog_EnableLayoutAdaptation(bool enable);
wxDialogLayoutAdapter * wxDialog_GetLayoutAdapter();
bool wxDialog_IsLayoutAdaptationEnabled();
wxDialogLayoutAdapter * wxDialog_SetLayoutAdapter(wxDialogLayoutAdapter * adapter);
// Mix-in(s) to wxDialog
wxTrackable *wxDialog_AsTrackable(wxDialog* obj);

// CLASS: wxDialogLayoutAdapter
void wxDialogLayoutAdapter_delete(wxDialogLayoutAdapter *self);
wxDialogLayoutAdapter *wxDialogLayoutAdapter_new();
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
// Mix-in(s) to wxDirDialog
wxTrackable *wxDirDialog_AsTrackable(wxDirDialog* obj);

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
// Mix-in(s) to wxDirPickerCtrl
wxTrackable *wxDirPickerCtrl_AsTrackable(wxDirPickerCtrl* obj);

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

// CLASS: wxDocChildFrame
wxClassInfo *wxDocChildFrame_CLASSINFO();
wxDocChildFrame *wxDocChildFrame_new(wxDocument * doc, wxView * view, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxDocument * wxDocChildFrame_GetDocument(const wxDocChildFrame * self);
wxView * wxDocChildFrame_GetView(const wxDocChildFrame * self);
void wxDocChildFrame_SetDocument(wxDocChildFrame * self, wxDocument * doc);
void wxDocChildFrame_SetView(wxDocChildFrame * self, wxView * view);
// Mix-in(s) to wxDocChildFrame
wxTrackable *wxDocChildFrame_AsTrackable(wxDocChildFrame* obj);

// CLASS: wxDocMDIChildFrame
wxClassInfo *wxDocMDIChildFrame_CLASSINFO();
wxDocMDIChildFrame *wxDocMDIChildFrame_new(wxDocument * doc, wxView * view, wxMDIParentFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxDocument * wxDocMDIChildFrame_GetDocument(const wxDocMDIChildFrame * self);
wxView * wxDocMDIChildFrame_GetView(const wxDocMDIChildFrame * self);
void wxDocMDIChildFrame_SetDocument(wxDocMDIChildFrame * self, wxDocument * doc);
void wxDocMDIChildFrame_SetView(wxDocMDIChildFrame * self, wxView * view);
// Mix-in(s) to wxDocMDIChildFrame
wxTrackable *wxDocMDIChildFrame_AsTrackable(wxDocMDIChildFrame* obj);

// CLASS: wxDocMDIParentFrame
wxClassInfo *wxDocMDIParentFrame_CLASSINFO();
wxDocMDIParentFrame *wxDocMDIParentFrame_new();
wxDocMDIParentFrame *wxDocMDIParentFrame_new1(wxDocManager * manager, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxDocMDIParentFrame_Create(wxDocMDIParentFrame * self, wxDocManager * manager, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxDocMDIParentFrame
wxTrackable *wxDocMDIParentFrame_AsTrackable(wxDocMDIParentFrame* obj);

// CLASS: wxDocManager
wxClassInfo *wxDocManager_CLASSINFO();
wxDocManager *wxDocManager_new(long flags, bool initialize);
void wxDocManager_ActivateView(wxDocManager * self, wxView * doc, bool activate);
void wxDocManager_AddDocument(wxDocManager * self, wxDocument * doc);
void wxDocManager_AddFileToHistory(wxDocManager * self, const wxString * filename);
void wxDocManager_AssociateTemplate(wxDocManager * self, wxDocTemplate * temp);
wxDocTemplate * wxDocManager_FindTemplate(wxDocManager * self, const wxClassInfo * classinfo);
wxDocument * wxDocManager_FindDocumentByPath(const wxDocManager * self, const wxString * path);
bool wxDocManager_CloseDocument(wxDocManager * self, wxDocument * doc, bool force);
bool wxDocManager_CloseDocuments(wxDocManager * self, bool force);
wxDocument * wxDocManager_CreateDocument(wxDocManager * self, const wxString * path, long flags);
wxDocument * wxDocManager_CreateNewDocument(wxDocManager * self);
wxView * wxDocManager_CreateView(wxDocManager * self, wxDocument * doc, long flags);
void wxDocManager_DisassociateTemplate(wxDocManager * self, wxDocTemplate * temp);
void wxDocManager_FileHistoryAddFilesToMenu(wxDocManager * self);
void wxDocManager_FileHistoryAddFilesToMenu1(wxDocManager * self, wxMenu * menu);
void wxDocManager_FileHistoryLoad(wxDocManager * self, const wxConfigBase * config);
void wxDocManager_FileHistoryRemoveMenu(wxDocManager * self, wxMenu * menu);
void wxDocManager_FileHistorySave(wxDocManager * self, wxConfigBase * resource_file);
void wxDocManager_FileHistoryUseMenu(wxDocManager * self, wxMenu * menu);
wxDocTemplate * wxDocManager_FindTemplateForPath(wxDocManager * self, const wxString * path);
wxView * wxDocManager_GetAnyUsableView(const wxDocManager * self);
wxDocument * wxDocManager_GetCurrentDocument(const wxDocManager * self);
wxView * wxDocManager_GetCurrentView(const wxDocManager * self);
wxList * wxDocManager_GetDocuments(wxDocManager * self);
wxFileHistory * wxDocManager_GetFileHistory(const wxDocManager * self);
size_t wxDocManager_GetHistoryFilesCount(const wxDocManager * self);
wxString *wxDocManager_GetLastDirectory(const wxDocManager * self);
int wxDocManager_GetMaxDocsOpen(const wxDocManager * self);
wxList * wxDocManager_GetTemplates(wxDocManager * self);
bool wxDocManager_Initialize(wxDocManager * self);
wxString *wxDocManager_MakeNewDocumentName(wxDocManager * self);
wxFileHistory * wxDocManager_OnCreateFileHistory(wxDocManager * self);
void wxDocManager_OnFileClose(wxDocManager * self, wxCommandEvent * event);
void wxDocManager_OnFileCloseAll(wxDocManager * self, wxCommandEvent * event);
void wxDocManager_OnFileNew(wxDocManager * self, wxCommandEvent * event);
void wxDocManager_OnFileOpen(wxDocManager * self, wxCommandEvent * event);
void wxDocManager_OnFileRevert(wxDocManager * self, wxCommandEvent * event);
void wxDocManager_OnFileSave(wxDocManager * self, wxCommandEvent * event);
void wxDocManager_OnFileSaveAs(wxDocManager * self, wxCommandEvent * event);
void wxDocManager_RemoveDocument(wxDocManager * self, wxDocument * doc);
wxDocTemplate * wxDocManager_SelectDocumentPath(wxDocManager * self, wxDocTemplate ** templates, int no_templates, wxString * path, long flags, bool save);
wxDocTemplate * wxDocManager_SelectDocumentType(wxDocManager * self, wxDocTemplate ** templates, int no_templates, bool sort);
wxDocTemplate * wxDocManager_SelectViewType(wxDocManager * self, wxDocTemplate ** templates, int no_templates, bool sort);
void wxDocManager_SetLastDirectory(wxDocManager * self, const wxString * dir);
void wxDocManager_SetMaxDocsOpen(wxDocManager * self, int n);
// Mix-in(s) to wxDocManager
wxTrackable *wxDocManager_AsTrackable(wxDocManager* obj);

// CLASS: wxDocParentFrame
wxClassInfo *wxDocParentFrame_CLASSINFO();
wxDocParentFrame *wxDocParentFrame_new();
wxDocParentFrame *wxDocParentFrame_new1(wxDocManager * manager, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxDocParentFrame_Create(wxDocParentFrame * self, wxDocManager * manager, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxDocManager * wxDocParentFrame_GetDocumentManager(const wxDocParentFrame * self);
// Mix-in(s) to wxDocParentFrame
wxTrackable *wxDocParentFrame_AsTrackable(wxDocParentFrame* obj);

// CLASS: wxDocTemplate
wxClassInfo *wxDocTemplate_CLASSINFO();
wxDocTemplate *wxDocTemplate_new(wxDocManager * manager, const wxString * descr, const wxString * filter, const wxString * dir, const wxString * ext, const wxString * doc_type_name, const wxString * view_type_name, wxClassInfo * doc_class_info, wxClassInfo * view_class_info, long flags);
wxDocument * wxDocTemplate_CreateDocument(wxDocTemplate * self, const wxString * path, long flags);
wxView * wxDocTemplate_CreateView(wxDocTemplate * self, wxDocument * doc, long flags);
bool wxDocTemplate_FileMatchesTemplate(wxDocTemplate * self, const wxString * path);
wxString *wxDocTemplate_GetDefaultExtension(const wxDocTemplate * self);
wxString *wxDocTemplate_GetDescription(const wxDocTemplate * self);
wxString *wxDocTemplate_GetDirectory(const wxDocTemplate * self);
wxClassInfo * wxDocTemplate_GetDocClassInfo(const wxDocTemplate * self);
wxDocManager * wxDocTemplate_GetDocumentManager(const wxDocTemplate * self);
wxString *wxDocTemplate_GetDocumentName(const wxDocTemplate * self);
wxString *wxDocTemplate_GetFileFilter(const wxDocTemplate * self);
long wxDocTemplate_GetFlags(const wxDocTemplate * self);
wxPageSetupDialogData * wxDocTemplate_GetPageSetupDialogData(wxDocTemplate * self);
wxPageSetupDialogData *wxDocTemplate_GetPageSetupDialogData1(const wxDocTemplate * self);
wxClassInfo * wxDocTemplate_GetViewClassInfo(const wxDocTemplate * self);
wxString *wxDocTemplate_GetViewName(const wxDocTemplate * self);
bool wxDocTemplate_InitDocument(wxDocTemplate * self, wxDocument * doc, const wxString * path, long flags);
bool wxDocTemplate_IsVisible(const wxDocTemplate * self);
void wxDocTemplate_SetDefaultExtension(wxDocTemplate * self, const wxString * ext);
void wxDocTemplate_SetDescription(wxDocTemplate * self, const wxString * descr);
void wxDocTemplate_SetDirectory(wxDocTemplate * self, const wxString * dir);
void wxDocTemplate_SetDocumentManager(wxDocTemplate * self, wxDocManager * manager);
void wxDocTemplate_SetFileFilter(wxDocTemplate * self, const wxString * filter);
void wxDocTemplate_SetFlags(wxDocTemplate * self, long flags);

// CLASS: wxDocument
wxClassInfo *wxDocument_CLASSINFO();
wxDocument *wxDocument_new(wxDocument * parent);
bool wxDocument_AddView(wxDocument * self, wxView * view);
bool wxDocument_AlreadySaved(const wxDocument * self);
void wxDocument_Activate(const wxDocument * self);
bool wxDocument_Close(wxDocument * self);
bool wxDocument_DeleteAllViews(wxDocument * self);
bool wxDocument_DeleteContents(wxDocument * self);
wxCommandProcessor * wxDocument_GetCommandProcessor(const wxDocument * self);
wxDocManager * wxDocument_GetDocumentManager(const wxDocument * self);
wxString *wxDocument_GetDocumentName(const wxDocument * self);
bool wxDocument_GetDocumentSaved(const wxDocument * self);
wxDocTemplate * wxDocument_GetDocumentTemplate(const wxDocument * self);
wxWindow * wxDocument_GetDocumentWindow(const wxDocument * self);
wxString *wxDocument_GetFilename(const wxDocument * self);
wxView * wxDocument_GetFirstView(const wxDocument * self);
wxString *wxDocument_GetTitle(const wxDocument * self);
wxString *wxDocument_GetUserReadableName(const wxDocument * self);
wxList * wxDocument_GetViews(wxDocument * self);
const wxList * wxDocument_GetViews1(const wxDocument * self);
bool wxDocument_IsChildDocument(const wxDocument * self);
bool wxDocument_IsModified(const wxDocument * self);
istream * wxDocument_LoadObject(wxDocument * self, istream * stream);
wxInputStream * wxDocument_LoadObject1(wxDocument * self, wxInputStream * stream);
void wxDocument_Modify(wxDocument * self, bool modify);
void wxDocument_OnChangedViewList(wxDocument * self);
bool wxDocument_OnCloseDocument(wxDocument * self);
bool wxDocument_OnCreate(wxDocument * self, const wxString * path, long flags);
wxCommandProcessor * wxDocument_OnCreateCommandProcessor(wxDocument * self);
bool wxDocument_OnNewDocument(wxDocument * self);
bool wxDocument_OnOpenDocument(wxDocument * self, const wxString * filename);
bool wxDocument_OnSaveDocument(wxDocument * self, const wxString * filename);
bool wxDocument_OnSaveModified(wxDocument * self);
bool wxDocument_RemoveView(wxDocument * self, wxView * view);
bool wxDocument_Save(wxDocument * self);
bool wxDocument_SaveAs(wxDocument * self);
bool wxDocument_Revert(wxDocument * self);
ostream * wxDocument_SaveObject(wxDocument * self, ostream * stream);
wxOutputStream * wxDocument_SaveObject1(wxDocument * self, wxOutputStream * stream);
void wxDocument_SetCommandProcessor(wxDocument * self, wxCommandProcessor * processor);
void wxDocument_SetDocumentName(wxDocument * self, const wxString * name);
void wxDocument_SetDocumentTemplate(wxDocument * self, wxDocTemplate * templ);
void wxDocument_SetDocumentSaved(wxDocument * self, bool saved);
void wxDocument_SetFilename(wxDocument * self, const wxString * filename, bool notify_views);
void wxDocument_OnChangeFilename(wxDocument * self, bool notify_views);
void wxDocument_SetTitle(wxDocument * self, const wxString * title);
void wxDocument_UpdateAllViews(wxDocument * self, wxView * sender, wxObject * hint);
// Mix-in(s) to wxDocument
wxTrackable *wxDocument_AsTrackable(wxDocument* obj);

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
bool wxDragImage_DoDrawImage(const wxDragImage * self, wxDC * dc, const wxPoint * pos);
bool wxDragImage_EndDrag(wxDragImage * self);
wxRect *wxDragImage_GetImageRect(const wxDragImage * self, const wxPoint * pos);
bool wxDragImage_Hide(wxDragImage * self);
bool wxDragImage_Move(wxDragImage * self, const wxPoint * pt);
bool wxDragImage_Show(wxDragImage * self);
bool wxDragImage_UpdateBackingFromWindow(const wxDragImage * self, wxDC * window_dc, wxMemoryDC * dest_dc, const wxRect * source_rect, const wxRect * dest_rect);

// CLASS: wxDropFilesEvent
wxClassInfo *wxDropFilesEvent_CLASSINFO();
wxString *wxDropFilesEvent_GetFiles(const wxDropFilesEvent * self);
int wxDropFilesEvent_GetNumberOfFiles(const wxDropFilesEvent * self);
wxPoint *wxDropFilesEvent_GetPosition(const wxDropFilesEvent * self);

// CLASS: wxDropSource
void wxDropSource_delete(wxDropSource *self);
wxDropSource *wxDropSource_new(wxWindow * win, const wxCursor * icon_copy, const wxCursor * icon_move, const wxCursor * icon_none);
wxDropSource *wxDropSource_new1(wxDataObject * data, wxWindow * win, const wxCursor * icon_copy, const wxCursor * icon_move, const wxCursor * icon_none);
wxDropSource *wxDropSource_new2(wxWindow * win, const wxIcon * icon_copy, const wxIcon * icon_move, const wxIcon * icon_none);
wxDropSource *wxDropSource_new3(wxDataObject * data, wxWindow * win, const wxIcon * icon_copy, const wxIcon * icon_move, const wxIcon * icon_none);
wxDataObject * wxDropSource_GetDataObject(wxDropSource * self);
void wxDropSource_SetData(wxDropSource * self, wxDataObject * data);

// CLASS: wxDropTarget
void wxDropTarget_delete(wxDropTarget *self);
wxDropTarget *wxDropTarget_new(wxDataObject * data);
bool wxDropTarget_GetData(wxDropTarget * self);
bool wxDropTarget_OnDrop(wxDropTarget * self, wxCoord x, wxCoord y);
void wxDropTarget_OnLeave(wxDropTarget * self);
wxDataObject * wxDropTarget_GetDataObject(const wxDropTarget * self);
void wxDropTarget_SetDataObject(wxDropTarget * self, wxDataObject * data);

// CLASS: wxEditableListBox
wxClassInfo *wxEditableListBox_CLASSINFO();
wxEditableListBox *wxEditableListBox_new();
wxEditableListBox *wxEditableListBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxEditableListBox_Create(wxEditableListBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxEditableListBox_SetStrings(wxEditableListBox * self, const wxArrayString * strings);
void wxEditableListBox_GetStrings(const wxEditableListBox * self, wxArrayString * strings);
// Mix-in(s) to wxEditableListBox
wxTrackable *wxEditableListBox_AsTrackable(wxEditableListBox* obj);

// CLASS: wxEraseEvent
wxClassInfo *wxEraseEvent_CLASSINFO();
wxEraseEvent *wxEraseEvent_new(int id, wxDC * dc);
wxDC * wxEraseEvent_GetDC(const wxEraseEvent * self);

// CLASS: wxEventBlocker
wxClassInfo *wxEventBlocker_CLASSINFO();
// Mix-in(s) to wxEventBlocker
wxTrackable *wxEventBlocker_AsTrackable(wxEventBlocker* obj);

// CLASS: wxExtHelpController
wxClassInfo *wxExtHelpController_CLASSINFO();
wxExtHelpController *wxExtHelpController_new(wxWindow * parent_window);
bool wxExtHelpController_DisplayHelp(wxExtHelpController * self, const wxString * relative_url);

// CLASS: wxFileCtrl
wxClassInfo *wxFileCtrl_CLASSINFO();
wxFileCtrl *wxFileCtrl_new();
wxFileCtrl *wxFileCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * default_directory, const wxString * default_filename, const wxString * wild_card, long style, const wxPoint * pos, const wxSize * size, const wxString * name);
bool wxFileCtrl_Create(wxFileCtrl * self, wxWindow * parent, wxWindowID id, const wxString * default_directory, const wxString * default_filename, const wxString * wild_card, long style, const wxPoint * pos, const wxSize * size, const wxString * name);
wxString *wxFileCtrl_GetDirectory(const wxFileCtrl * self);
wxString *wxFileCtrl_GetFilename(const wxFileCtrl * self);
void wxFileCtrl_GetFilenames(const wxFileCtrl * self, wxArrayString * filenames);
int wxFileCtrl_GetFilterIndex(const wxFileCtrl * self);
wxString *wxFileCtrl_GetPath(const wxFileCtrl * self);
void wxFileCtrl_GetPaths(const wxFileCtrl * self, wxArrayString * paths);
wxString *wxFileCtrl_GetWildcard(const wxFileCtrl * self);
bool wxFileCtrl_SetDirectory(wxFileCtrl * self, const wxString * directory);
bool wxFileCtrl_SetFilename(wxFileCtrl * self, const wxString * filename);
bool wxFileCtrl_SetPath(wxFileCtrl * self, const wxString * path);
void wxFileCtrl_SetFilterIndex(wxFileCtrl * self, int filter_index);
void wxFileCtrl_SetWildcard(wxFileCtrl * self, const wxString * wild_card);
void wxFileCtrl_ShowHidden(wxFileCtrl * self, bool show);
// Mix-in(s) to wxFileCtrl
wxTrackable *wxFileCtrl_AsTrackable(wxFileCtrl* obj);

// CLASS: wxFileCtrlEvent
wxClassInfo *wxFileCtrlEvent_CLASSINFO();
wxString *wxFileCtrlEvent_GetDirectory(const wxFileCtrlEvent * self);
wxString *wxFileCtrlEvent_GetFile(const wxFileCtrlEvent * self);
wxArrayString *wxFileCtrlEvent_GetFiles(const wxFileCtrlEvent * self);
int wxFileCtrlEvent_GetFilterIndex(const wxFileCtrlEvent * self);
void wxFileCtrlEvent_SetFiles(wxFileCtrlEvent * self, const wxArrayString * files);
void wxFileCtrlEvent_SetDirectory(wxFileCtrlEvent * self, const wxString * directory);
void wxFileCtrlEvent_SetFilterIndex(wxFileCtrlEvent * self, int index);

// CLASS: wxFileDataObject
void wxFileDataObject_delete(wxFileDataObject *self);
wxFileDataObject *wxFileDataObject_new();
void wxFileDataObject_AddFile(wxFileDataObject * self, const wxString * file);
wxArrayString *wxFileDataObject_GetFilenames(const wxFileDataObject * self);

// CLASS: wxFileDialog
wxClassInfo *wxFileDialog_CLASSINFO();
wxFileDialog *wxFileDialog_new(wxWindow * parent, const wxString * message, const wxString * default_dir, const wxString * default_file, const wxString * wildcard, long style, const wxPoint * pos, const wxSize * size, const wxString * name);
wxString *wxFileDialog_GetCurrentlySelectedFilename(const wxFileDialog * self);
int wxFileDialog_GetCurrentlySelectedFilterIndex(const wxFileDialog * self);
wxString *wxFileDialog_GetDirectory(const wxFileDialog * self);
wxWindow * wxFileDialog_GetExtraControl(const wxFileDialog * self);
wxString *wxFileDialog_GetFilename(const wxFileDialog * self);
void wxFileDialog_GetFilenames(const wxFileDialog * self, wxArrayString * filenames);
int wxFileDialog_GetFilterIndex(const wxFileDialog * self);
wxString *wxFileDialog_GetMessage(const wxFileDialog * self);
wxString *wxFileDialog_GetPath(const wxFileDialog * self);
void wxFileDialog_GetPaths(const wxFileDialog * self, wxArrayString * paths);
wxString *wxFileDialog_GetWildcard(const wxFileDialog * self);
bool wxFileDialog_SetCustomizeHook(wxFileDialog * self, wxFileDialogCustomizeHook * customize_hook);
void wxFileDialog_SetDirectory(wxFileDialog * self, const wxString * directory);
void wxFileDialog_SetFilename(wxFileDialog * self, const wxString * setfilename);
void wxFileDialog_SetFilterIndex(wxFileDialog * self, int filter_index);
void wxFileDialog_SetMessage(wxFileDialog * self, const wxString * message);
void wxFileDialog_SetPath(wxFileDialog * self, const wxString * path);
void wxFileDialog_SetWildcard(wxFileDialog * self, const wxString * wild_card);
// Mix-in(s) to wxFileDialog
wxTrackable *wxFileDialog_AsTrackable(wxFileDialog* obj);

// CLASS: wxFileDialogCustomize
void wxFileDialogCustomize_delete(wxFileDialogCustomize *self);
wxFileDialogButton * wxFileDialogCustomize_AddButton(wxFileDialogCustomize * self, const wxString * label);
wxFileDialogCheckBox * wxFileDialogCustomize_AddCheckBox(wxFileDialogCustomize * self, const wxString * label);
wxFileDialogRadioButton * wxFileDialogCustomize_AddRadioButton(wxFileDialogCustomize * self, const wxString * label);
wxFileDialogChoice * wxFileDialogCustomize_AddChoice(wxFileDialogCustomize * self, size_t n, const wxString * strings);
wxFileDialogTextCtrl * wxFileDialogCustomize_AddTextCtrl(wxFileDialogCustomize * self, const wxString * label);
wxFileDialogStaticText * wxFileDialogCustomize_AddStaticText(wxFileDialogCustomize * self, const wxString * label);

// CLASS: wxFileDialogCustomizeHook
void wxFileDialogCustomizeHook_delete(wxFileDialogCustomizeHook *self);
void wxFileDialogCustomizeHook_AddCustomControls(wxFileDialogCustomizeHook * self, wxFileDialogCustomize * customizer);
void wxFileDialogCustomizeHook_UpdateCustomControls(wxFileDialogCustomizeHook * self);
void wxFileDialogCustomizeHook_TransferDataFromCustomControls(wxFileDialogCustomizeHook * self);

// CLASS: wxFileDirPickerEvent
wxClassInfo *wxFileDirPickerEvent_CLASSINFO();
wxFileDirPickerEvent *wxFileDirPickerEvent_new();
wxString *wxFileDirPickerEvent_GetPath(const wxFileDirPickerEvent * self);
void wxFileDirPickerEvent_SetPath(wxFileDirPickerEvent * self, const wxString * path);

// CLASS: wxFileDropTarget
void wxFileDropTarget_delete(wxFileDropTarget *self);
wxFileDropTarget *wxFileDropTarget_new();
bool wxFileDropTarget_OnDropFiles(wxFileDropTarget * self, wxCoord x, wxCoord y, const wxArrayString * filenames);

// CLASS: wxFileHistory
wxClassInfo *wxFileHistory_CLASSINFO();
wxFileHistory *wxFileHistory_new(size_t max_files, wxWindowID id_base);
void wxFileHistory_AddFileToHistory(wxFileHistory * self, const wxString * filename);
void wxFileHistory_AddFilesToMenu(wxFileHistory * self);
void wxFileHistory_AddFilesToMenu1(wxFileHistory * self, wxMenu * menu);
wxWindowID wxFileHistory_GetBaseId(const wxFileHistory * self);
size_t wxFileHistory_GetCount(const wxFileHistory * self);
wxString *wxFileHistory_GetHistoryFile(const wxFileHistory * self, size_t index);
int wxFileHistory_GetMaxFiles(const wxFileHistory * self);
const wxList * wxFileHistory_GetMenus(const wxFileHistory * self);
void wxFileHistory_Load(wxFileHistory * self, const wxConfigBase * config);
void wxFileHistory_RemoveFileFromHistory(wxFileHistory * self, size_t i);
void wxFileHistory_RemoveMenu(wxFileHistory * self, wxMenu * menu);
void wxFileHistory_Save(wxFileHistory * self, wxConfigBase * config);
void wxFileHistory_SetBaseId(wxFileHistory * self, wxWindowID base_id);
void wxFileHistory_UseMenu(wxFileHistory * self, wxMenu * menu);

// CLASS: wxFilePickerCtrl
wxClassInfo *wxFilePickerCtrl_CLASSINFO();
wxFilePickerCtrl *wxFilePickerCtrl_new();
wxFilePickerCtrl *wxFilePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxString * wildcard, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxFilePickerCtrl_Create(wxFilePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxString * wildcard, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxFileName *wxFilePickerCtrl_GetFileName(const wxFilePickerCtrl * self);
wxString *wxFilePickerCtrl_GetPath(const wxFilePickerCtrl * self);
void wxFilePickerCtrl_SetFileName(wxFilePickerCtrl * self, const wxFileName * filename);
void wxFilePickerCtrl_SetInitialDirectory(wxFilePickerCtrl * self, const wxString * dir);
void wxFilePickerCtrl_SetPath(wxFilePickerCtrl * self, const wxString * filename);
// Mix-in(s) to wxFilePickerCtrl
wxTrackable *wxFilePickerCtrl_AsTrackable(wxFilePickerCtrl* obj);

// CLASS: wxFindDialogEvent
wxClassInfo *wxFindDialogEvent_CLASSINFO();
wxFindReplaceDialog * wxFindDialogEvent_GetDialog(const wxFindDialogEvent * self);
wxString *wxFindDialogEvent_GetFindString(const wxFindDialogEvent * self);
int wxFindDialogEvent_GetFlags(const wxFindDialogEvent * self);
wxString *wxFindDialogEvent_GetReplaceString(const wxFindDialogEvent * self);

// CLASS: wxFindReplaceData
wxClassInfo *wxFindReplaceData_CLASSINFO();
wxString *wxFindReplaceData_GetFindString(const wxFindReplaceData * self);
int wxFindReplaceData_GetFlags(const wxFindReplaceData * self);
wxString *wxFindReplaceData_GetReplaceString(const wxFindReplaceData * self);
void wxFindReplaceData_SetFindString(wxFindReplaceData * self, const wxString * str);
void wxFindReplaceData_SetReplaceString(wxFindReplaceData * self, const wxString * str);

// CLASS: wxFindReplaceDialog
wxClassInfo *wxFindReplaceDialog_CLASSINFO();
wxFindReplaceDialog *wxFindReplaceDialog_new();
wxFindReplaceDialog *wxFindReplaceDialog_new1(wxWindow * parent, wxFindReplaceData * data, const wxString * title, int style);
bool wxFindReplaceDialog_Create(wxFindReplaceDialog * self, wxWindow * parent, wxFindReplaceData * data, const wxString * title, int style);
const wxFindReplaceData * wxFindReplaceDialog_GetData(const wxFindReplaceDialog * self);
// Mix-in(s) to wxFindReplaceDialog
wxTrackable *wxFindReplaceDialog_AsTrackable(wxFindReplaceDialog* obj);

// CLASS: wxFlexGridSizer
wxClassInfo *wxFlexGridSizer_CLASSINFO();
wxFlexGridSizer *wxFlexGridSizer_new(int cols, int vgap, int hgap);
wxFlexGridSizer *wxFlexGridSizer_new1(int cols, const wxSize * gap);
wxFlexGridSizer *wxFlexGridSizer_new2(int rows, int cols, int vgap, int hgap);
wxFlexGridSizer *wxFlexGridSizer_new3(int rows, int cols, const wxSize * gap);
void wxFlexGridSizer_AddGrowableCol(wxFlexGridSizer * self, size_t idx, int proportion);
void wxFlexGridSizer_AddGrowableRow(wxFlexGridSizer * self, size_t idx, int proportion);
int wxFlexGridSizer_GetFlexibleDirection(const wxFlexGridSizer * self);
bool wxFlexGridSizer_IsColGrowable(wxFlexGridSizer * self, size_t idx);
bool wxFlexGridSizer_IsRowGrowable(wxFlexGridSizer * self, size_t idx);
void wxFlexGridSizer_RemoveGrowableCol(wxFlexGridSizer * self, size_t idx);
void wxFlexGridSizer_RemoveGrowableRow(wxFlexGridSizer * self, size_t idx);
void wxFlexGridSizer_SetFlexibleDirection(wxFlexGridSizer * self, int direction);
wxArrayInt *wxFlexGridSizer_GetRowHeights(const wxFlexGridSizer * self);
wxArrayInt *wxFlexGridSizer_GetColWidths(const wxFlexGridSizer * self);

// CLASS: wxFocusEvent
wxClassInfo *wxFocusEvent_CLASSINFO();
wxWindow * wxFocusEvent_GetWindow(const wxFocusEvent * self);
void wxFocusEvent_SetWindow(wxFocusEvent * self, wxWindow * win);

// CLASS: wxFont
wxClassInfo *wxFont_CLASSINFO();
#if wxCHECK_VERSION(3, 1, 0)
wxFont *wxFont_GetBaseFont(const wxFont * self);
#endif
wxString *wxFont_GetFaceName(const wxFont * self);
wxString *wxFont_GetNativeFontInfoDesc(const wxFont * self);
wxString *wxFont_GetNativeFontInfoUserDesc(const wxFont * self);
const wxNativeFontInfo * wxFont_GetNativeFontInfo(const wxFont * self);
int wxFont_GetPointSize(const wxFont * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxFont_GetFractionalPointSize(const wxFont * self);
#endif
wxSize *wxFont_GetPixelSize(const wxFont * self);
bool wxFont_GetUnderlined(const wxFont * self);
bool wxFont_GetStrikethrough(const wxFont * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxFont_GetNumericWeight(const wxFont * self);
#endif
bool wxFont_IsFixedWidth(const wxFont * self);
bool wxFont_IsOk(const wxFont * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxFont_AddPrivateFont(const wxString * filename);
#endif
wxFont *wxFont_Bold(const wxFont * self);
wxFont *wxFont_Italic(const wxFont * self);
wxFont *wxFont_Larger(const wxFont * self);
wxFont *wxFont_Smaller(const wxFont * self);
wxFont *wxFont_Underlined(const wxFont * self);
wxFont *wxFont_Strikethrough(const wxFont * self);
wxFont * wxFont_MakeBold(wxFont * self);
wxFont * wxFont_MakeItalic(wxFont * self);
wxFont * wxFont_MakeLarger(wxFont * self);
wxFont * wxFont_MakeSmaller(wxFont * self);
wxFont * wxFont_MakeUnderlined(wxFont * self);
wxFont * wxFont_MakeStrikethrough(wxFont * self);
bool wxFont_SetFaceName(wxFont * self, const wxString * face_name);
bool wxFont_SetNativeFontInfo(wxFont * self, const wxString * info);
bool wxFont_SetNativeFontInfoUserDesc(wxFont * self, const wxString * info);
void wxFont_SetNativeFontInfo1(wxFont * self, const wxNativeFontInfo * info);
void wxFont_SetPointSize(wxFont * self, int point_size);
#if wxCHECK_VERSION(3, 1, 0)
void wxFont_SetFractionalPointSize(wxFont * self, double point_size);
#endif
void wxFont_SetPixelSize(wxFont * self, const wxSize * pixel_size);
void wxFont_SetUnderlined(wxFont * self, bool underlined);
void wxFont_SetStrikethrough(wxFont * self, bool strikethrough);
#if wxCHECK_VERSION(3, 1, 0)
void wxFont_SetNumericWeight(wxFont * self, int weight);
#endif
wxFont * wxFont_New4(const wxNativeFontInfo * native_info);
wxFont * wxFont_New5(const wxString * native_info_string);
wxFont *wxFont_new();
wxFont *wxFont_new1(const wxFont * font);
wxFont *wxFont_new2(const wxFontInfo * font_info);
wxFont *wxFont_new5(const wxString * native_info_string);
wxFont *wxFont_new6(const wxNativeFontInfo * native_info);

// CLASS: wxFontData
wxClassInfo *wxFontData_CLASSINFO();
wxFontData *wxFontData_new();
void wxFontData_EnableEffects(wxFontData * self, bool enable);
bool wxFontData_GetAllowSymbols(const wxFontData * self);
wxFont *wxFontData_GetChosenFont(const wxFontData * self);
wxColour *wxFontData_GetColour(const wxFontData * self);
bool wxFontData_GetEnableEffects(const wxFontData * self);
int wxFontData_GetRestrictSelection(const wxFontData * self);
wxFont *wxFontData_GetInitialFont(const wxFontData * self);
bool wxFontData_GetShowHelp(const wxFontData * self);
void wxFontData_RestrictSelection(wxFontData * self, int flags);
void wxFontData_SetAllowSymbols(wxFontData * self, bool allow_symbols);
void wxFontData_SetChosenFont(wxFontData * self, const wxFont * font);
void wxFontData_SetColour(wxFontData * self, const wxColour * colour);
void wxFontData_SetInitialFont(wxFontData * self, const wxFont * font);
void wxFontData_SetRange(wxFontData * self, int min, int max);
void wxFontData_SetShowHelp(wxFontData * self, bool show_help);

// CLASS: wxFontDialog
wxClassInfo *wxFontDialog_CLASSINFO();
wxFontDialog *wxFontDialog_new();
wxFontDialog *wxFontDialog_new1(wxWindow * parent);
wxFontDialog *wxFontDialog_new2(wxWindow * parent, const wxFontData * data);
bool wxFontDialog_Create(wxFontDialog * self, wxWindow * parent);
bool wxFontDialog_Create1(wxFontDialog * self, wxWindow * parent, const wxFontData * data);
wxFontData *wxFontDialog_GetFontData(const wxFontDialog * self);
wxFontData * wxFontDialog_GetFontData1(wxFontDialog * self);
// Mix-in(s) to wxFontDialog
wxTrackable *wxFontDialog_AsTrackable(wxFontDialog* obj);

// CLASS: wxFontEnumerator
void wxFontEnumerator_delete(wxFontEnumerator *self);
wxFontEnumerator *wxFontEnumerator_new();
bool wxFontEnumerator_EnumerateEncodings(wxFontEnumerator * self, const wxString * font);
bool wxFontEnumerator_OnFacename(wxFontEnumerator * self, const wxString * font);
bool wxFontEnumerator_OnFontEncoding(wxFontEnumerator * self, const wxString * font, const wxString * encoding);
wxArrayString *wxFontEnumerator_GetEncodings(const wxString * facename);
bool wxFontEnumerator_IsValidFacename(const wxString * facename);
void wxFontEnumerator_InvalidateCache();

// CLASS: wxFontList
void wxFontList_delete(wxFontList *self);
wxFontList *wxFontList_new();
wxFont * wxFontList_FindOrCreateFont1(wxFontList * self, const wxFontInfo * font_info);

// CLASS: wxFontMapper
void wxFontMapper_delete(wxFontMapper *self);
wxFontMapper *wxFontMapper_new();
void wxFontMapper_SetConfigPath(wxFontMapper * self, const wxString * prefix);
void wxFontMapper_SetDialogParent(wxFontMapper * self, wxWindow * parent);
void wxFontMapper_SetDialogTitle(wxFontMapper * self, const wxString * title);
wxFontMapper * wxFontMapper_Get();
size_t wxFontMapper_GetSupportedEncodingsCount();
wxFontMapper * wxFontMapper_Set(wxFontMapper * mapper);

// CLASS: wxFontPickerCtrl
wxClassInfo *wxFontPickerCtrl_CLASSINFO();
wxFontPickerCtrl *wxFontPickerCtrl_new();
wxFontPickerCtrl *wxFontPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxFont * font, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxFontPickerCtrl_Create(wxFontPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxFont * font, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
unsigned int wxFontPickerCtrl_GetMaxPointSize(const wxFontPickerCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
unsigned int wxFontPickerCtrl_GetMinPointSize(const wxFontPickerCtrl * self);
wxColour *wxFontPickerCtrl_GetSelectedColour(const wxFontPickerCtrl * self);
#endif
wxFont *wxFontPickerCtrl_GetSelectedFont(const wxFontPickerCtrl * self);
void wxFontPickerCtrl_SetMaxPointSize(wxFontPickerCtrl * self, unsigned int max);
#if wxCHECK_VERSION(3, 1, 0)
void wxFontPickerCtrl_SetMinPointSize(wxFontPickerCtrl * self, unsigned int min);
void wxFontPickerCtrl_SetSelectedColour(wxFontPickerCtrl * self, const wxColour * colour);
#endif
void wxFontPickerCtrl_SetSelectedFont(wxFontPickerCtrl * self, const wxFont * font);
// Mix-in(s) to wxFontPickerCtrl
wxTrackable *wxFontPickerCtrl_AsTrackable(wxFontPickerCtrl* obj);

// CLASS: wxFontPickerEvent
wxClassInfo *wxFontPickerEvent_CLASSINFO();
wxFontPickerEvent *wxFontPickerEvent_new(wxObject * generator, int id, const wxFont * font);
wxFont *wxFontPickerEvent_GetFont(const wxFontPickerEvent * self);
void wxFontPickerEvent_SetFont(wxFontPickerEvent * self, const wxFont * f);

// CLASS: wxFrame
wxClassInfo *wxFrame_CLASSINFO();
wxFrame *wxFrame_new();
wxFrame *wxFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxFrame_Centre(wxFrame * self, int direction);
bool wxFrame_Create(wxFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxStatusBar * wxFrame_CreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name);
wxToolBar * wxFrame_CreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name);
void wxFrame_DoGiveHelp(wxFrame * self, const wxString * text, bool show);
wxMenuBar * wxFrame_GetMenuBar(const wxFrame * self);
wxStatusBar * wxFrame_GetStatusBar(const wxFrame * self);
int wxFrame_GetStatusBarPane(const wxFrame * self);
wxToolBar * wxFrame_GetToolBar(const wxFrame * self);
wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name);
wxToolBar * wxFrame_OnCreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name);
bool wxFrame_ProcessCommand(wxFrame * self, int id);
void wxFrame_SetMenuBar(wxFrame * self, wxMenuBar * menu_bar);
void wxFrame_SetStatusBar(wxFrame * self, wxStatusBar * status_bar);
void wxFrame_SetStatusBarPane(wxFrame * self, int n);
void wxFrame_SetStatusText(wxFrame * self, const wxString * text, int number);
void wxFrame_SetStatusWidths(wxFrame * self, int n, const int * widths_field);
void wxFrame_SetToolBar(wxFrame * self, wxToolBar * tool_bar);
#ifdef __WXMSW__
wxTaskBarButton * wxFrame_MSWGetTaskBarButton(wxFrame * self);
#endif
void wxFrame_PushStatusText(wxFrame * self, const wxString * text, int number);
void wxFrame_PopStatusText(wxFrame * self, int number);
// Mix-in(s) to wxFrame
wxTrackable *wxFrame_AsTrackable(wxFrame* obj);

// CLASS: wxFullScreenEvent
wxClassInfo *wxFullScreenEvent_CLASSINFO();
wxFullScreenEvent *wxFullScreenEvent_new(int id, bool fullscreen);
bool wxFullScreenEvent_IsFullScreen(const wxFullScreenEvent * self);

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

// CLASS: wxGCDC
wxClassInfo *wxGCDC_CLASSINFO();
wxGCDC *wxGCDC_new(const wxWindowDC * window_dc);
wxGCDC *wxGCDC_new1(const wxMemoryDC * memory_dc);
wxGCDC *wxGCDC_new2(const wxPrinterDC * printer_dc);
wxGCDC *wxGCDC_new3(wxGraphicsContext * context);
wxGCDC *wxGCDC_new4(const wxEnhMetaFileDC * emf_dc);
wxGCDC *wxGCDC_new5();

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
// Mix-in(s) to wxGauge
wxTrackable *wxGauge_AsTrackable(wxGauge* obj);

// CLASS: wxGenericAboutDialog
void wxGenericAboutDialog_delete(wxGenericAboutDialog *self);
wxGenericAboutDialog *wxGenericAboutDialog_new();
wxGenericAboutDialog *wxGenericAboutDialog_new1(const wxAboutDialogInfo * info, wxWindow * parent);
bool wxGenericAboutDialog_Create(wxGenericAboutDialog * self, const wxAboutDialogInfo * info, wxWindow * parent);

// CLASS: wxGenericAnimationCtrl
wxClassInfo *wxGenericAnimationCtrl_CLASSINFO();
wxGenericAnimationCtrl *wxGenericAnimationCtrl_new(wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxGenericAnimationCtrl_Create(wxGenericAnimationCtrl * self, wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxGenericAnimationCtrl_DrawCurrentFrame(wxGenericAnimationCtrl * self, wxDC * dc);
wxBitmap * wxGenericAnimationCtrl_GetBackingStore(wxGenericAnimationCtrl * self);
bool wxGenericAnimationCtrl_Play(wxGenericAnimationCtrl * self, bool looped);
void wxGenericAnimationCtrl_SetUseWindowBackgroundColour(wxGenericAnimationCtrl * self, bool use_win_background);
bool wxGenericAnimationCtrl_IsUsingWindowBackgroundColour(const wxGenericAnimationCtrl * self);
// Mix-in(s) to wxGenericAnimationCtrl
wxTrackable *wxGenericAnimationCtrl_AsTrackable(wxGenericAnimationCtrl* obj);

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
wxString *wxGenericDirCtrl_GetPath1(const wxGenericDirCtrl * self, wxTreeItemId item_id);
void wxGenericDirCtrl_GetPaths(const wxGenericDirCtrl * self, wxArrayString * paths);
wxTreeItemId *wxGenericDirCtrl_GetRootId(wxGenericDirCtrl * self);
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
// Mix-in(s) to wxGenericDirCtrl
wxTrackable *wxGenericDirCtrl_AsTrackable(wxGenericDirCtrl* obj);

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
// Mix-in(s) to wxGenericProgressDialog
wxTrackable *wxGenericProgressDialog_AsTrackable(wxGenericProgressDialog* obj);

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
// Mix-in(s) to wxGenericValidator
wxTrackable *wxGenericValidator_AsTrackable(wxGenericValidator* obj);

// CLASS: wxGestureEvent
wxClassInfo *wxGestureEvent_CLASSINFO();
wxPoint *wxGestureEvent_GetPosition(const wxGestureEvent * self);
bool wxGestureEvent_IsGestureStart(const wxGestureEvent * self);
bool wxGestureEvent_IsGestureEnd(const wxGestureEvent * self);
void wxGestureEvent_SetPosition(wxGestureEvent * self, const wxPoint * pos);
void wxGestureEvent_SetGestureStart(wxGestureEvent * self, bool is_start);
void wxGestureEvent_SetGestureEnd(wxGestureEvent * self, bool is_end);

// CLASS: wxGraphicsBrush
wxClassInfo *wxGraphicsBrush_CLASSINFO();

// CLASS: wxGraphicsContext
wxClassInfo *wxGraphicsContext_CLASSINFO();
wxGraphicsContext * wxGraphicsContext_Create(wxWindow * window);
wxGraphicsContext * wxGraphicsContext_Create1(const wxWindowDC * window_dc);
wxGraphicsContext * wxGraphicsContext_Create2(const wxMemoryDC * memory_dc);
wxGraphicsContext * wxGraphicsContext_Create3(const wxPrinterDC * printer_dc);
wxGraphicsContext * wxGraphicsContext_Create4(const wxEnhMetaFileDC * meta_file_dc);
wxGraphicsContext * wxGraphicsContext_CreateFromUnknownDC(wxDC * dc);
wxGraphicsContext * wxGraphicsContext_Create5(wxImage * image);
wxGraphicsContext * wxGraphicsContext_CreateFromNative(void * context);
wxGraphicsContext * wxGraphicsContext_CreateFromNativeWindow(void * window);
wxGraphicsContext * wxGraphicsContext_Create6();
void wxGraphicsContext_ResetClip(wxGraphicsContext * self);
void wxGraphicsContext_Clip(wxGraphicsContext * self, const wxRegion * region);
void wxGraphicsContext_GetClipBox(wxGraphicsContext * self, wxDouble * x, wxDouble * y, wxDouble * w, wxDouble * h);
wxGraphicsMatrix *wxGraphicsContext_CreateMatrix1(const wxGraphicsContext * self, const wxAffineMatrix2DBase * mat);
void wxGraphicsContext_ConcatTransform(wxGraphicsContext * self, const wxGraphicsMatrix * matrix);
wxGraphicsMatrix *wxGraphicsContext_GetTransform(const wxGraphicsContext * self);
void wxGraphicsContext_SetTransform(wxGraphicsContext * self, const wxGraphicsMatrix * matrix);
wxGraphicsBrush *wxGraphicsContext_CreateBrush(const wxGraphicsContext * self, const wxBrush * brush);
void wxGraphicsContext_SetBrush(wxGraphicsContext * self, const wxBrush * brush);
void wxGraphicsContext_SetBrush1(wxGraphicsContext * self, const wxGraphicsBrush * brush);
wxGraphicsPen *wxGraphicsContext_CreatePen(const wxGraphicsContext * self, const wxPen * pen);
wxGraphicsPen *wxGraphicsContext_CreatePen1(const wxGraphicsContext * self, const wxGraphicsPenInfo * info);
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
wxGraphicsBitmap *wxGraphicsContext_CreateBitmap(wxGraphicsContext * self, const wxBitmap * bitmap);
wxGraphicsBitmap *wxGraphicsContext_CreateBitmapFromImage(wxGraphicsContext * self, const wxImage * image);
void wxGraphicsContext_EndLayer(wxGraphicsContext * self);
void wxGraphicsContext_PushState(wxGraphicsContext * self);
void wxGraphicsContext_PopState(wxGraphicsContext * self);
void wxGraphicsContext_Flush(wxGraphicsContext * self);
void * wxGraphicsContext_GetNativeContext(wxGraphicsContext * self);
void wxGraphicsContext_GetSize(const wxGraphicsContext * self, wxDouble * width, wxDouble * height);
void wxGraphicsContext_GetDPI(const wxGraphicsContext * self, wxDouble * dpi_x, wxDouble * dpi_y);
wxWindow * wxGraphicsContext_GetWindow(const wxGraphicsContext * self);
bool wxGraphicsContext_ShouldOffset(const wxGraphicsContext * self);
void wxGraphicsContext_EnableOffset(wxGraphicsContext * self, bool enable);
void wxGraphicsContext_DisableOffset(wxGraphicsContext * self);
bool wxGraphicsContext_OffsetEnabled(const wxGraphicsContext * self);
wxSize *wxGraphicsContext_FromDIP(const wxGraphicsContext * self, const wxSize * sz);
wxPoint *wxGraphicsContext_FromDIP1(const wxGraphicsContext * self, const wxPoint * pt);
int wxGraphicsContext_FromDIP2(const wxGraphicsContext * self, int d);
wxSize *wxGraphicsContext_ToDIP(const wxGraphicsContext * self, const wxSize * sz);
wxPoint *wxGraphicsContext_ToDIP1(const wxGraphicsContext * self, const wxPoint * pt);
int wxGraphicsContext_ToDIP2(const wxGraphicsContext * self, int d);

// CLASS: wxGraphicsFont
wxClassInfo *wxGraphicsFont_CLASSINFO();

// CLASS: wxGraphicsGradientStop
void wxGraphicsGradientStop_delete(wxGraphicsGradientStop *self);
wxColour *wxGraphicsGradientStop_GetColour(const wxGraphicsGradientStop * self);
void wxGraphicsGradientStop_SetColour(wxGraphicsGradientStop * self, const wxColour * col);

// CLASS: wxGraphicsGradientStops
void wxGraphicsGradientStops_delete(wxGraphicsGradientStops *self);
wxGraphicsGradientStops *wxGraphicsGradientStops_new(wxColour start_col, wxColour end_col);
void wxGraphicsGradientStops_Add(wxGraphicsGradientStops * self, const wxGraphicsGradientStop * stop);
size_t wxGraphicsGradientStops_GetCount(const wxGraphicsGradientStops * self);
void wxGraphicsGradientStops_SetStartColour(wxGraphicsGradientStops * self, wxColour col);
wxColour *wxGraphicsGradientStops_GetStartColour(const wxGraphicsGradientStops * self);
void wxGraphicsGradientStops_SetEndColour(wxGraphicsGradientStops * self, wxColour col);
wxColour *wxGraphicsGradientStops_GetEndColour(const wxGraphicsGradientStops * self);

// CLASS: wxGraphicsMatrix
wxClassInfo *wxGraphicsMatrix_CLASSINFO();
void wxGraphicsMatrix_Concat(wxGraphicsMatrix * self, const wxGraphicsMatrix * t);
void wxGraphicsMatrix_Concat1(wxGraphicsMatrix * self, const wxGraphicsMatrix * t);
void wxGraphicsMatrix_Get(const wxGraphicsMatrix * self, wxDouble * a, wxDouble * b, wxDouble * c, wxDouble * d, wxDouble * tx, wxDouble * ty);
void * wxGraphicsMatrix_GetNativeMatrix(const wxGraphicsMatrix * self);
void wxGraphicsMatrix_Invert(wxGraphicsMatrix * self);
bool wxGraphicsMatrix_IsEqual(const wxGraphicsMatrix * self, const wxGraphicsMatrix * t);
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
wxRect2DDouble *wxGraphicsPath_GetBox(const wxGraphicsPath * self);
void wxGraphicsPath_GetBox1(const wxGraphicsPath * self, wxDouble * x, wxDouble * y, wxDouble * w, wxDouble * h);
void wxGraphicsPath_GetCurrentPoint(const wxGraphicsPath * self, wxDouble * x, wxDouble * y);
wxPoint2DDouble *wxGraphicsPath_GetCurrentPoint1(const wxGraphicsPath * self);
void * wxGraphicsPath_GetNativePath(const wxGraphicsPath * self);
void wxGraphicsPath_MoveToPoint1(wxGraphicsPath * self, const wxPoint2DDouble * p);
void wxGraphicsPath_Transform(wxGraphicsPath * self, const wxGraphicsMatrix * matrix);
void wxGraphicsPath_UnGetNativePath(const wxGraphicsPath * self, void * p);

// CLASS: wxGraphicsPen
wxClassInfo *wxGraphicsPen_CLASSINFO();

// CLASS: wxGraphicsRenderer
wxClassInfo *wxGraphicsRenderer_CLASSINFO();
wxGraphicsBitmap *wxGraphicsRenderer_CreateBitmap(wxGraphicsRenderer * self, const wxBitmap * bitmap);
wxGraphicsBitmap *wxGraphicsRenderer_CreateBitmapFromImage(wxGraphicsRenderer * self, const wxImage * image);
wxImage *wxGraphicsRenderer_CreateImageFromBitmap(wxGraphicsRenderer * self, const wxGraphicsBitmap * bmp);
wxGraphicsBitmap *wxGraphicsRenderer_CreateBitmapFromNativeBitmap(wxGraphicsRenderer * self, void * bitmap);
wxGraphicsContext * wxGraphicsRenderer_CreateContext(wxGraphicsRenderer * self, wxWindow * window);
wxGraphicsContext * wxGraphicsRenderer_CreateContext1(wxGraphicsRenderer * self, const wxWindowDC * window_dc);
wxGraphicsContext * wxGraphicsRenderer_CreateContext2(wxGraphicsRenderer * self, const wxMemoryDC * memory_dc);
wxGraphicsContext * wxGraphicsRenderer_CreateContext3(wxGraphicsRenderer * self, const wxPrinterDC * printer_dc);
wxGraphicsContext * wxGraphicsRenderer_CreateContext4(wxGraphicsRenderer * self, const wxEnhMetaFileDC * meta_file_dc);
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
wxGraphicsPen *wxGraphicsRenderer_CreatePen(wxGraphicsRenderer * self, const wxGraphicsPenInfo * info);
wxString *wxGraphicsRenderer_GetName(const wxGraphicsRenderer * self);
void wxGraphicsRenderer_GetVersion(const wxGraphicsRenderer * self, int * major, int * minor, int * micro);
wxGraphicsRenderer * wxGraphicsRenderer_GetDefaultRenderer();
wxGraphicsRenderer * wxGraphicsRenderer_GetCairoRenderer();
wxGraphicsRenderer * wxGraphicsRenderer_GetGDIPlusRenderer();
wxGraphicsRenderer * wxGraphicsRenderer_GetDirect2DRenderer();

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

// CLASS: wxGridCellAttr
void wxGridCellAttr_delete(wxGridCellAttr *self);
wxGridCellAttr *wxGridCellAttr_new(wxGridCellAttr * attr_default);
wxGridCellAttr *wxGridCellAttr_new1(const wxColour * col_text, const wxColour * col_back, const wxFont * font, int h_align, int v_align);
wxGridCellAttr * wxGridCellAttr_Clone(const wxGridCellAttr * self);
void wxGridCellAttr_DecRef(wxGridCellAttr * self);
void wxGridCellAttr_GetAlignment(const wxGridCellAttr * self, int * h_align, int * v_align);
wxColour *wxGridCellAttr_GetBackgroundColour(const wxGridCellAttr * self);
wxGridCellEditor * wxGridCellAttr_GetEditor(const wxGridCellAttr * self, const wxGrid * grid, int row, int col);
wxFont *wxGridCellAttr_GetFont(const wxGridCellAttr * self);
void wxGridCellAttr_GetNonDefaultAlignment(const wxGridCellAttr * self, int * h_align, int * v_align);
wxGridCellRenderer * wxGridCellAttr_GetRenderer(const wxGridCellAttr * self, const wxGrid * grid, int row, int col);
wxColour *wxGridCellAttr_GetTextColour(const wxGridCellAttr * self);
bool wxGridCellAttr_HasAlignment(const wxGridCellAttr * self);
bool wxGridCellAttr_HasBackgroundColour(const wxGridCellAttr * self);
bool wxGridCellAttr_HasEditor(const wxGridCellAttr * self);
bool wxGridCellAttr_HasFont(const wxGridCellAttr * self);
bool wxGridCellAttr_HasRenderer(const wxGridCellAttr * self);
bool wxGridCellAttr_HasTextColour(const wxGridCellAttr * self);
void wxGridCellAttr_IncRef(wxGridCellAttr * self);
bool wxGridCellAttr_IsReadOnly(const wxGridCellAttr * self);
void wxGridCellAttr_SetAlignment(wxGridCellAttr * self, int h_align, int v_align);
void wxGridCellAttr_SetBackgroundColour(wxGridCellAttr * self, const wxColour * col_back);
void wxGridCellAttr_SetDefAttr(wxGridCellAttr * self, wxGridCellAttr * def_attr);
void wxGridCellAttr_SetEditor(wxGridCellAttr * self, wxGridCellEditor * editor);
void wxGridCellAttr_SetFont(wxGridCellAttr * self, const wxFont * font);
void wxGridCellAttr_SetReadOnly(wxGridCellAttr * self, bool is_read_only);
void wxGridCellAttr_SetRenderer(wxGridCellAttr * self, wxGridCellRenderer * renderer);
void wxGridCellAttr_SetTextColour(wxGridCellAttr * self, const wxColour * col_text);
void wxGridCellAttr_MergeWith(wxGridCellAttr * self, wxGridCellAttr * mergefrom);
void wxGridCellAttr_SetSize(wxGridCellAttr * self, int num_rows, int num_cols);
void wxGridCellAttr_SetFitMode(wxGridCellAttr * self, wxGridFitMode fit_mode);
void wxGridCellAttr_SetOverflow(wxGridCellAttr * self, bool allow);
bool wxGridCellAttr_HasReadWriteMode(const wxGridCellAttr * self);
bool wxGridCellAttr_HasOverflowMode(const wxGridCellAttr * self);
bool wxGridCellAttr_HasSize(const wxGridCellAttr * self);
void wxGridCellAttr_GetSize(const wxGridCellAttr * self, int * num_rows, int * num_cols);
wxGridFitMode *wxGridCellAttr_GetFitMode(const wxGridCellAttr * self);
bool wxGridCellAttr_GetOverflow(const wxGridCellAttr * self);
bool wxGridCellAttr_CanOverflow(const wxGridCellAttr * self);
// Mix-in(s) to wxGridCellAttr
wxRefCounter *wxGridCellAttr_AsRefCounter(wxGridCellAttr* obj);

// CLASS: wxGridCellAutoWrapStringEditor
void wxGridCellAutoWrapStringEditor_delete(wxGridCellAutoWrapStringEditor *self);
wxGridCellAutoWrapStringEditor *wxGridCellAutoWrapStringEditor_new();
// Mix-in(s) to wxGridCellAutoWrapStringEditor
wxRefCounter *wxGridCellAutoWrapStringEditor_AsRefCounter(wxGridCellAutoWrapStringEditor* obj);

// CLASS: wxGridCellAutoWrapStringRenderer
void wxGridCellAutoWrapStringRenderer_delete(wxGridCellAutoWrapStringRenderer *self);
wxGridCellAutoWrapStringRenderer *wxGridCellAutoWrapStringRenderer_new();
// Mix-in(s) to wxGridCellAutoWrapStringRenderer
wxRefCounter *wxGridCellAutoWrapStringRenderer_AsRefCounter(wxGridCellAutoWrapStringRenderer* obj);

// CLASS: wxGridCellBoolEditor
void wxGridCellBoolEditor_delete(wxGridCellBoolEditor *self);
wxGridCellBoolEditor *wxGridCellBoolEditor_new();
bool wxGridCellBoolEditor_IsTrueValue(const wxString * value);
void wxGridCellBoolEditor_UseStringValues(const wxString * value_true, const wxString * value_false);
// Mix-in(s) to wxGridCellBoolEditor
wxRefCounter *wxGridCellBoolEditor_AsRefCounter(wxGridCellBoolEditor* obj);

// CLASS: wxGridCellBoolRenderer
void wxGridCellBoolRenderer_delete(wxGridCellBoolRenderer *self);
wxGridCellBoolRenderer *wxGridCellBoolRenderer_new();
// Mix-in(s) to wxGridCellBoolRenderer
wxRefCounter *wxGridCellBoolRenderer_AsRefCounter(wxGridCellBoolRenderer* obj);

// CLASS: wxGridCellChoiceEditor
void wxGridCellChoiceEditor_delete(wxGridCellChoiceEditor *self);
wxGridCellChoiceEditor *wxGridCellChoiceEditor_new1(const wxArrayString * choices, bool allow_others);
void wxGridCellChoiceEditor_SetParameters(wxGridCellChoiceEditor * self, const wxString * params);
// Mix-in(s) to wxGridCellChoiceEditor
wxRefCounter *wxGridCellChoiceEditor_AsRefCounter(wxGridCellChoiceEditor* obj);

// CLASS: wxGridCellDateEditor
void wxGridCellDateEditor_delete(wxGridCellDateEditor *self);
wxGridCellDateEditor *wxGridCellDateEditor_new(const wxString * format);
// Mix-in(s) to wxGridCellDateEditor
wxRefCounter *wxGridCellDateEditor_AsRefCounter(wxGridCellDateEditor* obj);

// CLASS: wxGridCellDateRenderer
void wxGridCellDateRenderer_delete(wxGridCellDateRenderer *self);
wxGridCellDateRenderer *wxGridCellDateRenderer_new(const wxString * outformat);
void wxGridCellDateRenderer_SetParameters(wxGridCellDateRenderer * self, const wxString * params);
// Mix-in(s) to wxGridCellDateRenderer
wxRefCounter *wxGridCellDateRenderer_AsRefCounter(wxGridCellDateRenderer* obj);

// CLASS: wxGridCellDateTimeRenderer
void wxGridCellDateTimeRenderer_delete(wxGridCellDateTimeRenderer *self);
wxGridCellDateTimeRenderer *wxGridCellDateTimeRenderer_new(const wxString * outformat, const wxString * informat);
// Mix-in(s) to wxGridCellDateTimeRenderer
wxRefCounter *wxGridCellDateTimeRenderer_AsRefCounter(wxGridCellDateTimeRenderer* obj);

// CLASS: wxGridCellEditor
void wxGridCellEditor_delete(wxGridCellEditor *self);
wxGridCellEditor *wxGridCellEditor_new();
void wxGridCellEditor_BeginEdit(wxGridCellEditor * self, int row, int col, wxGrid * grid);
wxGridCellEditor * wxGridCellEditor_Clone(const wxGridCellEditor * self);
void wxGridCellEditor_Create(wxGridCellEditor * self, wxWindow * parent, wxWindowID id, wxEvtHandler * evt_handler);
void wxGridCellEditor_Destroy(wxGridCellEditor * self);
bool wxGridCellEditor_EndEdit(wxGridCellEditor * self, int row, int col, const wxGrid * grid, const wxString * oldval, wxString * newval);
void wxGridCellEditor_ApplyEdit(wxGridCellEditor * self, int row, int col, wxGrid * grid);
void wxGridCellEditor_HandleReturn(wxGridCellEditor * self, wxKeyEvent * event);
bool wxGridCellEditor_IsCreated(wxGridCellEditor * self);
void wxGridCellEditor_PaintBackground(wxGridCellEditor * self, wxDC * dc, const wxRect * rect_cell, const wxGridCellAttr * attr);
void wxGridCellEditor_Reset(wxGridCellEditor * self);
void wxGridCellEditor_SetSize(wxGridCellEditor * self, const wxRect * rect);
void wxGridCellEditor_Show(wxGridCellEditor * self, bool show, wxGridCellAttr * attr);
void wxGridCellEditor_StartingClick(wxGridCellEditor * self);
void wxGridCellEditor_StartingKey(wxGridCellEditor * self, wxKeyEvent * event);
bool wxGridCellEditor_IsAcceptedKey(wxGridCellEditor * self, wxKeyEvent * event);
wxString *wxGridCellEditor_GetValue(const wxGridCellEditor * self);
wxWindow * wxGridCellEditor_GetWindow(const wxGridCellEditor * self);
void wxGridCellEditor_SetWindow(wxGridCellEditor * self, wxWindow * window);
wxControl * wxGridCellEditor_GetControl(wxGridCellEditor * self);
void wxGridCellEditor_SetControl(wxGridCellEditor * self, wxControl * control);
wxGridActivationResult *wxGridCellEditor_TryActivate(wxGridCellEditor * self, int row, int col, wxGrid * grid, const wxGridActivationSource * act_source);
void wxGridCellEditor_DoActivate(wxGridCellEditor * self, int row, int col, wxGrid * grid);
// Mix-in(s) to wxGridCellEditor
wxRefCounter *wxGridCellEditor_AsRefCounter(wxGridCellEditor* obj);

// CLASS: wxGridCellEnumEditor
void wxGridCellEnumEditor_delete(wxGridCellEnumEditor *self);
wxGridCellEnumEditor *wxGridCellEnumEditor_new(const wxString * choices);
// Mix-in(s) to wxGridCellEnumEditor
wxRefCounter *wxGridCellEnumEditor_AsRefCounter(wxGridCellEnumEditor* obj);

// CLASS: wxGridCellEnumRenderer
void wxGridCellEnumRenderer_delete(wxGridCellEnumRenderer *self);
wxGridCellEnumRenderer *wxGridCellEnumRenderer_new(const wxString * choices);
void wxGridCellEnumRenderer_SetParameters(wxGridCellEnumRenderer * self, const wxString * params);
// Mix-in(s) to wxGridCellEnumRenderer
wxRefCounter *wxGridCellEnumRenderer_AsRefCounter(wxGridCellEnumRenderer* obj);

// CLASS: wxGridCellFloatEditor
void wxGridCellFloatEditor_delete(wxGridCellFloatEditor *self);
wxGridCellFloatEditor *wxGridCellFloatEditor_new(int width, int precision, int format);
// Mix-in(s) to wxGridCellFloatEditor
wxRefCounter *wxGridCellFloatEditor_AsRefCounter(wxGridCellFloatEditor* obj);

// CLASS: wxGridCellFloatRenderer
void wxGridCellFloatRenderer_delete(wxGridCellFloatRenderer *self);
wxGridCellFloatRenderer *wxGridCellFloatRenderer_new(int width, int precision, int format);
int wxGridCellFloatRenderer_GetFormat(const wxGridCellFloatRenderer * self);
int wxGridCellFloatRenderer_GetPrecision(const wxGridCellFloatRenderer * self);
int wxGridCellFloatRenderer_GetWidth(const wxGridCellFloatRenderer * self);
void wxGridCellFloatRenderer_SetFormat(wxGridCellFloatRenderer * self, int format);
void wxGridCellFloatRenderer_SetParameters(wxGridCellFloatRenderer * self, const wxString * params);
void wxGridCellFloatRenderer_SetPrecision(wxGridCellFloatRenderer * self, int precision);
void wxGridCellFloatRenderer_SetWidth(wxGridCellFloatRenderer * self, int width);
// Mix-in(s) to wxGridCellFloatRenderer
wxRefCounter *wxGridCellFloatRenderer_AsRefCounter(wxGridCellFloatRenderer* obj);

// CLASS: wxGridCellNumberEditor
void wxGridCellNumberEditor_delete(wxGridCellNumberEditor *self);
wxGridCellNumberEditor *wxGridCellNumberEditor_new(int min, int max);
// Mix-in(s) to wxGridCellNumberEditor
wxRefCounter *wxGridCellNumberEditor_AsRefCounter(wxGridCellNumberEditor* obj);

// CLASS: wxGridCellNumberRenderer
void wxGridCellNumberRenderer_delete(wxGridCellNumberRenderer *self);
wxGridCellNumberRenderer *wxGridCellNumberRenderer_new();
// Mix-in(s) to wxGridCellNumberRenderer
wxRefCounter *wxGridCellNumberRenderer_AsRefCounter(wxGridCellNumberRenderer* obj);

// CLASS: wxGridCellRenderer
void wxGridCellRenderer_delete(wxGridCellRenderer *self);
wxGridCellRenderer *wxGridCellRenderer_new();
wxGridCellRenderer * wxGridCellRenderer_Clone(const wxGridCellRenderer * self);
void wxGridCellRenderer_Draw(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, const wxRect * rect, int row, int col, bool is_selected);
wxSize *wxGridCellRenderer_GetBestSize(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col);
int wxGridCellRenderer_GetBestHeight(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col, int width);
int wxGridCellRenderer_GetBestWidth(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col, int height);
wxSize *wxGridCellRenderer_GetMaxBestSize(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc);
// Mix-in(s) to wxGridCellRenderer
wxRefCounter *wxGridCellRenderer_AsRefCounter(wxGridCellRenderer* obj);

// CLASS: wxGridCellStringRenderer
void wxGridCellStringRenderer_delete(wxGridCellStringRenderer *self);
wxGridCellStringRenderer *wxGridCellStringRenderer_new();
// Mix-in(s) to wxGridCellStringRenderer
wxRefCounter *wxGridCellStringRenderer_AsRefCounter(wxGridCellStringRenderer* obj);

// CLASS: wxGridCellTextEditor
void wxGridCellTextEditor_delete(wxGridCellTextEditor *self);
wxGridCellTextEditor *wxGridCellTextEditor_new(size_t max_chars);
void wxGridCellTextEditor_SetParameters(wxGridCellTextEditor * self, const wxString * params);
void wxGridCellTextEditor_SetValidator(wxGridCellTextEditor * self, const wxValidator * validator);
// Mix-in(s) to wxGridCellTextEditor
wxRefCounter *wxGridCellTextEditor_AsRefCounter(wxGridCellTextEditor* obj);

// CLASS: wxGridEditorCreatedEvent
wxClassInfo *wxGridEditorCreatedEvent_CLASSINFO();
wxGridEditorCreatedEvent *wxGridEditorCreatedEvent_new();
int wxGridEditorCreatedEvent_GetCol(const wxGridEditorCreatedEvent * self);
wxControl * wxGridEditorCreatedEvent_GetControl(wxGridEditorCreatedEvent * self);
int wxGridEditorCreatedEvent_GetRow(const wxGridEditorCreatedEvent * self);
wxWindow * wxGridEditorCreatedEvent_GetWindow(const wxGridEditorCreatedEvent * self);
void wxGridEditorCreatedEvent_SetCol(wxGridEditorCreatedEvent * self, int col);
void wxGridEditorCreatedEvent_SetControl(wxGridEditorCreatedEvent * self, wxControl * ctrl);
void wxGridEditorCreatedEvent_SetRow(wxGridEditorCreatedEvent * self, int row);
void wxGridEditorCreatedEvent_SetWindow(wxGridEditorCreatedEvent * self, wxWindow * window);

// CLASS: wxGridEvent
wxClassInfo *wxGridEvent_CLASSINFO();
wxGridEvent *wxGridEvent_new();
bool wxGridEvent_AltDown(const wxGridEvent * self);
bool wxGridEvent_ControlDown(const wxGridEvent * self);
int wxGridEvent_GetCol(const wxGridEvent * self);
wxPoint *wxGridEvent_GetPosition(const wxGridEvent * self);
int wxGridEvent_GetRow(const wxGridEvent * self);
bool wxGridEvent_MetaDown(const wxGridEvent * self);
bool wxGridEvent_Selecting(const wxGridEvent * self);
bool wxGridEvent_ShiftDown(const wxGridEvent * self);

// CLASS: wxGridFitMode
void wxGridFitMode_delete(wxGridFitMode *self);
wxGridFitMode *wxGridFitMode_new();
bool wxGridFitMode_IsSpecified(const wxGridFitMode * self);
bool wxGridFitMode_IsClip(const wxGridFitMode * self);
bool wxGridFitMode_IsOverflow(const wxGridFitMode * self);
wxEllipsizeMode wxGridFitMode_GetEllipsizeMode(const wxGridFitMode * self);
wxGridFitMode *wxGridFitMode_Clip();
wxGridFitMode *wxGridFitMode_Overflow();
wxGridFitMode *wxGridFitMode_Ellipsize(wxEllipsizeMode ellipsize);

// CLASS: wxGridRangeSelectEvent
wxClassInfo *wxGridRangeSelectEvent_CLASSINFO();
wxGridRangeSelectEvent *wxGridRangeSelectEvent_new();
bool wxGridRangeSelectEvent_AltDown(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_ControlDown(const wxGridRangeSelectEvent * self);
wxGridCellCoords *wxGridRangeSelectEvent_GetBottomRightCoords(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetBottomRow(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetLeftCol(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetRightCol(const wxGridRangeSelectEvent * self);
wxGridCellCoords *wxGridRangeSelectEvent_GetTopLeftCoords(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetTopRow(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_MetaDown(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_Selecting(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_ShiftDown(const wxGridRangeSelectEvent * self);

// CLASS: wxGridSizeEvent
wxClassInfo *wxGridSizeEvent_CLASSINFO();
wxGridSizeEvent *wxGridSizeEvent_new();
bool wxGridSizeEvent_AltDown(const wxGridSizeEvent * self);
bool wxGridSizeEvent_ControlDown(const wxGridSizeEvent * self);
wxPoint *wxGridSizeEvent_GetPosition(const wxGridSizeEvent * self);
int wxGridSizeEvent_GetRowOrCol(const wxGridSizeEvent * self);
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
wxString *wxGridTableBase_GetCornerLabelValue(const wxGridTableBase * self);
void wxGridTableBase_SetRowLabelValue(wxGridTableBase * self, int row, const wxString * label);
void wxGridTableBase_SetColLabelValue(wxGridTableBase * self, int col, const wxString * label);
void wxGridTableBase_SetAttrProvider(wxGridTableBase * self, wxGridCellAttrProvider * attr_provider);
wxGridCellAttrProvider * wxGridTableBase_GetAttrProvider(const wxGridTableBase * self);
void wxGridTableBase_SetAttr(wxGridTableBase * self, wxGridCellAttr * attr, int row, int col);
void wxGridTableBase_SetRowAttr(wxGridTableBase * self, wxGridCellAttr * attr, int row);
void wxGridTableBase_SetColAttr(wxGridTableBase * self, wxGridCellAttr * attr, int col);
bool wxGridTableBase_CanHaveAttributes(wxGridTableBase * self);
bool wxGridTableBase_CanMeasureColUsingSameAttr(const wxGridTableBase * self, int col);
wxGridTableBase *wxGridTableBase_new();
int wxGridTableBase_GetNumberRows(wxGridTableBase * self);
int wxGridTableBase_GetNumberCols(wxGridTableBase * self);
int wxGridTableBase_GetRowsCount(const wxGridTableBase * self);
int wxGridTableBase_GetColsCount(const wxGridTableBase * self);

// CLASS: wxGridUpdateLocker
void wxGridUpdateLocker_delete(wxGridUpdateLocker *self);
wxGridUpdateLocker *wxGridUpdateLocker_new(wxGrid * grid);
void wxGridUpdateLocker_Create(wxGridUpdateLocker * self, wxGrid * grid);

// CLASS: wxHScrolledWindow
wxClassInfo *wxHScrolledWindow_CLASSINFO();
wxHScrolledWindow *wxHScrolledWindow_new();
wxHScrolledWindow *wxHScrolledWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxHScrolledWindow_Create(wxHScrolledWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxHScrolledWindow
wxVarHScrollHelper *wxHScrolledWindow_AsVarHScrollHelper(wxHScrolledWindow* obj);
wxTrackable *wxHScrolledWindow_AsTrackable(wxHScrolledWindow* obj);

// CLASS: wxHTMLDataObject
void wxHTMLDataObject_delete(wxHTMLDataObject *self);
wxHTMLDataObject *wxHTMLDataObject_new(const wxString * html);
wxString *wxHTMLDataObject_GetHTML(const wxHTMLDataObject * self);
void wxHTMLDataObject_SetHTML(wxHTMLDataObject * self, const wxString * html);

// CLASS: wxHVScrolledWindow
wxClassInfo *wxHVScrolledWindow_CLASSINFO();
wxHVScrolledWindow *wxHVScrolledWindow_new();
wxHVScrolledWindow *wxHVScrolledWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxHVScrolledWindow_Create(wxHVScrolledWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxHVScrolledWindow
wxVarHVScrollHelper *wxHVScrolledWindow_AsVarHVScrollHelper(wxHVScrolledWindow* obj);
wxTrackable *wxHVScrolledWindow_AsTrackable(wxHVScrolledWindow* obj);

// CLASS: wxHeaderColumn
void wxHeaderColumn_delete(wxHeaderColumn *self);
wxString *wxHeaderColumn_GetTitle(const wxHeaderColumn * self);
wxBitmap *wxHeaderColumn_GetBitmap(const wxHeaderColumn * self);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxHeaderColumn_GetBitmapBundle(const wxHeaderColumn * self);
#endif
int wxHeaderColumn_GetWidth(const wxHeaderColumn * self);
int wxHeaderColumn_GetMinWidth(const wxHeaderColumn * self);
wxAlignment wxHeaderColumn_GetAlignment(const wxHeaderColumn * self);
int wxHeaderColumn_GetFlags(const wxHeaderColumn * self);
bool wxHeaderColumn_HasFlag(const wxHeaderColumn * self, int flag);
bool wxHeaderColumn_IsResizeable(const wxHeaderColumn * self);
bool wxHeaderColumn_IsSortable(const wxHeaderColumn * self);
bool wxHeaderColumn_IsReorderable(const wxHeaderColumn * self);
bool wxHeaderColumn_IsHidden(const wxHeaderColumn * self);
bool wxHeaderColumn_IsShown(const wxHeaderColumn * self);
bool wxHeaderColumn_IsSortKey(const wxHeaderColumn * self);
bool wxHeaderColumn_IsSortOrderAscending(const wxHeaderColumn * self);

// CLASS: wxHeaderColumnSimple
void wxHeaderColumnSimple_delete(wxHeaderColumnSimple *self);
wxHeaderColumnSimple *wxHeaderColumnSimple_new(const wxString * title, int width, wxAlignment align, int flags);
wxHeaderColumnSimple *wxHeaderColumnSimple_new1(const wxBitmapBundle * bitmap, int width, wxAlignment align, int flags);

// CLASS: wxHeaderCtrl
wxClassInfo *wxHeaderCtrl_CLASSINFO();
bool wxHeaderCtrl_Create(wxHeaderCtrl * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxHeaderCtrl_SetColumnCount(wxHeaderCtrl * self, unsigned int count);
unsigned int wxHeaderCtrl_GetColumnCount(const wxHeaderCtrl * self);
bool wxHeaderCtrl_IsEmpty(const wxHeaderCtrl * self);
void wxHeaderCtrl_UpdateColumn(wxHeaderCtrl * self, unsigned int idx);
void wxHeaderCtrl_SetColumnsOrder(wxHeaderCtrl * self, const wxArrayInt * order);
wxArrayInt *wxHeaderCtrl_GetColumnsOrder(const wxHeaderCtrl * self);
unsigned int wxHeaderCtrl_GetColumnAt(const wxHeaderCtrl * self, unsigned int pos);
unsigned int wxHeaderCtrl_GetColumnPos(const wxHeaderCtrl * self, unsigned int idx);
void wxHeaderCtrl_ResetColumnsOrder(wxHeaderCtrl * self);
bool wxHeaderCtrl_ShowColumnsMenu(wxHeaderCtrl * self, const wxPoint * pt, const wxString * title);
void wxHeaderCtrl_AddColumnsItems(wxHeaderCtrl * self, wxMenu * menu, int id_columns_base);
bool wxHeaderCtrl_ShowCustomizeDialog(wxHeaderCtrl * self);
int wxHeaderCtrl_GetColumnTitleWidth(wxHeaderCtrl * self, const wxHeaderColumn * col);
#if wxCHECK_VERSION(3, 1, 0)
int wxHeaderCtrl_GetColumnTitleWidth1(wxHeaderCtrl * self, unsigned int idx);
#endif
void wxHeaderCtrl_MoveColumnInOrderArray(wxArrayInt * order, unsigned int idx, unsigned int pos);
// Mix-in(s) to wxHeaderCtrl
wxTrackable *wxHeaderCtrl_AsTrackable(wxHeaderCtrl* obj);

// CLASS: wxHeaderCtrlEvent
wxClassInfo *wxHeaderCtrlEvent_CLASSINFO();
wxHeaderCtrlEvent *wxHeaderCtrlEvent_new1(const wxHeaderCtrlEvent * event);
int wxHeaderCtrlEvent_GetColumn(const wxHeaderCtrlEvent * self);
void wxHeaderCtrlEvent_SetColumn(wxHeaderCtrlEvent * self, int col);
int wxHeaderCtrlEvent_GetWidth(const wxHeaderCtrlEvent * self);
void wxHeaderCtrlEvent_SetWidth(wxHeaderCtrlEvent * self, int width);
unsigned int wxHeaderCtrlEvent_GetNewOrder(const wxHeaderCtrlEvent * self);
void wxHeaderCtrlEvent_SetNewOrder(wxHeaderCtrlEvent * self, unsigned int order);

// CLASS: wxHeaderCtrlSimple
wxClassInfo *wxHeaderCtrlSimple_CLASSINFO();
wxHeaderCtrlSimple *wxHeaderCtrlSimple_new();
wxHeaderCtrlSimple *wxHeaderCtrlSimple_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxHeaderCtrlSimple_InsertColumn(wxHeaderCtrlSimple * self, const wxHeaderColumnSimple * col, unsigned int idx);
void wxHeaderCtrlSimple_AppendColumn(wxHeaderCtrlSimple * self, const wxHeaderColumnSimple * col);
void wxHeaderCtrlSimple_DeleteColumn(wxHeaderCtrlSimple * self, unsigned int idx);
void wxHeaderCtrlSimple_ShowColumn(wxHeaderCtrlSimple * self, unsigned int idx, bool show);
void wxHeaderCtrlSimple_HideColumn(wxHeaderCtrlSimple * self, unsigned int idx);
void wxHeaderCtrlSimple_ShowSortIndicator(wxHeaderCtrlSimple * self, unsigned int idx, bool sort_order);
void wxHeaderCtrlSimple_RemoveSortIndicator(wxHeaderCtrlSimple * self);
// Mix-in(s) to wxHeaderCtrlSimple
wxTrackable *wxHeaderCtrlSimple_AsTrackable(wxHeaderCtrlSimple* obj);

// CLASS: wxHelpController
wxClassInfo *wxHelpController_CLASSINFO();
wxHelpController *wxHelpController_new(wxWindow * parent_window);

// CLASS: wxHelpControllerBase
wxClassInfo *wxHelpControllerBase_CLASSINFO();
wxHelpControllerBase *wxHelpControllerBase_new(wxWindow * parent_window);
bool wxHelpControllerBase_DisplayBlock(wxHelpControllerBase * self, long block_no);
bool wxHelpControllerBase_DisplayContents(wxHelpControllerBase * self);
bool wxHelpControllerBase_DisplayContextPopup(wxHelpControllerBase * self, int context_id);
bool wxHelpControllerBase_DisplaySection(wxHelpControllerBase * self, const wxString * section);
bool wxHelpControllerBase_DisplaySection1(wxHelpControllerBase * self, int section_no);
bool wxHelpControllerBase_DisplayTextPopup(wxHelpControllerBase * self, const wxString * text, const wxPoint * pos);
wxFrame * wxHelpControllerBase_GetFrameParameters(wxHelpControllerBase * self, wxSize * size, wxPoint * pos, bool * new_frame_each_time);
wxWindow * wxHelpControllerBase_GetParentWindow(const wxHelpControllerBase * self);
bool wxHelpControllerBase_Initialize(wxHelpControllerBase * self, const wxString * file);
bool wxHelpControllerBase_LoadFile(wxHelpControllerBase * self, const wxString * file);
void wxHelpControllerBase_OnQuit(wxHelpControllerBase * self);
bool wxHelpControllerBase_Quit(wxHelpControllerBase * self);
void wxHelpControllerBase_SetFrameParameters(wxHelpControllerBase * self, const wxString * title_format, const wxSize * size, const wxPoint * pos, bool new_frame_each_time);
void wxHelpControllerBase_SetParentWindow(wxHelpControllerBase * self, wxWindow * parent_window);
void wxHelpControllerBase_SetViewer(wxHelpControllerBase * self, const wxString * viewer, long flags);

// CLASS: wxHelpControllerHelpProvider
void wxHelpControllerHelpProvider_delete(wxHelpControllerHelpProvider *self);
wxHelpControllerHelpProvider *wxHelpControllerHelpProvider_new(wxHelpControllerBase * hc);
wxHelpControllerBase * wxHelpControllerHelpProvider_GetHelpController(const wxHelpControllerHelpProvider * self);
void wxHelpControllerHelpProvider_SetHelpController(wxHelpControllerHelpProvider * self, wxHelpControllerBase * hc);

// CLASS: wxHelpEvent
wxClassInfo *wxHelpEvent_CLASSINFO();
wxPoint *wxHelpEvent_GetPosition(const wxHelpEvent * self);
void wxHelpEvent_SetPosition(wxHelpEvent * self, const wxPoint * pt);

// CLASS: wxHelpProvider
void wxHelpProvider_delete(wxHelpProvider *self);
void wxHelpProvider_AddHelp(wxHelpProvider * self, wxWindow * window, const wxString * text);
void wxHelpProvider_AddHelp1(wxHelpProvider * self, wxWindowID id, const wxString * text);
wxString *wxHelpProvider_GetHelp(wxHelpProvider * self, const wxWindow * window);
void wxHelpProvider_RemoveHelp(wxHelpProvider * self, wxWindow * window);
bool wxHelpProvider_ShowHelp(wxHelpProvider * self, wxWindow * window);
wxHelpProvider * wxHelpProvider_Get();
wxHelpProvider * wxHelpProvider_Set(wxHelpProvider * help_provider);

// CLASS: wxHyperlinkCtrl
wxClassInfo *wxHyperlinkCtrl_CLASSINFO();
wxHyperlinkCtrl *wxHyperlinkCtrl_new();
wxHyperlinkCtrl *wxHyperlinkCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxString * url, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxHyperlinkCtrl_Create(wxHyperlinkCtrl * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxString * url, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxColour *wxHyperlinkCtrl_GetHoverColour(const wxHyperlinkCtrl * self);
wxColour *wxHyperlinkCtrl_GetNormalColour(const wxHyperlinkCtrl * self);
wxString *wxHyperlinkCtrl_GetURL(const wxHyperlinkCtrl * self);
bool wxHyperlinkCtrl_GetVisited(const wxHyperlinkCtrl * self);
wxColour *wxHyperlinkCtrl_GetVisitedColour(const wxHyperlinkCtrl * self);
void wxHyperlinkCtrl_SetHoverColour(wxHyperlinkCtrl * self, const wxColour * colour);
void wxHyperlinkCtrl_SetNormalColour(wxHyperlinkCtrl * self, const wxColour * colour);
void wxHyperlinkCtrl_SetURL(wxHyperlinkCtrl * self, const wxString * url);
void wxHyperlinkCtrl_SetVisited(wxHyperlinkCtrl * self, bool visited);
void wxHyperlinkCtrl_SetVisitedColour(wxHyperlinkCtrl * self, const wxColour * colour);
// Mix-in(s) to wxHyperlinkCtrl
wxTrackable *wxHyperlinkCtrl_AsTrackable(wxHyperlinkCtrl* obj);

// CLASS: wxHyperlinkEvent
wxClassInfo *wxHyperlinkEvent_CLASSINFO();
wxHyperlinkEvent *wxHyperlinkEvent_new(wxObject * generator, int id, const wxString * url);
wxString *wxHyperlinkEvent_GetURL(const wxHyperlinkEvent * self);
void wxHyperlinkEvent_SetURL(wxHyperlinkEvent * self, const wxString * url);

// CLASS: wxIFFHandler
wxClassInfo *wxIFFHandler_CLASSINFO();
wxIFFHandler *wxIFFHandler_new();

// CLASS: wxIcon
wxClassInfo *wxIcon_CLASSINFO();
wxIcon *wxIcon_new();
wxIcon *wxIcon_new1(const wxIcon * icon);
wxIcon *wxIcon_new3(const char *const * bits);
wxIcon *wxIcon_new5(const wxIconLocation * loc);
void wxIcon_CopyFromBitmap(wxIcon * self, const wxBitmap * bmp);
int wxIcon_GetDepth(const wxIcon * self);
int wxIcon_GetHeight(const wxIcon * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxIcon_GetLogicalHeight(const wxIcon * self);
wxSize *wxIcon_GetLogicalSize(const wxIcon * self);
double wxIcon_GetLogicalWidth(const wxIcon * self);
#endif
double wxIcon_GetScaleFactor(const wxIcon * self);
wxSize *wxIcon_GetSize(const wxIcon * self);
int wxIcon_GetWidth(const wxIcon * self);
bool wxIcon_IsOk(const wxIcon * self);
#if wxCHECK_VERSION(3, 1, 7)
void wxIcon_SetDepth(wxIcon * self, int depth);
void wxIcon_SetHeight(wxIcon * self, int height);
void wxIcon_SetWidth(wxIcon * self, int width);
#endif

// CLASS: wxIconBundle
wxClassInfo *wxIconBundle_CLASSINFO();
wxIconBundle *wxIconBundle_new();
wxIconBundle *wxIconBundle_new3(const wxIcon * icon);
wxIconBundle *wxIconBundle_new5(const wxIconBundle * ic);
void wxIconBundle_AddIcon3(wxIconBundle * self, const wxIcon * icon);
wxIcon *wxIconBundle_GetIcon(const wxIconBundle * self, const wxSize * size, int flags);
wxIcon *wxIconBundle_GetIcon1(const wxIconBundle * self, wxCoord size, int flags);
wxIcon *wxIconBundle_GetIconOfExactSize(const wxIconBundle * self, const wxSize * size);
size_t wxIconBundle_GetIconCount(const wxIconBundle * self);
wxIcon *wxIconBundle_GetIconByIndex(const wxIconBundle * self, size_t n);
bool wxIconBundle_IsEmpty(const wxIconBundle * self);

// CLASS: wxIconizeEvent
wxClassInfo *wxIconizeEvent_CLASSINFO();
wxIconizeEvent *wxIconizeEvent_new(int id, bool iconized);
bool wxIconizeEvent_IsIconized(const wxIconizeEvent * self);
bool wxIconizeEvent_Iconized(const wxIconizeEvent * self);

// CLASS: wxIdManager
void wxIdManager_delete(wxIdManager *self);
wxWindowID wxIdManager_ReserveId(int count);
void wxIdManager_UnreserveId(wxWindowID id, int count);

// CLASS: wxImage
wxClassInfo *wxImage_CLASSINFO();
wxImage *wxImage_Copy(const wxImage * self);
bool wxImage_Create(wxImage * self, int width, int height, bool clear);
bool wxImage_Create1(wxImage * self, const wxSize * sz, bool clear);
bool wxImage_Create2(wxImage * self, int width, int height, unsigned char * data, bool static_data);
bool wxImage_Create3(wxImage * self, const wxSize * sz, unsigned char * data, bool static_data);
bool wxImage_Create4(wxImage * self, int width, int height, unsigned char * data, unsigned char * alpha, bool static_data);
bool wxImage_Create5(wxImage * self, const wxSize * sz, unsigned char * data, unsigned char * alpha, bool static_data);
void wxImage_Destroy(wxImage * self);
void wxImage_InitAlpha(wxImage * self);
wxImage *wxImage_Blur(const wxImage * self, int blur_radius);
wxImage *wxImage_BlurHorizontal(const wxImage * self, int blur_radius);
wxImage *wxImage_BlurVertical(const wxImage * self, int blur_radius);
wxImage *wxImage_Mirror(const wxImage * self, bool horizontally);
wxImage * wxImage_Resize(wxImage * self, const wxSize * size, const wxPoint * pos, int red, int green, int blue);
wxImage *wxImage_Rotate(const wxImage * self, double angle, const wxPoint * rotation_centre, bool interpolating, wxPoint * offset_after_rotation);
wxImage *wxImage_Rotate90(const wxImage * self, bool clockwise);
wxImage *wxImage_Rotate180(const wxImage * self);
void wxImage_RotateHue(wxImage * self, double angle);
void wxImage_ChangeSaturation(wxImage * self, double factor);
void wxImage_ChangeBrightness(wxImage * self, double factor);
void wxImage_ChangeHSV(wxImage * self, double angle_h, double factor_s, double factor_v);
wxImage *wxImage_Size(const wxImage * self, const wxSize * size, const wxPoint * pos, int red, int green, int blue);
wxImage *wxImage_ConvertToGreyscale(const wxImage * self, double weight_r, double weight_g, double weight_b);
wxImage *wxImage_ConvertToGreyscale1(const wxImage * self);
wxImage *wxImage_ChangeLightness(const wxImage * self, int alpha);
unsigned char * wxImage_GetAlpha(const wxImage * self);
unsigned char * wxImage_GetData(const wxImage * self);
int wxImage_GetWidth(const wxImage * self);
int wxImage_GetHeight(const wxImage * self);
wxSize *wxImage_GetSize(const wxImage * self);
wxString *wxImage_GetOption(const wxImage * self, const wxString * name);
int wxImage_GetOptionInt(const wxImage * self, const wxString * name);
bool wxImage_GetOrFindMaskColour(const wxImage * self, unsigned char * r, unsigned char * g, unsigned char * b);
wxPalette *wxImage_GetPalette(const wxImage * self);
wxImage *wxImage_GetSubImage(const wxImage * self, const wxRect * rect);
bool wxImage_HasAlpha(const wxImage * self);
bool wxImage_HasMask(const wxImage * self);
bool wxImage_HasOption(const wxImage * self, const wxString * name);
bool wxImage_IsOk(const wxImage * self);
bool wxImage_LoadFile2(wxImage * self, const wxString * name, const wxString * mimetype, int index);
bool wxImage_LoadFile3(wxImage * self, wxInputStream * stream, const wxString * mimetype, int index);
bool wxImage_SaveFile(const wxImage * self, wxOutputStream * stream, const wxString * mimetype);
bool wxImage_SaveFile2(const wxImage * self, const wxString * name, const wxString * mimetype);
bool wxImage_SaveFile3(const wxImage * self, const wxString * name);
void wxImage_SetAlpha(wxImage * self, unsigned char * alpha, bool static_data);
void wxImage_ClearAlpha(wxImage * self);
void wxImage_SetData(wxImage * self, unsigned char * data, bool static_data);
void wxImage_SetData1(wxImage * self, unsigned char * data, int new_width, int new_height, bool static_data);
void wxImage_SetLoadFlags(wxImage * self, int flags);
void wxImage_SetMask(wxImage * self, bool has_mask);
void wxImage_SetOption(wxImage * self, const wxString * name, const wxString * value);
void wxImage_SetOption1(wxImage * self, const wxString * name, int value);
void wxImage_SetPalette(wxImage * self, const wxPalette * palette);
void wxImage_SetDefaultLoadFlags(int flags);
int wxImage_GetLoadFlags(const wxImage * self);
void wxImage_AddHandler(wxImageHandler * handler);
void wxImage_CleanUpHandlers();
wxImageHandler * wxImage_FindHandler(const wxString * name);
wxImageHandler * wxImage_FindHandlerMime(const wxString * mimetype);
wxList * wxImage_GetHandlers();
void wxImage_InitStandardHandlers();
void wxImage_InsertHandler(wxImageHandler * handler);
bool wxImage_RemoveHandler(const wxString * name);
bool wxImage_CanRead(const wxString * filename);
bool wxImage_CanRead1(wxInputStream * stream);
int wxImage_GetDefaultLoadFlags();
wxString *wxImage_GetImageExtWildcard();
wxImage::HSVValue *wxImage_RGBtoHSV(const wxImage::RGBValue * rgb);
wxImage::RGBValue *wxImage_HSVtoRGB(const wxImage::HSVValue * hsv);
wxImage *wxImage_new();
wxImage *wxImage_new1(int width, int height, bool clear);
wxImage *wxImage_new2(const wxSize * sz, bool clear);
wxImage *wxImage_new3(int width, int height, unsigned char * data, bool static_data);
wxImage *wxImage_new4(const wxSize * sz, unsigned char * data, bool static_data);
wxImage *wxImage_new5(int width, int height, unsigned char * data, unsigned char * alpha, bool static_data);
wxImage *wxImage_new6(const wxSize * sz, unsigned char * data, unsigned char * alpha, bool static_data);
wxImage *wxImage_new7(const char *const * xpm_data);
wxImage *wxImage_new9(const wxString * name, const wxString * mimetype, int index);
wxImage *wxImage_new11(wxInputStream * stream, const wxString * mimetype, int index);

// CLASS: wxImageDataObject
void wxImageDataObject_delete(wxImageDataObject *self);
wxImageDataObject *wxImageDataObject_new(const wxImage * image);
wxImage *wxImageDataObject_GetImage(const wxImageDataObject * self);
void wxImageDataObject_SetImage(wxImageDataObject * self, const wxImage * image);

// CLASS: wxImageHandler
wxClassInfo *wxImageHandler_CLASSINFO();
wxImageHandler *wxImageHandler_new();
bool wxImageHandler_CanRead(wxImageHandler * self, wxInputStream * stream);
bool wxImageHandler_CanRead1(wxImageHandler * self, const wxString * filename);
wxString *wxImageHandler_GetExtension(const wxImageHandler * self);
wxArrayString *wxImageHandler_GetAltExtensions(const wxImageHandler * self);
int wxImageHandler_GetImageCount(wxImageHandler * self, wxInputStream * stream);
wxString *wxImageHandler_GetMimeType(const wxImageHandler * self);
wxString *wxImageHandler_GetName(const wxImageHandler * self);
bool wxImageHandler_LoadFile(wxImageHandler * self, wxImage * image, wxInputStream * stream, bool verbose, int index);
bool wxImageHandler_SaveFile(wxImageHandler * self, wxImage * image, wxOutputStream * stream, bool verbose);
void wxImageHandler_SetExtension(wxImageHandler * self, const wxString * extension);
void wxImageHandler_SetAltExtensions(wxImageHandler * self, const wxArrayString * extensions);
void wxImageHandler_SetMimeType(wxImageHandler * self, const wxString * mimetype);
void wxImageHandler_SetName(wxImageHandler * self, const wxString * name);
wxVersionInfo *wxImageHandler_GetLibraryVersionInfo();

// CLASS: wxImageList
wxClassInfo *wxImageList_CLASSINFO();
wxImageList *wxImageList_new();
wxImageList *wxImageList_new1(int width, int height, bool mask, int initial_count);
int wxImageList_Add(wxImageList * self, const wxBitmap * bitmap, const wxBitmap * mask);
int wxImageList_Add1(wxImageList * self, const wxBitmap * bitmap, const wxColour * mask_colour);
int wxImageList_Add2(wxImageList * self, const wxIcon * icon);
bool wxImageList_Create(wxImageList * self, int width, int height, bool mask, int initial_count);
void wxImageList_Destroy(wxImageList * self);
bool wxImageList_Draw(wxImageList * self, int index, wxDC * dc, int x, int y, int flags, bool solid_background);
wxBitmap *wxImageList_GetBitmap(const wxImageList * self, int index);
wxIcon *wxImageList_GetIcon(const wxImageList * self, int index);
int wxImageList_GetImageCount(const wxImageList * self);
bool wxImageList_GetSize(const wxImageList * self, int index, int * width, int * height);
wxSize *wxImageList_GetSize1(const wxImageList * self);
bool wxImageList_Remove(wxImageList * self, int index);
bool wxImageList_RemoveAll(wxImageList * self);
bool wxImageList_Replace(wxImageList * self, int index, const wxBitmap * bitmap, const wxBitmap * mask);
bool wxImageList_Replace1(wxImageList * self, int index, const wxIcon * icon);

// CLASS: wxInfoBar
wxClassInfo *wxInfoBar_CLASSINFO();
void wxInfoBar_SetEffectDuration(wxInfoBar * self, int duration);
int wxInfoBar_GetEffectDuration(const wxInfoBar * self);
wxInfoBar *wxInfoBar_new();
wxInfoBar *wxInfoBar_new1(wxWindow * parent, wxWindowID winid);
bool wxInfoBar_Create(wxInfoBar * self, wxWindow * parent, wxWindowID winid);
void wxInfoBar_AddButton(wxInfoBar * self, wxWindowID btnid, const wxString * label);
void wxInfoBar_Dismiss(wxInfoBar * self);
void wxInfoBar_RemoveButton(wxInfoBar * self, wxWindowID btnid);
void wxInfoBar_ShowMessage(wxInfoBar * self, const wxString * msg, int flags);
size_t wxInfoBar_GetButtonCount(const wxInfoBar * self);
wxWindowID wxInfoBar_GetButtonId(const wxInfoBar * self, size_t idx);
bool wxInfoBar_HasButtonId(const wxInfoBar * self, wxWindowID btnid);
// Mix-in(s) to wxInfoBar
wxTrackable *wxInfoBar_AsTrackable(wxInfoBar* obj);

// CLASS: wxInitDialogEvent
wxClassInfo *wxInitDialogEvent_CLASSINFO();
wxInitDialogEvent *wxInitDialogEvent_new(int id);

// CLASS: wxItemAttr
void wxItemAttr_delete(wxItemAttr *self);
wxItemAttr *wxItemAttr_new();
wxItemAttr *wxItemAttr_new1(const wxColour * col_text, const wxColour * col_back, const wxFont * font);
wxColour *wxItemAttr_GetBackgroundColour(const wxItemAttr * self);
wxFont *wxItemAttr_GetFont(const wxItemAttr * self);
wxColour *wxItemAttr_GetTextColour(const wxItemAttr * self);
bool wxItemAttr_HasBackgroundColour(const wxItemAttr * self);
bool wxItemAttr_HasColours(const wxItemAttr * self);
bool wxItemAttr_HasFont(const wxItemAttr * self);
bool wxItemAttr_HasTextColour(const wxItemAttr * self);
bool wxItemAttr_IsDefault(const wxItemAttr * self);
void wxItemAttr_SetBackgroundColour(wxItemAttr * self, const wxColour * colour);
void wxItemAttr_SetFont(wxItemAttr * self, const wxFont * font);
void wxItemAttr_SetTextColour(wxItemAttr * self, const wxColour * colour);

// CLASS: wxItemContainer
void wxItemContainer_delete(wxItemContainer *self);
int wxItemContainer_Append(wxItemContainer * self, const wxString * item);
int wxItemContainer_Append1(wxItemContainer * self, const wxString * item, void * client_data);
int wxItemContainer_Append2(wxItemContainer * self, const wxString * item, wxClientData * client_data);
int wxItemContainer_Append3(wxItemContainer * self, const wxArrayString * items);
int wxItemContainer_Append5(wxItemContainer * self, const wxArrayString * items, void ** client_data);
int wxItemContainer_Append6(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data);
int wxItemContainer_Append7(wxItemContainer * self, unsigned int n, const wxString * items);
int wxItemContainer_Append8(wxItemContainer * self, unsigned int n, const wxString * items, void ** client_data);
int wxItemContainer_Append9(wxItemContainer * self, unsigned int n, const wxString * items, wxClientData ** client_data);
void wxItemContainer_Clear(wxItemContainer * self);
void wxItemContainer_Delete(wxItemContainer * self, unsigned int n);
wxClientData * wxItemContainer_DetachClientObject(wxItemContainer * self, unsigned int n);
bool wxItemContainer_HasClientData(const wxItemContainer * self);
bool wxItemContainer_HasClientObjectData(const wxItemContainer * self);
bool wxItemContainer_HasClientUntypedData(const wxItemContainer * self);
void * wxItemContainer_GetClientData(const wxItemContainer * self, unsigned int n);
wxClientData * wxItemContainer_GetClientObject(const wxItemContainer * self, unsigned int n);
void wxItemContainer_SetClientData(wxItemContainer * self, unsigned int n, void * data);
void wxItemContainer_SetClientObject(wxItemContainer * self, unsigned int n, wxClientData * data);
int wxItemContainer_Insert(wxItemContainer * self, const wxString * item, unsigned int pos);
int wxItemContainer_Insert1(wxItemContainer * self, const wxString * item, unsigned int pos, void * client_data);
int wxItemContainer_Insert2(wxItemContainer * self, const wxString * item, unsigned int pos, wxClientData * client_data);
int wxItemContainer_Insert3(wxItemContainer * self, const wxArrayString * items, unsigned int pos);
int wxItemContainer_Insert5(wxItemContainer * self, const wxArrayString * items, unsigned int pos, void ** client_data);
int wxItemContainer_Insert6(wxItemContainer * self, const wxArrayString * items, unsigned int pos, wxClientData ** client_data);
int wxItemContainer_Insert7(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos);
int wxItemContainer_Insert8(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos, void ** client_data);
int wxItemContainer_Insert9(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos, wxClientData ** client_data);
void wxItemContainer_Set(wxItemContainer * self, const wxArrayString * items);
void wxItemContainer_Set2(wxItemContainer * self, const wxArrayString * items, void ** client_data);
void wxItemContainer_Set3(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data);
void wxItemContainer_Set4(wxItemContainer * self, unsigned int n, const wxString * items);
void wxItemContainer_Set5(wxItemContainer * self, unsigned int n, const wxString * items, void ** client_data);
void wxItemContainer_Set6(wxItemContainer * self, unsigned int n, const wxString * items, wxClientData ** client_data);

// CLASS: wxItemContainerImmutable
void wxItemContainerImmutable_delete(wxItemContainerImmutable *self);
void wxItemContainerImmutable_SetSelection(wxItemContainerImmutable * self, int n);
int wxItemContainerImmutable_GetSelection(const wxItemContainerImmutable * self);
bool wxItemContainerImmutable_SetStringSelection(wxItemContainerImmutable * self, const wxString * string);
wxString *wxItemContainerImmutable_GetStringSelection(const wxItemContainerImmutable * self);
void wxItemContainerImmutable_Select(wxItemContainerImmutable * self, int n);
unsigned int wxItemContainerImmutable_GetCount(const wxItemContainerImmutable * self);
bool wxItemContainerImmutable_IsEmpty(const wxItemContainerImmutable * self);
wxString *wxItemContainerImmutable_GetString(const wxItemContainerImmutable * self, unsigned int n);
wxArrayString *wxItemContainerImmutable_GetStrings(const wxItemContainerImmutable * self);
void wxItemContainerImmutable_SetString(wxItemContainerImmutable * self, unsigned int n, const wxString * string);
int wxItemContainerImmutable_FindString(const wxItemContainerImmutable * self, const wxString * string, bool case_sensitive);

// CLASS: wxJPEGHandler
wxClassInfo *wxJPEGHandler_CLASSINFO();
wxJPEGHandler *wxJPEGHandler_new();
wxVersionInfo *wxJPEGHandler_GetLibraryVersionInfo();

// CLASS: wxJoystick
wxClassInfo *wxJoystick_CLASSINFO();
wxJoystick *wxJoystick_new(int joystick);
int wxJoystick_GetButtonState(const wxJoystick * self);
bool wxJoystick_GetButtonState1(const wxJoystick * self, unsigned int id);
int wxJoystick_GetManufacturerId(const wxJoystick * self);
int wxJoystick_GetMovementThreshold(const wxJoystick * self);
int wxJoystick_GetNumberAxes(const wxJoystick * self);
int wxJoystick_GetNumberButtons(const wxJoystick * self);
int wxJoystick_GetPOVCTSPosition(const wxJoystick * self);
int wxJoystick_GetPOVPosition(const wxJoystick * self);
int wxJoystick_GetPollingMax(const wxJoystick * self);
int wxJoystick_GetPollingMin(const wxJoystick * self);
wxPoint *wxJoystick_GetPosition(const wxJoystick * self);
int wxJoystick_GetPosition1(const wxJoystick * self, unsigned int axis);
int wxJoystick_GetProductId(const wxJoystick * self);
wxString *wxJoystick_GetProductName(const wxJoystick * self);
int wxJoystick_GetRudderMax(const wxJoystick * self);
int wxJoystick_GetRudderMin(const wxJoystick * self);
int wxJoystick_GetRudderPosition(const wxJoystick * self);
int wxJoystick_GetUMax(const wxJoystick * self);
int wxJoystick_GetUMin(const wxJoystick * self);
int wxJoystick_GetUPosition(const wxJoystick * self);
int wxJoystick_GetVMax(const wxJoystick * self);
int wxJoystick_GetVMin(const wxJoystick * self);
int wxJoystick_GetVPosition(const wxJoystick * self);
int wxJoystick_GetXMax(const wxJoystick * self);
int wxJoystick_GetXMin(const wxJoystick * self);
int wxJoystick_GetYMax(const wxJoystick * self);
int wxJoystick_GetYMin(const wxJoystick * self);
int wxJoystick_GetZMax(const wxJoystick * self);
int wxJoystick_GetZMin(const wxJoystick * self);
int wxJoystick_GetZPosition(const wxJoystick * self);
bool wxJoystick_HasPOV(const wxJoystick * self);
bool wxJoystick_HasPOV4Dir(const wxJoystick * self);
bool wxJoystick_HasPOVCTS(const wxJoystick * self);
bool wxJoystick_HasRudder(const wxJoystick * self);
bool wxJoystick_HasU(const wxJoystick * self);
bool wxJoystick_HasV(const wxJoystick * self);
bool wxJoystick_HasZ(const wxJoystick * self);
bool wxJoystick_IsOk(const wxJoystick * self);
bool wxJoystick_ReleaseCapture(wxJoystick * self);
bool wxJoystick_SetCapture(wxJoystick * self, wxWindow * win, int polling_freq);
void wxJoystick_SetMovementThreshold(wxJoystick * self, int threshold);
int wxJoystick_GetNumberJoysticks();

// CLASS: wxJoystickEvent
wxClassInfo *wxJoystickEvent_CLASSINFO();
bool wxJoystickEvent_ButtonDown(const wxJoystickEvent * self, int button);
bool wxJoystickEvent_ButtonIsDown(const wxJoystickEvent * self, int button);
bool wxJoystickEvent_ButtonUp(const wxJoystickEvent * self, int button);
int wxJoystickEvent_GetButtonChange(const wxJoystickEvent * self);
int wxJoystickEvent_GetButtonOrdinal(const wxJoystickEvent * self);
int wxJoystickEvent_GetButtonState(const wxJoystickEvent * self);
int wxJoystickEvent_GetJoystick(const wxJoystickEvent * self);
wxPoint *wxJoystickEvent_GetPosition(const wxJoystickEvent * self);
int wxJoystickEvent_GetZPosition(const wxJoystickEvent * self);
bool wxJoystickEvent_IsButton(const wxJoystickEvent * self);
bool wxJoystickEvent_IsMove(const wxJoystickEvent * self);
bool wxJoystickEvent_IsZMove(const wxJoystickEvent * self);

// CLASS: wxKeyEvent
wxClassInfo *wxKeyEvent_CLASSINFO();
int wxKeyEvent_GetKeyCode(const wxKeyEvent * self);
bool wxKeyEvent_IsKeyInCategory(const wxKeyEvent * self, int category);
bool wxKeyEvent_IsAutoRepeat(const wxKeyEvent * self);
wxPoint *wxKeyEvent_GetPosition(const wxKeyEvent * self);
void wxKeyEvent_GetPosition1(const wxKeyEvent * self, wxCoord * x, wxCoord * y);
wxCoord wxKeyEvent_GetX(const wxKeyEvent * self);
wxCoord wxKeyEvent_GetY(const wxKeyEvent * self);
void wxKeyEvent_DoAllowNextEvent(wxKeyEvent * self);
bool wxKeyEvent_IsNextEventAllowed(const wxKeyEvent * self);
// Mix-in(s) to wxKeyEvent
wxKeyboardState *wxKeyEvent_AsKeyboardState(wxKeyEvent* obj);

// CLASS: wxLayoutAlgorithm
wxClassInfo *wxLayoutAlgorithm_CLASSINFO();
wxLayoutAlgorithm *wxLayoutAlgorithm_new();
bool wxLayoutAlgorithm_LayoutFrame(wxLayoutAlgorithm * self, wxFrame * frame, wxWindow * main_window);
bool wxLayoutAlgorithm_LayoutMDIFrame(wxLayoutAlgorithm * self, wxMDIParentFrame * frame, wxRect * rect);
bool wxLayoutAlgorithm_LayoutWindow(wxLayoutAlgorithm * self, wxWindow * parent, wxWindow * main_window);

// CLASS: wxListBox
wxClassInfo *wxListBox_CLASSINFO();
wxListBox *wxListBox_new();
wxListBox *wxListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxListBox_Create1(wxListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
void wxListBox_Deselect(wxListBox * self, int n);
bool wxListBox_SetStringSelection(wxListBox * self, const wxString * s, bool select);
bool wxListBox_SetStringSelection1(wxListBox * self, const wxString * s);
int wxListBox_GetSelections(const wxListBox * self, wxArrayInt * selections);
int wxListBox_HitTest(const wxListBox * self, const wxPoint * point);
int wxListBox_HitTest1(const wxListBox * self, int x, int y);
void wxListBox_InsertItems1(wxListBox * self, const wxArrayString * items, unsigned int pos);
bool wxListBox_IsSelected(const wxListBox * self, int n);
void wxListBox_SetFirstItem(wxListBox * self, int n);
void wxListBox_SetFirstItem1(wxListBox * self, const wxString * string);
void wxListBox_EnsureVisible(wxListBox * self, int n);
bool wxListBox_IsSorted(const wxListBox * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxListBox_GetCountPerPage(const wxListBox * self);
int wxListBox_GetTopItem(const wxListBox * self);
#endif
// Mix-in(s) to wxListBox
wxItemContainer *wxListBox_AsItemContainer(wxListBox* obj);
wxTrackable *wxListBox_AsTrackable(wxListBox* obj);

// CLASS: wxListCtrl
wxClassInfo *wxListCtrl_CLASSINFO();
wxListCtrl *wxListCtrl_new();
wxListCtrl *wxListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxListCtrl_Arrange(wxListCtrl * self, int flag);
void wxListCtrl_AssignImageList(wxListCtrl * self, wxImageList * image_list, int which);
void wxListCtrl_ClearAll(wxListCtrl * self);
bool wxListCtrl_Create(wxListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxListCtrl_DeleteAllColumns(wxListCtrl * self);
bool wxListCtrl_DeleteAllItems(wxListCtrl * self);
bool wxListCtrl_DeleteColumn(wxListCtrl * self, int col);
bool wxListCtrl_DeleteItem(wxListCtrl * self, long item);
wxTextCtrl * wxListCtrl_EditLabel(wxListCtrl * self, long item, wxClassInfo * text_control_class);
void wxListCtrl_EnableAlternateRowColours(wxListCtrl * self, bool enable);
void wxListCtrl_EnableBellOnNoMatch(wxListCtrl * self, bool on);
bool wxListCtrl_EndEditLabel(wxListCtrl * self, bool cancel);
bool wxListCtrl_EnsureVisible(wxListCtrl * self, long item);
long wxListCtrl_FindItem(wxListCtrl * self, long start, const wxString * str, bool partial);
long wxListCtrl_FindItem2(wxListCtrl * self, long start, const wxPoint * pt, int direction);
bool wxListCtrl_GetColumn(const wxListCtrl * self, int col, wxListItem * item);
int wxListCtrl_GetColumnCount(const wxListCtrl * self);
int wxListCtrl_GetColumnIndexFromOrder(const wxListCtrl * self, int pos);
int wxListCtrl_GetColumnOrder(const wxListCtrl * self, int col);
int wxListCtrl_GetColumnWidth(const wxListCtrl * self, int col);
wxArrayInt *wxListCtrl_GetColumnsOrder(const wxListCtrl * self);
int wxListCtrl_GetCountPerPage(const wxListCtrl * self);
wxTextCtrl * wxListCtrl_GetEditControl(const wxListCtrl * self);
wxImageList * wxListCtrl_GetImageList(const wxListCtrl * self, int which);
bool wxListCtrl_GetItem(const wxListCtrl * self, wxListItem * info);
wxColour *wxListCtrl_GetItemBackgroundColour(const wxListCtrl * self, long item);
int wxListCtrl_GetItemCount(const wxListCtrl * self);
wxFont *wxListCtrl_GetItemFont(const wxListCtrl * self, long item);
bool wxListCtrl_GetItemPosition(const wxListCtrl * self, long item, wxPoint * pos);
bool wxListCtrl_GetItemRect(const wxListCtrl * self, long item, wxRect * rect, int code);
wxSize *wxListCtrl_GetItemSpacing(const wxListCtrl * self);
int wxListCtrl_GetItemState(const wxListCtrl * self, long item, long state_mask);
wxString *wxListCtrl_GetItemText(const wxListCtrl * self, long item, int col);
wxColour *wxListCtrl_GetItemTextColour(const wxListCtrl * self, long item);
long wxListCtrl_GetNextItem(const wxListCtrl * self, long item, int geometry, int state);
int wxListCtrl_GetSelectedItemCount(const wxListCtrl * self);
bool wxListCtrl_GetSubItemRect(const wxListCtrl * self, long item, long sub_item, wxRect * rect, int code);
wxColour *wxListCtrl_GetTextColour(const wxListCtrl * self);
long wxListCtrl_GetTopItem(const wxListCtrl * self);
wxRect *wxListCtrl_GetViewRect(const wxListCtrl * self);
void wxListCtrl_SetAlternateRowColour(wxListCtrl * self, const wxColour * colour);
wxColour *wxListCtrl_GetAlternateRowColour(const wxListCtrl * self);
long wxListCtrl_HitTest(const wxListCtrl * self, const wxPoint * point, int * flags, long * ptr_sub_item);
bool wxListCtrl_InReportView(const wxListCtrl * self);
long wxListCtrl_InsertColumn(wxListCtrl * self, long col, const wxListItem * info);
long wxListCtrl_InsertColumn1(wxListCtrl * self, long col, const wxString * heading, int format, int width);
long wxListCtrl_InsertItem(wxListCtrl * self, wxListItem * info);
long wxListCtrl_InsertItem1(wxListCtrl * self, long index, const wxString * label);
long wxListCtrl_InsertItem2(wxListCtrl * self, long index, int image_index);
long wxListCtrl_InsertItem3(wxListCtrl * self, long index, const wxString * label, int image_index);
bool wxListCtrl_IsEmpty(const wxListCtrl * self);
bool wxListCtrl_IsVirtual(const wxListCtrl * self);
void wxListCtrl_RefreshItem(wxListCtrl * self, long item);
void wxListCtrl_RefreshItems(wxListCtrl * self, long item_from, long item_to);
bool wxListCtrl_ScrollList(wxListCtrl * self, int dx, int dy);
bool wxListCtrl_SetColumn(wxListCtrl * self, int col, wxListItem * item);
bool wxListCtrl_SetColumnWidth(wxListCtrl * self, int col, int width);
bool wxListCtrl_SetColumnsOrder(wxListCtrl * self, const wxArrayInt * orders);
bool wxListCtrl_SetHeaderAttr(wxListCtrl * self, const wxItemAttr * attr);
void wxListCtrl_SetImageList(wxListCtrl * self, wxImageList * image_list, int which);
void wxListCtrl_SetNormalImages(wxListCtrl * self, const wxVector< wxBitmapBundle > * images);
void wxListCtrl_SetSmallImages(wxListCtrl * self, const wxVector< wxBitmapBundle > * images);
bool wxListCtrl_IsVisible(const wxListCtrl * self, long item);
bool wxListCtrl_SetItem(wxListCtrl * self, wxListItem * info);
bool wxListCtrl_SetItem1(wxListCtrl * self, long index, int column, const wxString * label, int image_id);
void wxListCtrl_SetItemBackgroundColour(wxListCtrl * self, long item, const wxColour * col);
bool wxListCtrl_SetItemColumnImage(wxListCtrl * self, long item, long column, int image);
void wxListCtrl_SetItemCount(wxListCtrl * self, long count);
bool wxListCtrl_SetItemData(wxListCtrl * self, long item, long data);
void wxListCtrl_SetItemFont(wxListCtrl * self, long item, const wxFont * font);
bool wxListCtrl_SetItemImage(wxListCtrl * self, long item, int image, int sel_image);
bool wxListCtrl_SetItemPosition(wxListCtrl * self, long item, const wxPoint * pos);
bool wxListCtrl_SetItemState(wxListCtrl * self, long item, long state, long state_mask);
void wxListCtrl_SetItemText(wxListCtrl * self, long item, const wxString * text);
void wxListCtrl_SetItemTextColour(wxListCtrl * self, long item, const wxColour * col);
void wxListCtrl_SetSingleStyle(wxListCtrl * self, long style, bool add);
void wxListCtrl_SetTextColour(wxListCtrl * self, const wxColour * col);
bool wxListCtrl_HasCheckBoxes(const wxListCtrl * self);
bool wxListCtrl_EnableCheckBoxes(wxListCtrl * self, bool enable);
bool wxListCtrl_IsItemChecked(const wxListCtrl * self, long item);
void wxListCtrl_CheckItem(wxListCtrl * self, long item, bool check);
void wxListCtrl_ExtendRulesAndAlternateColour(wxListCtrl * self, bool extend);
void wxListCtrl_ShowSortIndicator(wxListCtrl * self, int col, bool ascending);
void wxListCtrl_RemoveSortIndicator(wxListCtrl * self);
int wxListCtrl_GetSortIndicator(const wxListCtrl * self);
bool wxListCtrl_GetUpdatedAscendingSortIndicator(const wxListCtrl * self, int col);
bool wxListCtrl_IsAscendingSortIndicator(const wxListCtrl * self);
// Mix-in(s) to wxListCtrl
wxTrackable *wxListCtrl_AsTrackable(wxListCtrl* obj);

// CLASS: wxListEvent
wxClassInfo *wxListEvent_CLASSINFO();
long wxListEvent_GetCacheFrom(const wxListEvent * self);
long wxListEvent_GetCacheTo(const wxListEvent * self);
int wxListEvent_GetColumn(const wxListEvent * self);
int wxListEvent_GetImage(const wxListEvent * self);
long wxListEvent_GetIndex(const wxListEvent * self);
wxListItem *wxListEvent_GetItem(const wxListEvent * self);
int wxListEvent_GetKeyCode(const wxListEvent * self);
wxString *wxListEvent_GetLabel(const wxListEvent * self);
long wxListEvent_GetMask(const wxListEvent * self);
wxPoint *wxListEvent_GetPoint(const wxListEvent * self);
wxString *wxListEvent_GetText(const wxListEvent * self);
bool wxListEvent_IsEditCancelled(const wxListEvent * self);
void wxListEvent_SetKeyCode(wxListEvent * self, int code);
void wxListEvent_SetIndex(wxListEvent * self, long index);
void wxListEvent_SetColumn(wxListEvent * self, int col);
void wxListEvent_SetPoint(wxListEvent * self, const wxPoint * point);
void wxListEvent_SetItem(wxListEvent * self, const wxListItem * item);
void wxListEvent_SetCacheFrom(wxListEvent * self, long cache_from);
void wxListEvent_SetCacheTo(wxListEvent * self, long cache_to);

// CLASS: wxListItem
wxClassInfo *wxListItem_CLASSINFO();
wxListItem *wxListItem_new();
void wxListItem_Clear(wxListItem * self);
wxColour *wxListItem_GetBackgroundColour(const wxListItem * self);
int wxListItem_GetColumn(const wxListItem * self);
wxFont *wxListItem_GetFont(const wxListItem * self);
long wxListItem_GetId(const wxListItem * self);
int wxListItem_GetImage(const wxListItem * self);
long wxListItem_GetMask(const wxListItem * self);
long wxListItem_GetState(const wxListItem * self);
wxString *wxListItem_GetText(const wxListItem * self);
wxColour *wxListItem_GetTextColour(const wxListItem * self);
int wxListItem_GetWidth(const wxListItem * self);
void wxListItem_SetBackgroundColour(wxListItem * self, const wxColour * col_back);
void wxListItem_SetColumn(wxListItem * self, int col);
void wxListItem_SetData(wxListItem * self, long data);
void wxListItem_SetData1(wxListItem * self, void * data);
void wxListItem_SetFont(wxListItem * self, const wxFont * font);
void wxListItem_SetId(wxListItem * self, long id);
void wxListItem_SetImage(wxListItem * self, int image);
void wxListItem_SetMask(wxListItem * self, long mask);
void wxListItem_SetState(wxListItem * self, long state);
void wxListItem_SetStateMask(wxListItem * self, long state_mask);
void wxListItem_SetText(wxListItem * self, const wxString * text);
void wxListItem_SetTextColour(wxListItem * self, const wxColour * col_text);
void wxListItem_SetWidth(wxListItem * self, int width);

// CLASS: wxListView
wxClassInfo *wxListView_CLASSINFO();
wxListView *wxListView_new();
wxListView *wxListView_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxListView_ClearColumnImage(wxListView * self, int col);
void wxListView_Focus(wxListView * self, long index);
long wxListView_GetFirstSelected(const wxListView * self);
long wxListView_GetFocusedItem(const wxListView * self);
long wxListView_GetNextSelected(const wxListView * self, long item);
bool wxListView_IsSelected(const wxListView * self, long index);
void wxListView_Select(wxListView * self, long n, bool on);
void wxListView_SetColumnImage(wxListView * self, int col, int image);
// Mix-in(s) to wxListView
wxTrackable *wxListView_AsTrackable(wxListView* obj);

// CLASS: wxListbook
wxClassInfo *wxListbook_CLASSINFO();
wxListbook *wxListbook_new();
wxListbook *wxListbook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxListbook_Create(wxListbook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxListView * wxListbook_GetListView(const wxListbook * self);
// Mix-in(s) to wxListbook
wxWithImages *wxListbook_AsWithImages(wxListbook* obj);
wxTrackable *wxListbook_AsTrackable(wxListbook* obj);

// CLASS: wxLogGui
void wxLogGui_delete(wxLogGui *self);
wxLogGui *wxLogGui_new();

// CLASS: wxLogTextCtrl
void wxLogTextCtrl_delete(wxLogTextCtrl *self);
wxLogTextCtrl *wxLogTextCtrl_new(wxTextCtrl * p_text_ctrl);

// CLASS: wxLogWindow
void wxLogWindow_delete(wxLogWindow *self);
wxLogWindow *wxLogWindow_new(wxWindow * p_parent, const wxString * sz_title, bool show, bool pass_to_old);
wxFrame * wxLogWindow_GetFrame(const wxLogWindow * self);
bool wxLogWindow_OnFrameClose(wxLogWindow * self, wxFrame * frame);
void wxLogWindow_OnFrameDelete(wxLogWindow * self, wxFrame * frame);
void wxLogWindow_Show(wxLogWindow * self, bool show);

// CLASS: wxLongPressEvent
wxClassInfo *wxLongPressEvent_CLASSINFO();
wxLongPressEvent *wxLongPressEvent_new(wxWindowID windid);

// CLASS: wxMDIChildFrame
wxClassInfo *wxMDIChildFrame_CLASSINFO();
wxMDIChildFrame *wxMDIChildFrame_new();
wxMDIChildFrame *wxMDIChildFrame_new1(wxMDIParentFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxMDIChildFrame_Activate(wxMDIChildFrame * self);
bool wxMDIChildFrame_Create(wxMDIChildFrame * self, wxMDIParentFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxMDIParentFrame * wxMDIChildFrame_GetMDIParent(const wxMDIChildFrame * self);
void wxMDIChildFrame_Restore(wxMDIChildFrame * self);
// Mix-in(s) to wxMDIChildFrame
wxTrackable *wxMDIChildFrame_AsTrackable(wxMDIChildFrame* obj);

// CLASS: wxMDIClientWindow
wxClassInfo *wxMDIClientWindow_CLASSINFO();
wxMDIClientWindow *wxMDIClientWindow_new();
bool wxMDIClientWindow_CreateClient(wxMDIClientWindow * self, wxMDIParentFrame * parent, long style);
// Mix-in(s) to wxMDIClientWindow
wxTrackable *wxMDIClientWindow_AsTrackable(wxMDIClientWindow* obj);

// CLASS: wxMDIParentFrame
wxClassInfo *wxMDIParentFrame_CLASSINFO();
wxMDIParentFrame *wxMDIParentFrame_new();
wxMDIParentFrame *wxMDIParentFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxMDIParentFrame_ActivateNext(wxMDIParentFrame * self);
void wxMDIParentFrame_ActivatePrevious(wxMDIParentFrame * self);
void wxMDIParentFrame_ArrangeIcons(wxMDIParentFrame * self);
void wxMDIParentFrame_Cascade(wxMDIParentFrame * self);
bool wxMDIParentFrame_Create(wxMDIParentFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxMDIChildFrame * wxMDIParentFrame_GetActiveChild(const wxMDIParentFrame * self);
wxMDIClientWindowBase * wxMDIParentFrame_GetClientWindow(const wxMDIParentFrame * self);
wxMenu * wxMDIParentFrame_GetWindowMenu(const wxMDIParentFrame * self);
wxMDIClientWindow * wxMDIParentFrame_OnCreateClient(wxMDIParentFrame * self);
void wxMDIParentFrame_SetWindowMenu(wxMDIParentFrame * self, wxMenu * menu);
bool wxMDIParentFrame_IsTDI();
// Mix-in(s) to wxMDIParentFrame
wxTrackable *wxMDIParentFrame_AsTrackable(wxMDIParentFrame* obj);

// CLASS: wxMask
wxClassInfo *wxMask_CLASSINFO();
wxMask *wxMask_new();
wxMask *wxMask_new1(const wxBitmap * bitmap, int index);
wxMask *wxMask_new2(const wxBitmap * bitmap);
wxMask *wxMask_new3(const wxBitmap * bitmap, const wxColour * colour);
bool wxMask_Create(wxMask * self, const wxBitmap * bitmap, int index);
bool wxMask_Create1(wxMask * self, const wxBitmap * bitmap);
bool wxMask_Create2(wxMask * self, const wxBitmap * bitmap, const wxColour * colour);
wxBitmap *wxMask_GetBitmap(const wxMask * self);

// CLASS: wxMaximizeEvent
wxClassInfo *wxMaximizeEvent_CLASSINFO();
wxMaximizeEvent *wxMaximizeEvent_new(int id);

// CLASS: wxMemoryDC
wxClassInfo *wxMemoryDC_CLASSINFO();
wxMemoryDC *wxMemoryDC_new();
wxMemoryDC *wxMemoryDC_new1(wxDC * dc);
wxMemoryDC *wxMemoryDC_new2(wxBitmap * bitmap);
void wxMemoryDC_SelectObject(wxMemoryDC * self, wxBitmap * bitmap);
void wxMemoryDC_SelectObjectAsSource(wxMemoryDC * self, const wxBitmap * bitmap);
wxBitmap *wxMemoryDC_GetSelectedBitmap(const wxMemoryDC * self);
wxBitmap * wxMemoryDC_GetSelectedBitmap1(wxMemoryDC * self);

// CLASS: wxMenu
wxClassInfo *wxMenu_CLASSINFO();
wxMenu *wxMenu_new();
wxMenu *wxMenu_new1(long style);
wxMenu *wxMenu_new2(const wxString * title, long style);
wxMenuItem * wxMenu_Append(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Append1(wxMenu * self, int id, const wxString * item, wxMenu * sub_menu, const wxString * help_string);
wxMenuItem * wxMenu_Append2(wxMenu * self, wxMenuItem * menu_item);
wxMenuItem * wxMenu_AppendCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help);
wxMenuItem * wxMenu_AppendRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help);
wxMenuItem * wxMenu_AppendSeparator(wxMenu * self);
wxMenuItem * wxMenu_AppendSubMenu(wxMenu * self, wxMenu * submenu, const wxString * text, const wxString * help);
void wxMenu_Break(wxMenu * self);
void wxMenu_Check(wxMenu * self, int id, bool check);
bool wxMenu_Delete(wxMenu * self, int id);
bool wxMenu_Delete1(wxMenu * self, wxMenuItem * item);
bool wxMenu_Destroy(wxMenu * self, int id);
bool wxMenu_Destroy1(wxMenu * self, wxMenuItem * item);
void wxMenu_Enable(wxMenu * self, int id, bool enable);
wxMenuItem * wxMenu_FindChildItem(const wxMenu * self, int id, size_t * pos);
int wxMenu_FindItem(const wxMenu * self, const wxString * item_string);
wxMenuItem * wxMenu_FindItem1(const wxMenu * self, int id, wxMenu ** menu);
wxMenuItem * wxMenu_FindItemByPosition(const wxMenu * self, size_t position);
wxString *wxMenu_GetHelpString(const wxMenu * self, int id);
wxString *wxMenu_GetLabel(const wxMenu * self, int id);
wxString *wxMenu_GetLabelText(const wxMenu * self, int id);
size_t wxMenu_GetMenuItemCount(const wxMenu * self);
wxString *wxMenu_GetTitle(const wxMenu * self);
wxMenuItem * wxMenu_Insert(wxMenu * self, size_t pos, wxMenuItem * menu_item);
wxMenuItem * wxMenu_Insert1(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Insert2(wxMenu * self, size_t pos, int id, const wxString * text, wxMenu * submenu, const wxString * help);
wxMenuItem * wxMenu_InsertCheckItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_InsertRadioItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_InsertSeparator(wxMenu * self, size_t pos);
bool wxMenu_IsChecked(const wxMenu * self, int id);
bool wxMenu_IsEnabled(const wxMenu * self, int id);
wxMenuItem * wxMenu_Prepend(wxMenu * self, wxMenuItem * item);
wxMenuItem * wxMenu_Prepend1(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Prepend2(wxMenu * self, int id, const wxString * text, wxMenu * submenu, const wxString * help);
wxMenuItem * wxMenu_PrependCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_PrependRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_PrependSeparator(wxMenu * self);
wxMenuItem * wxMenu_Remove(wxMenu * self, int id);
wxMenuItem * wxMenu_Remove1(wxMenu * self, wxMenuItem * item);
void wxMenu_SetHelpString(wxMenu * self, int id, const wxString * help_string);
void wxMenu_SetLabel(wxMenu * self, int id, const wxString * label);
void wxMenu_SetTitle(wxMenu * self, const wxString * title);
void wxMenu_UpdateUI(wxMenu * self, wxEvtHandler * source);
void wxMenu_SetInvokingWindow(wxMenu * self, wxWindow * win);
wxWindow * wxMenu_GetInvokingWindow(const wxMenu * self);
wxWindow * wxMenu_GetWindow(const wxMenu * self);
long wxMenu_GetStyle(const wxMenu * self);
void wxMenu_SetParent(wxMenu * self, wxMenu * parent);
wxMenu * wxMenu_GetParent(const wxMenu * self);
void wxMenu_Attach(wxMenu * self, wxMenuBar * menubar);
void wxMenu_Detach(wxMenu * self);
bool wxMenu_IsAttached(const wxMenu * self);
// Mix-in(s) to wxMenu
wxTrackable *wxMenu_AsTrackable(wxMenu* obj);

// CLASS: wxMenuBar
wxClassInfo *wxMenuBar_CLASSINFO();
wxMenuBar *wxMenuBar_new(long style);
bool wxMenuBar_Append(wxMenuBar * self, wxMenu * menu, const wxString * title);
void wxMenuBar_Check(wxMenuBar * self, int id, bool check);
void wxMenuBar_Enable(wxMenuBar * self, int id, bool enable);
bool wxMenuBar_IsEnabledTop(const wxMenuBar * self, size_t pos);
void wxMenuBar_EnableTop(wxMenuBar * self, size_t pos, bool enable);
wxMenuItem * wxMenuBar_FindItem(const wxMenuBar * self, int id, wxMenu ** menu);
int wxMenuBar_FindMenu(const wxMenuBar * self, const wxString * title);
int wxMenuBar_FindMenuItem(const wxMenuBar * self, const wxString * menu_string, const wxString * item_string);
wxString *wxMenuBar_GetHelpString(const wxMenuBar * self, int id);
wxString *wxMenuBar_GetLabel(const wxMenuBar * self, int id);
wxMenu * wxMenuBar_GetMenu(const wxMenuBar * self, size_t menu_index);
size_t wxMenuBar_GetMenuCount(const wxMenuBar * self);
wxString *wxMenuBar_GetMenuLabel(const wxMenuBar * self, size_t pos);
wxString *wxMenuBar_GetMenuLabelText(const wxMenuBar * self, size_t pos);
bool wxMenuBar_Insert(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title);
bool wxMenuBar_IsChecked(const wxMenuBar * self, int id);
bool wxMenuBar_IsEnabled(const wxMenuBar * self, int id);
wxMenu * wxMenuBar_Remove(wxMenuBar * self, size_t pos);
wxMenu * wxMenuBar_Replace(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title);
void wxMenuBar_SetHelpString(wxMenuBar * self, int id, const wxString * help_string);
void wxMenuBar_SetLabel(wxMenuBar * self, int id, const wxString * label);
void wxMenuBar_SetMenuLabel(wxMenuBar * self, size_t pos, const wxString * label);
#ifdef __WXOSX__
wxMenu * wxMenuBar_OSXGetAppleMenu(const wxMenuBar * self);
#endif
wxFrame * wxMenuBar_GetFrame(const wxMenuBar * self);
bool wxMenuBar_IsAttached(const wxMenuBar * self);
void wxMenuBar_Attach(wxMenuBar * self, wxFrame * frame);
void wxMenuBar_Detach(wxMenuBar * self);
#ifdef __WXOSX__
void wxMenuBar_MacSetCommonMenuBar(wxMenuBar * menubar);
wxMenuBar * wxMenuBar_MacGetCommonMenuBar();
#endif
// Mix-in(s) to wxMenuBar
wxTrackable *wxMenuBar_AsTrackable(wxMenuBar* obj);

// CLASS: wxMenuEvent
wxClassInfo *wxMenuEvent_CLASSINFO();
wxMenu * wxMenuEvent_GetMenu(const wxMenuEvent * self);
int wxMenuEvent_GetMenuId(const wxMenuEvent * self);
bool wxMenuEvent_IsPopup(const wxMenuEvent * self);

// CLASS: wxMenuItem
wxClassInfo *wxMenuItem_CLASSINFO();
wxBitmap *wxMenuItem_GetBitmap(const wxMenuItem * self);
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetBitmap1(const wxMenuItem * self, bool checked);
#endif
#if wxCHECK_VERSION(3, 2, 0)
wxBitmapBundle *wxMenuItem_GetBitmapBundle(const wxMenuItem * self);
#endif
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetDisabledBitmap(const wxMenuItem * self);
#endif
wxString *wxMenuItem_GetHelp(const wxMenuItem * self);
int wxMenuItem_GetId(const wxMenuItem * self);
wxString *wxMenuItem_GetItemLabel(const wxMenuItem * self);
wxString *wxMenuItem_GetItemLabelText(const wxMenuItem * self);
wxItemKind wxMenuItem_GetKind(const wxMenuItem * self);
#ifdef __WXMSW__
int wxMenuItem_GetMarginWidth(const wxMenuItem * self);
#endif
wxMenu * wxMenuItem_GetMenu(const wxMenuItem * self);
wxMenu * wxMenuItem_GetSubMenu(const wxMenuItem * self);
wxAcceleratorEntry * wxMenuItem_GetAccel(const wxMenuItem * self);
bool wxMenuItem_IsCheck(const wxMenuItem * self);
bool wxMenuItem_IsCheckable(const wxMenuItem * self);
bool wxMenuItem_IsChecked(const wxMenuItem * self);
bool wxMenuItem_IsEnabled(const wxMenuItem * self);
bool wxMenuItem_IsRadio(const wxMenuItem * self);
bool wxMenuItem_IsSeparator(const wxMenuItem * self);
bool wxMenuItem_IsSubMenu(const wxMenuItem * self);
#ifdef __WXMSW__
void wxMenuItem_SetBackgroundColour(wxMenuItem * self, const wxColour * colour);
#endif
void wxMenuItem_SetBitmap(wxMenuItem * self, const wxBitmapBundle * bmp);
#ifdef __WXMSW__
void wxMenuItem_SetBitmap1(wxMenuItem * self, const wxBitmapBundle * bmp, bool checked);
void wxMenuItem_SetBitmaps(wxMenuItem * self, const wxBitmapBundle * checked, const wxBitmapBundle * unchecked);
void wxMenuItem_SetDisabledBitmap(wxMenuItem * self, const wxBitmapBundle * disabled);
void wxMenuItem_SetFont(wxMenuItem * self, const wxFont * font);
#endif
void wxMenuItem_SetHelp(wxMenuItem * self, const wxString * help_string);
void wxMenuItem_SetItemLabel(wxMenuItem * self, const wxString * label);
#ifdef __WXMSW__
void wxMenuItem_SetMarginWidth(wxMenuItem * self, int width);
#endif
void wxMenuItem_SetMenu(wxMenuItem * self, wxMenu * menu);
void wxMenuItem_SetSubMenu(wxMenuItem * self, wxMenu * menu);
#ifdef __WXMSW__
void wxMenuItem_SetTextColour(wxMenuItem * self, const wxColour * colour);
#endif
void wxMenuItem_SetAccel(wxMenuItem * self, wxAcceleratorEntry * accel);
#if wxCHECK_VERSION(3, 1, 0)
void wxMenuItem_AddExtraAccel(wxMenuItem * self, const wxAcceleratorEntry * accel);
void wxMenuItem_ClearExtraAccels(wxMenuItem * self);
#endif
wxMenuItem *wxMenuItem_new(wxMenu * parent_menu, int id, const wxString * text, const wxString * help_string, wxItemKind kind, wxMenu * sub_menu);
void wxMenuItem_Check(wxMenuItem * self, bool check);
void wxMenuItem_Enable(wxMenuItem * self, bool enable);
wxString *wxMenuItem_GetLabelText(const wxString * text);

// CLASS: wxMessageDialog
wxClassInfo *wxMessageDialog_CLASSINFO();
wxMessageDialog *wxMessageDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, long style, const wxPoint * pos);
void wxMessageDialog_SetExtendedMessage(wxMessageDialog * self, const wxString * extended_message);
bool wxMessageDialog_SetHelpLabel(wxMessageDialog * self, const ButtonLabel * help);
void wxMessageDialog_SetMessage(wxMessageDialog * self, const wxString * message);
bool wxMessageDialog_SetOKCancelLabels(wxMessageDialog * self, const ButtonLabel * ok, const ButtonLabel * cancel);
bool wxMessageDialog_SetOKLabel(wxMessageDialog * self, const ButtonLabel * ok);
bool wxMessageDialog_SetYesNoCancelLabels(wxMessageDialog * self, const ButtonLabel * yes, const ButtonLabel * no, const ButtonLabel * cancel);
bool wxMessageDialog_SetYesNoLabels(wxMessageDialog * self, const ButtonLabel * yes, const ButtonLabel * no);
wxString *wxMessageDialog_GetCaption(const wxMessageDialog * self);
wxString *wxMessageDialog_GetMessage(const wxMessageDialog * self);
wxString *wxMessageDialog_GetExtendedMessage(const wxMessageDialog * self);
long wxMessageDialog_GetMessageDialogStyle(const wxMessageDialog * self);
bool wxMessageDialog_HasCustomLabels(const wxMessageDialog * self);
wxString *wxMessageDialog_GetYesLabel(const wxMessageDialog * self);
wxString *wxMessageDialog_GetNoLabel(const wxMessageDialog * self);
wxString *wxMessageDialog_GetOKLabel(const wxMessageDialog * self);
wxString *wxMessageDialog_GetCancelLabel(const wxMessageDialog * self);
wxString *wxMessageDialog_GetHelpLabel(const wxMessageDialog * self);
long wxMessageDialog_GetEffectiveIcon(const wxMessageDialog * self);
// Mix-in(s) to wxMessageDialog
wxTrackable *wxMessageDialog_AsTrackable(wxMessageDialog* obj);

// CLASS: wxMessageOutputMessageBox
void wxMessageOutputMessageBox_delete(wxMessageOutputMessageBox *self);
wxMessageOutputMessageBox *wxMessageOutputMessageBox_new();

// CLASS: wxMetafile
wxClassInfo *wxMetafile_CLASSINFO();
wxMetafile *wxMetafile_new(const wxString * filename);
bool wxMetafile_IsOk(wxMetafile * self);
bool wxMetafile_Play(wxMetafile * self, wxDC * dc);
bool wxMetafile_SetClipboard(wxMetafile * self, int width, int height);

// CLASS: wxMetafileDC
wxClassInfo *wxMetafileDC_CLASSINFO();
wxMetafileDC *wxMetafileDC_new(const wxString * filename);
wxMetafile * wxMetafileDC_Close(wxMetafileDC * self);

// CLASS: wxMiniFrame
wxClassInfo *wxMiniFrame_CLASSINFO();
wxMiniFrame *wxMiniFrame_new();
wxMiniFrame *wxMiniFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxMiniFrame_Create(wxMiniFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxMiniFrame
wxTrackable *wxMiniFrame_AsTrackable(wxMiniFrame* obj);

// CLASS: wxMirrorDC
wxClassInfo *wxMirrorDC_CLASSINFO();
wxMirrorDC *wxMirrorDC_new(wxDC * dc, bool mirror);

// CLASS: wxMouseCaptureChangedEvent
wxClassInfo *wxMouseCaptureChangedEvent_CLASSINFO();
wxMouseCaptureChangedEvent *wxMouseCaptureChangedEvent_new(wxWindowID window_id, wxWindow * gained_capture);
wxWindow * wxMouseCaptureChangedEvent_GetCapturedWindow(const wxMouseCaptureChangedEvent * self);

// CLASS: wxMouseCaptureLostEvent
wxClassInfo *wxMouseCaptureLostEvent_CLASSINFO();
wxMouseCaptureLostEvent *wxMouseCaptureLostEvent_new(wxWindowID window_id);

// CLASS: wxMouseEvent
wxClassInfo *wxMouseEvent_CLASSINFO();
bool wxMouseEvent_Aux1DClick(const wxMouseEvent * self);
bool wxMouseEvent_Aux1Down(const wxMouseEvent * self);
bool wxMouseEvent_Aux1Up(const wxMouseEvent * self);
bool wxMouseEvent_Aux2DClick(const wxMouseEvent * self);
bool wxMouseEvent_Aux2Down(const wxMouseEvent * self);
bool wxMouseEvent_Aux2Up(const wxMouseEvent * self);
bool wxMouseEvent_Dragging(const wxMouseEvent * self);
bool wxMouseEvent_Entering(const wxMouseEvent * self);
int wxMouseEvent_GetButton(const wxMouseEvent * self);
int wxMouseEvent_GetClickCount(const wxMouseEvent * self);
int wxMouseEvent_GetLinesPerAction(const wxMouseEvent * self);
int wxMouseEvent_GetColumnsPerAction(const wxMouseEvent * self);
wxPoint *wxMouseEvent_GetLogicalPosition(const wxMouseEvent * self, const wxDC * dc);
int wxMouseEvent_GetWheelDelta(const wxMouseEvent * self);
bool wxMouseEvent_IsWheelInverted(const wxMouseEvent * self);
int wxMouseEvent_GetWheelRotation(const wxMouseEvent * self);
bool wxMouseEvent_IsButton(const wxMouseEvent * self);
bool wxMouseEvent_IsPageScroll(const wxMouseEvent * self);
bool wxMouseEvent_Leaving(const wxMouseEvent * self);
bool wxMouseEvent_LeftDClick(const wxMouseEvent * self);
bool wxMouseEvent_LeftDown(const wxMouseEvent * self);
bool wxMouseEvent_LeftUp(const wxMouseEvent * self);
bool wxMouseEvent_Magnify(const wxMouseEvent * self);
bool wxMouseEvent_MetaDown(const wxMouseEvent * self);
bool wxMouseEvent_MiddleDClick(const wxMouseEvent * self);
bool wxMouseEvent_MiddleDown(const wxMouseEvent * self);
bool wxMouseEvent_MiddleUp(const wxMouseEvent * self);
bool wxMouseEvent_Moving(const wxMouseEvent * self);
bool wxMouseEvent_RightDClick(const wxMouseEvent * self);
bool wxMouseEvent_RightDown(const wxMouseEvent * self);
bool wxMouseEvent_RightUp(const wxMouseEvent * self);
// Mix-in(s) to wxMouseEvent
wxMouseState *wxMouseEvent_AsMouseState(wxMouseEvent* obj);

// CLASS: wxMouseEventsManager
wxClassInfo *wxMouseEventsManager_CLASSINFO();
wxMouseEventsManager *wxMouseEventsManager_new();
wxMouseEventsManager *wxMouseEventsManager_new1(wxWindow * win);
bool wxMouseEventsManager_Create(wxMouseEventsManager * self, wxWindow * win);
// Mix-in(s) to wxMouseEventsManager
wxTrackable *wxMouseEventsManager_AsTrackable(wxMouseEventsManager* obj);

// CLASS: wxMoveEvent
wxClassInfo *wxMoveEvent_CLASSINFO();
wxMoveEvent *wxMoveEvent_new(const wxPoint * pt, int id);
wxPoint *wxMoveEvent_GetPosition(const wxMoveEvent * self);
wxRect *wxMoveEvent_GetRect(const wxMoveEvent * self);
void wxMoveEvent_SetRect(wxMoveEvent * self, const wxRect * rect);
void wxMoveEvent_SetPosition(wxMoveEvent * self, const wxPoint * pos);

// CLASS: wxNativeFontInfo
void wxNativeFontInfo_delete(wxNativeFontInfo *self);
wxNativeFontInfo *wxNativeFontInfo_new();
wxNativeFontInfo *wxNativeFontInfo_new1(const wxNativeFontInfo * info);
void wxNativeFontInfo_Init(wxNativeFontInfo * self);
void wxNativeFontInfo_InitFromFont(wxNativeFontInfo * self, const wxFont * font);
int wxNativeFontInfo_GetPointSize(const wxNativeFontInfo * self);
wxSize *wxNativeFontInfo_GetPixelSize(const wxNativeFontInfo * self);
int wxNativeFontInfo_GetNumericWeight(const wxNativeFontInfo * self);
bool wxNativeFontInfo_GetUnderlined(const wxNativeFontInfo * self);
wxString *wxNativeFontInfo_GetFaceName(const wxNativeFontInfo * self);
void wxNativeFontInfo_SetPointSize(wxNativeFontInfo * self, int pointsize);
void wxNativeFontInfo_SetPixelSize(wxNativeFontInfo * self, const wxSize * pixel_size);
void wxNativeFontInfo_SetNumericWeight(wxNativeFontInfo * self, int weight);
void wxNativeFontInfo_SetUnderlined(wxNativeFontInfo * self, bool underlined);
bool wxNativeFontInfo_SetFaceName(wxNativeFontInfo * self, const wxString * facename);
void wxNativeFontInfo_SetFaceName1(wxNativeFontInfo * self, const wxArrayString * facenames);
bool wxNativeFontInfo_FromString(wxNativeFontInfo * self, const wxString * s);
wxString *wxNativeFontInfo_ToString(const wxNativeFontInfo * self);
bool wxNativeFontInfo_FromUserString(wxNativeFontInfo * self, const wxString * s);
wxString *wxNativeFontInfo_ToUserString(const wxNativeFontInfo * self);

// CLASS: wxNativeWindow
wxClassInfo *wxNativeWindow_CLASSINFO();
wxNativeWindow *wxNativeWindow_new();
void wxNativeWindow_Disown(wxNativeWindow * self);
// Mix-in(s) to wxNativeWindow
wxTrackable *wxNativeWindow_AsTrackable(wxNativeWindow* obj);

// CLASS: wxNavigationKeyEvent
wxClassInfo *wxNavigationKeyEvent_CLASSINFO();
wxNavigationKeyEvent *wxNavigationKeyEvent_new();
wxNavigationKeyEvent *wxNavigationKeyEvent_new1(const wxNavigationKeyEvent * event);
wxWindow * wxNavigationKeyEvent_GetCurrentFocus(const wxNavigationKeyEvent * self);
bool wxNavigationKeyEvent_GetDirection(const wxNavigationKeyEvent * self);
bool wxNavigationKeyEvent_IsFromTab(const wxNavigationKeyEvent * self);
bool wxNavigationKeyEvent_IsWindowChange(const wxNavigationKeyEvent * self);
void wxNavigationKeyEvent_SetCurrentFocus(wxNavigationKeyEvent * self, wxWindow * current_focus);
void wxNavigationKeyEvent_SetDirection(wxNavigationKeyEvent * self, bool direction);
void wxNavigationKeyEvent_SetFlags(wxNavigationKeyEvent * self, long flags);
void wxNavigationKeyEvent_SetFromTab(wxNavigationKeyEvent * self, bool from_tab);
void wxNavigationKeyEvent_SetWindowChange(wxNavigationKeyEvent * self, bool window_change);

// CLASS: wxNonOwnedWindow
wxClassInfo *wxNonOwnedWindow_CLASSINFO();
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region);
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path);
// Mix-in(s) to wxNonOwnedWindow
wxTrackable *wxNonOwnedWindow_AsTrackable(wxNonOwnedWindow* obj);

// CLASS: wxNotebook
wxClassInfo *wxNotebook_CLASSINFO();
wxNotebook *wxNotebook_new();
wxNotebook *wxNotebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
int wxNotebook_GetRowCount(const wxNotebook * self);
wxColour *wxNotebook_GetThemeBackgroundColour(const wxNotebook * self);
void wxNotebook_SetPadding(wxNotebook * self, const wxSize * padding);
// Mix-in(s) to wxNotebook
wxWithImages *wxNotebook_AsWithImages(wxNotebook* obj);
wxTrackable *wxNotebook_AsTrackable(wxNotebook* obj);

// CLASS: wxNotificationMessage
wxClassInfo *wxNotificationMessage_CLASSINFO();
wxNotificationMessage *wxNotificationMessage_new();
wxNotificationMessage *wxNotificationMessage_new1(const wxString * title, const wxString * message, wxWindow * parent, int flags);
bool wxNotificationMessage_AddAction(wxNotificationMessage * self, wxWindowID actionid, const wxString * label);
bool wxNotificationMessage_Close(wxNotificationMessage * self);
void wxNotificationMessage_SetFlags(wxNotificationMessage * self, int flags);
void wxNotificationMessage_SetIcon(wxNotificationMessage * self, const wxIcon * icon);
void wxNotificationMessage_SetMessage(wxNotificationMessage * self, const wxString * message);
void wxNotificationMessage_SetParent(wxNotificationMessage * self, wxWindow * parent);
void wxNotificationMessage_SetTitle(wxNotificationMessage * self, const wxString * title);
bool wxNotificationMessage_Show(wxNotificationMessage * self, int timeout);
wxTaskBarIcon * wxNotificationMessage_UseTaskBarIcon(wxTaskBarIcon * icon);
bool wxNotificationMessage_MSWUseToasts(const wxString * shortcut_path, const wxString * app_id);
// Mix-in(s) to wxNotificationMessage
wxTrackable *wxNotificationMessage_AsTrackable(wxNotificationMessage* obj);

// CLASS: wxNotifyEvent
wxClassInfo *wxNotifyEvent_CLASSINFO();
void wxNotifyEvent_Allow(wxNotifyEvent * self);
bool wxNotifyEvent_IsAllowed(const wxNotifyEvent * self);
void wxNotifyEvent_Veto(wxNotifyEvent * self);

// CLASS: wxNumberEntryDialog
wxClassInfo *wxNumberEntryDialog_CLASSINFO();
wxNumberEntryDialog *wxNumberEntryDialog_new();
wxNumberEntryDialog *wxNumberEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * prompt, const wxString * caption, long value, long min, long max, const wxPoint * pos);
bool wxNumberEntryDialog_Create(wxNumberEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * prompt, const wxString * caption, long value, long min, long max, const wxPoint * pos);
long wxNumberEntryDialog_GetValue(const wxNumberEntryDialog * self);
// Mix-in(s) to wxNumberEntryDialog
wxTrackable *wxNumberEntryDialog_AsTrackable(wxNumberEntryDialog* obj);

// CLASS: wxOverlay
void wxOverlay_delete(wxOverlay *self);
wxOverlay *wxOverlay_new();
void wxOverlay_Reset(wxOverlay * self);

// CLASS: wxOwnerDrawnComboBox
wxClassInfo *wxOwnerDrawnComboBox_CLASSINFO();
wxOwnerDrawnComboBox *wxOwnerDrawnComboBox_new();
wxOwnerDrawnComboBox *wxOwnerDrawnComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxOwnerDrawnComboBox_Create(wxOwnerDrawnComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxOwnerDrawnComboBox_Create2(wxOwnerDrawnComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxOwnerDrawnComboBox_IsListEmpty(const wxOwnerDrawnComboBox * self);
bool wxOwnerDrawnComboBox_IsTextEmpty(const wxOwnerDrawnComboBox * self);
int wxOwnerDrawnComboBox_GetWidestItem(wxOwnerDrawnComboBox * self);
int wxOwnerDrawnComboBox_GetWidestItemWidth(wxOwnerDrawnComboBox * self);
// Mix-in(s) to wxOwnerDrawnComboBox
wxItemContainer *wxOwnerDrawnComboBox_AsItemContainer(wxOwnerDrawnComboBox* obj);
wxTextEntryBase *wxOwnerDrawnComboBox_AsTextEntry(wxOwnerDrawnComboBox* obj);
wxTrackable *wxOwnerDrawnComboBox_AsTrackable(wxOwnerDrawnComboBox* obj);

// CLASS: wxPCXHandler
wxClassInfo *wxPCXHandler_CLASSINFO();
wxPCXHandler *wxPCXHandler_new();

// CLASS: wxPNGHandler
wxClassInfo *wxPNGHandler_CLASSINFO();
wxPNGHandler *wxPNGHandler_new();

// CLASS: wxPNMHandler
wxClassInfo *wxPNMHandler_CLASSINFO();
wxPNMHandler *wxPNMHandler_new();

// CLASS: wxPageSetupDialog
wxClassInfo *wxPageSetupDialog_CLASSINFO();
wxPageSetupDialog *wxPageSetupDialog_new(wxWindow * parent, wxPageSetupDialogData * data);
wxPageSetupDialogData * wxPageSetupDialog_GetPageSetupData(wxPageSetupDialog * self);
int wxPageSetupDialog_ShowModal(wxPageSetupDialog * self);

// CLASS: wxPageSetupDialogData
wxClassInfo *wxPageSetupDialogData_CLASSINFO();
wxPageSetupDialogData *wxPageSetupDialogData_new();
wxPageSetupDialogData *wxPageSetupDialogData_new1(const wxPageSetupDialogData * data);
wxPageSetupDialogData *wxPageSetupDialogData_new2(const wxPrintData * print_data);
void wxPageSetupDialogData_EnableHelp(wxPageSetupDialogData * self, bool flag);
void wxPageSetupDialogData_EnableMargins(wxPageSetupDialogData * self, bool flag);
void wxPageSetupDialogData_EnableOrientation(wxPageSetupDialogData * self, bool flag);
void wxPageSetupDialogData_EnablePaper(wxPageSetupDialogData * self, bool flag);
void wxPageSetupDialogData_EnablePrinter(wxPageSetupDialogData * self, bool flag);
bool wxPageSetupDialogData_GetDefaultInfo(const wxPageSetupDialogData * self);
bool wxPageSetupDialogData_GetDefaultMinMargins(const wxPageSetupDialogData * self);
bool wxPageSetupDialogData_GetEnableHelp(const wxPageSetupDialogData * self);
bool wxPageSetupDialogData_GetEnableMargins(const wxPageSetupDialogData * self);
bool wxPageSetupDialogData_GetEnableOrientation(const wxPageSetupDialogData * self);
bool wxPageSetupDialogData_GetEnablePaper(const wxPageSetupDialogData * self);
bool wxPageSetupDialogData_GetEnablePrinter(const wxPageSetupDialogData * self);
wxPoint *wxPageSetupDialogData_GetMarginBottomRight(const wxPageSetupDialogData * self);
wxPoint *wxPageSetupDialogData_GetMarginTopLeft(const wxPageSetupDialogData * self);
wxPoint *wxPageSetupDialogData_GetMinMarginBottomRight(const wxPageSetupDialogData * self);
wxPoint *wxPageSetupDialogData_GetMinMarginTopLeft(const wxPageSetupDialogData * self);
wxSize *wxPageSetupDialogData_GetPaperSize(const wxPageSetupDialogData * self);
wxPrintData * wxPageSetupDialogData_GetPrintData(wxPageSetupDialogData * self);
wxPrintData *wxPageSetupDialogData_GetPrintData1(const wxPageSetupDialogData * self);
bool wxPageSetupDialogData_IsOk(const wxPageSetupDialogData * self);
void wxPageSetupDialogData_SetDefaultInfo(wxPageSetupDialogData * self, bool flag);
void wxPageSetupDialogData_SetDefaultMinMargins(wxPageSetupDialogData * self, bool flag);
void wxPageSetupDialogData_SetMarginBottomRight(wxPageSetupDialogData * self, const wxPoint * pt);
void wxPageSetupDialogData_SetMarginTopLeft(wxPageSetupDialogData * self, const wxPoint * pt);
void wxPageSetupDialogData_SetMinMarginBottomRight(wxPageSetupDialogData * self, const wxPoint * pt);
void wxPageSetupDialogData_SetMinMarginTopLeft(wxPageSetupDialogData * self, const wxPoint * pt);
void wxPageSetupDialogData_SetPaperSize(wxPageSetupDialogData * self, const wxSize * size);
void wxPageSetupDialogData_SetPrintData(wxPageSetupDialogData * self, const wxPrintData * print_data);

// CLASS: wxPaintDC
wxClassInfo *wxPaintDC_CLASSINFO();
wxPaintDC *wxPaintDC_new(wxWindow * window);

// CLASS: wxPaintEvent
wxClassInfo *wxPaintEvent_CLASSINFO();
wxPaintEvent *wxPaintEvent_new(wxWindow * window);

// CLASS: wxPalette
wxClassInfo *wxPalette_CLASSINFO();
wxPalette *wxPalette_new();
wxPalette *wxPalette_new1(const wxPalette * palette);
wxPalette *wxPalette_new2(int n, const unsigned char * red, const unsigned char * green, const unsigned char * blue);
bool wxPalette_Create(wxPalette * self, int n, const unsigned char * red, const unsigned char * green, const unsigned char * blue);
int wxPalette_GetColoursCount(const wxPalette * self);
bool wxPalette_GetRGB(const wxPalette * self, int pixel, unsigned char * red, unsigned char * green, unsigned char * blue);
bool wxPalette_IsOk(const wxPalette * self);

// CLASS: wxPanGestureEvent
wxClassInfo *wxPanGestureEvent_CLASSINFO();
wxPanGestureEvent *wxPanGestureEvent_new(wxWindowID winid);
wxPoint *wxPanGestureEvent_GetDelta(const wxPanGestureEvent * self);
void wxPanGestureEvent_SetDelta(wxPanGestureEvent * self, const wxPoint * delta);

// CLASS: wxPanel
wxClassInfo *wxPanel_CLASSINFO();
wxPanel *wxPanel_new();
wxPanel *wxPanel_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxPanel_Create(wxPanel * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxPanel_OnSysColourChanged(wxPanel * self, wxSysColourChangedEvent * event);
void wxPanel_SetFocusIgnoringChildren(wxPanel * self);
// Mix-in(s) to wxPanel
wxTrackable *wxPanel_AsTrackable(wxPanel* obj);

// CLASS: wxPasswordEntryDialog
wxClassInfo *wxPasswordEntryDialog_CLASSINFO();
wxPasswordEntryDialog *wxPasswordEntryDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, const wxString * default_value, long style, const wxPoint * pos);
// Mix-in(s) to wxPasswordEntryDialog
wxTrackable *wxPasswordEntryDialog_AsTrackable(wxPasswordEntryDialog* obj);

// CLASS: wxPen
wxClassInfo *wxPen_CLASSINFO();
wxPen *wxPen_new();
wxPen *wxPen_new1(const wxPenInfo * info);
wxPen *wxPen_new3(const wxBitmap * stipple, int width);
wxPen *wxPen_new4(const wxPen * pen);
wxColour *wxPen_GetColour(const wxPen * self);
int wxPen_GetDashes(const wxPen * self, wxDash ** dashes);
wxBitmap * wxPen_GetStipple(const wxPen * self);
int wxPen_GetWidth(const wxPen * self);
bool wxPen_IsOk(const wxPen * self);
bool wxPen_IsNonTransparent(const wxPen * self);
bool wxPen_IsTransparent(const wxPen * self);
void wxPen_SetColour(wxPen * self, wxColour * colour);
void wxPen_SetDashes(wxPen * self, int n, const wxDash * dash);
void wxPen_SetStipple(wxPen * self, const wxBitmap * stipple);
void wxPen_SetWidth(wxPen * self, int width);

// CLASS: wxPenList
void wxPenList_delete(wxPenList *self);
wxPenList *wxPenList_new();

// CLASS: wxPersistenceManager
void wxPersistenceManager_delete(wxPersistenceManager *self);
void wxPersistenceManager_Set(wxPersistenceManager * manager);
wxPersistenceManager * wxPersistenceManager_Get();
void wxPersistenceManager_DisableSaving(wxPersistenceManager * self);
void wxPersistenceManager_DisableRestoring(wxPersistenceManager * self);
wxPersistentObject * wxPersistenceManager_Register(wxPersistenceManager * self, T * obj);
wxPersistentObject * wxPersistenceManager_Register1(wxPersistenceManager * self, void * obj, wxPersistentObject * po);
wxPersistentObject * wxPersistenceManager_Find(const wxPersistenceManager * self, void * obj);
void wxPersistenceManager_Unregister(wxPersistenceManager * self, void * obj);
void wxPersistenceManager_Save(wxPersistenceManager * self, void * obj);
bool wxPersistenceManager_Restore(wxPersistenceManager * self, void * obj);
void wxPersistenceManager_SaveAndUnregister(wxPersistenceManager * self, void * obj);
bool wxPersistenceManager_RegisterAndRestore(wxPersistenceManager * self, T * obj);
bool wxPersistenceManager_RegisterAndRestore1(wxPersistenceManager * self, void * obj, wxPersistentObject * po);

// CLASS: wxPickerBase
wxClassInfo *wxPickerBase_CLASSINFO();
bool wxPickerBase_CreateBase(wxPickerBase * self, wxWindow * parent, wxWindowID id, const wxString * text, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxPickerBase_GetInternalMargin(const wxPickerBase * self);
int wxPickerBase_GetPickerCtrlProportion(const wxPickerBase * self);
wxTextCtrl * wxPickerBase_GetTextCtrl(wxPickerBase * self);
wxControl * wxPickerBase_GetPickerCtrl(wxPickerBase * self);
int wxPickerBase_GetTextCtrlProportion(const wxPickerBase * self);
bool wxPickerBase_HasTextCtrl(const wxPickerBase * self);
bool wxPickerBase_IsPickerCtrlGrowable(const wxPickerBase * self);
bool wxPickerBase_IsTextCtrlGrowable(const wxPickerBase * self);
void wxPickerBase_SetInternalMargin(wxPickerBase * self, int margin);
void wxPickerBase_SetPickerCtrlGrowable(wxPickerBase * self, bool grow);
void wxPickerBase_SetPickerCtrlProportion(wxPickerBase * self, int prop);
void wxPickerBase_SetTextCtrlGrowable(wxPickerBase * self, bool grow);
void wxPickerBase_SetTextCtrlProportion(wxPickerBase * self, int prop);
void wxPickerBase_SetTextCtrl(wxPickerBase * self, wxTextCtrl * text);
void wxPickerBase_SetPickerCtrl(wxPickerBase * self, wxControl * picker);
void wxPickerBase_UpdatePickerFromTextCtrl(wxPickerBase * self);
void wxPickerBase_UpdateTextCtrlFromPicker(wxPickerBase * self);
// Mix-in(s) to wxPickerBase
wxTrackable *wxPickerBase_AsTrackable(wxPickerBase* obj);

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self);
bool wxPoint_IsFullySpecified(const wxPoint * self);
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt);
wxPoint *wxPoint_new();
wxPoint *wxPoint_new1(int x, int y);
wxPoint *wxPoint_new2(const wxRealPoint * pt);

// CLASS: wxPopupTransientWindow
wxClassInfo *wxPopupTransientWindow_CLASSINFO();
wxPopupTransientWindow *wxPopupTransientWindow_new();
wxPopupTransientWindow *wxPopupTransientWindow_new1(wxWindow * parent, int flags);
void wxPopupTransientWindow_Popup(wxPopupTransientWindow * self, wxWindow * focus);
void wxPopupTransientWindow_Dismiss(wxPopupTransientWindow * self);
bool wxPopupTransientWindow_ProcessLeftDown(wxPopupTransientWindow * self, wxMouseEvent * event);
// Mix-in(s) to wxPopupTransientWindow
wxTrackable *wxPopupTransientWindow_AsTrackable(wxPopupTransientWindow* obj);

// CLASS: wxPopupWindow
wxClassInfo *wxPopupWindow_CLASSINFO();
wxPopupWindow *wxPopupWindow_new();
wxPopupWindow *wxPopupWindow_new1(wxWindow * parent, int flags);
bool wxPopupWindow_Create(wxPopupWindow * self, wxWindow * parent, int flags);
void wxPopupWindow_Position(wxPopupWindow * self, const wxPoint * pt_origin, const wxSize * size_popup);
// Mix-in(s) to wxPopupWindow
wxTrackable *wxPopupWindow_AsTrackable(wxPopupWindow* obj);

// CLASS: wxPreferencesEditor
void wxPreferencesEditor_delete(wxPreferencesEditor *self);
wxPreferencesEditor *wxPreferencesEditor_new(const wxString * title);
void wxPreferencesEditor_AddPage(wxPreferencesEditor * self, wxPreferencesPage * page);
void wxPreferencesEditor_Show(wxPreferencesEditor * self, wxWindow * parent);
void wxPreferencesEditor_Dismiss(wxPreferencesEditor * self);
bool wxPreferencesEditor_ShouldApplyChangesImmediately();
bool wxPreferencesEditor_ShownModally();

// CLASS: wxPreferencesPage
void wxPreferencesPage_delete(wxPreferencesPage *self);
wxPreferencesPage *wxPreferencesPage_new();
wxString *wxPreferencesPage_GetName(const wxPreferencesPage * self);
wxBitmapBundle *wxPreferencesPage_GetIcon(const wxPreferencesPage * self);
wxBitmap *wxPreferencesPage_GetLargeIcon(const wxPreferencesPage * self);
wxWindow * wxPreferencesPage_CreateWindow(wxPreferencesPage * self, wxWindow * parent);

// CLASS: wxPressAndTapEvent
wxClassInfo *wxPressAndTapEvent_CLASSINFO();
wxPressAndTapEvent *wxPressAndTapEvent_new(wxWindowID windid);

// CLASS: wxPreviewControlBar
wxClassInfo *wxPreviewControlBar_CLASSINFO();
wxPreviewControlBar *wxPreviewControlBar_new(wxPrintPreview * preview, long buttons, wxWindow * parent, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxPreviewControlBar_CreateButtons(wxPreviewControlBar * self);
wxPrintPreviewBase * wxPreviewControlBar_GetPrintPreview(const wxPreviewControlBar * self);
int wxPreviewControlBar_GetZoomControl(wxPreviewControlBar * self);
void wxPreviewControlBar_SetZoomControl(wxPreviewControlBar * self, int percent);
// Mix-in(s) to wxPreviewControlBar
wxTrackable *wxPreviewControlBar_AsTrackable(wxPreviewControlBar* obj);

// CLASS: wxPreviewFrame
wxClassInfo *wxPreviewFrame_CLASSINFO();
wxPreviewFrame *wxPreviewFrame_new(wxPrintPreviewBase * preview, wxWindow * parent, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxPreviewFrame_CreateCanvas(wxPreviewFrame * self);
void wxPreviewFrame_CreateControlBar(wxPreviewFrame * self);
void wxPreviewFrame_Initialize(wxPreviewFrame * self);
void wxPreviewFrame_OnCloseWindow(wxPreviewFrame * self, wxCloseEvent * event);
// Mix-in(s) to wxPreviewFrame
wxTrackable *wxPreviewFrame_AsTrackable(wxPreviewFrame* obj);

// CLASS: wxPrintData
wxClassInfo *wxPrintData_CLASSINFO();
wxPrintData *wxPrintData_new();
wxPrintData *wxPrintData_new1(const wxPrintData * data);
bool wxPrintData_GetCollate(const wxPrintData * self);
bool wxPrintData_GetColour(const wxPrintData * self);
int wxPrintData_GetNoCopies(const wxPrintData * self);
wxString *wxPrintData_GetPrinterName(const wxPrintData * self);
bool wxPrintData_IsOk(const wxPrintData * self);
void wxPrintData_SetCollate(wxPrintData * self, bool flag);
void wxPrintData_SetColour(wxPrintData * self, bool flag);
void wxPrintData_SetNoCopies(wxPrintData * self, int n);
void wxPrintData_SetPaperSize(wxPrintData * self, const wxSize * size);
void wxPrintData_SetPrinterName(wxPrintData * self, const wxString * printer_name);
wxString *wxPrintData_GetFilename(const wxPrintData * self);
void wxPrintData_SetFilename(wxPrintData * self, const wxString * filename);

// CLASS: wxPrintDialog
wxClassInfo *wxPrintDialog_CLASSINFO();
wxPrintDialog *wxPrintDialog_new(wxWindow * parent, wxPrintDialogData * data);
wxPrintDialog *wxPrintDialog_new1(wxWindow * parent, wxPrintData * data);
wxDC * wxPrintDialog_GetPrintDC(wxPrintDialog * self);
wxPrintDialogData * wxPrintDialog_GetPrintDialogData(wxPrintDialog * self);
wxPrintData * wxPrintDialog_GetPrintData(wxPrintDialog * self);
int wxPrintDialog_ShowModal(wxPrintDialog * self);

// CLASS: wxPrintDialogData
wxClassInfo *wxPrintDialogData_CLASSINFO();
wxPrintDialogData *wxPrintDialogData_new();
wxPrintDialogData *wxPrintDialogData_new1(const wxPrintDialogData * dialog_data);
wxPrintDialogData *wxPrintDialogData_new2(const wxPrintData * print_data);
void wxPrintDialogData_EnableHelp(wxPrintDialogData * self, bool flag);
void wxPrintDialogData_EnablePageNumbers(wxPrintDialogData * self, bool flag);
void wxPrintDialogData_EnablePrintToFile(wxPrintDialogData * self, bool flag);
void wxPrintDialogData_EnableSelection(wxPrintDialogData * self, bool flag);
bool wxPrintDialogData_GetAllPages(const wxPrintDialogData * self);
bool wxPrintDialogData_GetCollate(const wxPrintDialogData * self);
int wxPrintDialogData_GetFromPage(const wxPrintDialogData * self);
int wxPrintDialogData_GetMaxPage(const wxPrintDialogData * self);
int wxPrintDialogData_GetMinPage(const wxPrintDialogData * self);
int wxPrintDialogData_GetNoCopies(const wxPrintDialogData * self);
wxPrintData * wxPrintDialogData_GetPrintData(wxPrintDialogData * self);
bool wxPrintDialogData_GetPrintToFile(const wxPrintDialogData * self);
bool wxPrintDialogData_GetSelection(const wxPrintDialogData * self);
int wxPrintDialogData_GetToPage(const wxPrintDialogData * self);
bool wxPrintDialogData_IsOk(const wxPrintDialogData * self);
void wxPrintDialogData_SetCollate(wxPrintDialogData * self, bool flag);
void wxPrintDialogData_SetFromPage(wxPrintDialogData * self, int page);
void wxPrintDialogData_SetMaxPage(wxPrintDialogData * self, int page);
void wxPrintDialogData_SetMinPage(wxPrintDialogData * self, int page);
void wxPrintDialogData_SetNoCopies(wxPrintDialogData * self, int n);
void wxPrintDialogData_SetPrintData(wxPrintDialogData * self, const wxPrintData * print_data);
void wxPrintDialogData_SetPrintToFile(wxPrintDialogData * self, bool flag);
void wxPrintDialogData_SetSelection(wxPrintDialogData * self, bool flag);
void wxPrintDialogData_SetSetupDialog(wxPrintDialogData * self, bool flag);
void wxPrintDialogData_SetToPage(wxPrintDialogData * self, int page);

// CLASS: wxPrintPreview
wxClassInfo *wxPrintPreview_CLASSINFO();
wxPrintPreview *wxPrintPreview_new(wxPrintout * printout, wxPrintout * printout_for_printing, wxPrintDialogData * data);
wxPrintPreview *wxPrintPreview_new1(wxPrintout * printout, wxPrintout * printout_for_printing, wxPrintData * data);
wxPreviewCanvas * wxPrintPreview_GetCanvas(const wxPrintPreview * self);
int wxPrintPreview_GetCurrentPage(const wxPrintPreview * self);
wxFrame * wxPrintPreview_GetFrame(const wxPrintPreview * self);
int wxPrintPreview_GetMaxPage(const wxPrintPreview * self);
int wxPrintPreview_GetMinPage(const wxPrintPreview * self);
wxPrintout * wxPrintPreview_GetPrintout(const wxPrintPreview * self);
wxPrintout * wxPrintPreview_GetPrintoutForPrinting(const wxPrintPreview * self);
bool wxPrintPreview_IsOk(const wxPrintPreview * self);
bool wxPrintPreview_PaintPage(wxPrintPreview * self, wxPreviewCanvas * canvas, wxDC * dc);
bool wxPrintPreview_Print(wxPrintPreview * self, bool prompt);
bool wxPrintPreview_RenderPage(wxPrintPreview * self, int page_num);
void wxPrintPreview_SetCanvas(wxPrintPreview * self, wxPreviewCanvas * window);
bool wxPrintPreview_SetCurrentPage(wxPrintPreview * self, int page_num);
void wxPrintPreview_SetFrame(wxPrintPreview * self, wxFrame * frame);
void wxPrintPreview_SetPrintout(wxPrintPreview * self, wxPrintout * printout);
void wxPrintPreview_SetZoom(wxPrintPreview * self, int percent);

// CLASS: wxPrinter
wxClassInfo *wxPrinter_CLASSINFO();
wxPrinter *wxPrinter_new(wxPrintDialogData * data);
wxPrintAbortDialog * wxPrinter_CreateAbortWindow(wxPrinter * self, wxWindow * parent, wxPrintout * printout);
bool wxPrinter_GetAbort(const wxPrinter * self);
wxPrintDialogData * wxPrinter_GetPrintDialogData(const wxPrinter * self);
bool wxPrinter_Print(wxPrinter * self, wxWindow * parent, wxPrintout * printout, bool prompt);
wxDC * wxPrinter_PrintDialog(wxPrinter * self, wxWindow * parent);
void wxPrinter_ReportError(wxPrinter * self, wxWindow * parent, wxPrintout * printout, const wxString * message);
bool wxPrinter_Setup(wxPrinter * self, wxWindow * parent);

// CLASS: wxPrinterDC
wxClassInfo *wxPrinterDC_CLASSINFO();
wxPrinterDC *wxPrinterDC_new(const wxPrintData * print_data);
wxRect *wxPrinterDC_GetPaperRect(const wxPrinterDC * self);

// CLASS: wxPrintout
wxClassInfo *wxPrintout_CLASSINFO();
wxPrintout *wxPrintout_new(const wxString * title);
void wxPrintout_FitThisSizeToPage(wxPrintout * self, const wxSize * image_size);
void wxPrintout_FitThisSizeToPageMargins(wxPrintout * self, const wxSize * image_size, const wxPageSetupDialogData * page_setup_data);
void wxPrintout_FitThisSizeToPaper(wxPrintout * self, const wxSize * image_size);
wxDC * wxPrintout_GetDC(const wxPrintout * self);
wxRect *wxPrintout_GetLogicalPageMarginsRect(const wxPrintout * self, const wxPageSetupDialogData * page_setup_data);
wxRect *wxPrintout_GetLogicalPageRect(const wxPrintout * self);
wxRect *wxPrintout_GetLogicalPaperRect(const wxPrintout * self);
void wxPrintout_GetPPIPrinter(const wxPrintout * self, int * w, int * h);
void wxPrintout_GetPPIScreen(const wxPrintout * self, int * w, int * h);
void wxPrintout_GetPageInfo(wxPrintout * self, int * min_page, int * max_page, int * page_from, int * page_to);
void wxPrintout_GetPageSizeMM(const wxPrintout * self, int * w, int * h);
void wxPrintout_GetPageSizePixels(const wxPrintout * self, int * w, int * h);
wxRect *wxPrintout_GetPaperRectPixels(const wxPrintout * self);
wxString *wxPrintout_GetTitle(const wxPrintout * self);
bool wxPrintout_HasPage(wxPrintout * self, int page_num);
bool wxPrintout_IsPreview(const wxPrintout * self);
wxPrintPreview * wxPrintout_GetPreview(const wxPrintout * self);
void wxPrintout_MapScreenSizeToDevice(wxPrintout * self);
void wxPrintout_MapScreenSizeToPage(wxPrintout * self);
void wxPrintout_MapScreenSizeToPageMargins(wxPrintout * self, const wxPageSetupDialogData * page_setup_data);
void wxPrintout_MapScreenSizeToPaper(wxPrintout * self);
void wxPrintout_OffsetLogicalOrigin(wxPrintout * self, wxCoord xoff, wxCoord yoff);
bool wxPrintout_OnBeginDocument(wxPrintout * self, int start_page, int end_page);
void wxPrintout_OnBeginPrinting(wxPrintout * self);
void wxPrintout_OnEndDocument(wxPrintout * self);
void wxPrintout_OnEndPrinting(wxPrintout * self);
void wxPrintout_OnPreparePrinting(wxPrintout * self);
bool wxPrintout_OnPrintPage(wxPrintout * self, int page_num);
void wxPrintout_SetLogicalOrigin(wxPrintout * self, wxCoord x, wxCoord y);

// CLASS: wxPropertySheetDialog
wxClassInfo *wxPropertySheetDialog_CLASSINFO();
wxPropertySheetDialog *wxPropertySheetDialog_new();
wxPropertySheetDialog *wxPropertySheetDialog_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxPropertySheetDialog_AddBookCtrl(wxPropertySheetDialog * self, wxSizer * sizer);
bool wxPropertySheetDialog_Create(wxPropertySheetDialog * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxBookCtrlBase * wxPropertySheetDialog_CreateBookCtrl(wxPropertySheetDialog * self);
void wxPropertySheetDialog_CreateButtons(wxPropertySheetDialog * self, int flags);
wxBookCtrlBase * wxPropertySheetDialog_GetBookCtrl(const wxPropertySheetDialog * self);
wxSizer * wxPropertySheetDialog_GetInnerSizer(const wxPropertySheetDialog * self);
void wxPropertySheetDialog_SetInnerSizer(wxPropertySheetDialog * self, wxSizer * sizer);
long wxPropertySheetDialog_GetSheetStyle(const wxPropertySheetDialog * self);
void wxPropertySheetDialog_LayoutDialog(wxPropertySheetDialog * self, int centre_flags);
void wxPropertySheetDialog_SetBookCtrl(wxPropertySheetDialog * self, wxBookCtrlBase * book_ctrl);
void wxPropertySheetDialog_SetSheetStyle(wxPropertySheetDialog * self, long style);
void wxPropertySheetDialog_SetSheetOuterBorder(wxPropertySheetDialog * self, int border);
int wxPropertySheetDialog_GetSheetOuterBorder(const wxPropertySheetDialog * self);
void wxPropertySheetDialog_SetSheetInnerBorder(wxPropertySheetDialog * self, int border);
int wxPropertySheetDialog_GetSheetInnerBorder(const wxPropertySheetDialog * self);
// Mix-in(s) to wxPropertySheetDialog
wxTrackable *wxPropertySheetDialog_AsTrackable(wxPropertySheetDialog* obj);

// CLASS: wxQuantize
wxClassInfo *wxQuantize_CLASSINFO();
wxQuantize *wxQuantize_new();
void wxQuantize_DoQuantize(unsigned int w, unsigned int h, unsigned char ** in_rows, unsigned char ** out_rows, unsigned char * palette, int desired_no_colours);
bool wxQuantize_Quantize(const wxImage * src, wxImage * dest, wxPalette ** p_palette, int desired_no_colours, unsigned char ** eight_bit_data, int flags);
bool wxQuantize_Quantize1(const wxImage * src, wxImage * dest, int desired_no_colours, unsigned char ** eight_bit_data, int flags);

// CLASS: wxQueryLayoutInfoEvent
wxClassInfo *wxQueryLayoutInfoEvent_CLASSINFO();
wxQueryLayoutInfoEvent *wxQueryLayoutInfoEvent_new(wxWindowID id);
int wxQueryLayoutInfoEvent_GetFlags(const wxQueryLayoutInfoEvent * self);
int wxQueryLayoutInfoEvent_GetRequestedLength(const wxQueryLayoutInfoEvent * self);
wxSize *wxQueryLayoutInfoEvent_GetSize(const wxQueryLayoutInfoEvent * self);
void wxQueryLayoutInfoEvent_SetFlags(wxQueryLayoutInfoEvent * self, int flags);
void wxQueryLayoutInfoEvent_SetRequestedLength(wxQueryLayoutInfoEvent * self, int length);
void wxQueryLayoutInfoEvent_SetSize(wxQueryLayoutInfoEvent * self, const wxSize * size);

// CLASS: wxRadioBox
wxClassInfo *wxRadioBox_CLASSINFO();
wxRadioBox *wxRadioBox_new();
wxRadioBox *wxRadioBox_new2(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name);
bool wxRadioBox_Create1(wxRadioBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name);
bool wxRadioBox_Enable(wxRadioBox * self, unsigned int n, bool enable);
unsigned int wxRadioBox_GetColumnCount(const wxRadioBox * self);
int wxRadioBox_GetItemFromPoint(const wxRadioBox * self, const wxPoint * pt);
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxRadioBox_GetItemHelpText(const wxRadioBox * self, unsigned int item);
#endif
wxToolTip * wxRadioBox_GetItemToolTip(const wxRadioBox * self, unsigned int item);
unsigned int wxRadioBox_GetRowCount(const wxRadioBox * self);
bool wxRadioBox_IsItemEnabled(const wxRadioBox * self, unsigned int n);
bool wxRadioBox_IsItemShown(const wxRadioBox * self, unsigned int n);
void wxRadioBox_SetItemHelpText(wxRadioBox * self, unsigned int item, const wxString * helptext);
void wxRadioBox_SetItemToolTip(wxRadioBox * self, unsigned int item, const wxString * text);
bool wxRadioBox_Show(wxRadioBox * self, unsigned int item, bool show);
// Mix-in(s) to wxRadioBox
wxItemContainerImmutable *wxRadioBox_AsItemContainerImmutable(wxRadioBox* obj);
wxTrackable *wxRadioBox_AsTrackable(wxRadioBox* obj);

// CLASS: wxRadioButton
wxClassInfo *wxRadioButton_CLASSINFO();
wxRadioButton *wxRadioButton_new();
wxRadioButton *wxRadioButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxRadioButton_Create(wxRadioButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxRadioButton_GetValue(const wxRadioButton * self);
void wxRadioButton_SetValue(wxRadioButton * self, bool value);
wxRadioButton * wxRadioButton_GetFirstInGroup(const wxRadioButton * self);
wxRadioButton * wxRadioButton_GetLastInGroup(const wxRadioButton * self);
wxRadioButton * wxRadioButton_GetPreviousInGroup(const wxRadioButton * self);
wxRadioButton * wxRadioButton_GetNextInGroup(const wxRadioButton * self);
// Mix-in(s) to wxRadioButton
wxTrackable *wxRadioButton_AsTrackable(wxRadioButton* obj);

// CLASS: wxRealPoint
void wxRealPoint_delete(wxRealPoint *self);
wxRealPoint *wxRealPoint_new();
wxRealPoint *wxRealPoint_new1(double x, double y);
wxRealPoint *wxRealPoint_new2(const wxPoint * pt);

// CLASS: wxRearrangeCtrl
wxClassInfo *wxRearrangeCtrl_CLASSINFO();
wxRearrangeCtrl *wxRearrangeCtrl_new();
wxRearrangeCtrl *wxRearrangeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name);
bool wxRearrangeCtrl_Create(wxRearrangeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name);
wxRearrangeList * wxRearrangeCtrl_GetList(const wxRearrangeCtrl * self);
// Mix-in(s) to wxRearrangeCtrl
wxTrackable *wxRearrangeCtrl_AsTrackable(wxRearrangeCtrl* obj);

// CLASS: wxRearrangeDialog
wxClassInfo *wxRearrangeDialog_CLASSINFO();
wxRearrangeDialog *wxRearrangeDialog_new();
wxRearrangeDialog *wxRearrangeDialog_new1(wxWindow * parent, const wxString * message, const wxString * title, const wxArrayInt * order, const wxArrayString * items, const wxPoint * pos, const wxString * name);
bool wxRearrangeDialog_Create(wxRearrangeDialog * self, wxWindow * parent, const wxString * message, const wxString * title, const wxArrayInt * order, const wxArrayString * items, const wxPoint * pos, const wxString * name);
void wxRearrangeDialog_AddExtraControls(wxRearrangeDialog * self, wxWindow * win);
wxRearrangeList * wxRearrangeDialog_GetList(const wxRearrangeDialog * self);
wxArrayInt *wxRearrangeDialog_GetOrder(const wxRearrangeDialog * self);
// Mix-in(s) to wxRearrangeDialog
wxTrackable *wxRearrangeDialog_AsTrackable(wxRearrangeDialog* obj);

// CLASS: wxRearrangeList
wxClassInfo *wxRearrangeList_CLASSINFO();
wxRearrangeList *wxRearrangeList_new();
wxRearrangeList *wxRearrangeList_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name);
bool wxRearrangeList_Create(wxRearrangeList * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name);
wxArrayInt *wxRearrangeList_GetCurrentOrder(const wxRearrangeList * self);
bool wxRearrangeList_CanMoveCurrentUp(const wxRearrangeList * self);
bool wxRearrangeList_CanMoveCurrentDown(const wxRearrangeList * self);
bool wxRearrangeList_MoveCurrentUp(wxRearrangeList * self);
bool wxRearrangeList_MoveCurrentDown(wxRearrangeList * self);
// Mix-in(s) to wxRearrangeList
wxItemContainer *wxRearrangeList_AsItemContainer(wxRearrangeList* obj);
wxTrackable *wxRearrangeList_AsTrackable(wxRearrangeList* obj);

// CLASS: wxRect
void wxRect_delete(wxRect *self);
wxRect *wxRect_new();
wxRect *wxRect_new1(int x, int y, int width, int height);
wxRect *wxRect_new2(const wxPoint * top_left, const wxPoint * bottom_right);
wxRect *wxRect_new3(const wxPoint * pos, const wxSize * size);
wxRect *wxRect_new4(const wxSize * size);
wxRect *wxRect_CentreIn(const wxRect * self, const wxRect * r, int dir);
wxRect *wxRect_CenterIn(const wxRect * self, const wxRect * r, int dir);
bool wxRect_Contains(const wxRect * self, int x, int y);
bool wxRect_Contains1(const wxRect * self, const wxPoint * pt);
bool wxRect_Contains2(const wxRect * self, const wxRect * rect);
wxRect *wxRect_Deflate3(const wxRect * self, wxCoord dx, wxCoord dy);
int wxRect_GetBottom(const wxRect * self);
wxPoint *wxRect_GetBottomLeft(const wxRect * self);
wxPoint *wxRect_GetBottomRight(const wxRect * self);
int wxRect_GetHeight(const wxRect * self);
int wxRect_GetLeft(const wxRect * self);
wxPoint *wxRect_GetPosition(const wxRect * self);
int wxRect_GetRight(const wxRect * self);
wxSize *wxRect_GetSize(const wxRect * self);
int wxRect_GetTop(const wxRect * self);
wxPoint *wxRect_GetTopLeft(const wxRect * self);
wxPoint *wxRect_GetTopRight(const wxRect * self);
int wxRect_GetWidth(const wxRect * self);
int wxRect_GetX(const wxRect * self);
int wxRect_GetY(const wxRect * self);
wxRect *wxRect_Inflate3(const wxRect * self, wxCoord dx, wxCoord dy);
wxRect *wxRect_Intersect1(const wxRect * self, const wxRect * rect);
bool wxRect_Intersects(const wxRect * self, const wxRect * rect);
bool wxRect_IsEmpty(const wxRect * self);
void wxRect_Offset(wxRect * self, wxCoord dx, wxCoord dy);
void wxRect_Offset1(wxRect * self, const wxPoint * pt);
void wxRect_SetHeight(wxRect * self, int height);
void wxRect_SetPosition(wxRect * self, const wxPoint * pos);
void wxRect_SetSize(wxRect * self, const wxSize * s);
void wxRect_SetWidth(wxRect * self, int width);
void wxRect_SetX(wxRect * self, int x);
void wxRect_SetY(wxRect * self, int y);
void wxRect_SetLeft(wxRect * self, int left);
void wxRect_SetRight(wxRect * self, int right);
void wxRect_SetTop(wxRect * self, int top);
void wxRect_SetBottom(wxRect * self, int bottom);
void wxRect_SetTopLeft(wxRect * self, const wxPoint * p);
void wxRect_SetBottomRight(wxRect * self, const wxPoint * p);
void wxRect_SetTopRight(wxRect * self, const wxPoint * p);
void wxRect_SetBottomLeft(wxRect * self, const wxPoint * p);
wxRect *wxRect_Union(const wxRect * self, const wxRect * rect);

// CLASS: wxRegion
wxClassInfo *wxRegion_CLASSINFO();
wxRegion *wxRegion_new();
wxRegion *wxRegion_new1(wxCoord x, wxCoord y, wxCoord width, wxCoord height);
wxRegion *wxRegion_new2(const wxPoint * top_left, const wxPoint * bottom_right);
wxRegion *wxRegion_new3(const wxRect * rect);
wxRegion *wxRegion_new4(const wxRegion * region);
wxRegion *wxRegion_new6(const wxBitmap * bmp);
wxRegion *wxRegion_new7(const wxBitmap * bmp, const wxColour * trans_colour, int tolerance);
void wxRegion_Clear(wxRegion * self);
wxBitmap *wxRegion_ConvertToBitmap(const wxRegion * self);
void wxRegion_GetBox(const wxRegion * self, wxCoord * x, wxCoord * y, wxCoord * width, wxCoord * height);
wxRect *wxRegion_GetBox1(const wxRegion * self);
bool wxRegion_Intersect(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
bool wxRegion_Intersect1(wxRegion * self, const wxRect * rect);
bool wxRegion_Intersect2(wxRegion * self, const wxRegion * region);
bool wxRegion_IsEmpty(const wxRegion * self);
bool wxRegion_IsEqual(const wxRegion * self, const wxRegion * region);
bool wxRegion_Offset(wxRegion * self, wxCoord x, wxCoord y);
bool wxRegion_Offset1(wxRegion * self, const wxPoint * pt);
bool wxRegion_Subtract(wxRegion * self, const wxRect * rect);
bool wxRegion_Subtract1(wxRegion * self, const wxRegion * region);
bool wxRegion_Union(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
bool wxRegion_Union1(wxRegion * self, const wxRect * rect);
bool wxRegion_Union2(wxRegion * self, const wxRegion * region);
bool wxRegion_Union3(wxRegion * self, const wxBitmap * bmp);
bool wxRegion_Union4(wxRegion * self, const wxBitmap * bmp, const wxColour * trans_colour, int tolerance);
bool wxRegion_Xor(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
bool wxRegion_Xor1(wxRegion * self, const wxRect * rect);
bool wxRegion_Xor2(wxRegion * self, const wxRegion * region);

// CLASS: wxRegionIterator
wxClassInfo *wxRegionIterator_CLASSINFO();
wxRegionIterator *wxRegionIterator_new();
wxRegionIterator *wxRegionIterator_new1(const wxRegion * region);
wxCoord wxRegionIterator_GetH(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetHeight(const wxRegionIterator * self);
wxRect *wxRegionIterator_GetRect(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetW(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetWidth(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetX(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetY(const wxRegionIterator * self);
bool wxRegionIterator_HaveRects(const wxRegionIterator * self);
void wxRegionIterator_Reset(wxRegionIterator * self);
void wxRegionIterator_Reset1(wxRegionIterator * self, const wxRegion * region);

// CLASS: wxRendererNative
void wxRendererNative_delete(wxRendererNative *self);
void wxRendererNative_DrawCheckBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawComboBoxDropButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawDropArrow(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawFocusRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawGauge(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int value, int max, int flags);
void wxRendererNative_DrawItemSelectionRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawItemText(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxString * text, const wxRect * rect, int align, int flags, wxEllipsizeMode ellipsize_mode);
void wxRendererNative_DrawPushButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawCollapseButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
wxSize *wxRendererNative_GetCollapseButtonSize(wxRendererNative * self, wxWindow * win, wxDC * dc);
void wxRendererNative_DrawSplitterBorder(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawTreeItemButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawChoice(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawComboBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawTextCtrl(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawRadioBitmap(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawCheckMark(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
wxSize *wxRendererNative_GetCheckBoxSize(wxRendererNative * self, wxWindow * win, int flags);
wxSize *wxRendererNative_GetCheckMarkSize(wxRendererNative * self, wxWindow * win);
wxSize *wxRendererNative_GetExpanderSize(wxRendererNative * self, wxWindow * win);
int wxRendererNative_GetHeaderButtonHeight(wxRendererNative * self, wxWindow * win);
int wxRendererNative_GetHeaderButtonMargin(wxRendererNative * self, wxWindow * win);
wxRendererNative * wxRendererNative_Get();
wxRendererNative * wxRendererNative_GetDefault();
wxRendererNative * wxRendererNative_GetGeneric();
wxRendererNative * wxRendererNative_Load(const wxString * name);
wxRendererNative * wxRendererNative_Set(wxRendererNative * renderer);

// CLASS: wxRichToolTip
void wxRichToolTip_delete(wxRichToolTip *self);
wxRichToolTip *wxRichToolTip_new(const wxString * title, const wxString * message);
void wxRichToolTip_SetBackgroundColour(wxRichToolTip * self, const wxColour * col, const wxColour * col_end);
void wxRichToolTip_SetIcon(wxRichToolTip * self, int icon);
void wxRichToolTip_SetIcon1(wxRichToolTip * self, const wxBitmapBundle * icon);
void wxRichToolTip_SetTitleFont(wxRichToolTip * self, const wxFont * font);
void wxRichToolTip_ShowFor(wxRichToolTip * self, wxWindow * win, const wxRect * rect);

// CLASS: wxRotateGestureEvent
wxClassInfo *wxRotateGestureEvent_CLASSINFO();
wxRotateGestureEvent *wxRotateGestureEvent_new(wxWindowID windid);
double wxRotateGestureEvent_GetRotationAngle(const wxRotateGestureEvent * self);
void wxRotateGestureEvent_SetRotationAngle(wxRotateGestureEvent * self, double rotation_angle);

// CLASS: wxSVGBitmapEmbedHandler
void wxSVGBitmapEmbedHandler_delete(wxSVGBitmapEmbedHandler *self);

// CLASS: wxSVGBitmapFileHandler
void wxSVGBitmapFileHandler_delete(wxSVGBitmapFileHandler *self);
wxSVGBitmapFileHandler *wxSVGBitmapFileHandler_new(const wxFileName * path);

// CLASS: wxSVGBitmapHandler
void wxSVGBitmapHandler_delete(wxSVGBitmapHandler *self);
bool wxSVGBitmapHandler_ProcessBitmap(const wxSVGBitmapHandler * self, const wxBitmap * bitmap, wxCoord x, wxCoord y, wxOutputStream * stream);

// CLASS: wxSVGFileDC
wxClassInfo *wxSVGFileDC_CLASSINFO();
wxSVGFileDC *wxSVGFileDC_new(const wxString * filename, int width, int height, double dpi, const wxString * title);
void wxSVGFileDC_Clear(wxSVGFileDC * self);
void wxSVGFileDC_SetBitmapHandler(wxSVGFileDC * self, wxSVGBitmapHandler * handler);
void wxSVGFileDC_DestroyClippingRegion(wxSVGFileDC * self);
void wxSVGFileDC_CrossHair(wxSVGFileDC * self, wxCoord x, wxCoord y);
bool wxSVGFileDC_GetPixel(const wxSVGFileDC * self, wxCoord x, wxCoord y, wxColour * colour);
void wxSVGFileDC_SetPalette(wxSVGFileDC * self, const wxPalette * palette);
int wxSVGFileDC_GetDepth(const wxSVGFileDC * self);
bool wxSVGFileDC_StartDoc(wxSVGFileDC * self, const wxString * message);
void wxSVGFileDC_EndDoc(wxSVGFileDC * self);
void wxSVGFileDC_StartPage(wxSVGFileDC * self);
void wxSVGFileDC_EndPage(wxSVGFileDC * self);

// CLASS: wxSashEvent
wxClassInfo *wxSashEvent_CLASSINFO();
wxRect *wxSashEvent_GetDragRect(const wxSashEvent * self);
void wxSashEvent_SetDragRect(wxSashEvent * self, const wxRect * rect);

// CLASS: wxSashLayoutWindow
wxClassInfo *wxSashLayoutWindow_CLASSINFO();
wxSashLayoutWindow *wxSashLayoutWindow_new();
wxSashLayoutWindow *wxSashLayoutWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSashLayoutWindow_Create(wxSashLayoutWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxSashLayoutWindow_OnCalculateLayout(wxSashLayoutWindow * self, wxCalculateLayoutEvent * event);
void wxSashLayoutWindow_OnQueryLayoutInfo(wxSashLayoutWindow * self, wxQueryLayoutInfoEvent * event);
void wxSashLayoutWindow_SetDefaultSize(wxSashLayoutWindow * self, const wxSize * size);
// Mix-in(s) to wxSashLayoutWindow
wxTrackable *wxSashLayoutWindow_AsTrackable(wxSashLayoutWindow* obj);

// CLASS: wxSashWindow
wxClassInfo *wxSashWindow_CLASSINFO();
wxSashWindow *wxSashWindow_new();
wxSashWindow *wxSashWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
int wxSashWindow_GetMaximumSizeX(const wxSashWindow * self);
int wxSashWindow_GetMaximumSizeY(const wxSashWindow * self);
int wxSashWindow_GetMinimumSizeX(const wxSashWindow * self);
int wxSashWindow_GetMinimumSizeY(const wxSashWindow * self);
void wxSashWindow_SetMaximumSizeX(wxSashWindow * self, int min);
void wxSashWindow_SetMaximumSizeY(wxSashWindow * self, int min);
void wxSashWindow_SetMinimumSizeX(wxSashWindow * self, int min);
void wxSashWindow_SetMinimumSizeY(wxSashWindow * self, int min);
void wxSashWindow_SetDefaultBorderSize(wxSashWindow * self, int width);
int wxSashWindow_GetDefaultBorderSize(const wxSashWindow * self);
void wxSashWindow_SetExtraBorderSize(wxSashWindow * self, int width);
int wxSashWindow_GetExtraBorderSize(const wxSashWindow * self);
void wxSashWindow_SizeWindows(wxSashWindow * self);
// Mix-in(s) to wxSashWindow
wxTrackable *wxSashWindow_AsTrackable(wxSashWindow* obj);

// CLASS: wxScreenDC
wxClassInfo *wxScreenDC_CLASSINFO();
wxScreenDC *wxScreenDC_new();
bool wxScreenDC_EndDrawingOnTop();
bool wxScreenDC_StartDrawingOnTop(wxWindow * window);
bool wxScreenDC_StartDrawingOnTop1(wxRect * rect);

// CLASS: wxScrollBar
wxClassInfo *wxScrollBar_CLASSINFO();
wxScrollBar *wxScrollBar_new();
wxScrollBar *wxScrollBar_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxScrollBar_Create(wxScrollBar * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxScrollBar_GetPageSize(const wxScrollBar * self);
int wxScrollBar_GetRange(const wxScrollBar * self);
int wxScrollBar_GetThumbPosition(const wxScrollBar * self);
int wxScrollBar_GetThumbSize(const wxScrollBar * self);
void wxScrollBar_SetThumbPosition(wxScrollBar * self, int view_start);
bool wxScrollBar_IsVertical(const wxScrollBar * self);
// Mix-in(s) to wxScrollBar
wxTrackable *wxScrollBar_AsTrackable(wxScrollBar* obj);

// CLASS: wxScrollEvent
wxClassInfo *wxScrollEvent_CLASSINFO();
int wxScrollEvent_GetOrientation(const wxScrollEvent * self);
int wxScrollEvent_GetPosition(const wxScrollEvent * self);
void wxScrollEvent_SetOrientation(wxScrollEvent * self, int orient);
void wxScrollEvent_SetPosition(wxScrollEvent * self, int pos);

// CLASS: wxScrollWinEvent
wxClassInfo *wxScrollWinEvent_CLASSINFO();
int wxScrollWinEvent_GetOrientation(const wxScrollWinEvent * self);
int wxScrollWinEvent_GetPosition(const wxScrollWinEvent * self);
void wxScrollWinEvent_SetOrientation(wxScrollWinEvent * self, int orient);
void wxScrollWinEvent_SetPosition(wxScrollWinEvent * self, int pos);

// CLASS: wxSearchCtrl
wxClassInfo *wxSearchCtrl_CLASSINFO();
wxSearchCtrl *wxSearchCtrl_new();
wxSearchCtrl *wxSearchCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxSearchCtrl_Create(wxSearchCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxMenu * wxSearchCtrl_GetMenu(wxSearchCtrl * self);
bool wxSearchCtrl_IsSearchButtonVisible(const wxSearchCtrl * self);
bool wxSearchCtrl_IsCancelButtonVisible(const wxSearchCtrl * self);
void wxSearchCtrl_SetMenu(wxSearchCtrl * self, wxMenu * menu);
void wxSearchCtrl_ShowCancelButton(wxSearchCtrl * self, bool show);
void wxSearchCtrl_ShowSearchButton(wxSearchCtrl * self, bool show);
void wxSearchCtrl_SetDescriptiveText(wxSearchCtrl * self, const wxString * text);
wxString *wxSearchCtrl_GetDescriptiveText(const wxSearchCtrl * self);
// Mix-in(s) to wxSearchCtrl
wxTextEntryBase *wxSearchCtrl_AsTextEntry(wxSearchCtrl* obj);
wxTrackable *wxSearchCtrl_AsTrackable(wxSearchCtrl* obj);

// CLASS: wxSetCursorEvent
wxClassInfo *wxSetCursorEvent_CLASSINFO();
wxSetCursorEvent *wxSetCursorEvent_new(wxCoord x, wxCoord y);
wxCursor *wxSetCursorEvent_GetCursor(const wxSetCursorEvent * self);
wxCoord wxSetCursorEvent_GetX(const wxSetCursorEvent * self);
wxCoord wxSetCursorEvent_GetY(const wxSetCursorEvent * self);
bool wxSetCursorEvent_HasCursor(const wxSetCursorEvent * self);
void wxSetCursorEvent_SetCursor(wxSetCursorEvent * self, const wxCursor * cursor);

// CLASS: wxSettableHeaderColumn
void wxSettableHeaderColumn_delete(wxSettableHeaderColumn *self);
void wxSettableHeaderColumn_SetTitle(wxSettableHeaderColumn * self, const wxString * title);
void wxSettableHeaderColumn_SetBitmap(wxSettableHeaderColumn * self, const wxBitmapBundle * bitmap);
void wxSettableHeaderColumn_SetWidth(wxSettableHeaderColumn * self, int width);
void wxSettableHeaderColumn_SetMinWidth(wxSettableHeaderColumn * self, int min_width);
void wxSettableHeaderColumn_SetAlignment(wxSettableHeaderColumn * self, wxAlignment align);
void wxSettableHeaderColumn_SetFlags(wxSettableHeaderColumn * self, int flags);
void wxSettableHeaderColumn_ChangeFlag(wxSettableHeaderColumn * self, int flag, bool set);
void wxSettableHeaderColumn_SetFlag(wxSettableHeaderColumn * self, int flag);
void wxSettableHeaderColumn_ClearFlag(wxSettableHeaderColumn * self, int flag);
void wxSettableHeaderColumn_ToggleFlag(wxSettableHeaderColumn * self, int flag);
void wxSettableHeaderColumn_SetResizeable(wxSettableHeaderColumn * self, bool resizable);
void wxSettableHeaderColumn_SetSortable(wxSettableHeaderColumn * self, bool sortable);
void wxSettableHeaderColumn_SetReorderable(wxSettableHeaderColumn * self, bool reorderable);
void wxSettableHeaderColumn_SetHidden(wxSettableHeaderColumn * self, bool hidden);
void wxSettableHeaderColumn_UnsetAsSortKey(wxSettableHeaderColumn * self);
void wxSettableHeaderColumn_SetSortOrder(wxSettableHeaderColumn * self, bool ascending);
void wxSettableHeaderColumn_ToggleSortOrder(wxSettableHeaderColumn * self);

// CLASS: wxShowEvent
wxClassInfo *wxShowEvent_CLASSINFO();
wxShowEvent *wxShowEvent_new(int winid, bool show);
void wxShowEvent_SetShow(wxShowEvent * self, bool show);
bool wxShowEvent_IsShown(const wxShowEvent * self);
bool wxShowEvent_GetShow(const wxShowEvent * self);

// CLASS: wxSimpleHelpProvider
void wxSimpleHelpProvider_delete(wxSimpleHelpProvider *self);

// CLASS: wxSimplebook
wxClassInfo *wxSimplebook_CLASSINFO();
wxSimplebook *wxSimplebook_new();
wxSimplebook *wxSimplebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSimplebook_Create(wxSimplebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSimplebook_ShowNewPage(wxSimplebook * self, wxWindow * page);
// Mix-in(s) to wxSimplebook
wxWithImages *wxSimplebook_AsWithImages(wxSimplebook* obj);
wxTrackable *wxSimplebook_AsTrackable(wxSimplebook* obj);

// CLASS: wxSize
void wxSize_delete(wxSize *self);
wxSize *wxSize_new();
wxSize *wxSize_new1(int width, int height);
void wxSize_DecBy(wxSize * self, const wxPoint * pt);
void wxSize_DecBy1(wxSize * self, const wxSize * size);
void wxSize_DecBy2(wxSize * self, int dx, int dy);
void wxSize_DecBy3(wxSize * self, int d);
void wxSize_DecTo(wxSize * self, const wxSize * size);
void wxSize_DecToIfSpecified(wxSize * self, const wxSize * size);
int wxSize_GetHeight(const wxSize * self);
int wxSize_GetWidth(const wxSize * self);
void wxSize_IncBy(wxSize * self, const wxPoint * pt);
void wxSize_IncBy1(wxSize * self, const wxSize * size);
void wxSize_IncBy2(wxSize * self, int dx, int dy);
void wxSize_IncBy3(wxSize * self, int d);
void wxSize_IncTo(wxSize * self, const wxSize * size);
bool wxSize_IsFullySpecified(const wxSize * self);
void wxSize_Set(wxSize * self, int width, int height);
void wxSize_SetDefaults(wxSize * self, const wxSize * size_default);
void wxSize_SetHeight(wxSize * self, int height);
void wxSize_SetWidth(wxSize * self, int width);

// CLASS: wxSizeEvent
wxClassInfo *wxSizeEvent_CLASSINFO();
wxSizeEvent *wxSizeEvent_new(const wxSize * sz, int id);
wxSize *wxSizeEvent_GetSize(const wxSizeEvent * self);
void wxSizeEvent_SetSize(wxSizeEvent * self, wxSize size);
wxRect *wxSizeEvent_GetRect(const wxSizeEvent * self);
void wxSizeEvent_SetRect(wxSizeEvent * self, wxRect rect);

// CLASS: wxSizer
wxClassInfo *wxSizer_CLASSINFO();
wxSizerItem * wxSizer_Add(wxSizer * self, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add5(wxSizer * self, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add6(wxSizer * self, wxSizerItem * item);
wxSizerItem * wxSizer_AddSpacer(wxSizer * self, int size);
wxSizerItem * wxSizer_AddStretchSpacer(wxSizer * self, int prop);
wxSize *wxSizer_CalcMin(wxSizer * self);
void wxSizer_Clear(wxSizer * self, bool delete_windows);
wxSize *wxSizer_ComputeFittingClientSize(wxSizer * self, wxWindow * window);
wxSize *wxSizer_ComputeFittingWindowSize(wxSizer * self, wxWindow * window);
bool wxSizer_Detach(wxSizer * self, wxWindow * window);
bool wxSizer_Detach1(wxSizer * self, wxSizer * sizer);
bool wxSizer_Detach2(wxSizer * self, int index);
wxSize *wxSizer_Fit(wxSizer * self, wxWindow * window);
void wxSizer_FitInside(wxSizer * self, wxWindow * window);
bool wxSizer_InformFirstDirection(wxSizer * self, int direction, int size, int available_other_dir);
wxSizerItemList * wxSizer_GetChildren(wxSizer * self);
wxWindow * wxSizer_GetContainingWindow(const wxSizer * self);
void wxSizer_SetContainingWindow(wxSizer * self, wxWindow * window);
size_t wxSizer_GetItemCount(const wxSizer * self);
wxSizerItem * wxSizer_GetItem(wxSizer * self, wxWindow * window, bool recursive);
wxSizerItem * wxSizer_GetItem1(wxSizer * self, wxSizer * sizer, bool recursive);
wxSizerItem * wxSizer_GetItem2(wxSizer * self, size_t index);
wxSizerItem * wxSizer_GetItemById(wxSizer * self, int id, bool recursive);
wxSize *wxSizer_GetMinSize(wxSizer * self);
wxPoint *wxSizer_GetPosition(const wxSizer * self);
wxSize *wxSizer_GetSize(const wxSizer * self);
bool wxSizer_Hide(wxSizer * self, wxWindow * window, bool recursive);
bool wxSizer_Hide1(wxSizer * self, wxSizer * sizer, bool recursive);
bool wxSizer_Hide2(wxSizer * self, size_t index);
wxSizerItem * wxSizer_Insert(wxSizer * self, size_t index, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert1(wxSizer * self, size_t index, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert2(wxSizer * self, size_t index, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert3(wxSizer * self, size_t index, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert4(wxSizer * self, size_t index, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert5(wxSizer * self, size_t index, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert6(wxSizer * self, size_t index, wxSizerItem * item);
wxSizerItem * wxSizer_InsertSpacer(wxSizer * self, size_t index, int size);
wxSizerItem * wxSizer_InsertStretchSpacer(wxSizer * self, size_t index, int prop);
bool wxSizer_IsEmpty(const wxSizer * self);
bool wxSizer_IsShown(const wxSizer * self, wxWindow * window);
bool wxSizer_IsShown1(const wxSizer * self, wxSizer * sizer);
bool wxSizer_IsShown2(const wxSizer * self, size_t index);
void wxSizer_Layout(wxSizer * self);
wxSizerItem * wxSizer_Prepend(wxSizer * self, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend5(wxSizer * self, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend6(wxSizer * self, wxSizerItem * item);
wxSizerItem * wxSizer_PrependSpacer(wxSizer * self, int size);
wxSizerItem * wxSizer_PrependStretchSpacer(wxSizer * self, int prop);
#if wxCHECK_VERSION(3, 1, 0)
void wxSizer_RepositionChildren(wxSizer * self, const wxSize * min_size);
#endif
bool wxSizer_Remove1(wxSizer * self, wxSizer * sizer);
bool wxSizer_Remove2(wxSizer * self, int index);
bool wxSizer_Replace(wxSizer * self, wxWindow * oldwin, wxWindow * newwin, bool recursive);
bool wxSizer_Replace1(wxSizer * self, wxSizer * oldsz, wxSizer * newsz, bool recursive);
bool wxSizer_Replace2(wxSizer * self, size_t index, wxSizerItem * newitem);
void wxSizer_SetDimension(wxSizer * self, int x, int y, int width, int height);
void wxSizer_SetDimension1(wxSizer * self, const wxPoint * pos, const wxSize * size);
bool wxSizer_SetItemMinSize(wxSizer * self, wxWindow * window, int width, int height);
bool wxSizer_SetItemMinSize1(wxSizer * self, wxWindow * window, const wxSize * size);
bool wxSizer_SetItemMinSize2(wxSizer * self, wxSizer * sizer, int width, int height);
bool wxSizer_SetItemMinSize3(wxSizer * self, wxSizer * sizer, const wxSize * size);
bool wxSizer_SetItemMinSize4(wxSizer * self, size_t index, int width, int height);
bool wxSizer_SetItemMinSize5(wxSizer * self, size_t index, const wxSize * size);
void wxSizer_SetMinSize(wxSizer * self, const wxSize * size);
void wxSizer_SetMinSize1(wxSizer * self, int width, int height);
void wxSizer_SetSizeHints(wxSizer * self, wxWindow * window);
bool wxSizer_Show(wxSizer * self, wxWindow * window, bool show, bool recursive);
bool wxSizer_Show1(wxSizer * self, wxSizer * sizer, bool show, bool recursive);
bool wxSizer_Show2(wxSizer * self, size_t index, bool show);
void wxSizer_ShowItems(wxSizer * self, bool show);

// CLASS: wxSizerFlags
void wxSizerFlags_delete(wxSizerFlags *self);
wxSizerFlags *wxSizerFlags_new(int proportion);
wxSizerFlags * wxSizerFlags_Align(wxSizerFlags * self, int alignment);
wxSizerFlags * wxSizerFlags_Border(wxSizerFlags * self, int direction, int borderinpixels);
wxSizerFlags * wxSizerFlags_Border1(wxSizerFlags * self, int direction);
wxSizerFlags * wxSizerFlags_Bottom(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Center(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Centre(wxSizerFlags * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSizerFlags * wxSizerFlags_CenterHorizontal(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CenterVertical(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CentreHorizontal(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CentreVertical(wxSizerFlags * self);
#endif
wxSizerFlags * wxSizerFlags_DoubleBorder(wxSizerFlags * self, int direction);
wxSizerFlags * wxSizerFlags_DoubleHorzBorder(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Expand(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_FixedMinSize(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_ReserveSpaceEvenIfHidden(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Left(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Proportion(wxSizerFlags * self, int proportion);
wxSizerFlags * wxSizerFlags_Right(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Shaped(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Top(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_TripleBorder(wxSizerFlags * self, int direction);
#if wxCHECK_VERSION(3, 1, 0)
void wxSizerFlags_DisableConsistencyChecks();
#endif
int wxSizerFlags_GetDefaultBorder();

// CLASS: wxSizerItem
wxClassInfo *wxSizerItem_CLASSINFO();
wxSizerItem *wxSizerItem_new(int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem *wxSizerItem_new1(wxWindow * window, const wxSizerFlags * flags);
wxSizerItem *wxSizerItem_new2(wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem *wxSizerItem_new3(wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem *wxSizerItem_new4(wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
void wxSizerItem_AssignWindow(wxSizerItem * self, wxWindow * window);
void wxSizerItem_AssignSizer(wxSizerItem * self, wxSizer * sizer);
void wxSizerItem_AssignSpacer(wxSizerItem * self, const wxSize * size);
void wxSizerItem_AssignSpacer1(wxSizerItem * self, int w, int h);
wxSize *wxSizerItem_CalcMin(wxSizerItem * self);
void wxSizerItem_DeleteWindows(wxSizerItem * self);
void wxSizerItem_DetachSizer(wxSizerItem * self);
int wxSizerItem_GetBorder(const wxSizerItem * self);
int wxSizerItem_GetFlag(const wxSizerItem * self);
int wxSizerItem_GetId(const wxSizerItem * self);
wxSize *wxSizerItem_GetMinSize(const wxSizerItem * self);
void wxSizerItem_SetMinSize(wxSizerItem * self, const wxSize * size);
void wxSizerItem_SetMinSize1(wxSizerItem * self, int x, int y);
wxPoint *wxSizerItem_GetPosition(const wxSizerItem * self);
int wxSizerItem_GetProportion(const wxSizerItem * self);
wxRect *wxSizerItem_GetRect(wxSizerItem * self);
wxSize *wxSizerItem_GetSize(const wxSizerItem * self);
wxSizer * wxSizerItem_GetSizer(const wxSizerItem * self);
wxSize *wxSizerItem_GetSpacer(const wxSizerItem * self);
wxObject * wxSizerItem_GetUserData(const wxSizerItem * self);
wxWindow * wxSizerItem_GetWindow(const wxSizerItem * self);
bool wxSizerItem_IsShown(const wxSizerItem * self);
bool wxSizerItem_IsSizer(const wxSizerItem * self);
bool wxSizerItem_IsSpacer(const wxSizerItem * self);
bool wxSizerItem_IsWindow(const wxSizerItem * self);
void wxSizerItem_SetBorder(wxSizerItem * self, int border);
void wxSizerItem_SetDimension(wxSizerItem * self, const wxPoint * pos, const wxSize * size);
void wxSizerItem_SetFlag(wxSizerItem * self, int flag);
void wxSizerItem_SetId(wxSizerItem * self, int id);
void wxSizerItem_SetInitSize(wxSizerItem * self, int x, int y);
void wxSizerItem_SetProportion(wxSizerItem * self, int proportion);
void wxSizerItem_SetRatio(wxSizerItem * self, int width, int height);
void wxSizerItem_SetRatio1(wxSizerItem * self, wxSize size);
void wxSizerItem_SetSizer(wxSizerItem * self, wxSizer * sizer);
void wxSizerItem_SetSpacer(wxSizerItem * self, const wxSize * size);
void wxSizerItem_SetUserData(wxSizerItem * self, wxObject * user_data);
void wxSizerItem_SetWindow(wxSizerItem * self, wxWindow * window);
void wxSizerItem_Show(wxSizerItem * self, bool show);

// CLASS: wxSlider
wxClassInfo *wxSlider_CLASSINFO();
wxSlider *wxSlider_new();
wxSlider *wxSlider_new1(wxWindow * parent, wxWindowID id, int value, int min_value, int max_value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxSlider_ClearSel(wxSlider * self);
void wxSlider_ClearTicks(wxSlider * self);
bool wxSlider_Create(wxSlider * self, wxWindow * parent, wxWindowID id, int value, int min_value, int max_value, const wxPoint * point, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxSlider_GetLineSize(const wxSlider * self);
int wxSlider_GetMax(const wxSlider * self);
int wxSlider_GetMin(const wxSlider * self);
int wxSlider_GetPageSize(const wxSlider * self);
int wxSlider_GetSelEnd(const wxSlider * self);
int wxSlider_GetSelStart(const wxSlider * self);
int wxSlider_GetThumbLength(const wxSlider * self);
int wxSlider_GetTickFreq(const wxSlider * self);
int wxSlider_GetValue(const wxSlider * self);
void wxSlider_SetLineSize(wxSlider * self, int line_size);
void wxSlider_SetMin(wxSlider * self, int min_value);
void wxSlider_SetMax(wxSlider * self, int max_value);
void wxSlider_SetPageSize(wxSlider * self, int page_size);
void wxSlider_SetRange(wxSlider * self, int min_value, int max_value);
void wxSlider_SetSelection(wxSlider * self, int start_pos, int end_pos);
void wxSlider_SetThumbLength(wxSlider * self, int len);
void wxSlider_SetTick(wxSlider * self, int tick_pos);
void wxSlider_SetTickFreq(wxSlider * self, int freq);
void wxSlider_SetValue(wxSlider * self, int value);
// Mix-in(s) to wxSlider
wxTrackable *wxSlider_AsTrackable(wxSlider* obj);

// CLASS: wxSound
wxClassInfo *wxSound_CLASSINFO();
wxSound *wxSound_new();
wxSound *wxSound_new1(const wxString * file_name, bool is_resource);
wxSound *wxSound_new2(size_t size, const void * data);
bool wxSound_Create(wxSound * self, const wxString * file_name, bool is_resource);
bool wxSound_Create1(wxSound * self, size_t size, const void * data);
bool wxSound_IsOk(const wxSound * self);
bool wxSound_IsPlaying();
void wxSound_Stop();

// CLASS: wxSpinButton
wxClassInfo *wxSpinButton_CLASSINFO();
wxSpinButton *wxSpinButton_new();
wxSpinButton *wxSpinButton_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSpinButton_Create(wxSpinButton * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
#if wxCHECK_VERSION(3, 1, 0)
int wxSpinButton_GetIncrement(const wxSpinButton * self);
#endif
int wxSpinButton_GetMax(const wxSpinButton * self);
int wxSpinButton_GetMin(const wxSpinButton * self);
int wxSpinButton_GetValue(const wxSpinButton * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxSpinButton_SetIncrement(wxSpinButton * self, int value);
#endif
void wxSpinButton_SetRange(wxSpinButton * self, int min, int max);
void wxSpinButton_SetValue(wxSpinButton * self, int value);
// Mix-in(s) to wxSpinButton
wxTrackable *wxSpinButton_AsTrackable(wxSpinButton* obj);

// CLASS: wxSpinCtrl
wxClassInfo *wxSpinCtrl_CLASSINFO();
wxSpinCtrl *wxSpinCtrl_new();
wxSpinCtrl *wxSpinCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, int min, int max, int initial, const wxString * name);
bool wxSpinCtrl_Create(wxSpinCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, int min, int max, int initial, const wxString * name);
int wxSpinCtrl_GetBase(const wxSpinCtrl * self);
int wxSpinCtrl_GetMax(const wxSpinCtrl * self);
int wxSpinCtrl_GetMin(const wxSpinCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxSpinCtrl_GetTextValue(const wxSpinCtrl * self);
#endif
int wxSpinCtrl_GetValue(const wxSpinCtrl * self);
int wxSpinCtrl_GetIncrement(const wxSpinCtrl * self);
bool wxSpinCtrl_SetBase(wxSpinCtrl * self, int base);
void wxSpinCtrl_SetRange(wxSpinCtrl * self, int min_val, int max_val);
void wxSpinCtrl_SetSelection(wxSpinCtrl * self, long from, long to);
void wxSpinCtrl_SetValue(wxSpinCtrl * self, const wxString * text);
void wxSpinCtrl_SetValue1(wxSpinCtrl * self, int value);
void wxSpinCtrl_SetIncrement(wxSpinCtrl * self, int value);
// Mix-in(s) to wxSpinCtrl
wxTrackable *wxSpinCtrl_AsTrackable(wxSpinCtrl* obj);

// CLASS: wxSpinCtrlDouble
wxClassInfo *wxSpinCtrlDouble_CLASSINFO();
wxSpinCtrlDouble *wxSpinCtrlDouble_new();
wxSpinCtrlDouble *wxSpinCtrlDouble_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, double min, double max, double initial, double inc, const wxString * name);
bool wxSpinCtrlDouble_Create(wxSpinCtrlDouble * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, double min, double max, double initial, double inc, const wxString * name);
unsigned int wxSpinCtrlDouble_GetDigits(const wxSpinCtrlDouble * self);
double wxSpinCtrlDouble_GetIncrement(const wxSpinCtrlDouble * self);
double wxSpinCtrlDouble_GetMax(const wxSpinCtrlDouble * self);
double wxSpinCtrlDouble_GetMin(const wxSpinCtrlDouble * self);
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxSpinCtrlDouble_GetTextValue(const wxSpinCtrlDouble * self);
#endif
double wxSpinCtrlDouble_GetValue(const wxSpinCtrlDouble * self);
void wxSpinCtrlDouble_SetDigits(wxSpinCtrlDouble * self, unsigned int digits);
void wxSpinCtrlDouble_SetIncrement(wxSpinCtrlDouble * self, double inc);
void wxSpinCtrlDouble_SetRange(wxSpinCtrlDouble * self, double min_val, double max_val);
void wxSpinCtrlDouble_SetValue(wxSpinCtrlDouble * self, const wxString * text);
void wxSpinCtrlDouble_SetValue1(wxSpinCtrlDouble * self, double value);
// Mix-in(s) to wxSpinCtrlDouble
wxTrackable *wxSpinCtrlDouble_AsTrackable(wxSpinCtrlDouble* obj);

// CLASS: wxSpinDoubleEvent
wxClassInfo *wxSpinDoubleEvent_CLASSINFO();
wxSpinDoubleEvent *wxSpinDoubleEvent_new1(const wxSpinDoubleEvent * event);
double wxSpinDoubleEvent_GetValue(const wxSpinDoubleEvent * self);
void wxSpinDoubleEvent_SetValue(wxSpinDoubleEvent * self, double value);

// CLASS: wxSpinEvent
wxClassInfo *wxSpinEvent_CLASSINFO();
int wxSpinEvent_GetPosition(const wxSpinEvent * self);
void wxSpinEvent_SetPosition(wxSpinEvent * self, int pos);

// CLASS: wxSplashScreen
wxClassInfo *wxSplashScreen_CLASSINFO();
wxSplashScreen *wxSplashScreen_new(const wxBitmap * bitmap, long splash_style, int milliseconds, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style);
long wxSplashScreen_GetSplashStyle(const wxSplashScreen * self);
wxSplashScreenWindow * wxSplashScreen_GetSplashWindow(const wxSplashScreen * self);
int wxSplashScreen_GetTimeout(const wxSplashScreen * self);
void wxSplashScreen_OnCloseWindow(wxSplashScreen * self, wxCloseEvent * event);
// Mix-in(s) to wxSplashScreen
wxTrackable *wxSplashScreen_AsTrackable(wxSplashScreen* obj);

// CLASS: wxSplitterEvent
wxClassInfo *wxSplitterEvent_CLASSINFO();
int wxSplitterEvent_GetSashPosition(const wxSplitterEvent * self);
wxWindow * wxSplitterEvent_GetWindowBeingRemoved(const wxSplitterEvent * self);
int wxSplitterEvent_GetX(const wxSplitterEvent * self);
int wxSplitterEvent_GetY(const wxSplitterEvent * self);
void wxSplitterEvent_SetSashPosition(wxSplitterEvent * self, int pos);
void wxSplitterEvent_SetSize(wxSplitterEvent * self, int old_size, int new_size);
int wxSplitterEvent_GetOldSize(const wxSplitterEvent * self);

// CLASS: wxSplitterWindow
wxClassInfo *wxSplitterWindow_CLASSINFO();
wxSplitterWindow *wxSplitterWindow_new();
wxSplitterWindow *wxSplitterWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSplitterWindow_Create(wxSplitterWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * point, const wxSize * size, long style, const wxString * name);
int wxSplitterWindow_GetMinimumPaneSize(const wxSplitterWindow * self);
double wxSplitterWindow_GetSashGravity(const wxSplitterWindow * self);
int wxSplitterWindow_GetSashPosition(const wxSplitterWindow * self);
int wxSplitterWindow_GetSashSize(const wxSplitterWindow * self);
int wxSplitterWindow_GetDefaultSashSize(const wxSplitterWindow * self);
wxWindow * wxSplitterWindow_GetWindow1(const wxSplitterWindow * self);
wxWindow * wxSplitterWindow_GetWindow2(const wxSplitterWindow * self);
void wxSplitterWindow_Initialize(wxSplitterWindow * self, wxWindow * window);
bool wxSplitterWindow_IsSashInvisible(const wxSplitterWindow * self);
bool wxSplitterWindow_IsSplit(const wxSplitterWindow * self);
void wxSplitterWindow_OnDoubleClickSash(wxSplitterWindow * self, int x, int y);
bool wxSplitterWindow_OnSashPositionChange(wxSplitterWindow * self, int new_sash_position);
void wxSplitterWindow_OnUnsplit(wxSplitterWindow * self, wxWindow * removed);
bool wxSplitterWindow_ReplaceWindow(wxSplitterWindow * self, wxWindow * win_old, wxWindow * win_new);
void wxSplitterWindow_SetMinimumPaneSize(wxSplitterWindow * self, int pane_size);
void wxSplitterWindow_SetSashGravity(wxSplitterWindow * self, double gravity);
void wxSplitterWindow_SetSashPosition(wxSplitterWindow * self, int position, bool redraw);
void wxSplitterWindow_SetSplitMode(wxSplitterWindow * self, int mode);
void wxSplitterWindow_SetSashInvisible(wxSplitterWindow * self, bool invisible);
bool wxSplitterWindow_SplitHorizontally(wxSplitterWindow * self, wxWindow * window1, wxWindow * window2, int sash_position);
bool wxSplitterWindow_SplitVertically(wxSplitterWindow * self, wxWindow * window1, wxWindow * window2, int sash_position);
bool wxSplitterWindow_Unsplit(wxSplitterWindow * self, wxWindow * to_remove);
void wxSplitterWindow_UpdateSize(wxSplitterWindow * self);
// Mix-in(s) to wxSplitterWindow
wxTrackable *wxSplitterWindow_AsTrackable(wxSplitterWindow* obj);

// CLASS: wxStaticBitmap
wxClassInfo *wxStaticBitmap_CLASSINFO();
wxStaticBitmap *wxStaticBitmap_new();
wxStaticBitmap *wxStaticBitmap_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticBitmap_Create(wxStaticBitmap * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxBitmap *wxStaticBitmap_GetBitmap(const wxStaticBitmap * self);
wxIcon *wxStaticBitmap_GetIcon(const wxStaticBitmap * self);
void wxStaticBitmap_SetBitmap(wxStaticBitmap * self, const wxBitmapBundle * label);
void wxStaticBitmap_SetIcon(wxStaticBitmap * self, const wxIcon * label);
// Mix-in(s) to wxStaticBitmap
wxTrackable *wxStaticBitmap_AsTrackable(wxStaticBitmap* obj);

// CLASS: wxStaticBox
wxClassInfo *wxStaticBox_CLASSINFO();
wxStaticBox *wxStaticBox_new();
wxStaticBox *wxStaticBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticBox_Create(wxStaticBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxStaticBox
wxTrackable *wxStaticBox_AsTrackable(wxStaticBox* obj);

// CLASS: wxStaticBoxSizer
wxClassInfo *wxStaticBoxSizer_CLASSINFO();
wxStaticBoxSizer *wxStaticBoxSizer_new(wxStaticBox * box_, int orient);
wxStaticBoxSizer *wxStaticBoxSizer_new1(int orient, wxWindow * parent, const wxString * label);
wxStaticBox * wxStaticBoxSizer_GetStaticBox(const wxStaticBoxSizer * self);

// CLASS: wxStaticLine
wxClassInfo *wxStaticLine_CLASSINFO();
wxStaticLine *wxStaticLine_new();
wxStaticLine *wxStaticLine_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticLine_Create(wxStaticLine * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticLine_IsVertical(const wxStaticLine * self);
int wxStaticLine_GetDefaultSize();
// Mix-in(s) to wxStaticLine
wxTrackable *wxStaticLine_AsTrackable(wxStaticLine* obj);

// CLASS: wxStaticText
wxClassInfo *wxStaticText_CLASSINFO();
wxStaticText *wxStaticText_new();
wxStaticText *wxStaticText_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticText_Create(wxStaticText * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticText_IsEllipsized(const wxStaticText * self);
void wxStaticText_Wrap(wxStaticText * self, int width);
// Mix-in(s) to wxStaticText
wxTrackable *wxStaticText_AsTrackable(wxStaticText* obj);

// CLASS: wxStatusBar
wxClassInfo *wxStatusBar_CLASSINFO();
wxStatusBar *wxStatusBar_new();
wxStatusBar *wxStatusBar_new1(wxWindow * parent, wxWindowID id, long style, const wxString * name);
bool wxStatusBar_Create(wxStatusBar * self, wxWindow * parent, wxWindowID id, long style, const wxString * name);
bool wxStatusBar_GetFieldRect(const wxStatusBar * self, int i, wxRect * rect);
int wxStatusBar_GetFieldsCount(const wxStatusBar * self);
wxStatusBarPane *wxStatusBar_GetField(const wxStatusBar * self, int n);
wxSize *wxStatusBar_GetBorders(const wxStatusBar * self);
wxString *wxStatusBar_GetStatusText(const wxStatusBar * self, int i);
int wxStatusBar_GetStatusWidth(const wxStatusBar * self, int n);
int wxStatusBar_GetStatusStyle(const wxStatusBar * self, int n);
void wxStatusBar_PopStatusText(wxStatusBar * self, int field);
void wxStatusBar_PushStatusText(wxStatusBar * self, const wxString * string, int field);
void wxStatusBar_SetFieldsCount(wxStatusBar * self, int number, const int * widths);
void wxStatusBar_SetMinHeight(wxStatusBar * self, int height);
void wxStatusBar_SetStatusStyles(wxStatusBar * self, int n, const int * styles);
void wxStatusBar_SetStatusText(wxStatusBar * self, const wxString * text, int i);
void wxStatusBar_SetStatusWidths(wxStatusBar * self, int n, const int * widths_field);
// Mix-in(s) to wxStatusBar
wxTrackable *wxStatusBar_AsTrackable(wxStatusBar* obj);

// CLASS: wxStatusBarPane
void wxStatusBarPane_delete(wxStatusBarPane *self);
wxStatusBarPane *wxStatusBarPane_new(int style, int width);
int wxStatusBarPane_GetWidth(const wxStatusBarPane * self);
int wxStatusBarPane_GetStyle(const wxStatusBarPane * self);
wxString *wxStatusBarPane_GetText(const wxStatusBarPane * self);

// CLASS: wxStdDialogButtonSizer
wxClassInfo *wxStdDialogButtonSizer_CLASSINFO();
wxStdDialogButtonSizer *wxStdDialogButtonSizer_new();
void wxStdDialogButtonSizer_AddButton(wxStdDialogButtonSizer * self, wxButton * button);
void wxStdDialogButtonSizer_Realize(wxStdDialogButtonSizer * self);
void wxStdDialogButtonSizer_SetAffirmativeButton(wxStdDialogButtonSizer * self, wxButton * button);
void wxStdDialogButtonSizer_SetCancelButton(wxStdDialogButtonSizer * self, wxButton * button);
void wxStdDialogButtonSizer_SetNegativeButton(wxStdDialogButtonSizer * self, wxButton * button);

// CLASS: wxStockPreferencesPage
void wxStockPreferencesPage_delete(wxStockPreferencesPage *self);

// CLASS: wxStreamToTextRedirector
void wxStreamToTextRedirector_delete(wxStreamToTextRedirector *self);
wxStreamToTextRedirector *wxStreamToTextRedirector_new(wxTextCtrl * text, ostream * ostr);

// CLASS: wxSysColourChangedEvent
wxClassInfo *wxSysColourChangedEvent_CLASSINFO();
wxSysColourChangedEvent *wxSysColourChangedEvent_new();

// CLASS: wxSystemSettings
void wxSystemSettings_delete(wxSystemSettings *self);
wxSystemSettings *wxSystemSettings_new();
wxSystemAppearance *wxSystemSettings_GetAppearance();

// CLASS: wxTGAHandler
wxClassInfo *wxTGAHandler_CLASSINFO();
wxTGAHandler *wxTGAHandler_new();

// CLASS: wxTIFFHandler
wxClassInfo *wxTIFFHandler_CLASSINFO();
wxTIFFHandler *wxTIFFHandler_new();
wxVersionInfo *wxTIFFHandler_GetLibraryVersionInfo();

// CLASS: wxTaskBarButton
void wxTaskBarButton_delete(wxTaskBarButton *self);
void wxTaskBarButton_SetProgressRange(wxTaskBarButton * self, int range);
void wxTaskBarButton_SetProgressValue(wxTaskBarButton * self, int value);
void wxTaskBarButton_PulseProgress(wxTaskBarButton * self);
void wxTaskBarButton_Show(wxTaskBarButton * self, bool show);
void wxTaskBarButton_Hide(wxTaskBarButton * self);
void wxTaskBarButton_SetThumbnailTooltip(wxTaskBarButton * self, const wxString * tooltip);
void wxTaskBarButton_SetOverlayIcon(wxTaskBarButton * self, const wxIcon * icon, const wxString * description);
void wxTaskBarButton_SetThumbnailClip(wxTaskBarButton * self, const wxRect * rect);
void wxTaskBarButton_SetThumbnailContents(wxTaskBarButton * self, const wxWindow * child);
bool wxTaskBarButton_InsertThumbBarButton(wxTaskBarButton * self, size_t pos, wxThumbBarButton * button);
bool wxTaskBarButton_AppendThumbBarButton(wxTaskBarButton * self, wxThumbBarButton * button);
bool wxTaskBarButton_AppendSeparatorInThumbBar(wxTaskBarButton * self);
wxThumbBarButton * wxTaskBarButton_RemoveThumbBarButton(wxTaskBarButton * self, wxThumbBarButton * button);
wxThumbBarButton * wxTaskBarButton_RemoveThumbBarButton1(wxTaskBarButton * self, int id);

// CLASS: wxTaskBarIcon
wxClassInfo *wxTaskBarIcon_CLASSINFO();
void wxTaskBarIcon_Destroy(wxTaskBarIcon * self);
bool wxTaskBarIcon_IsIconInstalled(const wxTaskBarIcon * self);
bool wxTaskBarIcon_IsOk(const wxTaskBarIcon * self);
bool wxTaskBarIcon_PopupMenu(wxTaskBarIcon * self, wxMenu * menu);
bool wxTaskBarIcon_RemoveIcon(wxTaskBarIcon * self);
bool wxTaskBarIcon_SetIcon(wxTaskBarIcon * self, const wxBitmapBundle * icon, const wxString * tooltip);
bool wxTaskBarIcon_IsAvailable();
// Mix-in(s) to wxTaskBarIcon
wxTrackable *wxTaskBarIcon_AsTrackable(wxTaskBarIcon* obj);

// CLASS: wxTaskBarIconEvent
wxClassInfo *wxTaskBarIconEvent_CLASSINFO();

// CLASS: wxTaskBarJumpList
void wxTaskBarJumpList_delete(wxTaskBarJumpList *self);
wxTaskBarJumpList *wxTaskBarJumpList_new(const wxString * app_id);
void wxTaskBarJumpList_ShowRecentCategory(wxTaskBarJumpList * self, bool shown);
void wxTaskBarJumpList_HideRecentCategory(wxTaskBarJumpList * self);
void wxTaskBarJumpList_ShowFrequentCategory(wxTaskBarJumpList * self, bool shown);
void wxTaskBarJumpList_HideFrequentCategory(wxTaskBarJumpList * self);
wxTaskBarJumpListCategory * wxTaskBarJumpList_GetTasks(const wxTaskBarJumpList * self);
wxTaskBarJumpListCategory *wxTaskBarJumpList_GetFrequentCategory(const wxTaskBarJumpList * self);
wxTaskBarJumpListCategory *wxTaskBarJumpList_GetRecentCategory(const wxTaskBarJumpList * self);
const wxTaskBarJumpListCategories * wxTaskBarJumpList_GetCustomCategories(const wxTaskBarJumpList * self);
void wxTaskBarJumpList_AddCustomCategory(wxTaskBarJumpList * self, wxTaskBarJumpListCategory * category);
wxTaskBarJumpListCategory * wxTaskBarJumpList_RemoveCustomCategory(wxTaskBarJumpList * self, const wxString * title);
void wxTaskBarJumpList_DeleteCustomCategory(wxTaskBarJumpList * self, const wxString * title);

// CLASS: wxTaskBarJumpListCategory
void wxTaskBarJumpListCategory_delete(wxTaskBarJumpListCategory *self);
wxTaskBarJumpListCategory *wxTaskBarJumpListCategory_new(wxTaskBarJumpList * parent, const wxString * title);
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_Append(wxTaskBarJumpListCategory * self, wxTaskBarJumpListItem * item);
void wxTaskBarJumpListCategory_Delete(wxTaskBarJumpListCategory * self, wxTaskBarJumpListItem * item);
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_Remove(wxTaskBarJumpListCategory * self, wxTaskBarJumpListItem * item);
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_FindItemByPosition(const wxTaskBarJumpListCategory * self, size_t pos);
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_Insert(wxTaskBarJumpListCategory * self, size_t pos, wxTaskBarJumpListItem * item);
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_Prepend(wxTaskBarJumpListCategory * self, wxTaskBarJumpListItem * item);
void wxTaskBarJumpListCategory_SetTitle(wxTaskBarJumpListCategory * self, const wxString * title);
wxString *wxTaskBarJumpListCategory_GetTitle(const wxTaskBarJumpListCategory * self);
const wxTaskBarJumpListItems * wxTaskBarJumpListCategory_GetItems(const wxTaskBarJumpListCategory * self);

// CLASS: wxTaskBarJumpListItem
void wxTaskBarJumpListItem_delete(wxTaskBarJumpListItem *self);
wxString *wxTaskBarJumpListItem_GetTitle(const wxTaskBarJumpListItem * self);
void wxTaskBarJumpListItem_SetTitle(wxTaskBarJumpListItem * self, const wxString * title);
wxString *wxTaskBarJumpListItem_GetFilePath(const wxTaskBarJumpListItem * self);
void wxTaskBarJumpListItem_SetFilePath(wxTaskBarJumpListItem * self, const wxString * file_path);
wxString *wxTaskBarJumpListItem_GetArguments(const wxTaskBarJumpListItem * self);
void wxTaskBarJumpListItem_SetArguments(wxTaskBarJumpListItem * self, const wxString * arguments);
wxString *wxTaskBarJumpListItem_GetTooltip(const wxTaskBarJumpListItem * self);
void wxTaskBarJumpListItem_SetTooltip(wxTaskBarJumpListItem * self, const wxString * tooltip);
wxString *wxTaskBarJumpListItem_GetIconPath(const wxTaskBarJumpListItem * self);
void wxTaskBarJumpListItem_SetIconPath(wxTaskBarJumpListItem * self, const wxString * icon_path);
int wxTaskBarJumpListItem_GetIconIndex(const wxTaskBarJumpListItem * self);
void wxTaskBarJumpListItem_SetIconIndex(wxTaskBarJumpListItem * self, int icon_index);
wxTaskBarJumpListCategory * wxTaskBarJumpListItem_GetCategory(const wxTaskBarJumpListItem * self);
void wxTaskBarJumpListItem_SetCategory(wxTaskBarJumpListItem * self, wxTaskBarJumpListCategory * category);

// CLASS: wxTextAttr
void wxTextAttr_delete(wxTextAttr *self);
wxColour *wxTextAttr_GetBackgroundColour(const wxTextAttr * self);
wxString *wxTextAttr_GetBulletFont(const wxTextAttr * self);
wxString *wxTextAttr_GetBulletName(const wxTextAttr * self);
int wxTextAttr_GetBulletNumber(const wxTextAttr * self);
int wxTextAttr_GetBulletStyle(const wxTextAttr * self);
wxString *wxTextAttr_GetBulletText(const wxTextAttr * self);
wxString *wxTextAttr_GetCharacterStyleName(const wxTextAttr * self);
long wxTextAttr_GetFlags(const wxTextAttr * self);
wxFont *wxTextAttr_GetFont(const wxTextAttr * self);
bool wxTextAttr_GetFontAttributes(wxTextAttr * self, const wxFont * font, int flags);
wxString *wxTextAttr_GetFontFaceName(const wxTextAttr * self);
int wxTextAttr_GetFontSize(const wxTextAttr * self);
bool wxTextAttr_GetFontUnderlined(const wxTextAttr * self);
#if wxCHECK_VERSION(3, 1, 0)
wxColour *wxTextAttr_GetUnderlineColour(const wxTextAttr * self);
#endif
long wxTextAttr_GetLeftIndent(const wxTextAttr * self);
long wxTextAttr_GetLeftSubIndent(const wxTextAttr * self);
int wxTextAttr_GetLineSpacing(const wxTextAttr * self);
wxString *wxTextAttr_GetListStyleName(const wxTextAttr * self);
int wxTextAttr_GetOutlineLevel(const wxTextAttr * self);
int wxTextAttr_GetParagraphSpacingAfter(const wxTextAttr * self);
int wxTextAttr_GetParagraphSpacingBefore(const wxTextAttr * self);
wxString *wxTextAttr_GetParagraphStyleName(const wxTextAttr * self);
long wxTextAttr_GetRightIndent(const wxTextAttr * self);
wxArrayInt *wxTextAttr_GetTabs(const wxTextAttr * self);
wxColour *wxTextAttr_GetTextColour(const wxTextAttr * self);
int wxTextAttr_GetTextEffectFlags(const wxTextAttr * self);
int wxTextAttr_GetTextEffects(const wxTextAttr * self);
wxString *wxTextAttr_GetURL(const wxTextAttr * self);
bool wxTextAttr_HasAlignment(const wxTextAttr * self);
bool wxTextAttr_HasBackgroundColour(const wxTextAttr * self);
bool wxTextAttr_HasBulletName(const wxTextAttr * self);
bool wxTextAttr_HasBulletNumber(const wxTextAttr * self);
bool wxTextAttr_HasBulletStyle(const wxTextAttr * self);
bool wxTextAttr_HasBulletText(const wxTextAttr * self);
bool wxTextAttr_HasCharacterStyleName(const wxTextAttr * self);
bool wxTextAttr_HasFlag(const wxTextAttr * self, long flag);
bool wxTextAttr_HasFont(const wxTextAttr * self);
bool wxTextAttr_HasFontEncoding(const wxTextAttr * self);
bool wxTextAttr_HasFontFaceName(const wxTextAttr * self);
bool wxTextAttr_HasFontFamily(const wxTextAttr * self);
bool wxTextAttr_HasFontItalic(const wxTextAttr * self);
bool wxTextAttr_HasFontSize(const wxTextAttr * self);
bool wxTextAttr_HasFontPointSize(const wxTextAttr * self);
bool wxTextAttr_HasFontPixelSize(const wxTextAttr * self);
bool wxTextAttr_HasFontUnderlined(const wxTextAttr * self);
bool wxTextAttr_HasFontWeight(const wxTextAttr * self);
bool wxTextAttr_HasLeftIndent(const wxTextAttr * self);
bool wxTextAttr_HasLineSpacing(const wxTextAttr * self);
bool wxTextAttr_HasListStyleName(const wxTextAttr * self);
bool wxTextAttr_HasOutlineLevel(const wxTextAttr * self);
bool wxTextAttr_HasPageBreak(const wxTextAttr * self);
bool wxTextAttr_HasParagraphSpacingAfter(const wxTextAttr * self);
bool wxTextAttr_HasParagraphSpacingBefore(const wxTextAttr * self);
bool wxTextAttr_HasParagraphStyleName(const wxTextAttr * self);
bool wxTextAttr_HasRightIndent(const wxTextAttr * self);
bool wxTextAttr_HasTabs(const wxTextAttr * self);
bool wxTextAttr_HasTextColour(const wxTextAttr * self);
bool wxTextAttr_HasTextEffects(const wxTextAttr * self);
bool wxTextAttr_HasURL(const wxTextAttr * self);
bool wxTextAttr_IsCharacterStyle(const wxTextAttr * self);
bool wxTextAttr_IsDefault(const wxTextAttr * self);
bool wxTextAttr_IsParagraphStyle(const wxTextAttr * self);
void wxTextAttr_SetBackgroundColour(wxTextAttr * self, const wxColour * col_back);
void wxTextAttr_SetBulletFont(wxTextAttr * self, const wxString * font);
void wxTextAttr_SetBulletName(wxTextAttr * self, const wxString * name);
void wxTextAttr_SetBulletNumber(wxTextAttr * self, int n);
void wxTextAttr_SetBulletStyle(wxTextAttr * self, int style);
void wxTextAttr_SetBulletText(wxTextAttr * self, const wxString * text);
void wxTextAttr_SetCharacterStyleName(wxTextAttr * self, const wxString * name);
void wxTextAttr_SetFlags(wxTextAttr * self, long flags);
void wxTextAttr_SetFont(wxTextAttr * self, const wxFont * font, int flags);
void wxTextAttr_SetFontFaceName(wxTextAttr * self, const wxString * face_name);
void wxTextAttr_SetFontSize(wxTextAttr * self, int point_size);
void wxTextAttr_SetFontPointSize(wxTextAttr * self, int point_size);
void wxTextAttr_SetFontPixelSize(wxTextAttr * self, int pixel_size);
void wxTextAttr_SetFontUnderlined(wxTextAttr * self, bool underlined);
void wxTextAttr_SetLeftIndent(wxTextAttr * self, int indent, int sub_indent);
void wxTextAttr_SetLineSpacing(wxTextAttr * self, int spacing);
void wxTextAttr_SetListStyleName(wxTextAttr * self, const wxString * name);
void wxTextAttr_SetOutlineLevel(wxTextAttr * self, int level);
void wxTextAttr_SetPageBreak(wxTextAttr * self, bool page_break);
void wxTextAttr_SetParagraphSpacingAfter(wxTextAttr * self, int spacing);
void wxTextAttr_SetParagraphSpacingBefore(wxTextAttr * self, int spacing);
void wxTextAttr_SetParagraphStyleName(wxTextAttr * self, const wxString * name);
void wxTextAttr_SetRightIndent(wxTextAttr * self, int indent);
void wxTextAttr_SetTabs(wxTextAttr * self, const wxArrayInt * tabs);
void wxTextAttr_SetTextColour(wxTextAttr * self, const wxColour * col_text);
void wxTextAttr_SetTextEffectFlags(wxTextAttr * self, int flags);
void wxTextAttr_SetTextEffects(wxTextAttr * self, int effects);
void wxTextAttr_SetURL(wxTextAttr * self, const wxString * url);
wxTextAttr *wxTextAttr_new();
wxTextAttr *wxTextAttr_new2(const wxTextAttr * attr);
bool wxTextAttr_Apply(wxTextAttr * self, const wxTextAttr * style, const wxTextAttr * compare_with);
void wxTextAttr_Merge(wxTextAttr * self, const wxTextAttr * overlay);
bool wxTextAttr_EqPartial(const wxTextAttr * self, const wxTextAttr * attr, bool weak_test);
wxTextAttr *wxTextAttr_Merge1(const wxTextAttr * base, const wxTextAttr * overlay);

// CLASS: wxTextCompleterSimple
void wxTextCompleterSimple_delete(wxTextCompleterSimple *self);
void wxTextCompleterSimple_GetCompletions(wxTextCompleterSimple * self, const wxString * prefix, wxArrayString * res);

// CLASS: wxTextCtrl
wxClassInfo *wxTextCtrl_CLASSINFO();
#ifdef __WXOSX__
void wxTextCtrl_OSXEnableNewLineReplacement(wxTextCtrl * self, bool enable);
#endif
wxTextCtrl *wxTextCtrl_new();
wxTextCtrl *wxTextCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxTextCtrl_Create(wxTextCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxTextCtrl_DiscardEdits(wxTextCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxTextCtrl_EmptyUndoBuffer(wxTextCtrl * self);
#endif
bool wxTextCtrl_EmulateKeyPress(wxTextCtrl * self, const wxKeyEvent * event);
#ifndef __WXGTK__
bool wxTextCtrl_EnableProofCheck(wxTextCtrl * self, const wxTextProofOptions * options);
#endif
wxTextAttr *wxTextCtrl_GetDefaultStyle(const wxTextCtrl * self);
int wxTextCtrl_GetLineLength(const wxTextCtrl * self, long line_no);
wxString *wxTextCtrl_GetLineText(const wxTextCtrl * self, long line_no);
int wxTextCtrl_GetNumberOfLines(const wxTextCtrl * self);
bool wxTextCtrl_GetStyle(wxTextCtrl * self, long position, wxTextAttr * style);
bool wxTextCtrl_IsModified(const wxTextCtrl * self);
bool wxTextCtrl_IsMultiLine(const wxTextCtrl * self);
bool wxTextCtrl_IsSingleLine(const wxTextCtrl * self);
wxTextProofOptions *wxTextCtrl_GetProofCheckOptions(wxTextCtrl * self);
bool wxTextCtrl_LoadFile(wxTextCtrl * self, const wxString * filename, int file_type);
void wxTextCtrl_MarkDirty(wxTextCtrl * self);
void wxTextCtrl_OnDropFiles(wxTextCtrl * self, wxDropFilesEvent * event);
bool wxTextCtrl_PositionToXY(const wxTextCtrl * self, long pos, long * x, long * y);
wxPoint *wxTextCtrl_PositionToCoords(const wxTextCtrl * self, long pos);
bool wxTextCtrl_SaveFile(wxTextCtrl * self, const wxString * filename, int file_type);
bool wxTextCtrl_SetDefaultStyle(wxTextCtrl * self, const wxTextAttr * style);
void wxTextCtrl_SetModified(wxTextCtrl * self, bool modified);
bool wxTextCtrl_SetStyle(wxTextCtrl * self, long start, long end, const wxTextAttr * style);
void wxTextCtrl_ShowPosition(wxTextCtrl * self, long pos);
long wxTextCtrl_XYToPosition(const wxTextCtrl * self, long x, long y);
// Mix-in(s) to wxTextCtrl
wxTextEntryBase *wxTextCtrl_AsTextEntry(wxTextCtrl* obj);
wxTrackable *wxTextCtrl_AsTrackable(wxTextCtrl* obj);

// CLASS: wxTextDataObject
void wxTextDataObject_delete(wxTextDataObject *self);
wxTextDataObject *wxTextDataObject_new(const wxString * text);
wxString *wxTextDataObject_GetText(const wxTextDataObject * self);
size_t wxTextDataObject_GetTextLength(const wxTextDataObject * self);
const wxDataFormat * wxTextDataObject_GetFormat(const wxTextDataObject * self);
void wxTextDataObject_SetText(wxTextDataObject * self, const wxString * str_text);

// CLASS: wxTextDropTarget
void wxTextDropTarget_delete(wxTextDropTarget *self);
wxTextDropTarget *wxTextDropTarget_new();
bool wxTextDropTarget_OnDropText(wxTextDropTarget * self, wxCoord x, wxCoord y, const wxString * data);

// CLASS: wxTextEntry
void wxTextEntry_delete(wxTextEntry *self);
void wxTextEntry_AppendText(wxTextEntry * self, const wxString * text);
bool wxTextEntry_AutoComplete(wxTextEntry * self, const wxArrayString * choices);
bool wxTextEntry_AutoComplete1(wxTextEntry * self, wxTextCompleter * completer);
bool wxTextEntry_AutoCompleteFileNames(wxTextEntry * self);
bool wxTextEntry_AutoCompleteDirectories(wxTextEntry * self);
bool wxTextEntry_CanCopy(const wxTextEntry * self);
bool wxTextEntry_CanCut(const wxTextEntry * self);
bool wxTextEntry_CanPaste(const wxTextEntry * self);
bool wxTextEntry_CanRedo(const wxTextEntry * self);
bool wxTextEntry_CanUndo(const wxTextEntry * self);
void wxTextEntry_ChangeValue(wxTextEntry * self, const wxString * value);
void wxTextEntry_Clear(wxTextEntry * self);
void wxTextEntry_Copy(wxTextEntry * self);
void wxTextEntry_Cut(wxTextEntry * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxTextEntry_ForceUpper(wxTextEntry * self);
#endif
long wxTextEntry_GetInsertionPoint(const wxTextEntry * self);
wxString *wxTextEntry_GetRange(const wxTextEntry * self, long from, long to);
void wxTextEntry_GetSelection(const wxTextEntry * self, long * from, long * to);
wxString *wxTextEntry_GetStringSelection(const wxTextEntry * self);
wxString *wxTextEntry_GetValue(const wxTextEntry * self);
bool wxTextEntry_IsEditable(const wxTextEntry * self);
bool wxTextEntry_IsEmpty(const wxTextEntry * self);
void wxTextEntry_Paste(wxTextEntry * self);
void wxTextEntry_Redo(wxTextEntry * self);
void wxTextEntry_Remove(wxTextEntry * self, long from, long to);
void wxTextEntry_Replace(wxTextEntry * self, long from, long to, const wxString * value);
void wxTextEntry_SetEditable(wxTextEntry * self, bool editable);
void wxTextEntry_SetInsertionPoint(wxTextEntry * self, long pos);
void wxTextEntry_SetInsertionPointEnd(wxTextEntry * self);
void wxTextEntry_SetSelection(wxTextEntry * self, long from, long to);
void wxTextEntry_SelectAll(wxTextEntry * self);
void wxTextEntry_SelectNone(wxTextEntry * self);
bool wxTextEntry_SetHint(wxTextEntry * self, const wxString * hint);
wxString *wxTextEntry_GetHint(const wxTextEntry * self);
bool wxTextEntry_SetMargins(wxTextEntry * self, const wxPoint * pt);
bool wxTextEntry_SetMargins1(wxTextEntry * self, wxCoord left, wxCoord top);
wxPoint *wxTextEntry_GetMargins(const wxTextEntry * self);
void wxTextEntry_SetValue(wxTextEntry * self, const wxString * value);
void wxTextEntry_Undo(wxTextEntry * self);
void wxTextEntry_WriteText(wxTextEntry * self, const wxString * text);

// CLASS: wxTextEntryDialog
wxClassInfo *wxTextEntryDialog_CLASSINFO();
wxTextEntryDialog *wxTextEntryDialog_new();
wxTextEntryDialog *wxTextEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * caption, const wxString * value, long style, const wxPoint * pos);
bool wxTextEntryDialog_Create(wxTextEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * caption, const wxString * value, long style, const wxPoint * pos);
wxString *wxTextEntryDialog_GetValue(const wxTextEntryDialog * self);
void wxTextEntryDialog_SetTextValidator(wxTextEntryDialog * self, const wxTextValidator * validator);
void wxTextEntryDialog_SetValue(wxTextEntryDialog * self, const wxString * value);
void wxTextEntryDialog_ForceUpper(wxTextEntryDialog * self);
// Mix-in(s) to wxTextEntryDialog
wxTrackable *wxTextEntryDialog_AsTrackable(wxTextEntryDialog* obj);

// CLASS: wxTextValidator
wxClassInfo *wxTextValidator_CLASSINFO();
wxTextValidator *wxTextValidator_new(const wxTextValidator * validator);
wxTextValidator *wxTextValidator_new1(long style, wxString * val_ptr);
wxString *wxTextValidator_GetCharExcludes(const wxTextValidator * self);
wxString *wxTextValidator_GetCharIncludes(const wxTextValidator * self);
wxArrayString *wxTextValidator_GetExcludes(const wxTextValidator * self);
wxArrayString *wxTextValidator_GetIncludes(const wxTextValidator * self);
long wxTextValidator_GetStyle(const wxTextValidator * self);
void wxTextValidator_OnChar(wxTextValidator * self, wxKeyEvent * event);
void wxTextValidator_SetExcludes(wxTextValidator * self, const wxArrayString * string_list);
void wxTextValidator_SetCharExcludes(wxTextValidator * self, const wxString * chars);
void wxTextValidator_SetIncludes(wxTextValidator * self, const wxArrayString * string_list);
void wxTextValidator_SetCharIncludes(wxTextValidator * self, const wxString * chars);
void wxTextValidator_AddExclude(wxTextValidator * self, const wxString * exclude);
void wxTextValidator_AddInclude(wxTextValidator * self, const wxString * include);
void wxTextValidator_AddCharExcludes(wxTextValidator * self, const wxString * chars);
void wxTextValidator_AddCharIncludes(wxTextValidator * self, const wxString * chars);
void wxTextValidator_SetStyle(wxTextValidator * self, long style);
wxString *wxTextValidator_IsValid(const wxTextValidator * self, const wxString * val);
// Mix-in(s) to wxTextValidator
wxTrackable *wxTextValidator_AsTrackable(wxTextValidator* obj);

// CLASS: wxThreadEvent
wxClassInfo *wxThreadEvent_CLASSINFO();
void wxThreadEvent_SetPayload(wxThreadEvent * self, const T * payload);
long wxThreadEvent_GetExtraLong(const wxThreadEvent * self);
int wxThreadEvent_GetInt(const wxThreadEvent * self);
wxString *wxThreadEvent_GetString(const wxThreadEvent * self);
void wxThreadEvent_SetExtraLong(wxThreadEvent * self, long extra_long);
void wxThreadEvent_SetInt(wxThreadEvent * self, int int_command);
void wxThreadEvent_SetString(wxThreadEvent * self, const wxString * string);

// CLASS: wxThumbBarButton
void wxThumbBarButton_delete(wxThumbBarButton *self);
wxThumbBarButton *wxThumbBarButton_new();
wxThumbBarButton *wxThumbBarButton_new1(int id, const wxIcon * icon, const wxString * tooltip, bool enable, bool dismiss_on_click, bool has_background, bool shown, bool interactive);
bool wxThumbBarButton_Create(wxThumbBarButton * self, int id, const wxIcon * icon, const wxString * tooltip, bool enable, bool dismiss_on_click, bool has_background, bool shown, bool interactive);
int wxThumbBarButton_GetID(const wxThumbBarButton * self);
wxIcon *wxThumbBarButton_GetIcon(const wxThumbBarButton * self);
wxString *wxThumbBarButton_GetTooltip(const wxThumbBarButton * self);
bool wxThumbBarButton_IsEnable(const wxThumbBarButton * self);
void wxThumbBarButton_Enable(wxThumbBarButton * self, bool enable);
void wxThumbBarButton_Disable(wxThumbBarButton * self);
bool wxThumbBarButton_IsDismissOnClick(const wxThumbBarButton * self);
void wxThumbBarButton_EnableDismissOnClick(wxThumbBarButton * self, bool enable);
void wxThumbBarButton_DisableDimissOnClick(wxThumbBarButton * self);
bool wxThumbBarButton_HasBackground(const wxThumbBarButton * self);
void wxThumbBarButton_SetHasBackground(wxThumbBarButton * self, bool has);
bool wxThumbBarButton_IsShown(const wxThumbBarButton * self);
void wxThumbBarButton_Show(wxThumbBarButton * self, bool shown);
void wxThumbBarButton_Hide(wxThumbBarButton * self);
bool wxThumbBarButton_IsInteractive(const wxThumbBarButton * self);
void wxThumbBarButton_SetInteractive(wxThumbBarButton * self, bool interactive);

// CLASS: wxTimePickerCtrl
wxClassInfo *wxTimePickerCtrl_CLASSINFO();
wxTimePickerCtrl *wxTimePickerCtrl_new();
wxTimePickerCtrl *wxTimePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxTimePickerCtrl_Create(wxTimePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxTimePickerCtrl_GetTime(const wxTimePickerCtrl * self, int * hour, int * min, int * sec);
wxDateTime *wxTimePickerCtrl_GetValue(const wxTimePickerCtrl * self);
bool wxTimePickerCtrl_SetTime(wxTimePickerCtrl * self, int hour, int min, int sec);
void wxTimePickerCtrl_SetValue(wxTimePickerCtrl * self, const wxDateTime * dt);
// Mix-in(s) to wxTimePickerCtrl
wxTrackable *wxTimePickerCtrl_AsTrackable(wxTimePickerCtrl* obj);

// CLASS: wxTipProvider
void wxTipProvider_delete(wxTipProvider *self);
wxTipProvider *wxTipProvider_new(size_t current_tip);
size_t wxTipProvider_GetCurrentTip(const wxTipProvider * self);
wxString *wxTipProvider_GetTip(wxTipProvider * self);

// CLASS: wxTipWindow
wxClassInfo *wxTipWindow_CLASSINFO();
wxTipWindow *wxTipWindow_new(wxWindow * parent, const wxString * text, wxCoord max_length, wxTipWindow ** window_ptr, wxRect * rect_bounds);
void wxTipWindow_SetBoundingRect(wxTipWindow * self, const wxRect * rect_bound);
void wxTipWindow_SetTipWindowPtr(wxTipWindow * self, wxTipWindow ** window_ptr);
// Mix-in(s) to wxTipWindow
wxTrackable *wxTipWindow_AsTrackable(wxTipWindow* obj);

// CLASS: wxToggleButton
wxClassInfo *wxToggleButton_CLASSINFO();
wxToggleButton *wxToggleButton_new();
wxToggleButton *wxToggleButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name);
bool wxToggleButton_Create(wxToggleButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name);
bool wxToggleButton_GetValue(const wxToggleButton * self);
void wxToggleButton_SetValue(wxToggleButton * self, bool state);
// Mix-in(s) to wxToggleButton
wxTrackable *wxToggleButton_AsTrackable(wxToggleButton* obj);

// CLASS: wxToolBar
wxClassInfo *wxToolBar_CLASSINFO();
wxToolBar *wxToolBar_new();
wxToolBar *wxToolBar_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxToolBarToolBase * wxToolBar_AddCheckTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap1, const wxBitmapBundle * bmp_disabled, const wxString * short_help, const wxString * long_help, wxObject * client_data);
wxToolBarToolBase * wxToolBar_AddControl(wxToolBar * self, wxControl * control, const wxString * label);
wxToolBarToolBase * wxToolBar_AddRadioTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap1, const wxBitmapBundle * bmp_disabled, const wxString * short_help, const wxString * long_help, wxObject * client_data);
wxToolBarToolBase * wxToolBar_AddSeparator(wxToolBar * self);
wxToolBarToolBase * wxToolBar_AddStretchableSpace(wxToolBar * self);
wxToolBarToolBase * wxToolBar_AddTool(wxToolBar * self, wxToolBarToolBase * tool);
wxToolBarToolBase * wxToolBar_AddTool1(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxString * short_help, wxItemKind kind);
wxToolBarToolBase * wxToolBar_AddTool2(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxBitmapBundle * bmp_disabled, wxItemKind kind, const wxString * short_help, const wxString * long_help, wxObject * client_data);
void wxToolBar_ClearTools(wxToolBar * self);
bool wxToolBar_DeleteTool(wxToolBar * self, int tool_id);
bool wxToolBar_DeleteToolByPos(wxToolBar * self, size_t pos);
void wxToolBar_EnableTool(wxToolBar * self, int tool_id, bool enable);
wxToolBarToolBase * wxToolBar_FindById(const wxToolBar * self, int id);
wxControl * wxToolBar_FindControl(wxToolBar * self, int id);
wxToolBarToolBase * wxToolBar_FindToolForPosition(const wxToolBar * self, wxCoord x, wxCoord y);
wxSize *wxToolBar_GetMargins(const wxToolBar * self);
wxSize *wxToolBar_GetToolBitmapSize(const wxToolBar * self);
const wxToolBarToolBase * wxToolBar_GetToolByPos1(const wxToolBar * self, int pos);
wxObject * wxToolBar_GetToolClientData(const wxToolBar * self, int tool_id);
bool wxToolBar_GetToolEnabled(const wxToolBar * self, int tool_id);
wxString *wxToolBar_GetToolLongHelp(const wxToolBar * self, int tool_id);
int wxToolBar_GetToolPacking(const wxToolBar * self);
int wxToolBar_GetToolPos(const wxToolBar * self, int tool_id);
int wxToolBar_GetToolSeparation(const wxToolBar * self);
wxString *wxToolBar_GetToolShortHelp(const wxToolBar * self, int tool_id);
wxSize *wxToolBar_GetToolSize(const wxToolBar * self);
bool wxToolBar_GetToolState(const wxToolBar * self, int tool_id);
size_t wxToolBar_GetToolsCount(const wxToolBar * self);
wxToolBarToolBase * wxToolBar_InsertControl(wxToolBar * self, size_t pos, wxControl * control, const wxString * label);
wxToolBarToolBase * wxToolBar_InsertSeparator(wxToolBar * self, size_t pos);
wxToolBarToolBase * wxToolBar_InsertStretchableSpace(wxToolBar * self, size_t pos);
wxToolBarToolBase * wxToolBar_InsertTool(wxToolBar * self, size_t pos, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxBitmapBundle * bmp_disabled, wxItemKind kind, const wxString * short_help, const wxString * long_help, wxObject * client_data);
wxToolBarToolBase * wxToolBar_InsertTool1(wxToolBar * self, size_t pos, wxToolBarToolBase * tool);
bool wxToolBar_OnLeftClick(wxToolBar * self, int tool_id, bool toggle_down);
void wxToolBar_OnMouseEnter(wxToolBar * self, int tool_id);
void wxToolBar_OnRightClick(wxToolBar * self, int tool_id, long x, long y);
bool wxToolBar_Realize(wxToolBar * self);
wxToolBarToolBase * wxToolBar_RemoveTool(wxToolBar * self, int id);
bool wxToolBar_SetDropdownMenu(wxToolBar * self, int id, wxMenu * menu);
void wxToolBar_SetMargins(wxToolBar * self, int x, int y);
void wxToolBar_SetMargins1(wxToolBar * self, const wxSize * size);
void wxToolBar_SetToolBitmapSize(wxToolBar * self, const wxSize * size);
void wxToolBar_SetToolClientData(wxToolBar * self, int id, wxObject * client_data);
void wxToolBar_SetToolDisabledBitmap(wxToolBar * self, int id, const wxBitmapBundle * bitmap);
void wxToolBar_SetToolLongHelp(wxToolBar * self, int tool_id, const wxString * help_string);
void wxToolBar_SetToolNormalBitmap(wxToolBar * self, int id, const wxBitmapBundle * bitmap);
void wxToolBar_SetToolPacking(wxToolBar * self, int packing);
void wxToolBar_SetToolSeparation(wxToolBar * self, int separation);
void wxToolBar_SetToolShortHelp(wxToolBar * self, int tool_id, const wxString * help_string);
void wxToolBar_ToggleTool(wxToolBar * self, int tool_id, bool toggle);
wxToolBarToolBase * wxToolBar_CreateTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bmp_normal, const wxBitmapBundle * bmp_disabled, wxItemKind kind, wxObject * client_data, const wxString * short_help, const wxString * long_help);
wxToolBarToolBase * wxToolBar_CreateTool1(wxToolBar * self, wxControl * control, const wxString * label);
wxToolBarToolBase * wxToolBar_CreateSeparator(wxToolBar * self);
// Mix-in(s) to wxToolBar
wxTrackable *wxToolBar_AsTrackable(wxToolBar* obj);

// CLASS: wxToolTip
wxClassInfo *wxToolTip_CLASSINFO();
wxToolTip *wxToolTip_new(const wxString * tip);
wxString *wxToolTip_GetTip(const wxToolTip * self);
wxWindow * wxToolTip_GetWindow(const wxToolTip * self);
void wxToolTip_SetTip(wxToolTip * self, const wxString * tip);
void wxToolTip_Enable(bool flag);
void wxToolTip_SetAutoPop(long msecs);
void wxToolTip_SetDelay(long msecs);
void wxToolTip_SetMaxWidth(int width);
void wxToolTip_SetReshow(long msecs);

// CLASS: wxToolbook
wxClassInfo *wxToolbook_CLASSINFO();
wxToolbook *wxToolbook_new();
wxToolbook *wxToolbook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxToolbook_Create(wxToolbook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxToolBarBase * wxToolbook_GetToolBar(const wxToolbook * self);
bool wxToolbook_EnablePage(wxToolbook * self, size_t page, bool enable);
bool wxToolbook_EnablePage1(wxToolbook * self, wxWindow * page, bool enable);
// Mix-in(s) to wxToolbook
wxWithImages *wxToolbook_AsWithImages(wxToolbook* obj);
wxTrackable *wxToolbook_AsTrackable(wxToolbook* obj);

// CLASS: wxTopLevelWindow
wxClassInfo *wxTopLevelWindow_CLASSINFO();
wxTopLevelWindow *wxTopLevelWindow_new();
wxTopLevelWindow *wxTopLevelWindow_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTopLevelWindow_Create(wxTopLevelWindow * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxTopLevelWindow_CenterOnScreen(wxTopLevelWindow * self, int direction);
void wxTopLevelWindow_CentreOnScreen(wxTopLevelWindow * self, int direction);
bool wxTopLevelWindow_EnableCloseButton(wxTopLevelWindow * self, bool enable);
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableMaximizeButton(wxTopLevelWindow * self, bool enable);
bool wxTopLevelWindow_EnableMinimizeButton(wxTopLevelWindow * self, bool enable);
#endif
wxWindow * wxTopLevelWindow_GetDefaultItem(const wxTopLevelWindow * self);
wxIcon *wxTopLevelWindow_GetIcon(const wxTopLevelWindow * self);
wxString *wxTopLevelWindow_GetTitle(const wxTopLevelWindow * self);
void wxTopLevelWindow_Iconize(wxTopLevelWindow * self, bool iconize);
bool wxTopLevelWindow_IsActive(wxTopLevelWindow * self);
bool wxTopLevelWindow_IsAlwaysMaximized(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsFullScreen(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsIconized(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsMaximized(const wxTopLevelWindow * self);
void wxTopLevelWindow_Maximize(wxTopLevelWindow * self, bool maximize);
#ifdef __WXMSW__
wxMenu * wxTopLevelWindow_MSWGetSystemMenu(const wxTopLevelWindow * self);
#endif
void wxTopLevelWindow_RequestUserAttention(wxTopLevelWindow * self, int flags);
void wxTopLevelWindow_Restore(wxTopLevelWindow * self);
wxWindow * wxTopLevelWindow_SetDefaultItem(wxTopLevelWindow * self, wxWindow * win);
wxWindow * wxTopLevelWindow_SetTmpDefaultItem(wxTopLevelWindow * self, wxWindow * win);
wxWindow * wxTopLevelWindow_GetTmpDefaultItem(const wxTopLevelWindow * self);
void wxTopLevelWindow_SetIcon(wxTopLevelWindow * self, const wxIcon * icon);
void wxTopLevelWindow_SetIcons(wxTopLevelWindow * self, const wxIconBundle * icons);
void wxTopLevelWindow_SetTitle(wxTopLevelWindow * self, const wxString * title);
bool wxTopLevelWindow_ShouldPreventAppExit(const wxTopLevelWindow * self);
void wxTopLevelWindow_OSXSetModified(wxTopLevelWindow * self, bool modified);
bool wxTopLevelWindow_OSXIsModified(const wxTopLevelWindow * self);
void wxTopLevelWindow_SetRepresentedFilename(wxTopLevelWindow * self, const wxString * filename);
void wxTopLevelWindow_ShowWithoutActivating(wxTopLevelWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableFullScreenView(wxTopLevelWindow * self, bool enable, long style);
#endif
bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow * self, bool show, long style);
wxSize *wxTopLevelWindow_GetDefaultSize();
// Mix-in(s) to wxTopLevelWindow
wxTrackable *wxTopLevelWindow_AsTrackable(wxTopLevelWindow* obj);

// CLASS: wxTreeCtrl
wxClassInfo *wxTreeCtrl_CLASSINFO();
wxTreeCtrl *wxTreeCtrl_new();
wxTreeCtrl *wxTreeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxTreeItemId *wxTreeCtrl_AddRoot(wxTreeCtrl * self, const wxString * text, int image, int sel_image, wxTreeItemData * data);
wxTreeItemId *wxTreeCtrl_AppendItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxString * text, int image, int sel_image, wxTreeItemData * data);
void wxTreeCtrl_AssignButtonsImageList(wxTreeCtrl * self, wxImageList * image_list);
void wxTreeCtrl_AssignStateImageList(wxTreeCtrl * self, wxImageList * image_list);
void wxTreeCtrl_Collapse(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_CollapseAll(wxTreeCtrl * self);
void wxTreeCtrl_CollapseAllChildren(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_CollapseAndReset(wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_Create(wxTreeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxTreeCtrl_Delete(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_DeleteAllItems(wxTreeCtrl * self);
void wxTreeCtrl_DeleteChildren(wxTreeCtrl * self, const wxTreeItemId * item);
wxTextCtrl * wxTreeCtrl_EditLabel(wxTreeCtrl * self, const wxTreeItemId * item, wxClassInfo * text_ctrl_class);
void wxTreeCtrl_EnableBellOnNoMatch(wxTreeCtrl * self, bool on);
void wxTreeCtrl_EndEditLabel(wxTreeCtrl * self, const wxTreeItemId * item, bool discard_changes);
void wxTreeCtrl_EnsureVisible(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_Expand(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_ExpandAll(wxTreeCtrl * self);
void wxTreeCtrl_ExpandAllChildren(wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_GetBoundingRect(const wxTreeCtrl * self, const wxTreeItemId * item, wxRect * rect, bool text_only);
wxImageList * wxTreeCtrl_GetButtonsImageList(const wxTreeCtrl * self);
size_t wxTreeCtrl_GetChildrenCount(const wxTreeCtrl * self, const wxTreeItemId * item, bool recursively);
unsigned int wxTreeCtrl_GetCount(const wxTreeCtrl * self);
wxTextCtrl * wxTreeCtrl_GetEditControl(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_GetFirstChild(const wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemIdValue * cookie);
wxTreeItemId *wxTreeCtrl_GetFirstVisibleItem(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_GetFocusedItem(const wxTreeCtrl * self);
void wxTreeCtrl_ClearFocusedItem(wxTreeCtrl * self);
void wxTreeCtrl_SetFocusedItem(wxTreeCtrl * self, const wxTreeItemId * item);
unsigned int wxTreeCtrl_GetIndent(const wxTreeCtrl * self);
unsigned int wxTreeCtrl_GetSpacing(const wxTreeCtrl * self);
wxColour *wxTreeCtrl_GetItemBackgroundColour(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemData * wxTreeCtrl_GetItemData(const wxTreeCtrl * self, const wxTreeItemId * item);
wxFont *wxTreeCtrl_GetItemFont(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetItemParent(const wxTreeCtrl * self, const wxTreeItemId * item);
int wxTreeCtrl_GetItemState(const wxTreeCtrl * self, const wxTreeItemId * item);
wxString *wxTreeCtrl_GetItemText(const wxTreeCtrl * self, const wxTreeItemId * item);
wxColour *wxTreeCtrl_GetItemTextColour(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetLastChild(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetNextChild(const wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemIdValue * cookie);
wxTreeItemId *wxTreeCtrl_GetNextSibling(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetNextVisible(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetPrevSibling(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetPrevVisible(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_GetQuickBestSize(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_GetRootItem(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_GetSelection(const wxTreeCtrl * self);
size_t wxTreeCtrl_GetSelections(const wxTreeCtrl * self, wxArrayTreeItemIds * selection);
wxImageList * wxTreeCtrl_GetStateImageList(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_HitTest(const wxTreeCtrl * self, const wxPoint * point, int * flags);
wxTreeItemId *wxTreeCtrl_InsertItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxTreeItemId * previous, const wxString * text, int image, int sel_image, wxTreeItemData * data);
wxTreeItemId *wxTreeCtrl_InsertItem1(wxTreeCtrl * self, const wxTreeItemId * parent, size_t pos, const wxString * text, int image, int sel_image, wxTreeItemData * data);
bool wxTreeCtrl_IsBold(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_IsEmpty(const wxTreeCtrl * self);
bool wxTreeCtrl_IsExpanded(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_IsSelected(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_IsVisible(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_ItemHasChildren(const wxTreeCtrl * self, const wxTreeItemId * item);
int wxTreeCtrl_OnCompareItems(wxTreeCtrl * self, const wxTreeItemId * item1, const wxTreeItemId * item2);
wxTreeItemId *wxTreeCtrl_PrependItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxString * text, int image, int sel_image, wxTreeItemData * data);
void wxTreeCtrl_ScrollTo(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_SelectItem(wxTreeCtrl * self, const wxTreeItemId * item, bool select);
void wxTreeCtrl_SetButtonsImageList(wxTreeCtrl * self, wxImageList * image_list);
void wxTreeCtrl_SetIndent(wxTreeCtrl * self, unsigned int indent);
void wxTreeCtrl_SetSpacing(wxTreeCtrl * self, unsigned int spacing);
void wxTreeCtrl_SetItemBackgroundColour(wxTreeCtrl * self, const wxTreeItemId * item, const wxColour * col);
void wxTreeCtrl_SetItemBold(wxTreeCtrl * self, const wxTreeItemId * item, bool bold);
void wxTreeCtrl_SetItemData(wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemData * data);
void wxTreeCtrl_SetItemDropHighlight(wxTreeCtrl * self, const wxTreeItemId * item, bool highlight);
void wxTreeCtrl_SetItemFont(wxTreeCtrl * self, const wxTreeItemId * item, const wxFont * font);
void wxTreeCtrl_SetItemHasChildren(wxTreeCtrl * self, const wxTreeItemId * item, bool has_children);
void wxTreeCtrl_SetItemState(wxTreeCtrl * self, const wxTreeItemId * item, int state);
void wxTreeCtrl_SetItemText(wxTreeCtrl * self, const wxTreeItemId * item, const wxString * text);
void wxTreeCtrl_SetItemTextColour(wxTreeCtrl * self, const wxTreeItemId * item, const wxColour * col);
void wxTreeCtrl_SetQuickBestSize(wxTreeCtrl * self, bool quick_best_size);
void wxTreeCtrl_SetStateImageList(wxTreeCtrl * self, wxImageList * image_list);
void wxTreeCtrl_SetWindowStyle(wxTreeCtrl * self, long styles);
void wxTreeCtrl_SortChildren(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_Toggle(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_ToggleItemSelection(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_Unselect(wxTreeCtrl * self);
void wxTreeCtrl_UnselectAll(wxTreeCtrl * self);
void wxTreeCtrl_UnselectItem(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_SelectChildren(wxTreeCtrl * self, const wxTreeItemId * parent);
// Mix-in(s) to wxTreeCtrl
wxWithImages *wxTreeCtrl_AsWithImages(wxTreeCtrl* obj);
wxTrackable *wxTreeCtrl_AsTrackable(wxTreeCtrl* obj);

// CLASS: wxTreeEvent
wxClassInfo *wxTreeEvent_CLASSINFO();
wxTreeItemId *wxTreeEvent_GetItem(const wxTreeEvent * self);
int wxTreeEvent_GetKeyCode(const wxTreeEvent * self);
wxKeyEvent *wxTreeEvent_GetKeyEvent(const wxTreeEvent * self);
wxString *wxTreeEvent_GetLabel(const wxTreeEvent * self);
wxTreeItemId *wxTreeEvent_GetOldItem(const wxTreeEvent * self);
wxPoint *wxTreeEvent_GetPoint(const wxTreeEvent * self);
bool wxTreeEvent_IsEditCancelled(const wxTreeEvent * self);
void wxTreeEvent_SetToolTip(wxTreeEvent * self, const wxString * tooltip);

// CLASS: wxTreeItemData
void wxTreeItemData_delete(wxTreeItemData *self);
wxTreeItemData *wxTreeItemData_new();
wxTreeItemId *wxTreeItemData_GetId(const wxTreeItemData * self);
void wxTreeItemData_SetId(wxTreeItemData * self, const wxTreeItemId * id);

// CLASS: wxTreeItemId
void wxTreeItemId_delete(wxTreeItemId *self);
wxTreeItemId *wxTreeItemId_new();
bool wxTreeItemId_IsOk(const wxTreeItemId * self);
void * wxTreeItemId_GetID(const wxTreeItemId * self);
void wxTreeItemId_Unset(wxTreeItemId * self);

// CLASS: wxTreeListCtrl
wxClassInfo *wxTreeListCtrl_CLASSINFO();
void wxTreeListCtrl_AssignImageList(wxTreeListCtrl * self, wxImageList * image_list);
void wxTreeListCtrl_SetImageList(wxTreeListCtrl * self, wxImageList * image_list);
int wxTreeListCtrl_AppendColumn(wxTreeListCtrl * self, const wxString * title, int width, wxAlignment align, int flags);
void wxTreeListCtrl_ClearColumns(wxTreeListCtrl * self);
int wxTreeListCtrl_WidthFor(const wxTreeListCtrl * self, const wxString * text);
wxTreeListItem *wxTreeListCtrl_AppendItem(wxTreeListCtrl * self, wxTreeListItem parent, const wxString * text, int image_closed, int image_opened, wxClientData * data);
wxTreeListItem *wxTreeListCtrl_InsertItem(wxTreeListCtrl * self, wxTreeListItem parent, wxTreeListItem previous, const wxString * text, int image_closed, int image_opened, wxClientData * data);
wxTreeListItem *wxTreeListCtrl_PrependItem(wxTreeListCtrl * self, wxTreeListItem parent, const wxString * text, int image_closed, int image_opened, wxClientData * data);
void wxTreeListCtrl_DeleteItem(wxTreeListCtrl * self, wxTreeListItem item);
void wxTreeListCtrl_DeleteAllItems(wxTreeListCtrl * self);
wxTreeListItem *wxTreeListCtrl_GetRootItem(const wxTreeListCtrl * self);
wxTreeListItem *wxTreeListCtrl_GetItemParent(const wxTreeListCtrl * self, wxTreeListItem item);
wxTreeListItem *wxTreeListCtrl_GetFirstChild(const wxTreeListCtrl * self, wxTreeListItem item);
wxTreeListItem *wxTreeListCtrl_GetNextSibling(const wxTreeListCtrl * self, wxTreeListItem item);
wxTreeListItem *wxTreeListCtrl_GetFirstItem(const wxTreeListCtrl * self);
wxTreeListItem *wxTreeListCtrl_GetNextItem(const wxTreeListCtrl * self, wxTreeListItem item);
void wxTreeListCtrl_SetItemText1(wxTreeListCtrl * self, wxTreeListItem item, const wxString * text);
void wxTreeListCtrl_SetItemImage(wxTreeListCtrl * self, wxTreeListItem item, int closed, int opened);
wxClientData * wxTreeListCtrl_GetItemData(const wxTreeListCtrl * self, wxTreeListItem item);
void wxTreeListCtrl_SetItemData(wxTreeListCtrl * self, wxTreeListItem item, wxClientData * data);
void wxTreeListCtrl_Expand(wxTreeListCtrl * self, wxTreeListItem item);
void wxTreeListCtrl_Collapse(wxTreeListCtrl * self, wxTreeListItem item);
bool wxTreeListCtrl_IsExpanded(const wxTreeListCtrl * self, wxTreeListItem item);
wxTreeListItem *wxTreeListCtrl_GetSelection(const wxTreeListCtrl * self);
void wxTreeListCtrl_Select(wxTreeListCtrl * self, wxTreeListItem item);
void wxTreeListCtrl_Unselect(wxTreeListCtrl * self, wxTreeListItem item);
bool wxTreeListCtrl_IsSelected(const wxTreeListCtrl * self, wxTreeListItem item);
void wxTreeListCtrl_SelectAll(wxTreeListCtrl * self);
void wxTreeListCtrl_UnselectAll(wxTreeListCtrl * self);
void wxTreeListCtrl_EnsureVisible(wxTreeListCtrl * self, wxTreeListItem item);
void wxTreeListCtrl_CheckItem(wxTreeListCtrl * self, wxTreeListItem item, wxCheckBoxState state);
void wxTreeListCtrl_CheckItemRecursively(wxTreeListCtrl * self, wxTreeListItem item, wxCheckBoxState state);
void wxTreeListCtrl_UncheckItem(wxTreeListCtrl * self, wxTreeListItem item);
void wxTreeListCtrl_UpdateItemParentStateRecursively(wxTreeListCtrl * self, wxTreeListItem item);
wxCheckBoxState wxTreeListCtrl_GetCheckedState(const wxTreeListCtrl * self, wxTreeListItem item);
bool wxTreeListCtrl_AreAllChildrenInState(const wxTreeListCtrl * self, wxTreeListItem item, wxCheckBoxState state);
bool wxTreeListCtrl_GetSortColumn(wxTreeListCtrl * self, unsigned * col, bool * ascending_order);
void wxTreeListCtrl_SetItemComparator(wxTreeListCtrl * self, wxTreeListItemComparator * comparator);
wxWindow * wxTreeListCtrl_GetView(const wxTreeListCtrl * self);
wxDataViewCtrl * wxTreeListCtrl_GetDataView(const wxTreeListCtrl * self);
wxTreeListCtrl *wxTreeListCtrl_new();
wxTreeListCtrl *wxTreeListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTreeListCtrl_Create(wxTreeListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxTreeListCtrl
wxTrackable *wxTreeListCtrl_AsTrackable(wxTreeListCtrl* obj);

// CLASS: wxTreeListItem
void wxTreeListItem_delete(wxTreeListItem *self);
wxTreeListItem *wxTreeListItem_new();
bool wxTreeListItem_IsOk(const wxTreeListItem * self);

// CLASS: wxTreeListItemComparator
void wxTreeListItemComparator_delete(wxTreeListItemComparator *self);
wxTreeListItemComparator *wxTreeListItemComparator_new();

// CLASS: wxTreebook
wxClassInfo *wxTreebook_CLASSINFO();
wxTreebook *wxTreebook_new();
wxTreebook *wxTreebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTreebook_AddSubPage(wxTreebook * self, wxWindow * page, const wxString * text, bool b_select, int image_id);
bool wxTreebook_CollapseNode(wxTreebook * self, size_t page_id);
bool wxTreebook_Create(wxTreebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTreebook_ExpandNode(wxTreebook * self, size_t page_id, bool expand);
int wxTreebook_GetPageParent(const wxTreebook * self, size_t page);
bool wxTreebook_InsertSubPage(wxTreebook * self, size_t page_pos, wxWindow * page, const wxString * text, bool b_select, int image_id);
bool wxTreebook_IsNodeExpanded(const wxTreebook * self, size_t page_id);
// Mix-in(s) to wxTreebook
wxWithImages *wxTreebook_AsWithImages(wxTreebook* obj);
wxTrackable *wxTreebook_AsTrackable(wxTreebook* obj);

// CLASS: wxTwoFingerTapEvent
wxClassInfo *wxTwoFingerTapEvent_CLASSINFO();
wxTwoFingerTapEvent *wxTwoFingerTapEvent_new(wxWindowID windid);

// CLASS: wxUIActionSimulator
void wxUIActionSimulator_delete(wxUIActionSimulator *self);
wxUIActionSimulator *wxUIActionSimulator_new();
bool wxUIActionSimulator_MouseMove(wxUIActionSimulator * self, long x, long y);
bool wxUIActionSimulator_MouseMove1(wxUIActionSimulator * self, const wxPoint * point);
bool wxUIActionSimulator_MouseDown(wxUIActionSimulator * self, int button);
bool wxUIActionSimulator_MouseUp(wxUIActionSimulator * self, int button);
bool wxUIActionSimulator_MouseClick(wxUIActionSimulator * self, int button);
bool wxUIActionSimulator_MouseDblClick(wxUIActionSimulator * self, int button);
bool wxUIActionSimulator_MouseDragDrop(wxUIActionSimulator * self, long x1, long y1, long x2, long y2, int button);
bool wxUIActionSimulator_KeyDown(wxUIActionSimulator * self, int keycode, int modifiers);
bool wxUIActionSimulator_KeyUp(wxUIActionSimulator * self, int keycode, int modifiers);
bool wxUIActionSimulator_Char(wxUIActionSimulator * self, int keycode, int modifiers);
bool wxUIActionSimulator_Select(wxUIActionSimulator * self, const wxString * text);
bool wxUIActionSimulator_Text(wxUIActionSimulator * self, const char * text);

// CLASS: wxURLDataObject
void wxURLDataObject_delete(wxURLDataObject *self);
wxURLDataObject *wxURLDataObject_new(const wxString * url);
wxString *wxURLDataObject_GetURL(const wxURLDataObject * self);
void wxURLDataObject_SetURL(wxURLDataObject * self, const wxString * url);

// CLASS: wxUpdateUIEvent
wxClassInfo *wxUpdateUIEvent_CLASSINFO();
wxUpdateUIEvent *wxUpdateUIEvent_new(wxWindowID command_id);
void wxUpdateUIEvent_Check(wxUpdateUIEvent * self, bool check);
void wxUpdateUIEvent_Enable(wxUpdateUIEvent * self, bool enable);
bool wxUpdateUIEvent_GetChecked(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetEnabled(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_IsCheckable(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetSetChecked(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetSetEnabled(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetSetShown(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetSetText(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetShown(const wxUpdateUIEvent * self);
wxString *wxUpdateUIEvent_GetText(const wxUpdateUIEvent * self);
void wxUpdateUIEvent_SetText(wxUpdateUIEvent * self, const wxString * text);
void wxUpdateUIEvent_Show(wxUpdateUIEvent * self, bool show);
bool wxUpdateUIEvent_CanUpdate(wxWindow * window);
long wxUpdateUIEvent_GetUpdateInterval();
void wxUpdateUIEvent_ResetUpdateTime();
void wxUpdateUIEvent_SetUpdateInterval(long update_interval);

// CLASS: wxVListBox
wxClassInfo *wxVListBox_CLASSINFO();
wxVListBox *wxVListBox_new();
wxVListBox *wxVListBox_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxVListBox_Clear(wxVListBox * self);
bool wxVListBox_Create(wxVListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxVListBox_DeselectAll(wxVListBox * self);
int wxVListBox_GetFirstSelected(const wxVListBox * self, unsigned long * cookie);
size_t wxVListBox_GetItemCount(const wxVListBox * self);
wxPoint *wxVListBox_GetMargins(const wxVListBox * self);
wxRect *wxVListBox_GetItemRect(const wxVListBox * self, size_t item);
int wxVListBox_GetNextSelected(const wxVListBox * self, unsigned long * cookie);
size_t wxVListBox_GetSelectedCount(const wxVListBox * self);
int wxVListBox_GetSelection(const wxVListBox * self);
wxColour *wxVListBox_GetSelectionBackground(const wxVListBox * self);
bool wxVListBox_HasMultipleSelection(const wxVListBox * self);
bool wxVListBox_IsCurrent(const wxVListBox * self, size_t item);
bool wxVListBox_IsSelected(const wxVListBox * self, size_t item);
bool wxVListBox_Select(wxVListBox * self, size_t item, bool select);
bool wxVListBox_SelectAll(wxVListBox * self);
bool wxVListBox_SelectRange(wxVListBox * self, size_t from, size_t to);
void wxVListBox_SetItemCount(wxVListBox * self, size_t count);
void wxVListBox_SetMargins(wxVListBox * self, const wxPoint * pt);
void wxVListBox_SetMargins1(wxVListBox * self, wxCoord x, wxCoord y);
void wxVListBox_SetSelection(wxVListBox * self, int selection);
void wxVListBox_SetSelectionBackground(wxVListBox * self, const wxColour * col);
void wxVListBox_Toggle(wxVListBox * self, size_t item);
// Mix-in(s) to wxVListBox
wxVarVScrollHelper *wxVListBox_AsVarVScrollHelper(wxVListBox* obj);
wxTrackable *wxVListBox_AsTrackable(wxVListBox* obj);

// CLASS: wxVScrolledWindow
wxClassInfo *wxVScrolledWindow_CLASSINFO();
wxVScrolledWindow *wxVScrolledWindow_new();
wxVScrolledWindow *wxVScrolledWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxVScrolledWindow_Create(wxVScrolledWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxVScrolledWindow
wxVarVScrollHelper *wxVScrolledWindow_AsVarVScrollHelper(wxVScrolledWindow* obj);
wxTrackable *wxVScrolledWindow_AsTrackable(wxVScrolledWindow* obj);

// CLASS: wxValidator
wxClassInfo *wxValidator_CLASSINFO();
wxValidator *wxValidator_new();
wxObject * wxValidator_Clone(const wxValidator * self);
wxWindow * wxValidator_GetWindow(const wxValidator * self);
void wxValidator_SetWindow(wxValidator * self, wxWindow * window);
bool wxValidator_TransferFromWindow(wxValidator * self);
bool wxValidator_TransferToWindow(wxValidator * self);
bool wxValidator_Validate(wxValidator * self, wxWindow * parent);
void wxValidator_SuppressBellOnError(bool suppress);
bool wxValidator_IsSilent();
// Mix-in(s) to wxValidator
wxTrackable *wxValidator_AsTrackable(wxValidator* obj);

// CLASS: wxVarHScrollHelper
void wxVarHScrollHelper_delete(wxVarHScrollHelper *self);
wxVarHScrollHelper *wxVarHScrollHelper_new(wxWindow * win_to_scroll);
size_t wxVarHScrollHelper_GetColumnCount(const wxVarHScrollHelper * self);
size_t wxVarHScrollHelper_GetVisibleColumnsBegin(const wxVarHScrollHelper * self);
size_t wxVarHScrollHelper_GetVisibleColumnsEnd(const wxVarHScrollHelper * self);
bool wxVarHScrollHelper_IsColumnVisible(const wxVarHScrollHelper * self, size_t column);
void wxVarHScrollHelper_RefreshColumn(wxVarHScrollHelper * self, size_t column);
void wxVarHScrollHelper_RefreshColumns(wxVarHScrollHelper * self, size_t from, size_t to);
bool wxVarHScrollHelper_ScrollColumnPages(wxVarHScrollHelper * self, int pages);
bool wxVarHScrollHelper_ScrollColumns(wxVarHScrollHelper * self, int columns);
bool wxVarHScrollHelper_ScrollToColumn(wxVarHScrollHelper * self, size_t column);
void wxVarHScrollHelper_SetColumnCount(wxVarHScrollHelper * self, size_t column_count);

// CLASS: wxVarHVScrollHelper
void wxVarHVScrollHelper_delete(wxVarHVScrollHelper *self);
wxVarHVScrollHelper *wxVarHVScrollHelper_new(wxWindow * win_to_scroll);
void wxVarHVScrollHelper_EnablePhysicalScrolling(wxVarHVScrollHelper * self, bool vscrolling, bool hscrolling);
wxSize *wxVarHVScrollHelper_GetRowColumnCount(const wxVarHVScrollHelper * self);
wxPosition *wxVarHVScrollHelper_GetVisibleBegin(const wxVarHVScrollHelper * self);
wxPosition *wxVarHVScrollHelper_GetVisibleEnd(const wxVarHVScrollHelper * self);
bool wxVarHVScrollHelper_IsVisible(const wxVarHVScrollHelper * self, size_t row, size_t column);
bool wxVarHVScrollHelper_IsVisible1(const wxVarHVScrollHelper * self, const wxPosition * pos);
void wxVarHVScrollHelper_RefreshRowColumn(wxVarHVScrollHelper * self, size_t row, size_t column);
void wxVarHVScrollHelper_RefreshRowColumn1(wxVarHVScrollHelper * self, const wxPosition * pos);
void wxVarHVScrollHelper_RefreshRowsColumns(wxVarHVScrollHelper * self, size_t from_row, size_t to_row, size_t from_column, size_t to_column);
void wxVarHVScrollHelper_RefreshRowsColumns1(wxVarHVScrollHelper * self, const wxPosition * from, const wxPosition * to);
bool wxVarHVScrollHelper_ScrollToRowColumn(wxVarHVScrollHelper * self, size_t row, size_t column);
bool wxVarHVScrollHelper_ScrollToRowColumn1(wxVarHVScrollHelper * self, const wxPosition * pos);
void wxVarHVScrollHelper_SetRowColumnCount(wxVarHVScrollHelper * self, size_t row_count, size_t column_count);
wxPosition *wxVarHVScrollHelper_VirtualHitTest(const wxVarHVScrollHelper * self, wxCoord x, wxCoord y);
wxPosition *wxVarHVScrollHelper_VirtualHitTest1(const wxVarHVScrollHelper * self, const wxPoint * pos);
// Mix-in(s) to wxVarHVScrollHelper
wxVarHScrollHelper *wxVarHVScrollHelper_AsVarHScrollHelper(wxVarHVScrollHelper* obj);

// CLASS: wxVarScrollHelperBase
void wxVarScrollHelperBase_delete(wxVarScrollHelperBase *self);
wxVarScrollHelperBase *wxVarScrollHelperBase_new(wxWindow * win_to_scroll);
int wxVarScrollHelperBase_CalcScrolledPosition(const wxVarScrollHelperBase * self, int coord);
int wxVarScrollHelperBase_CalcUnscrolledPosition(const wxVarScrollHelperBase * self, int coord);
void wxVarScrollHelperBase_EnablePhysicalScrolling(wxVarScrollHelperBase * self, bool scrolling);
int wxVarScrollHelperBase_GetNonOrientationTargetSize(const wxVarScrollHelperBase * self);
int wxVarScrollHelperBase_GetOrientationTargetSize(const wxVarScrollHelperBase * self);
wxWindow * wxVarScrollHelperBase_GetTargetWindow(const wxVarScrollHelperBase * self);
size_t wxVarScrollHelperBase_GetVisibleBegin(const wxVarScrollHelperBase * self);
size_t wxVarScrollHelperBase_GetVisibleEnd(const wxVarScrollHelperBase * self);
bool wxVarScrollHelperBase_IsVisible(const wxVarScrollHelperBase * self, size_t unit);
void wxVarScrollHelperBase_RefreshAll(wxVarScrollHelperBase * self);
void wxVarScrollHelperBase_SetTargetWindow(wxVarScrollHelperBase * self, wxWindow * target);
void wxVarScrollHelperBase_UpdateScrollbar(wxVarScrollHelperBase * self);
int wxVarScrollHelperBase_VirtualHitTest(const wxVarScrollHelperBase * self, wxCoord coord);

// CLASS: wxVarVScrollHelper
void wxVarVScrollHelper_delete(wxVarVScrollHelper *self);
wxVarVScrollHelper *wxVarVScrollHelper_new(wxWindow * win_to_scroll);
size_t wxVarVScrollHelper_GetRowCount(const wxVarVScrollHelper * self);
size_t wxVarVScrollHelper_GetVisibleRowsBegin(const wxVarVScrollHelper * self);
size_t wxVarVScrollHelper_GetVisibleRowsEnd(const wxVarVScrollHelper * self);
bool wxVarVScrollHelper_IsRowVisible(const wxVarVScrollHelper * self, size_t row);
void wxVarVScrollHelper_RefreshRow(wxVarVScrollHelper * self, size_t row);
void wxVarVScrollHelper_RefreshRows(wxVarVScrollHelper * self, size_t from, size_t to);
bool wxVarVScrollHelper_ScrollRowPages(wxVarVScrollHelper * self, int pages);
bool wxVarVScrollHelper_ScrollRows(wxVarVScrollHelper * self, int rows);
bool wxVarVScrollHelper_ScrollToRow(wxVarVScrollHelper * self, size_t row);
void wxVarVScrollHelper_SetRowCount(wxVarVScrollHelper * self, size_t row_count);

// CLASS: wxVariantDataCurrency
void wxVariantDataCurrency_delete(wxVariantDataCurrency *self);
wxVariantDataCurrency *wxVariantDataCurrency_new();
bool wxVariantDataCurrency_GetAsAny(const wxVariantDataCurrency * self, wxAny * any);

// CLASS: wxVariantDataErrorCode
void wxVariantDataErrorCode_delete(wxVariantDataErrorCode *self);
bool wxVariantDataErrorCode_GetAsAny(const wxVariantDataErrorCode * self, wxAny * any);

// CLASS: wxVariantDataSafeArray
void wxVariantDataSafeArray_delete(wxVariantDataSafeArray *self);
wxVariantDataSafeArray *wxVariantDataSafeArray_new(SAFEARRAY * value);
SAFEARRAY * wxVariantDataSafeArray_GetValue(const wxVariantDataSafeArray * self);
void wxVariantDataSafeArray_SetValue(wxVariantDataSafeArray * self, SAFEARRAY * value);
bool wxVariantDataSafeArray_GetAsAny(const wxVariantDataSafeArray * self, wxAny * any);

// CLASS: wxView
wxClassInfo *wxView_CLASSINFO();
wxView *wxView_new();
void wxView_Activate(wxView * self, bool activate);
bool wxView_Close(wxView * self, bool delete_window);
wxDocument * wxView_GetDocument(const wxView * self);
wxDocManager * wxView_GetDocumentManager(const wxView * self);
wxWindow * wxView_GetFrame(const wxView * self);
wxString *wxView_GetViewName(const wxView * self);
void wxView_OnActivateView(wxView * self, bool activate, wxView * active_view, wxView * deactive_view);
void wxView_OnChangeFilename(wxView * self);
bool wxView_OnClose(wxView * self, bool delete_window);
void wxView_OnClosingDocument(wxView * self);
bool wxView_OnCreate(wxView * self, wxDocument * doc, long flags);
wxPrintout * wxView_OnCreatePrintout(wxView * self);
void wxView_OnDraw(wxView * self, wxDC * dc);
void wxView_OnUpdate(wxView * self, wxView * sender, wxObject * hint);
void wxView_SetDocument(wxView * self, wxDocument * doc);
void wxView_SetFrame(wxView * self, wxWindow * frame);
void wxView_SetViewName(wxView * self, const wxString * name);
// Mix-in(s) to wxView
wxTrackable *wxView_AsTrackable(wxView* obj);

// CLASS: wxWindow
wxClassInfo *wxWindow_CLASSINFO();
bool wxWindow_AcceptsFocus(const wxWindow * self);
bool wxWindow_AcceptsFocusFromKeyboard(const wxWindow * self);
bool wxWindow_AcceptsFocusRecursively(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_DisableFocusFromKeyboard(wxWindow * self);
#endif
bool wxWindow_IsFocusable(const wxWindow * self);
bool wxWindow_CanAcceptFocus(const wxWindow * self);
bool wxWindow_CanAcceptFocusFromKeyboard(const wxWindow * self);
bool wxWindow_HasFocus(const wxWindow * self);
void wxWindow_SetCanFocus(wxWindow * self, bool can_focus);
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_EnableVisibleFocus(wxWindow * self, bool enable);
#endif
void wxWindow_SetFocus(wxWindow * self);
void wxWindow_SetFocusFromKbd(wxWindow * self);
void wxWindow_AddChild(wxWindow * self, wxWindow * child);
bool wxWindow_DestroyChildren(wxWindow * self);
wxWindow * wxWindow_FindWindow(const wxWindow * self, long id);
wxWindow * wxWindow_FindWindow1(const wxWindow * self, const wxString * name);
wxWindowList *wxWindow_GetChildren1(const wxWindow * self);
void wxWindow_RemoveChild(wxWindow * self, wxWindow * child);
wxWindow * wxWindow_GetGrandParent(const wxWindow * self);
wxWindow * wxWindow_GetNextSibling(const wxWindow * self);
wxWindow * wxWindow_GetParent(const wxWindow * self);
wxWindow * wxWindow_GetPrevSibling(const wxWindow * self);
bool wxWindow_IsDescendant(const wxWindow * self, wxWindow * win);
bool wxWindow_Reparent(wxWindow * self, wxWindow * new_parent);
void wxWindow_AlwaysShowScrollbars(wxWindow * self, bool hflag, bool vflag);
int wxWindow_GetScrollPos(const wxWindow * self, int orientation);
int wxWindow_GetScrollRange(const wxWindow * self, int orientation);
int wxWindow_GetScrollThumb(const wxWindow * self, int orientation);
bool wxWindow_CanScroll(const wxWindow * self, int orient);
bool wxWindow_HasScrollbar(const wxWindow * self, int orient);
bool wxWindow_IsScrollbarAlwaysShown(const wxWindow * self, int orient);
bool wxWindow_ScrollLines(wxWindow * self, int lines);
bool wxWindow_ScrollPages(wxWindow * self, int pages);
void wxWindow_ScrollWindow(wxWindow * self, int dx, int dy, const wxRect * rect);
bool wxWindow_LineUp(wxWindow * self);
bool wxWindow_LineDown(wxWindow * self);
bool wxWindow_PageUp(wxWindow * self);
bool wxWindow_PageDown(wxWindow * self);
void wxWindow_SetScrollPos(wxWindow * self, int orientation, int pos, bool refresh);
void wxWindow_SetScrollbar(wxWindow * self, int orientation, int position, int thumb_size, int range, bool refresh);
bool wxWindow_BeginRepositioningChildren(wxWindow * self);
void wxWindow_EndRepositioningChildren(wxWindow * self);
void wxWindow_CacheBestSize(const wxWindow * self, const wxSize * size);
wxSize *wxWindow_ClientToWindowSize(const wxWindow * self, const wxSize * size);
wxSize *wxWindow_WindowToClientSize(const wxWindow * self, const wxSize * size);
void wxWindow_Fit(wxWindow * self);
void wxWindow_FitInside(wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_FromDIP1(const wxWindow * self, const wxPoint * pt);
int wxWindow_FromDIP2(const wxWindow * self, int d);
wxSize *wxWindow_ToDIP(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ToDIP1(const wxWindow * self, const wxPoint * pt);
int wxWindow_ToDIP2(const wxWindow * self, int d);
wxSize *wxWindow_FromPhys(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_FromPhys1(const wxWindow * self, const wxPoint * pt);
int wxWindow_FromPhys2(const wxWindow * self, int d);
wxSize *wxWindow_ToPhys(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ToPhys1(const wxWindow * self, const wxPoint * pt);
int wxWindow_ToPhys2(const wxWindow * self, int d);
#endif
wxSize *wxWindow_GetBestSize(const wxWindow * self);
int wxWindow_GetBestHeight(const wxWindow * self, int width);
int wxWindow_GetBestWidth(const wxWindow * self, int height);
void wxWindow_GetClientSize(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetClientSize1(const wxWindow * self);
wxSize *wxWindow_GetEffectiveMinSize(const wxWindow * self);
wxSize *wxWindow_GetMaxClientSize(const wxWindow * self);
wxSize *wxWindow_GetMaxSize(const wxWindow * self);
wxSize *wxWindow_GetMinClientSize(const wxWindow * self);
wxSize *wxWindow_GetMinSize(const wxWindow * self);
int wxWindow_GetMinWidth(const wxWindow * self);
int wxWindow_GetMinHeight(const wxWindow * self);
int wxWindow_GetMaxWidth(const wxWindow * self);
int wxWindow_GetMaxHeight(const wxWindow * self);
void wxWindow_GetSize(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetSize1(const wxWindow * self);
wxSize *wxWindow_GetVirtualSize(const wxWindow * self);
void wxWindow_GetVirtualSize1(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetBestVirtualSize(const wxWindow * self);
double wxWindow_GetContentScaleFactor(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxWindow_GetDPIScaleFactor(const wxWindow * self);
#endif
wxSize *wxWindow_GetWindowBorderSize(const wxWindow * self);
bool wxWindow_InformFirstDirection(wxWindow * self, int direction, int size, int available_other_dir);
void wxWindow_InvalidateBestSize(wxWindow * self);
void wxWindow_PostSizeEvent(wxWindow * self);
void wxWindow_PostSizeEventToParent(wxWindow * self);
void wxWindow_SendSizeEvent(wxWindow * self, int flags);
void wxWindow_SendSizeEventToParent(wxWindow * self, int flags);
void wxWindow_SetClientSize(wxWindow * self, int width, int height);
void wxWindow_SetClientSize1(wxWindow * self, const wxSize * size);
void wxWindow_SetClientSize2(wxWindow * self, const wxRect * rect);
void wxWindow_SetContainingSizer(wxWindow * self, wxSizer * sizer);
void wxWindow_SetInitialSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMaxClientSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMaxSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMinClientSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMinSize(wxWindow * self, const wxSize * size);
void wxWindow_SetSize(wxWindow * self, int x, int y, int width, int height, int size_flags);
void wxWindow_SetSize1(wxWindow * self, const wxRect * rect);
void wxWindow_SetSize2(wxWindow * self, const wxSize * size);
void wxWindow_SetSize3(wxWindow * self, int width, int height);
void wxWindow_SetSizeHints(wxWindow * self, const wxSize * min_size, const wxSize * max_size, const wxSize * inc_size);
void wxWindow_SetSizeHints1(wxWindow * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h);
void wxWindow_SetVirtualSize(wxWindow * self, int width, int height);
void wxWindow_SetVirtualSize1(wxWindow * self, const wxSize * size);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_FromDIP4(const wxPoint * pt, const wxWindow * w);
int wxWindow_FromDIP5(int d, const wxWindow * w);
wxSize *wxWindow_ToDIP3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_ToDIP4(const wxPoint * pt, const wxWindow * w);
int wxWindow_ToDIP5(int d, const wxWindow * w);
wxSize *wxWindow_FromPhys3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_FromPhys4(const wxPoint * pt, const wxWindow * w);
int wxWindow_FromPhys5(int d, const wxWindow * w);
wxSize *wxWindow_ToPhys3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_ToPhys4(const wxPoint * pt, const wxWindow * w);
int wxWindow_ToPhys5(int d, const wxWindow * w);
#endif
void wxWindow_Center(wxWindow * self, int dir);
void wxWindow_CenterOnParent(wxWindow * self, int dir);
void wxWindow_Centre(wxWindow * self, int direction);
void wxWindow_CentreOnParent(wxWindow * self, int direction);
void wxWindow_GetPosition(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_GetPosition1(const wxWindow * self);
wxRect *wxWindow_GetRect(const wxWindow * self);
void wxWindow_GetScreenPosition(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_GetScreenPosition1(const wxWindow * self);
wxRect *wxWindow_GetScreenRect(const wxWindow * self);
wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow * self);
wxRect *wxWindow_GetClientRect(const wxWindow * self);
void wxWindow_Move(wxWindow * self, int x, int y, int flags);
void wxWindow_Move1(wxWindow * self, const wxPoint * pt, int flags);
void wxWindow_SetPosition(wxWindow * self, const wxPoint * pt);
void wxWindow_ClientToScreen(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_ClientToScreen1(const wxWindow * self, const wxPoint * pt);
wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow * self, const wxPoint * pt);
wxSize *wxWindow_ConvertDialogToPixels1(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow * self, const wxPoint * pt);
wxSize *wxWindow_ConvertPixelsToDialog1(const wxWindow * self, const wxSize * sz);
void wxWindow_ScreenToClient(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_ScreenToClient1(const wxWindow * self, const wxPoint * pt);
void wxWindow_ClearBackground(wxWindow * self);
void wxWindow_Freeze(wxWindow * self);
void wxWindow_Thaw(wxWindow * self);
bool wxWindow_IsFrozen(const wxWindow * self);
wxColour *wxWindow_GetBackgroundColour(const wxWindow * self);
int wxWindow_GetCharHeight(const wxWindow * self);
int wxWindow_GetCharWidth(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_GetDPI(const wxWindow * self);
#endif
wxFont *wxWindow_GetFont(const wxWindow * self);
wxColour *wxWindow_GetForegroundColour(const wxWindow * self);
void wxWindow_GetTextExtent(const wxWindow * self, const wxString * string, int * w, int * h, int * descent, int * external_leading, const wxFont * font);
wxSize *wxWindow_GetTextExtent1(const wxWindow * self, const wxString * string);
wxRect *wxWindow_GetUpdateClientRect(const wxWindow * self);
bool wxWindow_HasTransparentBackground(wxWindow * self);
void wxWindow_Refresh(wxWindow * self, bool erase_background, const wxRect * rect);
void wxWindow_RefreshRect(wxWindow * self, const wxRect * rect, bool erase_background);
void wxWindow_Update(wxWindow * self);
bool wxWindow_SetBackgroundColour(wxWindow * self, const wxColour * colour);
bool wxWindow_IsTransparentBackgroundSupported(const wxWindow * self, wxString * reason);
bool wxWindow_SetFont(wxWindow * self, const wxFont * font);
bool wxWindow_SetForegroundColour(wxWindow * self, const wxColour * colour);
void wxWindow_SetOwnBackgroundColour(wxWindow * self, const wxColour * colour);
bool wxWindow_InheritsBackgroundColour(const wxWindow * self);
bool wxWindow_UseBgCol(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseBackgroundColour(const wxWindow * self);
#endif
void wxWindow_SetOwnFont(wxWindow * self, const wxFont * font);
void wxWindow_SetOwnForegroundColour(wxWindow * self, const wxColour * colour);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseForegroundColour(const wxWindow * self);
bool wxWindow_InheritsForegroundColour(const wxWindow * self);
#endif
void wxWindow_SetPalette(wxWindow * self, const wxPalette * pal);
bool wxWindow_ShouldInheritColours(const wxWindow * self);
void wxWindow_SetThemeEnabled(wxWindow * self, bool enable);
bool wxWindow_GetThemeEnabled(const wxWindow * self);
bool wxWindow_CanSetTransparent(wxWindow * self);
bool wxWindow_SetTransparent(wxWindow * self, wxByte alpha);
wxEvtHandler * wxWindow_GetEventHandler(const wxWindow * self);
bool wxWindow_HandleAsNavigationKey(wxWindow * self, const wxKeyEvent * event);
bool wxWindow_HandleWindowEvent(const wxWindow * self, wxEvent * event);
bool wxWindow_ProcessWindowEvent(wxWindow * self, wxEvent * event);
bool wxWindow_ProcessWindowEventLocally(wxWindow * self, wxEvent * event);
wxEvtHandler * wxWindow_PopEventHandler(wxWindow * self, bool delete_handler);
void wxWindow_PushEventHandler(wxWindow * self, wxEvtHandler * handler);
bool wxWindow_RemoveEventHandler(wxWindow * self, wxEvtHandler * handler);
void wxWindow_SetEventHandler(wxWindow * self, wxEvtHandler * handler);
long wxWindow_GetExtraStyle(const wxWindow * self);
long wxWindow_GetWindowStyleFlag(const wxWindow * self);
long wxWindow_GetWindowStyle(const wxWindow * self);
bool wxWindow_HasExtraStyle(const wxWindow * self, int ex_flag);
bool wxWindow_HasFlag(const wxWindow * self, int flag);
void wxWindow_SetExtraStyle(wxWindow * self, long ex_style);
void wxWindow_SetWindowStyleFlag(wxWindow * self, long style);
void wxWindow_SetWindowStyle(wxWindow * self, long style);
bool wxWindow_ToggleWindowStyle(wxWindow * self, int flag);
void wxWindow_MoveAfterInTabOrder(wxWindow * self, wxWindow * win);
void wxWindow_MoveBeforeInTabOrder(wxWindow * self, wxWindow * win);
bool wxWindow_Navigate(wxWindow * self, int flags);
bool wxWindow_NavigateIn(wxWindow * self, int flags);
void wxWindow_Lower(wxWindow * self);
void wxWindow_Raise(wxWindow * self);
bool wxWindow_Hide(wxWindow * self);
bool wxWindow_IsEnabled(const wxWindow * self);
bool wxWindow_IsExposed(const wxWindow * self, int x, int y);
bool wxWindow_IsExposed1(const wxWindow * self, wxPoint * pt);
bool wxWindow_IsExposed2(const wxWindow * self, int x, int y, int w, int h);
bool wxWindow_IsExposed3(const wxWindow * self, wxRect * rect);
bool wxWindow_IsShown(const wxWindow * self);
bool wxWindow_IsShownOnScreen(const wxWindow * self);
bool wxWindow_Disable(wxWindow * self);
bool wxWindow_Enable(wxWindow * self, bool enable);
bool wxWindow_Show(wxWindow * self, bool show);
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxWindow_GetHelpText(const wxWindow * self);
#endif
void wxWindow_SetHelpText(wxWindow * self, const wxString * help_text);
wxToolTip * wxWindow_GetToolTip(const wxWindow * self);
wxString *wxWindow_GetToolTipText(const wxWindow * self);
void wxWindow_SetToolTip(wxWindow * self, const wxString * tip_string);
void wxWindow_SetToolTip1(wxWindow * self, wxToolTip * tip);
void wxWindow_UnsetToolTip(wxWindow * self);
int wxWindow_GetPopupMenuSelectionFromUser(wxWindow * self, wxMenu * menu, const wxPoint * pos);
int wxWindow_GetPopupMenuSelectionFromUser1(wxWindow * self, wxMenu * menu, int x, int y);
bool wxWindow_PopupMenu(wxWindow * self, wxMenu * menu, const wxPoint * pos);
bool wxWindow_PopupMenu1(wxWindow * self, wxMenu * menu, int x, int y);
wxValidator * wxWindow_GetValidator(wxWindow * self);
void wxWindow_SetValidator(wxWindow * self, const wxValidator * validator);
bool wxWindow_TransferDataFromWindow(wxWindow * self);
bool wxWindow_TransferDataToWindow(wxWindow * self);
bool wxWindow_Validate(wxWindow * self);
wxWindowID wxWindow_GetId(const wxWindow * self);
wxString *wxWindow_GetLabel(const wxWindow * self);
wxLayoutDirection wxWindow_GetLayoutDirection(const wxWindow * self);
wxCoord wxWindow_AdjustForLayoutDirection(const wxWindow * self, wxCoord x, wxCoord width, wxCoord width_total);
wxString *wxWindow_GetName(const wxWindow * self);
void wxWindow_SetId(wxWindow * self, wxWindowID winid);
void wxWindow_SetLabel(wxWindow * self, const wxString * label);
void wxWindow_SetLayoutDirection(wxWindow * self, wxLayoutDirection dir);
void wxWindow_SetName(wxWindow * self, const wxString * name);
wxAcceleratorTable * wxWindow_GetAcceleratorTable(wxWindow * self);
void wxWindow_SetAcceleratorTable(wxWindow * self, const wxAcceleratorTable * accel);
bool wxWindow_Close(wxWindow * self, bool force);
bool wxWindow_Destroy(wxWindow * self);
bool wxWindow_IsBeingDeleted(const wxWindow * self);
wxDropTarget * wxWindow_GetDropTarget(const wxWindow * self);
void wxWindow_SetDropTarget(wxWindow * self, wxDropTarget * target);
void wxWindow_DragAcceptFiles(wxWindow * self, bool accept);
wxSizer * wxWindow_GetContainingSizer(const wxWindow * self);
wxSizer * wxWindow_GetSizer(const wxWindow * self);
void wxWindow_SetSizer(wxWindow * self, wxSizer * sizer, bool delete_old);
void wxWindow_SetSizerAndFit(wxWindow * self, wxSizer * sizer, bool delete_old);
wxLayoutConstraints * wxWindow_GetConstraints(const wxWindow * self);
void wxWindow_SetConstraints(wxWindow * self, wxLayoutConstraints * constraints);
bool wxWindow_Layout(wxWindow * self);
void wxWindow_SetAutoLayout(wxWindow * self, bool auto_layout);
bool wxWindow_GetAutoLayout(const wxWindow * self);
void wxWindow_CaptureMouse(wxWindow * self);
wxCaret * wxWindow_GetCaret(const wxWindow * self);
bool wxWindow_HasCapture(const wxWindow * self);
void wxWindow_ReleaseMouse(wxWindow * self);
void wxWindow_SetCaret(wxWindow * self, wxCaret * caret);
bool wxWindow_SetCursor(wxWindow * self, const wxCursor * cursor);
void wxWindow_WarpPointer(wxWindow * self, int x, int y);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_EnableTouchEvents(wxWindow * self, int events_mask);
#endif
void wxWindow_DoUpdateWindowUI(wxWindow * self, wxUpdateUIEvent * event);
bool wxWindow_HasMultiplePages(const wxWindow * self);
void wxWindow_InheritAttributes(wxWindow * self);
void wxWindow_InitDialog(wxWindow * self);
bool wxWindow_IsDoubleBuffered(const wxWindow * self);
void wxWindow_SetDoubleBuffered(wxWindow * self, bool on);
bool wxWindow_IsRetained(const wxWindow * self);
bool wxWindow_IsThisEnabled(const wxWindow * self);
bool wxWindow_IsTopLevel(const wxWindow * self);
void wxWindow_OnInternalIdle(wxWindow * self);
bool wxWindow_SendIdleEvents(wxWindow * self, wxIdleEvent * event);
#ifndef __WXGTK__
bool wxWindow_RegisterHotKey(wxWindow * self, int hotkey_id, int modifiers, int virtual_key_code);
bool wxWindow_UnregisterHotKey(wxWindow * self, int hotkey_id);
#endif
void wxWindow_UpdateWindowUI(wxWindow * self, long flags);
wxWindow * wxWindow_FindFocus();
wxWindow * wxWindow_FindWindowById(long id, const wxWindow * parent);
wxWindow * wxWindow_FindWindowByLabel(const wxString * label, const wxWindow * parent);
wxWindow * wxWindow_FindWindowByName(const wxString * name, const wxWindow * parent);
wxWindow * wxWindow_GetCapture();
wxWindowID wxWindow_NewControlId(int count);
void wxWindow_UnreserveControlId(wxWindowID id, int count);
wxWindow *wxWindow_new();
wxWindow *wxWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxWindow_Create(wxWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
// Mix-in(s) to wxWindow
wxTrackable *wxWindow_AsTrackable(wxWindow* obj);

// CLASS: wxWindowCreateEvent
wxClassInfo *wxWindowCreateEvent_CLASSINFO();
wxWindowCreateEvent *wxWindowCreateEvent_new(wxWindow * win);
wxWindow * wxWindowCreateEvent_GetWindow(const wxWindowCreateEvent * self);

// CLASS: wxWindowDC
wxClassInfo *wxWindowDC_CLASSINFO();
wxWindowDC *wxWindowDC_new(wxWindow * window);

// CLASS: wxWindowDestroyEvent
wxClassInfo *wxWindowDestroyEvent_CLASSINFO();
wxWindowDestroyEvent *wxWindowDestroyEvent_new(wxWindow * win);
wxWindow * wxWindowDestroyEvent_GetWindow(const wxWindowDestroyEvent * self);

// CLASS: wxWindowDisabler
void wxWindowDisabler_delete(wxWindowDisabler *self);
wxWindowDisabler *wxWindowDisabler_new(bool disable);
wxWindowDisabler *wxWindowDisabler_new1(wxWindow * win_to_skip, wxWindow * win_to_skip2);

// CLASS: wxWizard
wxClassInfo *wxWizard_CLASSINFO();
wxWizard *wxWizard_new();
wxWizard *wxWizard_new1(wxWindow * parent, int id, const wxString * title, const wxBitmapBundle * bitmap, const wxPoint * pos, long style);
bool wxWizard_Create(wxWizard * self, wxWindow * parent, int id, const wxString * title, const wxBitmapBundle * bitmap, const wxPoint * pos, long style);
void wxWizard_FitToPage(wxWizard * self, const wxWizardPage * first_page);
wxBitmap *wxWizard_GetBitmap(const wxWizard * self);
wxColour *wxWizard_GetBitmapBackgroundColour(const wxWizard * self);
int wxWizard_GetBitmapPlacement(const wxWizard * self);
wxWizardPage * wxWizard_GetCurrentPage(const wxWizard * self);
int wxWizard_GetMinimumBitmapWidth(const wxWizard * self);
wxSizer * wxWizard_GetPageAreaSizer(const wxWizard * self);
wxSize *wxWizard_GetPageSize(const wxWizard * self);
bool wxWizard_HasNextPage(wxWizard * self, wxWizardPage * page);
bool wxWizard_HasPrevPage(wxWizard * self, wxWizardPage * page);
bool wxWizard_RunWizard(wxWizard * self, wxWizardPage * first_page);
void wxWizard_SetBitmap(wxWizard * self, const wxBitmapBundle * bitmap);
void wxWizard_SetBitmapBackgroundColour(wxWizard * self, const wxColour * colour);
void wxWizard_SetBitmapPlacement(wxWizard * self, int placement);
void wxWizard_SetBorder(wxWizard * self, int border);
void wxWizard_SetMinimumBitmapWidth(wxWizard * self, int width);
void wxWizard_SetPageSize(wxWizard * self, const wxSize * size_page);
// Mix-in(s) to wxWizard
wxTrackable *wxWizard_AsTrackable(wxWizard* obj);

// CLASS: wxWizardEvent
wxClassInfo *wxWizardEvent_CLASSINFO();
bool wxWizardEvent_GetDirection(const wxWizardEvent * self);
wxWizardPage * wxWizardEvent_GetPage(const wxWizardEvent * self);

// CLASS: wxWizardPage
wxClassInfo *wxWizardPage_CLASSINFO();
wxWizardPage *wxWizardPage_new();
wxWizardPage *wxWizardPage_new1(wxWizard * parent, const wxBitmapBundle * bitmap);
bool wxWizardPage_Create(wxWizardPage * self, wxWizard * parent, const wxBitmapBundle * bitmap);
wxBitmap *wxWizardPage_GetBitmap(const wxWizardPage * self);
wxWizardPage * wxWizardPage_GetNext(const wxWizardPage * self);
wxWizardPage * wxWizardPage_GetPrev(const wxWizardPage * self);
// Mix-in(s) to wxWizardPage
wxTrackable *wxWizardPage_AsTrackable(wxWizardPage* obj);

// CLASS: wxWizardPageSimple
wxClassInfo *wxWizardPageSimple_CLASSINFO();
wxWizardPageSimple *wxWizardPageSimple_new();
wxWizardPageSimple *wxWizardPageSimple_new1(wxWizard * parent, wxWizardPage * prev, wxWizardPage * next, const wxBitmapBundle * bitmap);
bool wxWizardPageSimple_Create(wxWizardPageSimple * self, wxWizard * parent, wxWizardPage * prev, wxWizardPage * next, const wxBitmapBundle * bitmap);
wxWizardPageSimple * wxWizardPageSimple_Chain(wxWizardPageSimple * self, wxWizardPageSimple * next);
void wxWizardPageSimple_SetNext(wxWizardPageSimple * self, wxWizardPage * next);
void wxWizardPageSimple_SetPrev(wxWizardPageSimple * self, wxWizardPage * prev);
void wxWizardPageSimple_Chain1(wxWizardPageSimple * first, wxWizardPageSimple * second);
// Mix-in(s) to wxWizardPageSimple
wxTrackable *wxWizardPageSimple_AsTrackable(wxWizardPageSimple* obj);

// CLASS: wxWrapSizer
wxClassInfo *wxWrapSizer_CLASSINFO();
wxWrapSizer *wxWrapSizer_new(int orient, int flags);

// CLASS: wxXPMHandler
wxClassInfo *wxXPMHandler_CLASSINFO();
wxXPMHandler *wxXPMHandler_new();

// CLASS: wxZoomGestureEvent
wxClassInfo *wxZoomGestureEvent_CLASSINFO();
wxZoomGestureEvent *wxZoomGestureEvent_new(wxWindowID windid);
double wxZoomGestureEvent_GetZoomFactor(const wxZoomGestureEvent * self);
void wxZoomGestureEvent_SetZoomFactor(wxZoomGestureEvent * self, double zoom_factor);

} // extern "C"

