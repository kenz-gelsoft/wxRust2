#include "generated.h"

extern "C" {

// CLASS: wxAboutDialogInfo
void wxAboutDialogInfo_delete(wxAboutDialogInfo *self) {
    delete self;
}
wxAboutDialogInfo *wxAboutDialogInfo_new() {
    return new wxAboutDialogInfo();
}
void wxAboutDialogInfo_AddArtist(wxAboutDialogInfo * self, const wxString * artist) {
    return self->AddArtist(*artist);
}
void wxAboutDialogInfo_AddDeveloper(wxAboutDialogInfo * self, const wxString * developer) {
    return self->AddDeveloper(*developer);
}
void wxAboutDialogInfo_AddDocWriter(wxAboutDialogInfo * self, const wxString * docwriter) {
    return self->AddDocWriter(*docwriter);
}
void wxAboutDialogInfo_AddTranslator(wxAboutDialogInfo * self, const wxString * translator) {
    return self->AddTranslator(*translator);
}
wxString *wxAboutDialogInfo_GetName(const wxAboutDialogInfo * self) {
    return new wxString(self->GetName());
}
bool wxAboutDialogInfo_HasDescription(const wxAboutDialogInfo * self) {
    return self->HasDescription();
}
wxString *wxAboutDialogInfo_GetDescription(wxAboutDialogInfo * self) {
    return new wxString(self->GetDescription());
}
bool wxAboutDialogInfo_HasCopyright(const wxAboutDialogInfo * self) {
    return self->HasCopyright();
}
wxString *wxAboutDialogInfo_GetCopyright(const wxAboutDialogInfo * self) {
    return new wxString(self->GetCopyright());
}
void wxAboutDialogInfo_SetArtists(wxAboutDialogInfo * self, const wxArrayString * artists) {
    return self->SetArtists(*artists);
}
void wxAboutDialogInfo_SetCopyright(wxAboutDialogInfo * self, const wxString * copyright) {
    return self->SetCopyright(*copyright);
}
void wxAboutDialogInfo_SetDescription(wxAboutDialogInfo * self, const wxString * desc) {
    return self->SetDescription(*desc);
}
void wxAboutDialogInfo_SetDevelopers(wxAboutDialogInfo * self, const wxArrayString * developers) {
    return self->SetDevelopers(*developers);
}
void wxAboutDialogInfo_SetDocWriters(wxAboutDialogInfo * self, const wxArrayString * docwriters) {
    return self->SetDocWriters(*docwriters);
}
bool wxAboutDialogInfo_HasIcon(const wxAboutDialogInfo * self) {
    return self->HasIcon();
}
wxIcon *wxAboutDialogInfo_GetIcon(const wxAboutDialogInfo * self) {
    return new wxIcon(self->GetIcon());
}
void wxAboutDialogInfo_SetIcon(wxAboutDialogInfo * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
bool wxAboutDialogInfo_HasLicence(const wxAboutDialogInfo * self) {
    return self->HasLicence();
}
wxString *wxAboutDialogInfo_GetLicence(const wxAboutDialogInfo * self) {
    return new wxString(self->GetLicence());
}
void wxAboutDialogInfo_SetLicence(wxAboutDialogInfo * self, const wxString * licence) {
    return self->SetLicence(*licence);
}
void wxAboutDialogInfo_SetLicense(wxAboutDialogInfo * self, const wxString * licence) {
    return self->SetLicense(*licence);
}
void wxAboutDialogInfo_SetName(wxAboutDialogInfo * self, const wxString * name) {
    return self->SetName(*name);
}
void wxAboutDialogInfo_SetTranslators(wxAboutDialogInfo * self, const wxArrayString * translators) {
    return self->SetTranslators(*translators);
}
void wxAboutDialogInfo_SetVersion(wxAboutDialogInfo * self, const wxString * version, const wxString * long_version) {
    return self->SetVersion(*version, *long_version);
}
wxString *wxAboutDialogInfo_GetVersion(const wxAboutDialogInfo * self) {
    return new wxString(self->GetVersion());
}
wxString *wxAboutDialogInfo_GetLongVersion(const wxAboutDialogInfo * self) {
    return new wxString(self->GetLongVersion());
}
bool wxAboutDialogInfo_HasWebSite(const wxAboutDialogInfo * self) {
    return self->HasWebSite();
}
wxString *wxAboutDialogInfo_GetWebSiteURL(const wxAboutDialogInfo * self) {
    return new wxString(self->GetWebSiteURL());
}
wxString *wxAboutDialogInfo_GetWebSiteDescription(const wxAboutDialogInfo * self) {
    return new wxString(self->GetWebSiteDescription());
}
void wxAboutDialogInfo_SetWebSite(wxAboutDialogInfo * self, const wxString * url, const wxString * desc) {
    return self->SetWebSite(*url, *desc);
}
bool wxAboutDialogInfo_HasDevelopers(const wxAboutDialogInfo * self) {
    return self->HasDevelopers();
}
wxArrayString *wxAboutDialogInfo_GetDevelopers(const wxAboutDialogInfo * self) {
    return new wxArrayString(self->GetDevelopers());
}
bool wxAboutDialogInfo_HasDocWriters(const wxAboutDialogInfo * self) {
    return self->HasDocWriters();
}
wxArrayString *wxAboutDialogInfo_GetDocWriters(const wxAboutDialogInfo * self) {
    return new wxArrayString(self->GetDocWriters());
}
bool wxAboutDialogInfo_HasArtists(const wxAboutDialogInfo * self) {
    return self->HasArtists();
}
wxArrayString *wxAboutDialogInfo_GetArtists(const wxAboutDialogInfo * self) {
    return new wxArrayString(self->GetArtists());
}
bool wxAboutDialogInfo_HasTranslators(const wxAboutDialogInfo * self) {
    return self->HasTranslators();
}
wxArrayString *wxAboutDialogInfo_GetTranslators(const wxAboutDialogInfo * self) {
    return new wxArrayString(self->GetTranslators());
}

// CLASS: wxAcceleratorEntry
void wxAcceleratorEntry_delete(wxAcceleratorEntry *self) {
    delete self;
}
wxAcceleratorEntry *wxAcceleratorEntry_new(int flags, int key_code, int cmd, wxMenuItem * item) {
    return new wxAcceleratorEntry(flags, key_code, cmd, item);
}
wxAcceleratorEntry *wxAcceleratorEntry_new1(const wxAcceleratorEntry * entry) {
    return new wxAcceleratorEntry(*entry);
}
int wxAcceleratorEntry_GetCommand(const wxAcceleratorEntry * self) {
    return self->GetCommand();
}
int wxAcceleratorEntry_GetFlags(const wxAcceleratorEntry * self) {
    return self->GetFlags();
}
int wxAcceleratorEntry_GetKeyCode(const wxAcceleratorEntry * self) {
    return self->GetKeyCode();
}
wxMenuItem * wxAcceleratorEntry_GetMenuItem(const wxAcceleratorEntry * self) {
    return self->GetMenuItem();
}
void wxAcceleratorEntry_Set(wxAcceleratorEntry * self, int flags, int key_code, int cmd, wxMenuItem * item) {
    return self->Set(flags, key_code, cmd, item);
}
bool wxAcceleratorEntry_IsOk(const wxAcceleratorEntry * self) {
    return self->IsOk();
}
wxString *wxAcceleratorEntry_ToString(const wxAcceleratorEntry * self) {
    return new wxString(self->ToString());
}
wxString *wxAcceleratorEntry_ToRawString(const wxAcceleratorEntry * self) {
    return new wxString(self->ToRawString());
}
bool wxAcceleratorEntry_FromString(wxAcceleratorEntry * self, const wxString * str) {
    return self->FromString(*str);
}

// CLASS: wxAcceleratorTable
wxClassInfo *wxAcceleratorTable_CLASSINFO() {
    return wxCLASSINFO(wxAcceleratorTable);
}
wxAcceleratorTable *wxAcceleratorTable_new() {
    return new wxAcceleratorTable();
}
wxAcceleratorTable *wxAcceleratorTable_new2(const wxString * resource) {
    return new wxAcceleratorTable(*resource);
}
bool wxAcceleratorTable_IsOk(const wxAcceleratorTable * self) {
    return self->IsOk();
}

// CLASS: wxAccessible
wxClassInfo *wxAccessible_CLASSINFO() {
    return wxCLASSINFO(wxAccessible);
}
wxAccessible *wxAccessible_new(wxWindow * win) {
    return new wxAccessible(win);
}
wxWindow * wxAccessible_GetWindow(wxAccessible * self) {
    return self->GetWindow();
}
void wxAccessible_SetWindow(wxAccessible * self, wxWindow * window) {
    return self->SetWindow(window);
}

// CLASS: wxActivateEvent
wxClassInfo *wxActivateEvent_CLASSINFO() {
    return wxCLASSINFO(wxActivateEvent);
}
bool wxActivateEvent_GetActive(const wxActivateEvent * self) {
    return self->GetActive();
}

// CLASS: wxActiveXContainer
wxClassInfo *wxActiveXContainer_CLASSINFO() {
    return wxCLASSINFO(wxActiveXContainer);
}
// Mix-in(s) to wxActiveXContainer
wxTrackable *wxActiveXContainer_AsTrackable(wxActiveXContainer* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxActiveXEvent
wxClassInfo *wxActiveXEvent_CLASSINFO() {
    return wxCLASSINFO(wxActiveXEvent);
}
size_t wxActiveXEvent_ParamCount(const wxActiveXEvent * self) {
    return self->ParamCount();
}
wxString *wxActiveXEvent_ParamName(const wxActiveXEvent * self, size_t idx) {
    return new wxString(self->ParamName(idx));
}
wxString *wxActiveXEvent_ParamType(const wxActiveXEvent * self, size_t idx) {
    return new wxString(self->ParamType(idx));
}
wxActiveXEventNativeMSW * wxActiveXEvent_GetNativeParameters(const wxActiveXEvent * self) {
    return self->GetNativeParameters();
}

// CLASS: wxActivityIndicator
wxClassInfo *wxActivityIndicator_CLASSINFO() {
    return wxCLASSINFO(wxActivityIndicator);
}
wxActivityIndicator *wxActivityIndicator_new() {
    return new wxActivityIndicator();
}
wxActivityIndicator *wxActivityIndicator_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxActivityIndicator(parent, winid, *pos, *size, style, *name);
}
bool wxActivityIndicator_Create(wxActivityIndicator * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, winid, *pos, *size, style, *name);
}
void wxActivityIndicator_Start(wxActivityIndicator * self) {
    return self->Start();
}
void wxActivityIndicator_Stop(wxActivityIndicator * self) {
    return self->Stop();
}
bool wxActivityIndicator_IsRunning(const wxActivityIndicator * self) {
    return self->IsRunning();
}
// Mix-in(s) to wxActivityIndicator
wxTrackable *wxActivityIndicator_AsTrackable(wxActivityIndicator* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxAddRemoveAdaptor
void wxAddRemoveAdaptor_delete(wxAddRemoveAdaptor *self) {
    delete self;
}
wxAddRemoveAdaptor *wxAddRemoveAdaptor_new() {
    return new wxAddRemoveAdaptor();
}
wxWindow * wxAddRemoveAdaptor_GetItemsCtrl(const wxAddRemoveAdaptor * self) {
    return self->GetItemsCtrl();
}
bool wxAddRemoveAdaptor_CanAdd(const wxAddRemoveAdaptor * self) {
    return self->CanAdd();
}
bool wxAddRemoveAdaptor_CanRemove(const wxAddRemoveAdaptor * self) {
    return self->CanRemove();
}
void wxAddRemoveAdaptor_OnAdd(wxAddRemoveAdaptor * self) {
    return self->OnAdd();
}
void wxAddRemoveAdaptor_OnRemove(wxAddRemoveAdaptor * self) {
    return self->OnRemove();
}

// CLASS: wxAddRemoveCtrl
wxClassInfo *wxAddRemoveCtrl_CLASSINFO() {
    return wxCLASSINFO(wxAddRemoveCtrl);
}
wxAddRemoveCtrl *wxAddRemoveCtrl_new() {
    return new wxAddRemoveCtrl();
}
wxAddRemoveCtrl *wxAddRemoveCtrl_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxAddRemoveCtrl(parent, winid, *pos, *size, style, *name);
}
bool wxAddRemoveCtrl_Create(wxAddRemoveCtrl * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, winid, *pos, *size, style, *name);
}
void wxAddRemoveCtrl_SetAdaptor(wxAddRemoveCtrl * self, wxAddRemoveAdaptor * adaptor) {
    return self->SetAdaptor(adaptor);
}
void wxAddRemoveCtrl_SetButtonsToolTips(wxAddRemoveCtrl * self, const wxString * addtip, const wxString * removetip) {
    return self->SetButtonsToolTips(*addtip, *removetip);
}
// Mix-in(s) to wxAddRemoveCtrl
wxTrackable *wxAddRemoveCtrl_AsTrackable(wxAddRemoveCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxAffineMatrix2D
void wxAffineMatrix2D_delete(wxAffineMatrix2D *self) {
    delete self;
}
wxAffineMatrix2D *wxAffineMatrix2D_new() {
    return new wxAffineMatrix2D();
}
void wxAffineMatrix2D_IsEqual(wxAffineMatrix2D * self, const wxAffineMatrix2DBase * t) {
    return self->IsEqual(*t);
}
void wxAffineMatrix2D_Mirror(wxAffineMatrix2D * self, int direction) {
    return self->Mirror(direction);
}
wxPoint2DDouble *wxAffineMatrix2D_TransformPoint(const wxAffineMatrix2D * self, const wxPoint2DDouble * p) {
    return new wxPoint2DDouble(self->TransformPoint(*p));
}
void wxAffineMatrix2D_TransformPoint1(const wxAffineMatrix2D * self, wxDouble * x, wxDouble * y) {
    return self->TransformPoint(x, y);
}
wxPoint2DDouble *wxAffineMatrix2D_TransformDistance(const wxAffineMatrix2D * self, const wxPoint2DDouble * p) {
    return new wxPoint2DDouble(self->TransformDistance(*p));
}
void wxAffineMatrix2D_TransformDistance1(const wxAffineMatrix2D * self, wxDouble * dx, wxDouble * dy) {
    return self->TransformDistance(dx, dy);
}

// CLASS: wxAffineMatrix2DBase
void wxAffineMatrix2DBase_delete(wxAffineMatrix2DBase *self) {
    delete self;
}
wxAffineMatrix2DBase *wxAffineMatrix2DBase_new() {
    return new wxAffineMatrix2DBase();
}
void wxAffineMatrix2DBase_Set(wxAffineMatrix2DBase * self, const wxMatrix2D * mat2_d, const wxPoint2DDouble * tr) {
    return self->Set(*mat2_d, *tr);
}
void wxAffineMatrix2DBase_Get(const wxAffineMatrix2DBase * self, wxMatrix2D * mat2_d, wxPoint2DDouble * tr) {
    return self->Get(mat2_d, tr);
}
void wxAffineMatrix2DBase_Concat(wxAffineMatrix2DBase * self, const wxAffineMatrix2DBase * t) {
    return self->Concat(*t);
}
bool wxAffineMatrix2DBase_Invert(wxAffineMatrix2DBase * self) {
    return self->Invert();
}
bool wxAffineMatrix2DBase_IsIdentity(const wxAffineMatrix2DBase * self) {
    return self->IsIdentity();
}
bool wxAffineMatrix2DBase_IsEqual(const wxAffineMatrix2DBase * self, const wxAffineMatrix2DBase * t) {
    return self->IsEqual(*t);
}
void wxAffineMatrix2DBase_Mirror(wxAffineMatrix2DBase * self, int direction) {
    return self->Mirror(direction);
}
wxPoint2DDouble *wxAffineMatrix2DBase_TransformPoint(const wxAffineMatrix2DBase * self, const wxPoint2DDouble * p) {
    return new wxPoint2DDouble(self->TransformPoint(*p));
}
void wxAffineMatrix2DBase_TransformPoint1(const wxAffineMatrix2DBase * self, wxDouble * x, wxDouble * y) {
    return self->TransformPoint(x, y);
}
wxPoint2DDouble *wxAffineMatrix2DBase_TransformDistance(const wxAffineMatrix2DBase * self, const wxPoint2DDouble * p) {
    return new wxPoint2DDouble(self->TransformDistance(*p));
}
void wxAffineMatrix2DBase_TransformDistance1(const wxAffineMatrix2DBase * self, wxDouble * dx, wxDouble * dy) {
    return self->TransformDistance(dx, dy);
}

// CLASS: wxAnimationCtrl
wxClassInfo *wxAnimationCtrl_CLASSINFO() {
    return wxCLASSINFO(wxAnimationCtrl);
}
wxAnimationCtrl *wxAnimationCtrl_new(wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxAnimationCtrl(parent, id, *anim, *pos, *size, style, *name);
}
bool wxAnimationCtrl_Create(wxAnimationCtrl * self, wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *anim, *pos, *size, style, *name);
}
wxAnimation *wxAnimationCtrl_CreateAnimation(const wxAnimationCtrl * self) {
    return new wxAnimation(self->CreateAnimation());
}
wxAnimation *wxAnimationCtrl_GetAnimation(const wxAnimationCtrl * self) {
    return new wxAnimation(self->GetAnimation());
}
wxBitmap *wxAnimationCtrl_GetInactiveBitmap(const wxAnimationCtrl * self) {
    return new wxBitmap(self->GetInactiveBitmap());
}
bool wxAnimationCtrl_IsPlaying(const wxAnimationCtrl * self) {
    return self->IsPlaying();
}
bool wxAnimationCtrl_Play(wxAnimationCtrl * self) {
    return self->Play();
}
void wxAnimationCtrl_SetAnimation(wxAnimationCtrl * self, const wxAnimation * anim) {
    return self->SetAnimation(*anim);
}
void wxAnimationCtrl_SetInactiveBitmap(wxAnimationCtrl * self, const wxBitmapBundle * bmp) {
    return self->SetInactiveBitmap(*bmp);
}
void wxAnimationCtrl_Stop(wxAnimationCtrl * self) {
    return self->Stop();
}
wxAnimation *wxAnimationCtrl_CreateCompatibleAnimation() {
    return new wxAnimation(wxAnimationCtrl::CreateCompatibleAnimation());
}
// Mix-in(s) to wxAnimationCtrl
wxTrackable *wxAnimationCtrl_AsTrackable(wxAnimationCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

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
// Mix-in(s) to wxAnyButton
wxTrackable *wxAnyButton_AsTrackable(wxAnyButton* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxAppProgressIndicator
void wxAppProgressIndicator_delete(wxAppProgressIndicator *self) {
    delete self;
}
wxAppProgressIndicator *wxAppProgressIndicator_new(wxWindow * parent, int max_value) {
    return new wxAppProgressIndicator(parent, max_value);
}
bool wxAppProgressIndicator_IsAvailable(const wxAppProgressIndicator * self) {
    return self->IsAvailable();
}
void wxAppProgressIndicator_SetValue(wxAppProgressIndicator * self, int value) {
    return self->SetValue(value);
}
void wxAppProgressIndicator_SetRange(wxAppProgressIndicator * self, int range) {
    return self->SetRange(range);
}
bool wxAppProgressIndicator_Pulse(wxAppProgressIndicator * self) {
    return self->Pulse();
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
wxIconBundle *wxArtProvider_GetIconBundle(const wxArtID * id, const wxArtClient * client) {
    return new wxIconBundle(wxArtProvider::GetIconBundle(*id, *client));
}
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

// CLASS: wxAutoBufferedPaintDC
wxClassInfo *wxAutoBufferedPaintDC_CLASSINFO() {
    return wxCLASSINFO(wxAutoBufferedPaintDC);
}
wxAutoBufferedPaintDC *wxAutoBufferedPaintDC_new(wxWindow * window) {
    return new wxAutoBufferedPaintDC(window);
}

// CLASS: wxAutomationObject
wxClassInfo *wxAutomationObject_CLASSINFO() {
    return wxCLASSINFO(wxAutomationObject);
}
wxAutomationObject *wxAutomationObject_new(WXIDISPATCH * dispatch_ptr) {
    return new wxAutomationObject(dispatch_ptr);
}
bool wxAutomationObject_CreateInstance(const wxAutomationObject * self, const wxString * prog_id) {
    return self->CreateInstance(*prog_id);
}
bool wxAutomationObject_IsOk(const wxAutomationObject * self) {
    return self->IsOk();
}
WXIDISPATCH * wxAutomationObject_GetDispatchPtr(const wxAutomationObject * self) {
    return self->GetDispatchPtr();
}
bool wxAutomationObject_GetInstance(const wxAutomationObject * self, const wxString * prog_id, int flags) {
    return self->GetInstance(*prog_id, flags);
}
void wxAutomationObject_SetDispatchPtr(wxAutomationObject * self, WXIDISPATCH * dispatch_ptr) {
    return self->SetDispatchPtr(dispatch_ptr);
}
long wxAutomationObject_GetConvertVariantFlags(const wxAutomationObject * self) {
    return self->GetConvertVariantFlags();
}
void wxAutomationObject_SetConvertVariantFlags(wxAutomationObject * self, long flags) {
    return self->SetConvertVariantFlags(flags);
}

// CLASS: wxBannerWindow
wxClassInfo *wxBannerWindow_CLASSINFO() {
    return wxCLASSINFO(wxBannerWindow);
}
wxBannerWindow *wxBannerWindow_new() {
    return new wxBannerWindow();
}
wxBannerWindow *wxBannerWindow_new1(wxWindow * parent, wxDirection dir) {
    return new wxBannerWindow(parent, dir);
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
// Mix-in(s) to wxBannerWindow
wxTrackable *wxBannerWindow_AsTrackable(wxBannerWindow* obj) {
    return static_cast<wxTrackable*>(obj);
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
wxImage *wxBitmap_ConvertToImage(const wxBitmap * self) {
    return new wxImage(self->ConvertToImage());
}
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
void wxBitmapBundleImpl_delete(wxBitmapBundleImpl *self) {
    delete self;
}
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
// Mix-in(s) to wxBitmapButton
wxTrackable *wxBitmapButton_AsTrackable(wxBitmapButton* obj) {
    return static_cast<wxTrackable*>(obj);
}

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
wxTrackable *wxBitmapComboBox_AsTrackable(wxBitmapComboBox* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxBitmapToggleButton
wxTrackable *wxBitmapToggleButton_AsTrackable(wxBitmapToggleButton* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxBookCtrlBase
wxWithImages *wxBookCtrlBase_AsWithImages(wxBookCtrlBase* obj) {
    return static_cast<wxWithImages*>(obj);
}
wxTrackable *wxBookCtrlBase_AsTrackable(wxBookCtrlBase* obj) {
    return static_cast<wxTrackable*>(obj);
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

// CLASS: wxBufferedDC
wxClassInfo *wxBufferedDC_CLASSINFO() {
    return wxCLASSINFO(wxBufferedDC);
}
wxBufferedDC *wxBufferedDC_new() {
    return new wxBufferedDC();
}
wxBufferedDC *wxBufferedDC_new1(wxDC * dc, const wxSize * area, int style) {
    return new wxBufferedDC(dc, *area, style);
}
wxBufferedDC *wxBufferedDC_new2(wxDC * dc, wxBitmap * buffer, int style) {
    return new wxBufferedDC(dc, *buffer, style);
}
void wxBufferedDC_Init(wxBufferedDC * self, wxDC * dc, const wxSize * area, int style) {
    return self->Init(dc, *area, style);
}
void wxBufferedDC_Init1(wxBufferedDC * self, wxDC * dc, wxBitmap * buffer, int style) {
    return self->Init(dc, *buffer, style);
}
void wxBufferedDC_UnMask(wxBufferedDC * self) {
    return self->UnMask();
}
void wxBufferedDC_SetStyle(wxBufferedDC * self, int style) {
    return self->SetStyle(style);
}
int wxBufferedDC_GetStyle(const wxBufferedDC * self) {
    return self->GetStyle();
}

// CLASS: wxBufferedPaintDC
wxClassInfo *wxBufferedPaintDC_CLASSINFO() {
    return wxCLASSINFO(wxBufferedPaintDC);
}
wxBufferedPaintDC *wxBufferedPaintDC_new(wxWindow * window, wxBitmap * buffer, int style) {
    return new wxBufferedPaintDC(window, *buffer, style);
}
wxBufferedPaintDC *wxBufferedPaintDC_new1(wxWindow * window, int style) {
    return new wxBufferedPaintDC(window, style);
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
// Mix-in(s) to wxButton
wxTrackable *wxButton_AsTrackable(wxButton* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxCalculateLayoutEvent
wxClassInfo *wxCalculateLayoutEvent_CLASSINFO() {
    return wxCLASSINFO(wxCalculateLayoutEvent);
}
wxCalculateLayoutEvent *wxCalculateLayoutEvent_new(wxWindowID id) {
    return new wxCalculateLayoutEvent(id);
}
int wxCalculateLayoutEvent_GetFlags(const wxCalculateLayoutEvent * self) {
    return self->GetFlags();
}
wxRect *wxCalculateLayoutEvent_GetRect(const wxCalculateLayoutEvent * self) {
    return new wxRect(self->GetRect());
}
void wxCalculateLayoutEvent_SetFlags(wxCalculateLayoutEvent * self, int flags) {
    return self->SetFlags(flags);
}
void wxCalculateLayoutEvent_SetRect(wxCalculateLayoutEvent * self, const wxRect * rect) {
    return self->SetRect(*rect);
}

// CLASS: wxCalendarCtrl
wxClassInfo *wxCalendarCtrl_CLASSINFO() {
    return wxCLASSINFO(wxCalendarCtrl);
}
bool wxCalendarCtrl_SetDateRange(wxCalendarCtrl * self, const wxDateTime * lowerdate, const wxDateTime * upperdate) {
    return self->SetDateRange(*lowerdate, *upperdate);
}
bool wxCalendarCtrl_GetDateRange(const wxCalendarCtrl * self, wxDateTime * lowerdate, wxDateTime * upperdate) {
    return self->GetDateRange(lowerdate, upperdate);
}
wxCalendarCtrl *wxCalendarCtrl_new() {
    return new wxCalendarCtrl();
}
wxCalendarCtrl *wxCalendarCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * date, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxCalendarCtrl(parent, id, *date, *pos, *size, style, *name);
}
bool wxCalendarCtrl_Create(wxCalendarCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * date, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *date, *pos, *size, style, *name);
}
void wxCalendarCtrl_EnableHolidayDisplay(wxCalendarCtrl * self, bool display) {
    return self->EnableHolidayDisplay(display);
}
bool wxCalendarCtrl_EnableMonthChange(wxCalendarCtrl * self, bool enable) {
    return self->EnableMonthChange(enable);
}
void wxCalendarCtrl_EnableYearChange(wxCalendarCtrl * self, bool enable) {
    return self->EnableYearChange(enable);
}
wxCalendarDateAttr * wxCalendarCtrl_GetAttr(const wxCalendarCtrl * self, size_t day) {
    return self->GetAttr(day);
}
wxDateTime *wxCalendarCtrl_GetDate(const wxCalendarCtrl * self) {
    return new wxDateTime(self->GetDate());
}
wxColour *wxCalendarCtrl_GetHeaderColourBg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHeaderColourBg());
}
wxColour *wxCalendarCtrl_GetHeaderColourFg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHeaderColourFg());
}
wxColour *wxCalendarCtrl_GetHighlightColourBg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHighlightColourBg());
}
wxColour *wxCalendarCtrl_GetHighlightColourFg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHighlightColourFg());
}
wxColour *wxCalendarCtrl_GetHolidayColourBg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHolidayColourBg());
}
wxColour *wxCalendarCtrl_GetHolidayColourFg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHolidayColourFg());
}
void wxCalendarCtrl_ResetAttr(wxCalendarCtrl * self, size_t day) {
    return self->ResetAttr(day);
}
void wxCalendarCtrl_SetAttr(wxCalendarCtrl * self, size_t day, wxCalendarDateAttr * attr) {
    return self->SetAttr(day, attr);
}
bool wxCalendarCtrl_SetDate(wxCalendarCtrl * self, const wxDateTime * date) {
    return self->SetDate(*date);
}
void wxCalendarCtrl_SetHeaderColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg) {
    return self->SetHeaderColours(*col_fg, *col_bg);
}
void wxCalendarCtrl_SetHighlightColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg) {
    return self->SetHighlightColours(*col_fg, *col_bg);
}
void wxCalendarCtrl_SetHoliday(wxCalendarCtrl * self, size_t day) {
    return self->SetHoliday(day);
}
void wxCalendarCtrl_SetHolidayColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg) {
    return self->SetHolidayColours(*col_fg, *col_bg);
}
void wxCalendarCtrl_Mark(wxCalendarCtrl * self, size_t day, bool mark) {
    return self->Mark(day, mark);
}
// Mix-in(s) to wxCalendarCtrl
wxTrackable *wxCalendarCtrl_AsTrackable(wxCalendarCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxCalendarDateAttr
void wxCalendarDateAttr_delete(wxCalendarDateAttr *self) {
    delete self;
}
wxColour *wxCalendarDateAttr_GetBackgroundColour(const wxCalendarDateAttr * self) {
    return new wxColour(self->GetBackgroundColour());
}
wxColour *wxCalendarDateAttr_GetBorderColour(const wxCalendarDateAttr * self) {
    return new wxColour(self->GetBorderColour());
}
wxFont *wxCalendarDateAttr_GetFont(const wxCalendarDateAttr * self) {
    return new wxFont(self->GetFont());
}
wxColour *wxCalendarDateAttr_GetTextColour(const wxCalendarDateAttr * self) {
    return new wxColour(self->GetTextColour());
}
bool wxCalendarDateAttr_HasBackgroundColour(const wxCalendarDateAttr * self) {
    return self->HasBackgroundColour();
}
bool wxCalendarDateAttr_HasBorder(const wxCalendarDateAttr * self) {
    return self->HasBorder();
}
bool wxCalendarDateAttr_HasBorderColour(const wxCalendarDateAttr * self) {
    return self->HasBorderColour();
}
bool wxCalendarDateAttr_HasFont(const wxCalendarDateAttr * self) {
    return self->HasFont();
}
bool wxCalendarDateAttr_HasTextColour(const wxCalendarDateAttr * self) {
    return self->HasTextColour();
}
bool wxCalendarDateAttr_IsHoliday(const wxCalendarDateAttr * self) {
    return self->IsHoliday();
}
void wxCalendarDateAttr_SetBackgroundColour(wxCalendarDateAttr * self, const wxColour * col_back) {
    return self->SetBackgroundColour(*col_back);
}
void wxCalendarDateAttr_SetBorderColour(wxCalendarDateAttr * self, const wxColour * col) {
    return self->SetBorderColour(*col);
}
void wxCalendarDateAttr_SetFont(wxCalendarDateAttr * self, const wxFont * font) {
    return self->SetFont(*font);
}
void wxCalendarDateAttr_SetHoliday(wxCalendarDateAttr * self, bool holiday) {
    return self->SetHoliday(holiday);
}
void wxCalendarDateAttr_SetTextColour(wxCalendarDateAttr * self, const wxColour * col_text) {
    return self->SetTextColour(*col_text);
}
wxCalendarDateAttr *wxCalendarDateAttr_GetMark() {
    return new wxCalendarDateAttr(wxCalendarDateAttr::GetMark());
}
void wxCalendarDateAttr_SetMark(const wxCalendarDateAttr * m) {
    return wxCalendarDateAttr::SetMark(*m);
}

// CLASS: wxCalendarEvent
wxClassInfo *wxCalendarEvent_CLASSINFO() {
    return wxCLASSINFO(wxCalendarEvent);
}
wxCalendarEvent *wxCalendarEvent_new() {
    return new wxCalendarEvent();
}

// CLASS: wxCaret
void wxCaret_delete(wxCaret *self) {
    delete self;
}
wxCaret *wxCaret_new() {
    return new wxCaret();
}
wxCaret *wxCaret_new1(wxWindow * window, int width, int height) {
    return new wxCaret(window, width, height);
}
wxCaret *wxCaret_new2(wxWindow * window, const wxSize * size) {
    return new wxCaret(window, *size);
}
bool wxCaret_Create(wxCaret * self, wxWindow * window, int width, int height) {
    return self->Create(window, width, height);
}
bool wxCaret_Create1(wxCaret * self, wxWindow * window, const wxSize * size) {
    return self->Create(window, *size);
}
void wxCaret_GetPosition(const wxCaret * self, int * x, int * y) {
    return self->GetPosition(x, y);
}
wxPoint *wxCaret_GetPosition1(const wxCaret * self) {
    return new wxPoint(self->GetPosition());
}
void wxCaret_GetSize(const wxCaret * self, int * width, int * height) {
    return self->GetSize(width, height);
}
wxSize *wxCaret_GetSize1(const wxCaret * self) {
    return new wxSize(self->GetSize());
}
wxWindow * wxCaret_GetWindow(const wxCaret * self) {
    return self->GetWindow();
}
void wxCaret_Hide(wxCaret * self) {
    return self->Hide();
}
bool wxCaret_IsOk(const wxCaret * self) {
    return self->IsOk();
}
bool wxCaret_IsVisible(const wxCaret * self) {
    return self->IsVisible();
}
void wxCaret_Move(wxCaret * self, int x, int y) {
    return self->Move(x, y);
}
void wxCaret_Move1(wxCaret * self, const wxPoint * pt) {
    return self->Move(*pt);
}
void wxCaret_SetSize(wxCaret * self, int width, int height) {
    return self->SetSize(width, height);
}
void wxCaret_SetSize1(wxCaret * self, const wxSize * size) {
    return self->SetSize(*size);
}
void wxCaret_Show(wxCaret * self, bool show) {
    return self->Show(show);
}
int wxCaret_GetBlinkTime() {
    return wxCaret::GetBlinkTime();
}
void wxCaret_SetBlinkTime(int milliseconds) {
    return wxCaret::SetBlinkTime(milliseconds);
}

// CLASS: wxCheckBox
wxClassInfo *wxCheckBox_CLASSINFO() {
    return wxCLASSINFO(wxCheckBox);
}
wxCheckBox *wxCheckBox_new() {
    return new wxCheckBox();
}
wxCheckBox *wxCheckBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxCheckBox(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxCheckBox_Create(wxCheckBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxCheckBox_GetValue(const wxCheckBox * self) {
    return self->GetValue();
}
wxCheckBoxState wxCheckBox_Get3StateValue(const wxCheckBox * self) {
    return self->Get3StateValue();
}
bool wxCheckBox_Is3State(const wxCheckBox * self) {
    return self->Is3State();
}
bool wxCheckBox_Is3rdStateAllowedForUser(const wxCheckBox * self) {
    return self->Is3rdStateAllowedForUser();
}
bool wxCheckBox_IsChecked(const wxCheckBox * self) {
    return self->IsChecked();
}
void wxCheckBox_SetValue(wxCheckBox * self, bool state) {
    return self->SetValue(state);
}
void wxCheckBox_Set3StateValue(wxCheckBox * self, wxCheckBoxState state) {
    return self->Set3StateValue(state);
}
// Mix-in(s) to wxCheckBox
wxTrackable *wxCheckBox_AsTrackable(wxCheckBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxCheckListBox
wxClassInfo *wxCheckListBox_CLASSINFO() {
    return wxCLASSINFO(wxCheckListBox);
}
wxCheckListBox *wxCheckListBox_new() {
    return new wxCheckListBox();
}
wxCheckListBox *wxCheckListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxCheckListBox(parent, id, *pos, *size, *choices, style, *validator, *name);
}
bool wxCheckListBox_Create1(wxCheckListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *choices, style, *validator, *name);
}
void wxCheckListBox_Check(wxCheckListBox * self, unsigned int item, bool check) {
    return self->Check(item, check);
}
bool wxCheckListBox_IsChecked(const wxCheckListBox * self, unsigned int item) {
    return self->IsChecked(item);
}
unsigned int wxCheckListBox_GetCheckedItems(const wxCheckListBox * self, wxArrayInt * checked_items) {
    return self->GetCheckedItems(*checked_items);
}
// Mix-in(s) to wxCheckListBox
wxItemContainer *wxCheckListBox_AsItemContainer(wxCheckListBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTrackable *wxCheckListBox_AsTrackable(wxCheckListBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxChildFocusEvent
wxClassInfo *wxChildFocusEvent_CLASSINFO() {
    return wxCLASSINFO(wxChildFocusEvent);
}
wxChildFocusEvent *wxChildFocusEvent_new(wxWindow * win) {
    return new wxChildFocusEvent(win);
}
wxWindow * wxChildFocusEvent_GetWindow(const wxChildFocusEvent * self) {
    return self->GetWindow();
}

// CLASS: wxChoice
wxClassInfo *wxChoice_CLASSINFO() {
    return wxCLASSINFO(wxChoice);
}
wxChoice *wxChoice_new() {
    return new wxChoice();
}
wxChoice *wxChoice_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxChoice(parent, id, *pos, *size, *choices, style, *validator, *name);
}
bool wxChoice_Create1(wxChoice * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *choices, style, *validator, *name);
}
int wxChoice_GetColumns(const wxChoice * self) {
    return self->GetColumns();
}
int wxChoice_GetCurrentSelection(const wxChoice * self) {
    return self->GetCurrentSelection();
}
void wxChoice_SetColumns(wxChoice * self, int n) {
    return self->SetColumns(n);
}
bool wxChoice_IsSorted(const wxChoice * self) {
    return self->IsSorted();
}
// Mix-in(s) to wxChoice
wxItemContainer *wxChoice_AsItemContainer(wxChoice* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTrackable *wxChoice_AsTrackable(wxChoice* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxChoicebook
wxClassInfo *wxChoicebook_CLASSINFO() {
    return wxCLASSINFO(wxChoicebook);
}
wxChoicebook *wxChoicebook_new() {
    return new wxChoicebook();
}
wxChoicebook *wxChoicebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxChoicebook(parent, id, *pos, *size, style, *name);
}
bool wxChoicebook_Create(wxChoicebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
wxChoice * wxChoicebook_GetChoiceCtrl(const wxChoicebook * self) {
    return self->GetChoiceCtrl();
}
// Mix-in(s) to wxChoicebook
wxWithImages *wxChoicebook_AsWithImages(wxChoicebook* obj) {
    return static_cast<wxWithImages*>(obj);
}
wxTrackable *wxChoicebook_AsTrackable(wxChoicebook* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxClientDC
wxClassInfo *wxClientDC_CLASSINFO() {
    return wxCLASSINFO(wxClientDC);
}
wxClientDC *wxClientDC_new(wxWindow * window) {
    return new wxClientDC(window);
}

// CLASS: wxClipboard
wxClassInfo *wxClipboard_CLASSINFO() {
    return wxCLASSINFO(wxClipboard);
}
wxClipboard *wxClipboard_new() {
    return new wxClipboard();
}
bool wxClipboard_AddData(wxClipboard * self, wxDataObject * data) {
    return self->AddData(data);
}
void wxClipboard_Clear(wxClipboard * self) {
    return self->Clear();
}
void wxClipboard_Close(wxClipboard * self) {
    return self->Close();
}
bool wxClipboard_Flush(wxClipboard * self) {
    return self->Flush();
}
bool wxClipboard_GetData(wxClipboard * self, wxDataObject * data) {
    return self->GetData(*data);
}
bool wxClipboard_IsOpened(const wxClipboard * self) {
    return self->IsOpened();
}
bool wxClipboard_IsSupported(wxClipboard * self, const wxDataFormat * format) {
    return self->IsSupported(*format);
}
bool wxClipboard_IsUsingPrimarySelection(const wxClipboard * self) {
    return self->IsUsingPrimarySelection();
}
bool wxClipboard_Open(wxClipboard * self) {
    return self->Open();
}
bool wxClipboard_SetData(wxClipboard * self, wxDataObject * data) {
    return self->SetData(data);
}
void wxClipboard_UsePrimarySelection(wxClipboard * self, bool primary) {
    return self->UsePrimarySelection(primary);
}
wxClipboard * wxClipboard_Get() {
    return wxClipboard::Get();
}

// CLASS: wxClipboardTextEvent
wxClassInfo *wxClipboardTextEvent_CLASSINFO() {
    return wxCLASSINFO(wxClipboardTextEvent);
}

// CLASS: wxCloseEvent
wxClassInfo *wxCloseEvent_CLASSINFO() {
    return wxCLASSINFO(wxCloseEvent);
}
bool wxCloseEvent_CanVeto(const wxCloseEvent * self) {
    return self->CanVeto();
}
bool wxCloseEvent_GetLoggingOff(const wxCloseEvent * self) {
    return self->GetLoggingOff();
}
void wxCloseEvent_SetCanVeto(wxCloseEvent * self, bool can_veto) {
    return self->SetCanVeto(can_veto);
}
void wxCloseEvent_SetLoggingOff(wxCloseEvent * self, bool logging_off) {
    return self->SetLoggingOff(logging_off);
}
void wxCloseEvent_Veto(wxCloseEvent * self, bool veto) {
    return self->Veto(veto);
}
bool wxCloseEvent_GetVeto(const wxCloseEvent * self) {
    return self->GetVeto();
}

// CLASS: wxCollapsibleHeaderCtrl
wxClassInfo *wxCollapsibleHeaderCtrl_CLASSINFO() {
    return wxCLASSINFO(wxCollapsibleHeaderCtrl);
}
wxCollapsibleHeaderCtrl *wxCollapsibleHeaderCtrl_new() {
    return new wxCollapsibleHeaderCtrl();
}
wxCollapsibleHeaderCtrl *wxCollapsibleHeaderCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxCollapsibleHeaderCtrl(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxCollapsibleHeaderCtrl_Create(wxCollapsibleHeaderCtrl * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
void wxCollapsibleHeaderCtrl_SetCollapsed(wxCollapsibleHeaderCtrl * self, bool collapsed) {
    return self->SetCollapsed(collapsed);
}
bool wxCollapsibleHeaderCtrl_IsCollapsed(const wxCollapsibleHeaderCtrl * self) {
    return self->IsCollapsed();
}
// Mix-in(s) to wxCollapsibleHeaderCtrl
wxTrackable *wxCollapsibleHeaderCtrl_AsTrackable(wxCollapsibleHeaderCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxCollapsiblePane
wxClassInfo *wxCollapsiblePane_CLASSINFO() {
    return wxCLASSINFO(wxCollapsiblePane);
}
wxCollapsiblePane *wxCollapsiblePane_new() {
    return new wxCollapsiblePane();
}
wxCollapsiblePane *wxCollapsiblePane_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxCollapsiblePane(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxCollapsiblePane_Create(wxCollapsiblePane * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
void wxCollapsiblePane_Collapse(wxCollapsiblePane * self, bool collapse) {
    return self->Collapse(collapse);
}
void wxCollapsiblePane_Expand(wxCollapsiblePane * self) {
    return self->Expand();
}
wxWindow * wxCollapsiblePane_GetPane(const wxCollapsiblePane * self) {
    return self->GetPane();
}
bool wxCollapsiblePane_IsCollapsed(const wxCollapsiblePane * self) {
    return self->IsCollapsed();
}
bool wxCollapsiblePane_IsExpanded(const wxCollapsiblePane * self) {
    return self->IsExpanded();
}
// Mix-in(s) to wxCollapsiblePane
wxTrackable *wxCollapsiblePane_AsTrackable(wxCollapsiblePane* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxCollapsiblePaneEvent
wxClassInfo *wxCollapsiblePaneEvent_CLASSINFO() {
    return wxCLASSINFO(wxCollapsiblePaneEvent);
}
wxCollapsiblePaneEvent *wxCollapsiblePaneEvent_new(wxObject * generator, int id, bool collapsed) {
    return new wxCollapsiblePaneEvent(generator, id, collapsed);
}
bool wxCollapsiblePaneEvent_GetCollapsed(const wxCollapsiblePaneEvent * self) {
    return self->GetCollapsed();
}
void wxCollapsiblePaneEvent_SetCollapsed(wxCollapsiblePaneEvent * self, bool collapsed) {
    return self->SetCollapsed(collapsed);
}

// CLASS: wxColour
wxClassInfo *wxColour_CLASSINFO() {
    return wxCLASSINFO(wxColour);
}
wxColour *wxColour_new() {
    return new wxColour();
}
wxColour *wxColour_new2(const wxString * colour_name) {
    return new wxColour(*colour_name);
}
wxColour *wxColour_new4(const wxColour * colour) {
    return new wxColour(*colour);
}
#if wxCHECK_VERSION(3, 1, 0)
unsigned int wxColour_GetAlpha(const wxColour * self) {
    return self->GetAlpha();
}
unsigned int wxColour_GetBlue(const wxColour * self) {
    return self->GetBlue();
}
unsigned int wxColour_GetGreen(const wxColour * self) {
    return self->GetGreen();
}
unsigned int wxColour_GetRed(const wxColour * self) {
    return self->GetRed();
}
#endif
wxString *wxColour_GetAsString(const wxColour * self, long flags) {
    return new wxString(self->GetAsString(flags));
}
#if wxCHECK_VERSION(3, 1, 0)
double wxColour_GetLuminance(const wxColour * self) {
    return self->GetLuminance();
}
#endif
bool wxColour_IsOk(const wxColour * self) {
    return self->IsOk();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxColour_IsSolid(const wxColour * self) {
    return self->IsSolid();
}
#endif
bool wxColour_Set2(wxColour * self, const wxString * str) {
    return self->Set(*str);
}
void wxColour_MakeMono(unsigned char * r, unsigned char * g, unsigned char * b, bool on) {
    return wxColour::MakeMono(r, g, b, on);
}
void wxColour_MakeGrey(unsigned char * r, unsigned char * g, unsigned char * b) {
    return wxColour::MakeGrey(r, g, b);
}
void wxColour_MakeGrey1(unsigned char * r, unsigned char * g, unsigned char * b, double weight_r, double weight_g, double weight_b) {
    return wxColour::MakeGrey(r, g, b, weight_r, weight_g, weight_b);
}
void wxColour_ChangeLightness1(unsigned char * r, unsigned char * g, unsigned char * b, int ialpha) {
    return wxColour::ChangeLightness(r, g, b, ialpha);
}

// CLASS: wxColourData
wxClassInfo *wxColourData_CLASSINFO() {
    return wxCLASSINFO(wxColourData);
}
wxColourData *wxColourData_new() {
    return new wxColourData();
}
bool wxColourData_GetChooseFull(const wxColourData * self) {
    return self->GetChooseFull();
}
bool wxColourData_GetChooseAlpha(const wxColourData * self) {
    return self->GetChooseAlpha();
}
wxColour * wxColourData_GetColour(wxColourData * self) {
    return &(self->GetColour());
}
wxColour *wxColourData_GetCustomColour(const wxColourData * self, int i) {
    return new wxColour(self->GetCustomColour(i));
}
void wxColourData_SetChooseFull(wxColourData * self, bool flag) {
    return self->SetChooseFull(flag);
}
void wxColourData_SetChooseAlpha(wxColourData * self, bool flag) {
    return self->SetChooseAlpha(flag);
}
void wxColourData_SetColour(wxColourData * self, const wxColour * colour) {
    return self->SetColour(*colour);
}
void wxColourData_SetCustomColour(wxColourData * self, int i, const wxColour * colour) {
    return self->SetCustomColour(i, *colour);
}
wxString *wxColourData_ToString(const wxColourData * self) {
    return new wxString(self->ToString());
}
bool wxColourData_FromString(wxColourData * self, const wxString * str) {
    return self->FromString(*str);
}

// CLASS: wxColourDatabase
void wxColourDatabase_delete(wxColourDatabase *self) {
    delete self;
}
wxColourDatabase *wxColourDatabase_new() {
    return new wxColourDatabase();
}
void wxColourDatabase_AddColour(wxColourDatabase * self, const wxString * colour_name, const wxColour * colour) {
    return self->AddColour(*colour_name, *colour);
}
wxColour *wxColourDatabase_Find(const wxColourDatabase * self, const wxString * colour_name) {
    return new wxColour(self->Find(*colour_name));
}
wxString *wxColourDatabase_FindName(const wxColourDatabase * self, const wxColour * colour) {
    return new wxString(self->FindName(*colour));
}

// CLASS: wxColourDialog
wxClassInfo *wxColourDialog_CLASSINFO() {
    return wxCLASSINFO(wxColourDialog);
}
wxColourDialog *wxColourDialog_new(wxWindow * parent, const wxColourData * data) {
    return new wxColourDialog(parent, data);
}
bool wxColourDialog_Create(wxColourDialog * self, wxWindow * parent, const wxColourData * data) {
    return self->Create(parent, data);
}
wxColourData * wxColourDialog_GetColourData(wxColourDialog * self) {
    return &(self->GetColourData());
}
// Mix-in(s) to wxColourDialog
wxTrackable *wxColourDialog_AsTrackable(wxColourDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxColourDialogEvent
wxClassInfo *wxColourDialogEvent_CLASSINFO() {
    return wxCLASSINFO(wxColourDialogEvent);
}
wxColourDialogEvent *wxColourDialogEvent_new() {
    return new wxColourDialogEvent();
}
wxColour *wxColourDialogEvent_GetColour(const wxColourDialogEvent * self) {
    return new wxColour(self->GetColour());
}
void wxColourDialogEvent_SetColour(wxColourDialogEvent * self, const wxColour * colour) {
    return self->SetColour(*colour);
}

// CLASS: wxColourPickerCtrl
wxClassInfo *wxColourPickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxColourPickerCtrl);
}
wxColourPickerCtrl *wxColourPickerCtrl_new() {
    return new wxColourPickerCtrl();
}
wxColourPickerCtrl *wxColourPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxColourPickerCtrl(parent, id, *colour, *pos, *size, style, *validator, *name);
}
bool wxColourPickerCtrl_Create(wxColourPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *colour, *pos, *size, style, *validator, *name);
}
wxColour *wxColourPickerCtrl_GetColour(const wxColourPickerCtrl * self) {
    return new wxColour(self->GetColour());
}
void wxColourPickerCtrl_SetColour(wxColourPickerCtrl * self, const wxColour * col) {
    return self->SetColour(*col);
}
// Mix-in(s) to wxColourPickerCtrl
wxTrackable *wxColourPickerCtrl_AsTrackable(wxColourPickerCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxColourPickerEvent
wxClassInfo *wxColourPickerEvent_CLASSINFO() {
    return wxCLASSINFO(wxColourPickerEvent);
}
wxColourPickerEvent *wxColourPickerEvent_new() {
    return new wxColourPickerEvent();
}
wxColourPickerEvent *wxColourPickerEvent_new1(wxObject * generator, int id, const wxColour * colour) {
    return new wxColourPickerEvent(generator, id, *colour);
}
wxColour *wxColourPickerEvent_GetColour(const wxColourPickerEvent * self) {
    return new wxColour(self->GetColour());
}
void wxColourPickerEvent_SetColour(wxColourPickerEvent * self, const wxColour * pos) {
    return self->SetColour(*pos);
}

// CLASS: wxComboBox
wxClassInfo *wxComboBox_CLASSINFO() {
    return wxCLASSINFO(wxComboBox);
}
wxComboBox *wxComboBox_new() {
    return new wxComboBox();
}
wxComboBox *wxComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxComboBox(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
bool wxComboBox_Create1(wxComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
int wxComboBox_GetCurrentSelection(const wxComboBox * self) {
    return self->GetCurrentSelection();
}
bool wxComboBox_IsListEmpty(const wxComboBox * self) {
    return self->IsListEmpty();
}
bool wxComboBox_IsTextEmpty(const wxComboBox * self) {
    return self->IsTextEmpty();
}
void wxComboBox_Popup(wxComboBox * self) {
    return self->Popup();
}
void wxComboBox_Dismiss(wxComboBox * self) {
    return self->Dismiss();
}
// Mix-in(s) to wxComboBox
wxItemContainer *wxComboBox_AsItemContainer(wxComboBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTextEntryBase *wxComboBox_AsTextEntry(wxComboBox* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}
wxTrackable *wxComboBox_AsTrackable(wxComboBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxComboCtrl
wxClassInfo *wxComboCtrl_CLASSINFO() {
    return wxCLASSINFO(wxComboCtrl);
}
wxComboCtrl *wxComboCtrl_new() {
    return new wxComboCtrl();
}
wxComboCtrl *wxComboCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxComboCtrl(parent, id, *value, *pos, *size, style, *validator, *name);
}
bool wxComboCtrl_Create(wxComboCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, *validator, *name);
}
void wxComboCtrl_Dismiss(wxComboCtrl * self) {
    return self->Dismiss();
}
void wxComboCtrl_EnablePopupAnimation(wxComboCtrl * self, bool enable) {
    return self->EnablePopupAnimation(enable);
}
bool wxComboCtrl_IsKeyPopupToggle(const wxComboCtrl * self, const wxKeyEvent * event) {
    return self->IsKeyPopupToggle(*event);
}
void wxComboCtrl_PrepareBackground(const wxComboCtrl * self, wxDC * dc, const wxRect * rect, int flags) {
    return self->PrepareBackground(*dc, *rect, flags);
}
bool wxComboCtrl_ShouldDrawFocus(const wxComboCtrl * self) {
    return self->ShouldDrawFocus();
}
wxBitmap *wxComboCtrl_GetBitmapDisabled(const wxComboCtrl * self) {
    return new wxBitmap(self->GetBitmapDisabled());
}
wxBitmap *wxComboCtrl_GetBitmapHover(const wxComboCtrl * self) {
    return new wxBitmap(self->GetBitmapHover());
}
wxBitmap *wxComboCtrl_GetBitmapNormal(const wxComboCtrl * self) {
    return new wxBitmap(self->GetBitmapNormal());
}
wxBitmap *wxComboCtrl_GetBitmapPressed(const wxComboCtrl * self) {
    return new wxBitmap(self->GetBitmapPressed());
}
wxSize *wxComboCtrl_GetButtonSize(wxComboCtrl * self) {
    return new wxSize(self->GetButtonSize());
}
int wxComboCtrl_GetCustomPaintWidth(const wxComboCtrl * self) {
    return self->GetCustomPaintWidth();
}
wxPoint *wxComboCtrl_GetMargins(const wxComboCtrl * self) {
    return new wxPoint(self->GetMargins());
}
wxComboPopup * wxComboCtrl_GetPopupControl(wxComboCtrl * self) {
    return self->GetPopupControl();
}
wxWindow * wxComboCtrl_GetPopupWindow(const wxComboCtrl * self) {
    return self->GetPopupWindow();
}
wxTextCtrl * wxComboCtrl_GetTextCtrl(const wxComboCtrl * self) {
    return self->GetTextCtrl();
}
wxCoord wxComboCtrl_GetTextIndent(const wxComboCtrl * self) {
    return self->GetTextIndent();
}
wxRect *wxComboCtrl_GetTextRect(const wxComboCtrl * self) {
    return new wxRect(self->GetTextRect());
}
void wxComboCtrl_HidePopup(wxComboCtrl * self, bool generate_event) {
    return self->HidePopup(generate_event);
}
bool wxComboCtrl_IsPopupShown(const wxComboCtrl * self) {
    return self->IsPopupShown();
}
bool wxComboCtrl_IsPopupWindowState(const wxComboCtrl * self, int state) {
    return self->IsPopupWindowState(state);
}
void wxComboCtrl_OnButtonClick(wxComboCtrl * self) {
    return self->OnButtonClick();
}
void wxComboCtrl_Popup(wxComboCtrl * self) {
    return self->Popup();
}
void wxComboCtrl_SetButtonBitmaps(wxComboCtrl * self, const wxBitmapBundle * bmp_normal, bool push_button_bg, const wxBitmapBundle * bmp_pressed, const wxBitmapBundle * bmp_hover, const wxBitmapBundle * bmp_disabled) {
    return self->SetButtonBitmaps(*bmp_normal, push_button_bg, *bmp_pressed, *bmp_hover, *bmp_disabled);
}
void wxComboCtrl_SetButtonPosition(wxComboCtrl * self, int width, int height, int side, int spacing_x) {
    return self->SetButtonPosition(width, height, side, spacing_x);
}
void wxComboCtrl_SetCustomPaintWidth(wxComboCtrl * self, int width) {
    return self->SetCustomPaintWidth(width);
}
void wxComboCtrl_SetMainControl(wxComboCtrl * self, wxWindow * win) {
    return self->SetMainControl(win);
}
bool wxComboCtrl_SetMargins(wxComboCtrl * self, const wxPoint * pt) {
    return self->SetMargins(*pt);
}
bool wxComboCtrl_SetMargins1(wxComboCtrl * self, wxCoord left, wxCoord top) {
    return self->SetMargins(left, top);
}
void wxComboCtrl_SetPopupAnchor(wxComboCtrl * self, int anchor_side) {
    return self->SetPopupAnchor(anchor_side);
}
void wxComboCtrl_SetPopupControl(wxComboCtrl * self, wxComboPopup * popup) {
    return self->SetPopupControl(popup);
}
void wxComboCtrl_SetPopupExtents(wxComboCtrl * self, int ext_left, int ext_right) {
    return self->SetPopupExtents(ext_left, ext_right);
}
void wxComboCtrl_SetPopupMaxHeight(wxComboCtrl * self, int height) {
    return self->SetPopupMaxHeight(height);
}
void wxComboCtrl_SetPopupMinWidth(wxComboCtrl * self, int width) {
    return self->SetPopupMinWidth(width);
}
void wxComboCtrl_SetText(wxComboCtrl * self, const wxString * value) {
    return self->SetText(*value);
}
void wxComboCtrl_SetTextCtrlStyle(wxComboCtrl * self, int style) {
    return self->SetTextCtrlStyle(style);
}
void wxComboCtrl_SetTextIndent(wxComboCtrl * self, int indent) {
    return self->SetTextIndent(indent);
}
void wxComboCtrl_SetValueByUser(wxComboCtrl * self, const wxString * value) {
    return self->SetValueByUser(*value);
}
void wxComboCtrl_ShowPopup(wxComboCtrl * self) {
    return self->ShowPopup();
}
void wxComboCtrl_UseAltPopupWindow(wxComboCtrl * self, bool enable) {
    return self->UseAltPopupWindow(enable);
}
int wxComboCtrl_GetFeatures() {
    return wxComboCtrl::GetFeatures();
}
// Mix-in(s) to wxComboCtrl
wxTextEntryBase *wxComboCtrl_AsTextEntry(wxComboCtrl* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}
wxTrackable *wxComboCtrl_AsTrackable(wxComboCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxComboPopup
void wxComboPopup_delete(wxComboPopup *self) {
    delete self;
}
wxComboPopup *wxComboPopup_new() {
    return new wxComboPopup();
}
bool wxComboPopup_Create(wxComboPopup * self, wxWindow * parent) {
    return self->Create(parent);
}
void wxComboPopup_DestroyPopup(wxComboPopup * self) {
    return self->DestroyPopup();
}
void wxComboPopup_Dismiss(wxComboPopup * self) {
    return self->Dismiss();
}
bool wxComboPopup_FindItem(wxComboPopup * self, const wxString * item, wxString * true_item) {
    return self->FindItem(*item, true_item);
}
wxSize *wxComboPopup_GetAdjustedSize(wxComboPopup * self, int min_width, int pref_height, int max_height) {
    return new wxSize(self->GetAdjustedSize(min_width, pref_height, max_height));
}
wxComboCtrl * wxComboPopup_GetComboCtrl(const wxComboPopup * self) {
    return self->GetComboCtrl();
}
wxWindow * wxComboPopup_GetControl(wxComboPopup * self) {
    return self->GetControl();
}
wxString *wxComboPopup_GetStringValue(const wxComboPopup * self) {
    return new wxString(self->GetStringValue());
}
void wxComboPopup_Init(wxComboPopup * self) {
    return self->Init();
}
bool wxComboPopup_IsCreated(const wxComboPopup * self) {
    return self->IsCreated();
}
bool wxComboPopup_LazyCreate(wxComboPopup * self) {
    return self->LazyCreate();
}
void wxComboPopup_OnComboDoubleClick(wxComboPopup * self) {
    return self->OnComboDoubleClick();
}
void wxComboPopup_OnComboKeyEvent(wxComboPopup * self, wxKeyEvent * event) {
    return self->OnComboKeyEvent(*event);
}
void wxComboPopup_OnDismiss(wxComboPopup * self) {
    return self->OnDismiss();
}
void wxComboPopup_OnPopup(wxComboPopup * self) {
    return self->OnPopup();
}
void wxComboPopup_PaintComboControl(wxComboPopup * self, wxDC * dc, const wxRect * rect) {
    return self->PaintComboControl(*dc, *rect);
}
void wxComboPopup_SetStringValue(wxComboPopup * self, const wxString * value) {
    return self->SetStringValue(*value);
}

// CLASS: wxCommand
wxClassInfo *wxCommand_CLASSINFO() {
    return wxCLASSINFO(wxCommand);
}
wxCommand *wxCommand_new(bool can_undo, const wxString * name) {
    return new wxCommand(can_undo, *name);
}
bool wxCommand_CanUndo(const wxCommand * self) {
    return self->CanUndo();
}
bool wxCommand_Do(wxCommand * self) {
    return self->Do();
}
wxString *wxCommand_GetName(const wxCommand * self) {
    return new wxString(self->GetName());
}
bool wxCommand_Undo(wxCommand * self) {
    return self->Undo();
}

// CLASS: wxCommandEvent
wxClassInfo *wxCommandEvent_CLASSINFO() {
    return wxCLASSINFO(wxCommandEvent);
}
void * wxCommandEvent_GetClientData(const wxCommandEvent * self) {
    return self->GetClientData();
}
wxClientData * wxCommandEvent_GetClientObject(const wxCommandEvent * self) {
    return self->GetClientObject();
}
long wxCommandEvent_GetExtraLong(const wxCommandEvent * self) {
    return self->GetExtraLong();
}
int wxCommandEvent_GetInt(const wxCommandEvent * self) {
    return self->GetInt();
}
int wxCommandEvent_GetSelection(const wxCommandEvent * self) {
    return self->GetSelection();
}
wxString *wxCommandEvent_GetString(const wxCommandEvent * self) {
    return new wxString(self->GetString());
}
bool wxCommandEvent_IsChecked(const wxCommandEvent * self) {
    return self->IsChecked();
}
bool wxCommandEvent_IsSelection(const wxCommandEvent * self) {
    return self->IsSelection();
}
void wxCommandEvent_SetClientData(wxCommandEvent * self, void * client_data) {
    return self->SetClientData(client_data);
}
void wxCommandEvent_SetClientObject(wxCommandEvent * self, wxClientData * client_object) {
    return self->SetClientObject(client_object);
}
void wxCommandEvent_SetExtraLong(wxCommandEvent * self, long extra_long) {
    return self->SetExtraLong(extra_long);
}
void wxCommandEvent_SetInt(wxCommandEvent * self, int int_command) {
    return self->SetInt(int_command);
}
void wxCommandEvent_SetString(wxCommandEvent * self, const wxString * string) {
    return self->SetString(*string);
}

// CLASS: wxCommandLinkButton
wxClassInfo *wxCommandLinkButton_CLASSINFO() {
    return wxCLASSINFO(wxCommandLinkButton);
}
wxCommandLinkButton *wxCommandLinkButton_new() {
    return new wxCommandLinkButton();
}
wxCommandLinkButton *wxCommandLinkButton_new1(wxWindow * parent, wxWindowID id, const wxString * main_label, const wxString * note, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxCommandLinkButton(parent, id, *main_label, *note, *pos, *size, style, *validator, *name);
}
bool wxCommandLinkButton_Create(wxCommandLinkButton * self, wxWindow * parent, wxWindowID id, const wxString * main_label, const wxString * note, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *main_label, *note, *pos, *size, style, *validator, *name);
}
void wxCommandLinkButton_SetMainLabelAndNote(wxCommandLinkButton * self, const wxString * main_label, const wxString * note) {
    return self->SetMainLabelAndNote(*main_label, *note);
}
void wxCommandLinkButton_SetMainLabel(wxCommandLinkButton * self, const wxString * main_label) {
    return self->SetMainLabel(*main_label);
}
void wxCommandLinkButton_SetNote(wxCommandLinkButton * self, const wxString * note) {
    return self->SetNote(*note);
}
wxString *wxCommandLinkButton_GetMainLabel(const wxCommandLinkButton * self) {
    return new wxString(self->GetMainLabel());
}
wxString *wxCommandLinkButton_GetNote(const wxCommandLinkButton * self) {
    return new wxString(self->GetNote());
}
// Mix-in(s) to wxCommandLinkButton
wxTrackable *wxCommandLinkButton_AsTrackable(wxCommandLinkButton* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxCommandProcessor
wxClassInfo *wxCommandProcessor_CLASSINFO() {
    return wxCLASSINFO(wxCommandProcessor);
}
wxCommandProcessor *wxCommandProcessor_new(int max_commands) {
    return new wxCommandProcessor(max_commands);
}
bool wxCommandProcessor_CanUndo(const wxCommandProcessor * self) {
    return self->CanUndo();
}
bool wxCommandProcessor_CanRedo(const wxCommandProcessor * self) {
    return self->CanRedo();
}
void wxCommandProcessor_ClearCommands(wxCommandProcessor * self) {
    return self->ClearCommands();
}
wxList * wxCommandProcessor_GetCommands(wxCommandProcessor * self) {
    return self->GetCommands();
}
wxCommand * wxCommandProcessor_GetCurrentCommand(const wxCommandProcessor * self) {
    return self->GetCurrentCommand();
}
wxMenu * wxCommandProcessor_GetEditMenu(const wxCommandProcessor * self) {
    return self->GetEditMenu();
}
int wxCommandProcessor_GetMaxCommands(const wxCommandProcessor * self) {
    return self->GetMaxCommands();
}
wxString *wxCommandProcessor_GetRedoAccelerator(const wxCommandProcessor * self) {
    return new wxString(self->GetRedoAccelerator());
}
wxString *wxCommandProcessor_GetRedoMenuLabel(const wxCommandProcessor * self) {
    return new wxString(self->GetRedoMenuLabel());
}
wxString *wxCommandProcessor_GetUndoAccelerator(const wxCommandProcessor * self) {
    return new wxString(self->GetUndoAccelerator());
}
wxString *wxCommandProcessor_GetUndoMenuLabel(const wxCommandProcessor * self) {
    return new wxString(self->GetUndoMenuLabel());
}
void wxCommandProcessor_Initialize(wxCommandProcessor * self) {
    return self->Initialize();
}
bool wxCommandProcessor_IsDirty(const wxCommandProcessor * self) {
    return self->IsDirty();
}
void wxCommandProcessor_MarkAsSaved(wxCommandProcessor * self) {
    return self->MarkAsSaved();
}
bool wxCommandProcessor_Redo(wxCommandProcessor * self) {
    return self->Redo();
}
void wxCommandProcessor_SetEditMenu(wxCommandProcessor * self, wxMenu * menu) {
    return self->SetEditMenu(menu);
}
void wxCommandProcessor_SetMenuStrings(wxCommandProcessor * self) {
    return self->SetMenuStrings();
}
void wxCommandProcessor_SetRedoAccelerator(wxCommandProcessor * self, const wxString * accel) {
    return self->SetRedoAccelerator(*accel);
}
void wxCommandProcessor_SetUndoAccelerator(wxCommandProcessor * self, const wxString * accel) {
    return self->SetUndoAccelerator(*accel);
}
bool wxCommandProcessor_Submit(wxCommandProcessor * self, wxCommand * command, bool store_it) {
    return self->Submit(command, store_it);
}
void wxCommandProcessor_Store(wxCommandProcessor * self, wxCommand * command) {
    return self->Store(command);
}
bool wxCommandProcessor_Undo(wxCommandProcessor * self) {
    return self->Undo();
}

// CLASS: wxContextHelp
wxClassInfo *wxContextHelp_CLASSINFO() {
    return wxCLASSINFO(wxContextHelp);
}
wxContextHelp *wxContextHelp_new(wxWindow * window, bool do_now) {
    return new wxContextHelp(window, do_now);
}
bool wxContextHelp_BeginContextHelp(wxContextHelp * self, wxWindow * window) {
    return self->BeginContextHelp(window);
}
bool wxContextHelp_EndContextHelp(wxContextHelp * self) {
    return self->EndContextHelp();
}

// CLASS: wxContextHelpButton
wxClassInfo *wxContextHelpButton_CLASSINFO() {
    return wxCLASSINFO(wxContextHelpButton);
}
wxContextHelpButton *wxContextHelpButton_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style) {
    return new wxContextHelpButton(parent, id, *pos, *size, style);
}
// Mix-in(s) to wxContextHelpButton
wxTrackable *wxContextHelpButton_AsTrackable(wxContextHelpButton* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxContextMenuEvent
wxClassInfo *wxContextMenuEvent_CLASSINFO() {
    return wxCLASSINFO(wxContextMenuEvent);
}
wxPoint *wxContextMenuEvent_GetPosition(const wxContextMenuEvent * self) {
    return new wxPoint(self->GetPosition());
}
void wxContextMenuEvent_SetPosition(wxContextMenuEvent * self, const wxPoint * point) {
    return self->SetPosition(*point);
}

// CLASS: wxControl
wxClassInfo *wxControl_CLASSINFO() {
    return wxCLASSINFO(wxControl);
}
wxControl *wxControl_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxControl(parent, id, *pos, *size, style, *validator, *name);
}
wxControl *wxControl_new1() {
    return new wxControl();
}
bool wxControl_Create(wxControl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
void wxControl_Command(wxControl * self, wxCommandEvent * event) {
    return self->Command(*event);
}
wxString *wxControl_GetLabelText(const wxControl * self) {
    return new wxString(self->GetLabelText());
}
wxSize *wxControl_GetSizeFromTextSize(const wxControl * self, int xlen, int ylen) {
    return new wxSize(self->GetSizeFromTextSize(xlen, ylen));
}
wxSize *wxControl_GetSizeFromTextSize1(const wxControl * self, const wxSize * tsize) {
    return new wxSize(self->GetSizeFromTextSize(*tsize));
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxControl_GetSizeFromText(const wxControl * self, const wxString * text) {
    return new wxSize(self->GetSizeFromText(*text));
}
#endif
void wxControl_SetLabelText(wxControl * self, const wxString * text) {
    return self->SetLabelText(*text);
}
bool wxControl_SetLabelMarkup(wxControl * self, const wxString * markup) {
    return self->SetLabelMarkup(*markup);
}
wxString *wxControl_GetLabelText1(const wxString * label) {
    return new wxString(wxControl::GetLabelText(*label));
}
wxString *wxControl_RemoveMnemonics(const wxString * str) {
    return new wxString(wxControl::RemoveMnemonics(*str));
}
wxString *wxControl_EscapeMnemonics(const wxString * text) {
    return new wxString(wxControl::EscapeMnemonics(*text));
}
wxString *wxControl_Ellipsize(const wxString * label, const wxDC * dc, wxEllipsizeMode mode, int max_width, int flags) {
    return new wxString(wxControl::Ellipsize(*label, *dc, mode, max_width, flags));
}
// Mix-in(s) to wxControl
wxTrackable *wxControl_AsTrackable(wxControl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxControlWithItems
wxClassInfo *wxControlWithItems_CLASSINFO() {
    return wxCLASSINFO(wxControlWithItems);
}
// Mix-in(s) to wxControlWithItems
wxItemContainer *wxControlWithItems_AsItemContainer(wxControlWithItems* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTrackable *wxControlWithItems_AsTrackable(wxControlWithItems* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxCredentialEntryDialog
wxClassInfo *wxCredentialEntryDialog_CLASSINFO() {
    return wxCLASSINFO(wxCredentialEntryDialog);
}
wxCredentialEntryDialog *wxCredentialEntryDialog_new() {
    return new wxCredentialEntryDialog();
}
wxCredentialEntryDialog *wxCredentialEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * title, const wxWebCredentials * cred) {
    return new wxCredentialEntryDialog(parent, *message, *title, *cred);
}
bool wxCredentialEntryDialog_Create(wxCredentialEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * title, const wxWebCredentials * cred) {
    return self->Create(parent, *message, *title, *cred);
}
wxWebCredentials *wxCredentialEntryDialog_GetCredentials(const wxCredentialEntryDialog * self) {
    return new wxWebCredentials(self->GetCredentials());
}
void wxCredentialEntryDialog_SetUser(wxCredentialEntryDialog * self, const wxString * user) {
    return self->SetUser(*user);
}
void wxCredentialEntryDialog_SetPassword(wxCredentialEntryDialog * self, const wxString * password) {
    return self->SetPassword(*password);
}
// Mix-in(s) to wxCredentialEntryDialog
wxTrackable *wxCredentialEntryDialog_AsTrackable(wxCredentialEntryDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxCursor
wxClassInfo *wxCursor_CLASSINFO() {
    return wxCLASSINFO(wxCursor);
}
wxCursor *wxCursor_new() {
    return new wxCursor();
}
wxCursor *wxCursor_new4(const wxImage * image) {
    return new wxCursor(*image);
}
wxCursor *wxCursor_new5(const char *const * xpm_data) {
    return new wxCursor(xpm_data);
}
wxCursor *wxCursor_new6(const wxCursor * cursor) {
    return new wxCursor(*cursor);
}
bool wxCursor_IsOk(const wxCursor * self) {
    return self->IsOk();
}
wxPoint *wxCursor_GetHotSpot(const wxCursor * self) {
    return new wxPoint(self->GetHotSpot());
}

// CLASS: wxCustomDataObject
void wxCustomDataObject_delete(wxCustomDataObject *self) {
    delete self;
}
wxCustomDataObject *wxCustomDataObject_new(const wxDataFormat * format) {
    return new wxCustomDataObject(*format);
}
void * wxCustomDataObject_Alloc(wxCustomDataObject * self, size_t size) {
    return self->Alloc(size);
}
void wxCustomDataObject_Free(wxCustomDataObject * self) {
    return self->Free();
}
void * wxCustomDataObject_GetData(const wxCustomDataObject * self) {
    return self->GetData();
}
size_t wxCustomDataObject_GetSize(const wxCustomDataObject * self) {
    return self->GetSize();
}
void wxCustomDataObject_TakeData(wxCustomDataObject * self, size_t size, void * data) {
    return self->TakeData(size, data);
}

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
wxPen *wxDC_GetPen(const wxDC * self) {
    return new wxPen(self->GetPen());
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

// CLASS: wxDataObject
void wxDataObject_delete(wxDataObject *self) {
    delete self;
}
wxDataObject *wxDataObject_new() {
    return new wxDataObject();
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
const wxDataFormat * wxDataObjectSimple_GetFormat(const wxDataObjectSimple * self) {
    return self->GetFormat();
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
wxVector< wxDataViewColumn * > wxDataViewCtrl_GetSortingColumns(const wxDataViewCtrl * self) {
    return self->GetSortingColumns();
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
// Mix-in(s) to wxDataViewCtrl
wxTrackable *wxDataViewCtrl_AsTrackable(wxDataViewCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
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
const wxVariant * wxDataViewEvent_GetValue(const wxDataViewEvent * self) {
    return self->GetValue();
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
void wxDataViewIndexListModel_delete(wxDataViewIndexListModel *self) {
    delete self;
}
wxDataViewIndexListModel *wxDataViewIndexListModel_new(unsigned int initial_size) {
    return new wxDataViewIndexListModel(initial_size);
}
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
void wxDataViewListCtrl_AppendColumn(wxDataViewListCtrl * self, wxDataViewColumn * column, const wxString * varianttype) {
    return self->AppendColumn(column, *varianttype);
}
void wxDataViewListCtrl_InsertColumn(wxDataViewListCtrl * self, unsigned int pos, wxDataViewColumn * column, const wxString * varianttype) {
    return self->InsertColumn(pos, column, *varianttype);
}
void wxDataViewListCtrl_PrependColumn(wxDataViewListCtrl * self, wxDataViewColumn * column, const wxString * varianttype) {
    return self->PrependColumn(column, *varianttype);
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
// Mix-in(s) to wxDataViewListCtrl
wxTrackable *wxDataViewListCtrl_AsTrackable(wxDataViewListCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxDataViewListModel
void wxDataViewListModel_delete(wxDataViewListModel *self) {
    delete self;
}
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
void wxDataViewListStore_delete(wxDataViewListStore *self) {
    delete self;
}
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
void wxDataViewModel_delete(wxDataViewModel *self) {
    delete self;
}
wxDataViewModel *wxDataViewModel_new() {
    return new wxDataViewModel();
}
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
wxDataViewModelNotifier *wxDataViewModelNotifier_new() {
    return new wxDataViewModelNotifier();
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
wxString *wxDataViewRenderer_GetAccessibleDescription(const wxDataViewRenderer * self) {
    return new wxString(self->GetAccessibleDescription());
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
// Mix-in(s) to wxDataViewTreeCtrl
wxTrackable *wxDataViewTreeCtrl_AsTrackable(wxDataViewTreeCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxDataViewTreeStore
void wxDataViewTreeStore_delete(wxDataViewTreeStore *self) {
    delete self;
}
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
void wxDataViewVirtualListModel_delete(wxDataViewVirtualListModel *self) {
    delete self;
}
wxDataViewVirtualListModel *wxDataViewVirtualListModel_new(unsigned int initial_size) {
    return new wxDataViewVirtualListModel(initial_size);
}
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
// Mix-in(s) to wxDatePickerCtrl
wxTrackable *wxDatePickerCtrl_AsTrackable(wxDatePickerCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
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

// CLASS: wxDialUpEvent
wxClassInfo *wxDialUpEvent_CLASSINFO() {
    return wxCLASSINFO(wxDialUpEvent);
}
wxDialUpEvent *wxDialUpEvent_new(bool is_connected, bool is_own_event) {
    return new wxDialUpEvent(is_connected, is_own_event);
}
bool wxDialUpEvent_IsConnectedEvent(const wxDialUpEvent * self) {
    return self->IsConnectedEvent();
}
bool wxDialUpEvent_IsOwnEvent(const wxDialUpEvent * self) {
    return self->IsOwnEvent();
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
wxSizer * wxDialog_CreateTextSizer(wxDialog * self, const wxString * message, int width_max) {
    return self->CreateTextSizer(*message, width_max);
}
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
wxToolBar * wxDialog_GetToolBar(const wxDialog * self) {
    return self->GetToolBar();
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
void wxDialog_ShowWindowModalThenDo(wxDialog * self, const Functor * on_end_modal) {
    return self->ShowWindowModalThenDo(*on_end_modal);
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
// Mix-in(s) to wxDialog
wxTrackable *wxDialog_AsTrackable(wxDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxDialogLayoutAdapter
void wxDialogLayoutAdapter_delete(wxDialogLayoutAdapter *self) {
    delete self;
}
wxDialogLayoutAdapter *wxDialogLayoutAdapter_new() {
    return new wxDialogLayoutAdapter();
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
// Mix-in(s) to wxDirDialog
wxTrackable *wxDirDialog_AsTrackable(wxDirDialog* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxDirPickerCtrl
wxTrackable *wxDirPickerCtrl_AsTrackable(wxDirPickerCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxDocChildFrame
wxTrackable *wxDocChildFrame_AsTrackable(wxDocChildFrame* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxDocMDIChildFrame
wxTrackable *wxDocMDIChildFrame_AsTrackable(wxDocMDIChildFrame* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxDocMDIParentFrame
wxTrackable *wxDocMDIParentFrame_AsTrackable(wxDocMDIParentFrame* obj) {
    return static_cast<wxTrackable*>(obj);
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
wxList * wxDocManager_GetDocuments(wxDocManager * self) {
    return self->GetDocuments();
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
wxList * wxDocManager_GetTemplates(wxDocManager * self) {
    return self->GetTemplates();
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
// Mix-in(s) to wxDocManager
wxTrackable *wxDocManager_AsTrackable(wxDocManager* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxDocParentFrame
wxTrackable *wxDocParentFrame_AsTrackable(wxDocParentFrame* obj) {
    return static_cast<wxTrackable*>(obj);
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
wxPageSetupDialogData * wxDocTemplate_GetPageSetupDialogData(wxDocTemplate * self) {
    return &(self->GetPageSetupDialogData());
}
wxPageSetupDialogData *wxDocTemplate_GetPageSetupDialogData1(const wxDocTemplate * self) {
    return new wxPageSetupDialogData(self->GetPageSetupDialogData());
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
void wxDocument_Activate(const wxDocument * self) {
    return self->Activate();
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
wxList * wxDocument_GetViews(wxDocument * self) {
    return self->GetViews();
}
const wxList * wxDocument_GetViews1(const wxDocument * self) {
    return self->GetViews();
}
bool wxDocument_IsChildDocument(const wxDocument * self) {
    return self->IsChildDocument();
}
bool wxDocument_IsModified(const wxDocument * self) {
    return self->IsModified();
}
istream * wxDocument_LoadObject(wxDocument * self, istream * stream) {
    return self->LoadObject(*stream);
}
wxInputStream * wxDocument_LoadObject1(wxDocument * self, wxInputStream * stream) {
    return self->LoadObject(*stream);
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
ostream * wxDocument_SaveObject(wxDocument * self, ostream * stream) {
    return self->SaveObject(*stream);
}
wxOutputStream * wxDocument_SaveObject1(wxDocument * self, wxOutputStream * stream) {
    return self->SaveObject(*stream);
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
// Mix-in(s) to wxDocument
wxTrackable *wxDocument_AsTrackable(wxDocument* obj) {
    return static_cast<wxTrackable*>(obj);
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
wxString *wxDropFilesEvent_GetFiles(const wxDropFilesEvent * self) {
    return new wxString(self->GetFiles());
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
wxDropSource *wxDropSource_new2(wxWindow * win, const wxIcon * icon_copy, const wxIcon * icon_move, const wxIcon * icon_none) {
    return new wxDropSource(win, *icon_copy, *icon_move, *icon_none);
}
wxDropSource *wxDropSource_new3(wxDataObject * data, wxWindow * win, const wxIcon * icon_copy, const wxIcon * icon_move, const wxIcon * icon_none) {
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

// CLASS: wxEditableListBox
wxClassInfo *wxEditableListBox_CLASSINFO() {
    return wxCLASSINFO(wxEditableListBox);
}
wxEditableListBox *wxEditableListBox_new() {
    return new wxEditableListBox();
}
wxEditableListBox *wxEditableListBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxEditableListBox(parent, id, *label, *pos, *size, style, *name);
}
bool wxEditableListBox_Create(wxEditableListBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}
void wxEditableListBox_SetStrings(wxEditableListBox * self, const wxArrayString * strings) {
    return self->SetStrings(*strings);
}
void wxEditableListBox_GetStrings(const wxEditableListBox * self, wxArrayString * strings) {
    return self->GetStrings(*strings);
}
// Mix-in(s) to wxEditableListBox
wxTrackable *wxEditableListBox_AsTrackable(wxEditableListBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxEraseEvent
wxClassInfo *wxEraseEvent_CLASSINFO() {
    return wxCLASSINFO(wxEraseEvent);
}
wxEraseEvent *wxEraseEvent_new(int id, wxDC * dc) {
    return new wxEraseEvent(id, dc);
}
wxDC * wxEraseEvent_GetDC(const wxEraseEvent * self) {
    return self->GetDC();
}

// CLASS: wxEventBlocker
wxClassInfo *wxEventBlocker_CLASSINFO() {
    return wxCLASSINFO(wxEventBlocker);
}
// Mix-in(s) to wxEventBlocker
wxTrackable *wxEventBlocker_AsTrackable(wxEventBlocker* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxExtHelpController
wxClassInfo *wxExtHelpController_CLASSINFO() {
    return wxCLASSINFO(wxExtHelpController);
}
wxExtHelpController *wxExtHelpController_new(wxWindow * parent_window) {
    return new wxExtHelpController(parent_window);
}
bool wxExtHelpController_DisplayHelp(wxExtHelpController * self, const wxString * relative_url) {
    return self->DisplayHelp(*relative_url);
}

// CLASS: wxFileCtrl
wxClassInfo *wxFileCtrl_CLASSINFO() {
    return wxCLASSINFO(wxFileCtrl);
}
wxFileCtrl *wxFileCtrl_new() {
    return new wxFileCtrl();
}
wxFileCtrl *wxFileCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * default_directory, const wxString * default_filename, const wxString * wild_card, long style, const wxPoint * pos, const wxSize * size, const wxString * name) {
    return new wxFileCtrl(parent, id, *default_directory, *default_filename, *wild_card, style, *pos, *size, *name);
}
bool wxFileCtrl_Create(wxFileCtrl * self, wxWindow * parent, wxWindowID id, const wxString * default_directory, const wxString * default_filename, const wxString * wild_card, long style, const wxPoint * pos, const wxSize * size, const wxString * name) {
    return self->Create(parent, id, *default_directory, *default_filename, *wild_card, style, *pos, *size, *name);
}
wxString *wxFileCtrl_GetDirectory(const wxFileCtrl * self) {
    return new wxString(self->GetDirectory());
}
wxString *wxFileCtrl_GetFilename(const wxFileCtrl * self) {
    return new wxString(self->GetFilename());
}
void wxFileCtrl_GetFilenames(const wxFileCtrl * self, wxArrayString * filenames) {
    return self->GetFilenames(*filenames);
}
int wxFileCtrl_GetFilterIndex(const wxFileCtrl * self) {
    return self->GetFilterIndex();
}
wxString *wxFileCtrl_GetPath(const wxFileCtrl * self) {
    return new wxString(self->GetPath());
}
void wxFileCtrl_GetPaths(const wxFileCtrl * self, wxArrayString * paths) {
    return self->GetPaths(*paths);
}
wxString *wxFileCtrl_GetWildcard(const wxFileCtrl * self) {
    return new wxString(self->GetWildcard());
}
bool wxFileCtrl_SetDirectory(wxFileCtrl * self, const wxString * directory) {
    return self->SetDirectory(*directory);
}
bool wxFileCtrl_SetFilename(wxFileCtrl * self, const wxString * filename) {
    return self->SetFilename(*filename);
}
bool wxFileCtrl_SetPath(wxFileCtrl * self, const wxString * path) {
    return self->SetPath(*path);
}
void wxFileCtrl_SetFilterIndex(wxFileCtrl * self, int filter_index) {
    return self->SetFilterIndex(filter_index);
}
void wxFileCtrl_SetWildcard(wxFileCtrl * self, const wxString * wild_card) {
    return self->SetWildcard(*wild_card);
}
void wxFileCtrl_ShowHidden(wxFileCtrl * self, bool show) {
    return self->ShowHidden(show);
}
// Mix-in(s) to wxFileCtrl
wxTrackable *wxFileCtrl_AsTrackable(wxFileCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFileCtrlEvent
wxClassInfo *wxFileCtrlEvent_CLASSINFO() {
    return wxCLASSINFO(wxFileCtrlEvent);
}
wxString *wxFileCtrlEvent_GetDirectory(const wxFileCtrlEvent * self) {
    return new wxString(self->GetDirectory());
}
wxString *wxFileCtrlEvent_GetFile(const wxFileCtrlEvent * self) {
    return new wxString(self->GetFile());
}
wxArrayString *wxFileCtrlEvent_GetFiles(const wxFileCtrlEvent * self) {
    return new wxArrayString(self->GetFiles());
}
int wxFileCtrlEvent_GetFilterIndex(const wxFileCtrlEvent * self) {
    return self->GetFilterIndex();
}
void wxFileCtrlEvent_SetFiles(wxFileCtrlEvent * self, const wxArrayString * files) {
    return self->SetFiles(*files);
}
void wxFileCtrlEvent_SetDirectory(wxFileCtrlEvent * self, const wxString * directory) {
    return self->SetDirectory(*directory);
}
void wxFileCtrlEvent_SetFilterIndex(wxFileCtrlEvent * self, int index) {
    return self->SetFilterIndex(index);
}

// CLASS: wxFileDataObject
void wxFileDataObject_delete(wxFileDataObject *self) {
    delete self;
}
wxFileDataObject *wxFileDataObject_new() {
    return new wxFileDataObject();
}
void wxFileDataObject_AddFile(wxFileDataObject * self, const wxString * file) {
    return self->AddFile(*file);
}
wxArrayString *wxFileDataObject_GetFilenames(const wxFileDataObject * self) {
    return new wxArrayString(self->GetFilenames());
}

// CLASS: wxFileDialog
wxClassInfo *wxFileDialog_CLASSINFO() {
    return wxCLASSINFO(wxFileDialog);
}
wxFileDialog *wxFileDialog_new(wxWindow * parent, const wxString * message, const wxString * default_dir, const wxString * default_file, const wxString * wildcard, long style, const wxPoint * pos, const wxSize * size, const wxString * name) {
    return new wxFileDialog(parent, *message, *default_dir, *default_file, *wildcard, style, *pos, *size, *name);
}
wxString *wxFileDialog_GetCurrentlySelectedFilename(const wxFileDialog * self) {
    return new wxString(self->GetCurrentlySelectedFilename());
}
int wxFileDialog_GetCurrentlySelectedFilterIndex(const wxFileDialog * self) {
    return self->GetCurrentlySelectedFilterIndex();
}
wxString *wxFileDialog_GetDirectory(const wxFileDialog * self) {
    return new wxString(self->GetDirectory());
}
wxWindow * wxFileDialog_GetExtraControl(const wxFileDialog * self) {
    return self->GetExtraControl();
}
wxString *wxFileDialog_GetFilename(const wxFileDialog * self) {
    return new wxString(self->GetFilename());
}
void wxFileDialog_GetFilenames(const wxFileDialog * self, wxArrayString * filenames) {
    return self->GetFilenames(*filenames);
}
int wxFileDialog_GetFilterIndex(const wxFileDialog * self) {
    return self->GetFilterIndex();
}
wxString *wxFileDialog_GetMessage(const wxFileDialog * self) {
    return new wxString(self->GetMessage());
}
wxString *wxFileDialog_GetPath(const wxFileDialog * self) {
    return new wxString(self->GetPath());
}
void wxFileDialog_GetPaths(const wxFileDialog * self, wxArrayString * paths) {
    return self->GetPaths(*paths);
}
wxString *wxFileDialog_GetWildcard(const wxFileDialog * self) {
    return new wxString(self->GetWildcard());
}
bool wxFileDialog_SetCustomizeHook(wxFileDialog * self, wxFileDialogCustomizeHook * customize_hook) {
    return self->SetCustomizeHook(*customize_hook);
}
void wxFileDialog_SetDirectory(wxFileDialog * self, const wxString * directory) {
    return self->SetDirectory(*directory);
}
void wxFileDialog_SetFilename(wxFileDialog * self, const wxString * setfilename) {
    return self->SetFilename(*setfilename);
}
void wxFileDialog_SetFilterIndex(wxFileDialog * self, int filter_index) {
    return self->SetFilterIndex(filter_index);
}
void wxFileDialog_SetMessage(wxFileDialog * self, const wxString * message) {
    return self->SetMessage(*message);
}
void wxFileDialog_SetPath(wxFileDialog * self, const wxString * path) {
    return self->SetPath(*path);
}
void wxFileDialog_SetWildcard(wxFileDialog * self, const wxString * wild_card) {
    return self->SetWildcard(*wild_card);
}
// Mix-in(s) to wxFileDialog
wxTrackable *wxFileDialog_AsTrackable(wxFileDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFileDialogCustomize
void wxFileDialogCustomize_delete(wxFileDialogCustomize *self) {
    delete self;
}
wxFileDialogButton * wxFileDialogCustomize_AddButton(wxFileDialogCustomize * self, const wxString * label) {
    return self->AddButton(*label);
}
wxFileDialogCheckBox * wxFileDialogCustomize_AddCheckBox(wxFileDialogCustomize * self, const wxString * label) {
    return self->AddCheckBox(*label);
}
wxFileDialogRadioButton * wxFileDialogCustomize_AddRadioButton(wxFileDialogCustomize * self, const wxString * label) {
    return self->AddRadioButton(*label);
}
wxFileDialogChoice * wxFileDialogCustomize_AddChoice(wxFileDialogCustomize * self, size_t n, const wxString * strings) {
    return self->AddChoice(n, strings);
}
wxFileDialogTextCtrl * wxFileDialogCustomize_AddTextCtrl(wxFileDialogCustomize * self, const wxString * label) {
    return self->AddTextCtrl(*label);
}
wxFileDialogStaticText * wxFileDialogCustomize_AddStaticText(wxFileDialogCustomize * self, const wxString * label) {
    return self->AddStaticText(*label);
}

// CLASS: wxFileDialogCustomizeHook
void wxFileDialogCustomizeHook_delete(wxFileDialogCustomizeHook *self) {
    delete self;
}
void wxFileDialogCustomizeHook_AddCustomControls(wxFileDialogCustomizeHook * self, wxFileDialogCustomize * customizer) {
    return self->AddCustomControls(*customizer);
}
void wxFileDialogCustomizeHook_UpdateCustomControls(wxFileDialogCustomizeHook * self) {
    return self->UpdateCustomControls();
}
void wxFileDialogCustomizeHook_TransferDataFromCustomControls(wxFileDialogCustomizeHook * self) {
    return self->TransferDataFromCustomControls();
}

// CLASS: wxFileDirPickerEvent
wxClassInfo *wxFileDirPickerEvent_CLASSINFO() {
    return wxCLASSINFO(wxFileDirPickerEvent);
}
wxFileDirPickerEvent *wxFileDirPickerEvent_new() {
    return new wxFileDirPickerEvent();
}
wxString *wxFileDirPickerEvent_GetPath(const wxFileDirPickerEvent * self) {
    return new wxString(self->GetPath());
}
void wxFileDirPickerEvent_SetPath(wxFileDirPickerEvent * self, const wxString * path) {
    return self->SetPath(*path);
}

// CLASS: wxFileDropTarget
void wxFileDropTarget_delete(wxFileDropTarget *self) {
    delete self;
}
wxFileDropTarget *wxFileDropTarget_new() {
    return new wxFileDropTarget();
}
bool wxFileDropTarget_OnDropFiles(wxFileDropTarget * self, wxCoord x, wxCoord y, const wxArrayString * filenames) {
    return self->OnDropFiles(x, y, *filenames);
}

// CLASS: wxFileHistory
wxClassInfo *wxFileHistory_CLASSINFO() {
    return wxCLASSINFO(wxFileHistory);
}
wxFileHistory *wxFileHistory_new(size_t max_files, wxWindowID id_base) {
    return new wxFileHistory(max_files, id_base);
}
void wxFileHistory_AddFileToHistory(wxFileHistory * self, const wxString * filename) {
    return self->AddFileToHistory(*filename);
}
void wxFileHistory_AddFilesToMenu(wxFileHistory * self) {
    return self->AddFilesToMenu();
}
void wxFileHistory_AddFilesToMenu1(wxFileHistory * self, wxMenu * menu) {
    return self->AddFilesToMenu(menu);
}
wxWindowID wxFileHistory_GetBaseId(const wxFileHistory * self) {
    return self->GetBaseId();
}
size_t wxFileHistory_GetCount(const wxFileHistory * self) {
    return self->GetCount();
}
wxString *wxFileHistory_GetHistoryFile(const wxFileHistory * self, size_t index) {
    return new wxString(self->GetHistoryFile(index));
}
int wxFileHistory_GetMaxFiles(const wxFileHistory * self) {
    return self->GetMaxFiles();
}
const wxList * wxFileHistory_GetMenus(const wxFileHistory * self) {
    return self->GetMenus();
}
void wxFileHistory_Load(wxFileHistory * self, const wxConfigBase * config) {
    return self->Load(*config);
}
void wxFileHistory_RemoveFileFromHistory(wxFileHistory * self, size_t i) {
    return self->RemoveFileFromHistory(i);
}
void wxFileHistory_RemoveMenu(wxFileHistory * self, wxMenu * menu) {
    return self->RemoveMenu(menu);
}
void wxFileHistory_Save(wxFileHistory * self, wxConfigBase * config) {
    return self->Save(*config);
}
void wxFileHistory_SetBaseId(wxFileHistory * self, wxWindowID base_id) {
    return self->SetBaseId(base_id);
}
void wxFileHistory_UseMenu(wxFileHistory * self, wxMenu * menu) {
    return self->UseMenu(menu);
}

// CLASS: wxFilePickerCtrl
wxClassInfo *wxFilePickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxFilePickerCtrl);
}
wxFilePickerCtrl *wxFilePickerCtrl_new() {
    return new wxFilePickerCtrl();
}
wxFilePickerCtrl *wxFilePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxString * wildcard, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxFilePickerCtrl(parent, id, *path, *message, *wildcard, *pos, *size, style, *validator, *name);
}
bool wxFilePickerCtrl_Create(wxFilePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxString * wildcard, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *path, *message, *wildcard, *pos, *size, style, *validator, *name);
}
wxFileName *wxFilePickerCtrl_GetFileName(const wxFilePickerCtrl * self) {
    return new wxFileName(self->GetFileName());
}
wxString *wxFilePickerCtrl_GetPath(const wxFilePickerCtrl * self) {
    return new wxString(self->GetPath());
}
void wxFilePickerCtrl_SetFileName(wxFilePickerCtrl * self, const wxFileName * filename) {
    return self->SetFileName(*filename);
}
void wxFilePickerCtrl_SetInitialDirectory(wxFilePickerCtrl * self, const wxString * dir) {
    return self->SetInitialDirectory(*dir);
}
void wxFilePickerCtrl_SetPath(wxFilePickerCtrl * self, const wxString * filename) {
    return self->SetPath(*filename);
}
// Mix-in(s) to wxFilePickerCtrl
wxTrackable *wxFilePickerCtrl_AsTrackable(wxFilePickerCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFindDialogEvent
wxClassInfo *wxFindDialogEvent_CLASSINFO() {
    return wxCLASSINFO(wxFindDialogEvent);
}
wxFindReplaceDialog * wxFindDialogEvent_GetDialog(const wxFindDialogEvent * self) {
    return self->GetDialog();
}
wxString *wxFindDialogEvent_GetFindString(const wxFindDialogEvent * self) {
    return new wxString(self->GetFindString());
}
int wxFindDialogEvent_GetFlags(const wxFindDialogEvent * self) {
    return self->GetFlags();
}
wxString *wxFindDialogEvent_GetReplaceString(const wxFindDialogEvent * self) {
    return new wxString(self->GetReplaceString());
}

// CLASS: wxFindReplaceData
wxClassInfo *wxFindReplaceData_CLASSINFO() {
    return wxCLASSINFO(wxFindReplaceData);
}
wxString *wxFindReplaceData_GetFindString(const wxFindReplaceData * self) {
    return new wxString(self->GetFindString());
}
int wxFindReplaceData_GetFlags(const wxFindReplaceData * self) {
    return self->GetFlags();
}
wxString *wxFindReplaceData_GetReplaceString(const wxFindReplaceData * self) {
    return new wxString(self->GetReplaceString());
}
void wxFindReplaceData_SetFindString(wxFindReplaceData * self, const wxString * str) {
    return self->SetFindString(*str);
}
void wxFindReplaceData_SetReplaceString(wxFindReplaceData * self, const wxString * str) {
    return self->SetReplaceString(*str);
}

// CLASS: wxFindReplaceDialog
wxClassInfo *wxFindReplaceDialog_CLASSINFO() {
    return wxCLASSINFO(wxFindReplaceDialog);
}
wxFindReplaceDialog *wxFindReplaceDialog_new() {
    return new wxFindReplaceDialog();
}
wxFindReplaceDialog *wxFindReplaceDialog_new1(wxWindow * parent, wxFindReplaceData * data, const wxString * title, int style) {
    return new wxFindReplaceDialog(parent, data, *title, style);
}
bool wxFindReplaceDialog_Create(wxFindReplaceDialog * self, wxWindow * parent, wxFindReplaceData * data, const wxString * title, int style) {
    return self->Create(parent, data, *title, style);
}
const wxFindReplaceData * wxFindReplaceDialog_GetData(const wxFindReplaceDialog * self) {
    return self->GetData();
}
// Mix-in(s) to wxFindReplaceDialog
wxTrackable *wxFindReplaceDialog_AsTrackable(wxFindReplaceDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFlexGridSizer
wxClassInfo *wxFlexGridSizer_CLASSINFO() {
    return wxCLASSINFO(wxFlexGridSizer);
}
wxFlexGridSizer *wxFlexGridSizer_new(int cols, int vgap, int hgap) {
    return new wxFlexGridSizer(cols, vgap, hgap);
}
wxFlexGridSizer *wxFlexGridSizer_new1(int cols, const wxSize * gap) {
    return new wxFlexGridSizer(cols, *gap);
}
wxFlexGridSizer *wxFlexGridSizer_new2(int rows, int cols, int vgap, int hgap) {
    return new wxFlexGridSizer(rows, cols, vgap, hgap);
}
wxFlexGridSizer *wxFlexGridSizer_new3(int rows, int cols, const wxSize * gap) {
    return new wxFlexGridSizer(rows, cols, *gap);
}
void wxFlexGridSizer_AddGrowableCol(wxFlexGridSizer * self, size_t idx, int proportion) {
    return self->AddGrowableCol(idx, proportion);
}
void wxFlexGridSizer_AddGrowableRow(wxFlexGridSizer * self, size_t idx, int proportion) {
    return self->AddGrowableRow(idx, proportion);
}
int wxFlexGridSizer_GetFlexibleDirection(const wxFlexGridSizer * self) {
    return self->GetFlexibleDirection();
}
bool wxFlexGridSizer_IsColGrowable(wxFlexGridSizer * self, size_t idx) {
    return self->IsColGrowable(idx);
}
bool wxFlexGridSizer_IsRowGrowable(wxFlexGridSizer * self, size_t idx) {
    return self->IsRowGrowable(idx);
}
void wxFlexGridSizer_RemoveGrowableCol(wxFlexGridSizer * self, size_t idx) {
    return self->RemoveGrowableCol(idx);
}
void wxFlexGridSizer_RemoveGrowableRow(wxFlexGridSizer * self, size_t idx) {
    return self->RemoveGrowableRow(idx);
}
void wxFlexGridSizer_SetFlexibleDirection(wxFlexGridSizer * self, int direction) {
    return self->SetFlexibleDirection(direction);
}
wxArrayInt *wxFlexGridSizer_GetRowHeights(const wxFlexGridSizer * self) {
    return new wxArrayInt(self->GetRowHeights());
}
wxArrayInt *wxFlexGridSizer_GetColWidths(const wxFlexGridSizer * self) {
    return new wxArrayInt(self->GetColWidths());
}

// CLASS: wxFocusEvent
wxClassInfo *wxFocusEvent_CLASSINFO() {
    return wxCLASSINFO(wxFocusEvent);
}
wxWindow * wxFocusEvent_GetWindow(const wxFocusEvent * self) {
    return self->GetWindow();
}
void wxFocusEvent_SetWindow(wxFocusEvent * self, wxWindow * win) {
    return self->SetWindow(win);
}

// CLASS: wxFont
wxClassInfo *wxFont_CLASSINFO() {
    return wxCLASSINFO(wxFont);
}
#if wxCHECK_VERSION(3, 1, 0)
wxFont *wxFont_GetBaseFont(const wxFont * self) {
    return new wxFont(self->GetBaseFont());
}
#endif
wxString *wxFont_GetFaceName(const wxFont * self) {
    return new wxString(self->GetFaceName());
}
wxString *wxFont_GetNativeFontInfoDesc(const wxFont * self) {
    return new wxString(self->GetNativeFontInfoDesc());
}
wxString *wxFont_GetNativeFontInfoUserDesc(const wxFont * self) {
    return new wxString(self->GetNativeFontInfoUserDesc());
}
const wxNativeFontInfo * wxFont_GetNativeFontInfo(const wxFont * self) {
    return self->GetNativeFontInfo();
}
int wxFont_GetPointSize(const wxFont * self) {
    return self->GetPointSize();
}
#if wxCHECK_VERSION(3, 1, 0)
double wxFont_GetFractionalPointSize(const wxFont * self) {
    return self->GetFractionalPointSize();
}
#endif
wxSize *wxFont_GetPixelSize(const wxFont * self) {
    return new wxSize(self->GetPixelSize());
}
bool wxFont_GetUnderlined(const wxFont * self) {
    return self->GetUnderlined();
}
bool wxFont_GetStrikethrough(const wxFont * self) {
    return self->GetStrikethrough();
}
#if wxCHECK_VERSION(3, 1, 0)
int wxFont_GetNumericWeight(const wxFont * self) {
    return self->GetNumericWeight();
}
#endif
bool wxFont_IsFixedWidth(const wxFont * self) {
    return self->IsFixedWidth();
}
bool wxFont_IsOk(const wxFont * self) {
    return self->IsOk();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxFont_AddPrivateFont(const wxString * filename) {
    return wxFont::AddPrivateFont(*filename);
}
#endif
wxFont *wxFont_Bold(const wxFont * self) {
    return new wxFont(self->Bold());
}
wxFont *wxFont_Italic(const wxFont * self) {
    return new wxFont(self->Italic());
}
wxFont *wxFont_Larger(const wxFont * self) {
    return new wxFont(self->Larger());
}
wxFont *wxFont_Smaller(const wxFont * self) {
    return new wxFont(self->Smaller());
}
wxFont *wxFont_Underlined(const wxFont * self) {
    return new wxFont(self->Underlined());
}
wxFont *wxFont_Strikethrough(const wxFont * self) {
    return new wxFont(self->Strikethrough());
}
wxFont * wxFont_MakeBold(wxFont * self) {
    return &(self->MakeBold());
}
wxFont * wxFont_MakeItalic(wxFont * self) {
    return &(self->MakeItalic());
}
wxFont * wxFont_MakeLarger(wxFont * self) {
    return &(self->MakeLarger());
}
wxFont * wxFont_MakeSmaller(wxFont * self) {
    return &(self->MakeSmaller());
}
wxFont * wxFont_MakeUnderlined(wxFont * self) {
    return &(self->MakeUnderlined());
}
wxFont * wxFont_MakeStrikethrough(wxFont * self) {
    return &(self->MakeStrikethrough());
}
bool wxFont_SetFaceName(wxFont * self, const wxString * face_name) {
    return self->SetFaceName(*face_name);
}
bool wxFont_SetNativeFontInfo(wxFont * self, const wxString * info) {
    return self->SetNativeFontInfo(*info);
}
bool wxFont_SetNativeFontInfoUserDesc(wxFont * self, const wxString * info) {
    return self->SetNativeFontInfoUserDesc(*info);
}
void wxFont_SetNativeFontInfo1(wxFont * self, const wxNativeFontInfo * info) {
    return self->SetNativeFontInfo(*info);
}
void wxFont_SetPointSize(wxFont * self, int point_size) {
    return self->SetPointSize(point_size);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxFont_SetFractionalPointSize(wxFont * self, double point_size) {
    return self->SetFractionalPointSize(point_size);
}
#endif
void wxFont_SetPixelSize(wxFont * self, const wxSize * pixel_size) {
    return self->SetPixelSize(*pixel_size);
}
void wxFont_SetUnderlined(wxFont * self, bool underlined) {
    return self->SetUnderlined(underlined);
}
void wxFont_SetStrikethrough(wxFont * self, bool strikethrough) {
    return self->SetStrikethrough(strikethrough);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxFont_SetNumericWeight(wxFont * self, int weight) {
    return self->SetNumericWeight(weight);
}
#endif
wxFont * wxFont_New4(const wxNativeFontInfo * native_info) {
    return wxFont::New(*native_info);
}
wxFont * wxFont_New5(const wxString * native_info_string) {
    return wxFont::New(*native_info_string);
}
wxFont *wxFont_new() {
    return new wxFont();
}
wxFont *wxFont_new1(const wxFont * font) {
    return new wxFont(*font);
}
wxFont *wxFont_new2(const wxFontInfo * font_info) {
    return new wxFont(*font_info);
}
wxFont *wxFont_new5(const wxString * native_info_string) {
    return new wxFont(*native_info_string);
}
wxFont *wxFont_new6(const wxNativeFontInfo * native_info) {
    return new wxFont(*native_info);
}

// CLASS: wxFontData
wxClassInfo *wxFontData_CLASSINFO() {
    return wxCLASSINFO(wxFontData);
}
wxFontData *wxFontData_new() {
    return new wxFontData();
}
void wxFontData_EnableEffects(wxFontData * self, bool enable) {
    return self->EnableEffects(enable);
}
bool wxFontData_GetAllowSymbols(const wxFontData * self) {
    return self->GetAllowSymbols();
}
wxFont *wxFontData_GetChosenFont(const wxFontData * self) {
    return new wxFont(self->GetChosenFont());
}
wxColour *wxFontData_GetColour(const wxFontData * self) {
    return new wxColour(self->GetColour());
}
bool wxFontData_GetEnableEffects(const wxFontData * self) {
    return self->GetEnableEffects();
}
int wxFontData_GetRestrictSelection(const wxFontData * self) {
    return self->GetRestrictSelection();
}
wxFont *wxFontData_GetInitialFont(const wxFontData * self) {
    return new wxFont(self->GetInitialFont());
}
bool wxFontData_GetShowHelp(const wxFontData * self) {
    return self->GetShowHelp();
}
void wxFontData_RestrictSelection(wxFontData * self, int flags) {
    return self->RestrictSelection(flags);
}
void wxFontData_SetAllowSymbols(wxFontData * self, bool allow_symbols) {
    return self->SetAllowSymbols(allow_symbols);
}
void wxFontData_SetChosenFont(wxFontData * self, const wxFont * font) {
    return self->SetChosenFont(*font);
}
void wxFontData_SetColour(wxFontData * self, const wxColour * colour) {
    return self->SetColour(*colour);
}
void wxFontData_SetInitialFont(wxFontData * self, const wxFont * font) {
    return self->SetInitialFont(*font);
}
void wxFontData_SetRange(wxFontData * self, int min, int max) {
    return self->SetRange(min, max);
}
void wxFontData_SetShowHelp(wxFontData * self, bool show_help) {
    return self->SetShowHelp(show_help);
}

// CLASS: wxFontDialog
wxClassInfo *wxFontDialog_CLASSINFO() {
    return wxCLASSINFO(wxFontDialog);
}
wxFontDialog *wxFontDialog_new() {
    return new wxFontDialog();
}
wxFontDialog *wxFontDialog_new1(wxWindow * parent) {
    return new wxFontDialog(parent);
}
wxFontDialog *wxFontDialog_new2(wxWindow * parent, const wxFontData * data) {
    return new wxFontDialog(parent, *data);
}
bool wxFontDialog_Create(wxFontDialog * self, wxWindow * parent) {
    return self->Create(parent);
}
bool wxFontDialog_Create1(wxFontDialog * self, wxWindow * parent, const wxFontData * data) {
    return self->Create(parent, *data);
}
wxFontData *wxFontDialog_GetFontData(const wxFontDialog * self) {
    return new wxFontData(self->GetFontData());
}
wxFontData * wxFontDialog_GetFontData1(wxFontDialog * self) {
    return &(self->GetFontData());
}
// Mix-in(s) to wxFontDialog
wxTrackable *wxFontDialog_AsTrackable(wxFontDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFontEnumerator
void wxFontEnumerator_delete(wxFontEnumerator *self) {
    delete self;
}
wxFontEnumerator *wxFontEnumerator_new() {
    return new wxFontEnumerator();
}
bool wxFontEnumerator_EnumerateEncodings(wxFontEnumerator * self, const wxString * font) {
    return self->EnumerateEncodings(*font);
}
bool wxFontEnumerator_OnFacename(wxFontEnumerator * self, const wxString * font) {
    return self->OnFacename(*font);
}
bool wxFontEnumerator_OnFontEncoding(wxFontEnumerator * self, const wxString * font, const wxString * encoding) {
    return self->OnFontEncoding(*font, *encoding);
}
wxArrayString *wxFontEnumerator_GetEncodings(const wxString * facename) {
    return new wxArrayString(wxFontEnumerator::GetEncodings(*facename));
}
bool wxFontEnumerator_IsValidFacename(const wxString * facename) {
    return wxFontEnumerator::IsValidFacename(*facename);
}
void wxFontEnumerator_InvalidateCache() {
    return wxFontEnumerator::InvalidateCache();
}

// CLASS: wxFontList
void wxFontList_delete(wxFontList *self) {
    delete self;
}
wxFontList *wxFontList_new() {
    return new wxFontList();
}
wxFont * wxFontList_FindOrCreateFont1(wxFontList * self, const wxFontInfo * font_info) {
    return self->FindOrCreateFont(*font_info);
}

// CLASS: wxFontMapper
void wxFontMapper_delete(wxFontMapper *self) {
    delete self;
}
wxFontMapper *wxFontMapper_new() {
    return new wxFontMapper();
}
void wxFontMapper_SetConfigPath(wxFontMapper * self, const wxString * prefix) {
    return self->SetConfigPath(*prefix);
}
void wxFontMapper_SetDialogParent(wxFontMapper * self, wxWindow * parent) {
    return self->SetDialogParent(parent);
}
void wxFontMapper_SetDialogTitle(wxFontMapper * self, const wxString * title) {
    return self->SetDialogTitle(*title);
}
wxFontMapper * wxFontMapper_Get() {
    return wxFontMapper::Get();
}
size_t wxFontMapper_GetSupportedEncodingsCount() {
    return wxFontMapper::GetSupportedEncodingsCount();
}
wxFontMapper * wxFontMapper_Set(wxFontMapper * mapper) {
    return wxFontMapper::Set(mapper);
}

// CLASS: wxFontPickerCtrl
wxClassInfo *wxFontPickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxFontPickerCtrl);
}
wxFontPickerCtrl *wxFontPickerCtrl_new() {
    return new wxFontPickerCtrl();
}
wxFontPickerCtrl *wxFontPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxFont * font, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxFontPickerCtrl(parent, id, *font, *pos, *size, style, *validator, *name);
}
bool wxFontPickerCtrl_Create(wxFontPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxFont * font, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *font, *pos, *size, style, *validator, *name);
}
unsigned int wxFontPickerCtrl_GetMaxPointSize(const wxFontPickerCtrl * self) {
    return self->GetMaxPointSize();
}
#if wxCHECK_VERSION(3, 1, 0)
unsigned int wxFontPickerCtrl_GetMinPointSize(const wxFontPickerCtrl * self) {
    return self->GetMinPointSize();
}
wxColour *wxFontPickerCtrl_GetSelectedColour(const wxFontPickerCtrl * self) {
    return new wxColour(self->GetSelectedColour());
}
#endif
wxFont *wxFontPickerCtrl_GetSelectedFont(const wxFontPickerCtrl * self) {
    return new wxFont(self->GetSelectedFont());
}
void wxFontPickerCtrl_SetMaxPointSize(wxFontPickerCtrl * self, unsigned int max) {
    return self->SetMaxPointSize(max);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxFontPickerCtrl_SetMinPointSize(wxFontPickerCtrl * self, unsigned int min) {
    return self->SetMinPointSize(min);
}
void wxFontPickerCtrl_SetSelectedColour(wxFontPickerCtrl * self, const wxColour * colour) {
    return self->SetSelectedColour(*colour);
}
#endif
void wxFontPickerCtrl_SetSelectedFont(wxFontPickerCtrl * self, const wxFont * font) {
    return self->SetSelectedFont(*font);
}
// Mix-in(s) to wxFontPickerCtrl
wxTrackable *wxFontPickerCtrl_AsTrackable(wxFontPickerCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFontPickerEvent
wxClassInfo *wxFontPickerEvent_CLASSINFO() {
    return wxCLASSINFO(wxFontPickerEvent);
}
wxFontPickerEvent *wxFontPickerEvent_new(wxObject * generator, int id, const wxFont * font) {
    return new wxFontPickerEvent(generator, id, *font);
}
wxFont *wxFontPickerEvent_GetFont(const wxFontPickerEvent * self) {
    return new wxFont(self->GetFont());
}
void wxFontPickerEvent_SetFont(wxFontPickerEvent * self, const wxFont * f) {
    return self->SetFont(*f);
}

// CLASS: wxFrame
wxClassInfo *wxFrame_CLASSINFO() {
    return wxCLASSINFO(wxFrame);
}
wxFrame *wxFrame_new() {
    return new wxFrame();
}
wxFrame *wxFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxFrame(parent, id, *title, *pos, *size, style, *name);
}
void wxFrame_Centre(wxFrame * self, int direction) {
    return self->Centre(direction);
}
bool wxFrame_Create(wxFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxStatusBar * wxFrame_CreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name) {
    return self->CreateStatusBar(number, style, id, *name);
}
wxToolBar * wxFrame_CreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name) {
    return self->CreateToolBar(style, id, *name);
}
void wxFrame_DoGiveHelp(wxFrame * self, const wxString * text, bool show) {
    return self->DoGiveHelp(*text, show);
}
wxMenuBar * wxFrame_GetMenuBar(const wxFrame * self) {
    return self->GetMenuBar();
}
wxStatusBar * wxFrame_GetStatusBar(const wxFrame * self) {
    return self->GetStatusBar();
}
int wxFrame_GetStatusBarPane(const wxFrame * self) {
    return self->GetStatusBarPane();
}
wxToolBar * wxFrame_GetToolBar(const wxFrame * self) {
    return self->GetToolBar();
}
wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name) {
    return self->OnCreateStatusBar(number, style, id, *name);
}
wxToolBar * wxFrame_OnCreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name) {
    return self->OnCreateToolBar(style, id, *name);
}
bool wxFrame_ProcessCommand(wxFrame * self, int id) {
    return self->ProcessCommand(id);
}
void wxFrame_SetMenuBar(wxFrame * self, wxMenuBar * menu_bar) {
    return self->SetMenuBar(menu_bar);
}
void wxFrame_SetStatusBar(wxFrame * self, wxStatusBar * status_bar) {
    return self->SetStatusBar(status_bar);
}
void wxFrame_SetStatusBarPane(wxFrame * self, int n) {
    return self->SetStatusBarPane(n);
}
void wxFrame_SetStatusText(wxFrame * self, const wxString * text, int number) {
    return self->SetStatusText(*text, number);
}
void wxFrame_SetStatusWidths(wxFrame * self, int n, const int * widths_field) {
    return self->SetStatusWidths(n, widths_field);
}
void wxFrame_SetToolBar(wxFrame * self, wxToolBar * tool_bar) {
    return self->SetToolBar(tool_bar);
}
#ifdef __WXMSW__
wxTaskBarButton * wxFrame_MSWGetTaskBarButton(wxFrame * self) {
    return self->MSWGetTaskBarButton();
}
#endif
void wxFrame_PushStatusText(wxFrame * self, const wxString * text, int number) {
    return self->PushStatusText(*text, number);
}
void wxFrame_PopStatusText(wxFrame * self, int number) {
    return self->PopStatusText(number);
}
// Mix-in(s) to wxFrame
wxTrackable *wxFrame_AsTrackable(wxFrame* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFullScreenEvent
wxClassInfo *wxFullScreenEvent_CLASSINFO() {
    return wxCLASSINFO(wxFullScreenEvent);
}
wxFullScreenEvent *wxFullScreenEvent_new(int id, bool fullscreen) {
    return new wxFullScreenEvent(id, fullscreen);
}
bool wxFullScreenEvent_IsFullScreen(const wxFullScreenEvent * self) {
    return self->IsFullScreen();
}

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

// CLASS: wxGCDC
wxClassInfo *wxGCDC_CLASSINFO() {
    return wxCLASSINFO(wxGCDC);
}
wxGCDC *wxGCDC_new(const wxWindowDC * window_dc) {
    return new wxGCDC(*window_dc);
}
wxGCDC *wxGCDC_new1(const wxMemoryDC * memory_dc) {
    return new wxGCDC(*memory_dc);
}
wxGCDC *wxGCDC_new2(const wxPrinterDC * printer_dc) {
    return new wxGCDC(*printer_dc);
}
wxGCDC *wxGCDC_new3(wxGraphicsContext * context) {
    return new wxGCDC(context);
}
wxGCDC *wxGCDC_new4(const wxEnhMetaFileDC * emf_dc) {
    return new wxGCDC(*emf_dc);
}
wxGCDC *wxGCDC_new5() {
    return new wxGCDC();
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
// Mix-in(s) to wxGauge
wxTrackable *wxGauge_AsTrackable(wxGauge* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxGenericAnimationCtrl
wxTrackable *wxGenericAnimationCtrl_AsTrackable(wxGenericAnimationCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
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
wxString *wxGenericDirCtrl_GetPath1(const wxGenericDirCtrl * self, wxTreeItemId item_id) {
    return new wxString(self->GetPath(item_id));
}
void wxGenericDirCtrl_GetPaths(const wxGenericDirCtrl * self, wxArrayString * paths) {
    return self->GetPaths(*paths);
}
wxTreeItemId *wxGenericDirCtrl_GetRootId(wxGenericDirCtrl * self) {
    return new wxTreeItemId(self->GetRootId());
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
// Mix-in(s) to wxGenericDirCtrl
wxTrackable *wxGenericDirCtrl_AsTrackable(wxGenericDirCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxGenericProgressDialog
wxTrackable *wxGenericProgressDialog_AsTrackable(wxGenericProgressDialog* obj) {
    return static_cast<wxTrackable*>(obj);
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
// Mix-in(s) to wxGenericValidator
wxTrackable *wxGenericValidator_AsTrackable(wxGenericValidator* obj) {
    return static_cast<wxTrackable*>(obj);
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
wxGraphicsContext * wxGraphicsContext_Create4(const wxEnhMetaFileDC * meta_file_dc) {
    return wxGraphicsContext::Create(*meta_file_dc);
}
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
wxGraphicsBitmap *wxGraphicsContext_CreateBitmap(wxGraphicsContext * self, const wxBitmap * bitmap) {
    return new wxGraphicsBitmap(self->CreateBitmap(*bitmap));
}
wxGraphicsBitmap *wxGraphicsContext_CreateBitmapFromImage(wxGraphicsContext * self, const wxImage * image) {
    return new wxGraphicsBitmap(self->CreateBitmapFromImage(*image));
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
void wxGraphicsMatrix_Concat1(wxGraphicsMatrix * self, const wxGraphicsMatrix * t) {
    return self->Concat(*t);
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
bool wxGraphicsMatrix_IsEqual(const wxGraphicsMatrix * self, const wxGraphicsMatrix * t) {
    return self->IsEqual(t);
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
wxRect2DDouble *wxGraphicsPath_GetBox(const wxGraphicsPath * self) {
    return new wxRect2DDouble(self->GetBox());
}
void wxGraphicsPath_GetBox1(const wxGraphicsPath * self, wxDouble * x, wxDouble * y, wxDouble * w, wxDouble * h) {
    return self->GetBox(x, y, w, h);
}
void wxGraphicsPath_GetCurrentPoint(const wxGraphicsPath * self, wxDouble * x, wxDouble * y) {
    return self->GetCurrentPoint(x, y);
}
wxPoint2DDouble *wxGraphicsPath_GetCurrentPoint1(const wxGraphicsPath * self) {
    return new wxPoint2DDouble(self->GetCurrentPoint());
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
wxGraphicsBitmap *wxGraphicsRenderer_CreateBitmap(wxGraphicsRenderer * self, const wxBitmap * bitmap) {
    return new wxGraphicsBitmap(self->CreateBitmap(*bitmap));
}
wxGraphicsBitmap *wxGraphicsRenderer_CreateBitmapFromImage(wxGraphicsRenderer * self, const wxImage * image) {
    return new wxGraphicsBitmap(self->CreateBitmapFromImage(*image));
}
wxImage *wxGraphicsRenderer_CreateImageFromBitmap(wxGraphicsRenderer * self, const wxGraphicsBitmap * bmp) {
    return new wxImage(self->CreateImageFromBitmap(*bmp));
}
wxGraphicsBitmap *wxGraphicsRenderer_CreateBitmapFromNativeBitmap(wxGraphicsRenderer * self, void * bitmap) {
    return new wxGraphicsBitmap(self->CreateBitmapFromNativeBitmap(bitmap));
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
wxGraphicsContext * wxGraphicsRenderer_CreateContext4(wxGraphicsRenderer * self, const wxEnhMetaFileDC * meta_file_dc) {
    return self->CreateContext(*meta_file_dc);
}
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
wxGraphicsRenderer * wxGraphicsRenderer_GetGDIPlusRenderer() {
    return wxGraphicsRenderer::GetGDIPlusRenderer();
}
wxGraphicsRenderer * wxGraphicsRenderer_GetDirect2DRenderer() {
    return wxGraphicsRenderer::GetDirect2DRenderer();
}

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
void wxGridCellAttr_delete(wxGridCellAttr *self) {
    delete self;
}
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
void wxGridCellAutoWrapStringEditor_delete(wxGridCellAutoWrapStringEditor *self) {
    delete self;
}
wxGridCellAutoWrapStringEditor *wxGridCellAutoWrapStringEditor_new() {
    return new wxGridCellAutoWrapStringEditor();
}
// Mix-in(s) to wxGridCellAutoWrapStringEditor
wxRefCounter *wxGridCellAutoWrapStringEditor_AsRefCounter(wxGridCellAutoWrapStringEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellAutoWrapStringRenderer
void wxGridCellAutoWrapStringRenderer_delete(wxGridCellAutoWrapStringRenderer *self) {
    delete self;
}
wxGridCellAutoWrapStringRenderer *wxGridCellAutoWrapStringRenderer_new() {
    return new wxGridCellAutoWrapStringRenderer();
}
// Mix-in(s) to wxGridCellAutoWrapStringRenderer
wxRefCounter *wxGridCellAutoWrapStringRenderer_AsRefCounter(wxGridCellAutoWrapStringRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellBoolEditor
void wxGridCellBoolEditor_delete(wxGridCellBoolEditor *self) {
    delete self;
}
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
void wxGridCellBoolRenderer_delete(wxGridCellBoolRenderer *self) {
    delete self;
}
wxGridCellBoolRenderer *wxGridCellBoolRenderer_new() {
    return new wxGridCellBoolRenderer();
}
// Mix-in(s) to wxGridCellBoolRenderer
wxRefCounter *wxGridCellBoolRenderer_AsRefCounter(wxGridCellBoolRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellChoiceEditor
void wxGridCellChoiceEditor_delete(wxGridCellChoiceEditor *self) {
    delete self;
}
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
void wxGridCellDateEditor_delete(wxGridCellDateEditor *self) {
    delete self;
}
wxGridCellDateEditor *wxGridCellDateEditor_new(const wxString * format) {
    return new wxGridCellDateEditor(*format);
}
// Mix-in(s) to wxGridCellDateEditor
wxRefCounter *wxGridCellDateEditor_AsRefCounter(wxGridCellDateEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellDateRenderer
void wxGridCellDateRenderer_delete(wxGridCellDateRenderer *self) {
    delete self;
}
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
void wxGridCellDateTimeRenderer_delete(wxGridCellDateTimeRenderer *self) {
    delete self;
}
wxGridCellDateTimeRenderer *wxGridCellDateTimeRenderer_new(const wxString * outformat, const wxString * informat) {
    return new wxGridCellDateTimeRenderer(*outformat, *informat);
}
// Mix-in(s) to wxGridCellDateTimeRenderer
wxRefCounter *wxGridCellDateTimeRenderer_AsRefCounter(wxGridCellDateTimeRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellEditor
void wxGridCellEditor_delete(wxGridCellEditor *self) {
    delete self;
}
wxGridCellEditor *wxGridCellEditor_new() {
    return new wxGridCellEditor();
}
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
wxGridActivationResult *wxGridCellEditor_TryActivate(wxGridCellEditor * self, int row, int col, wxGrid * grid, const wxGridActivationSource * act_source) {
    return new wxGridActivationResult(self->TryActivate(row, col, grid, *act_source));
}
void wxGridCellEditor_DoActivate(wxGridCellEditor * self, int row, int col, wxGrid * grid) {
    return self->DoActivate(row, col, grid);
}
// Mix-in(s) to wxGridCellEditor
wxRefCounter *wxGridCellEditor_AsRefCounter(wxGridCellEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellEnumEditor
void wxGridCellEnumEditor_delete(wxGridCellEnumEditor *self) {
    delete self;
}
wxGridCellEnumEditor *wxGridCellEnumEditor_new(const wxString * choices) {
    return new wxGridCellEnumEditor(*choices);
}
// Mix-in(s) to wxGridCellEnumEditor
wxRefCounter *wxGridCellEnumEditor_AsRefCounter(wxGridCellEnumEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellEnumRenderer
void wxGridCellEnumRenderer_delete(wxGridCellEnumRenderer *self) {
    delete self;
}
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
void wxGridCellFloatEditor_delete(wxGridCellFloatEditor *self) {
    delete self;
}
wxGridCellFloatEditor *wxGridCellFloatEditor_new(int width, int precision, int format) {
    return new wxGridCellFloatEditor(width, precision, format);
}
// Mix-in(s) to wxGridCellFloatEditor
wxRefCounter *wxGridCellFloatEditor_AsRefCounter(wxGridCellFloatEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellFloatRenderer
void wxGridCellFloatRenderer_delete(wxGridCellFloatRenderer *self) {
    delete self;
}
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
void wxGridCellNumberEditor_delete(wxGridCellNumberEditor *self) {
    delete self;
}
wxGridCellNumberEditor *wxGridCellNumberEditor_new(int min, int max) {
    return new wxGridCellNumberEditor(min, max);
}
// Mix-in(s) to wxGridCellNumberEditor
wxRefCounter *wxGridCellNumberEditor_AsRefCounter(wxGridCellNumberEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellNumberRenderer
void wxGridCellNumberRenderer_delete(wxGridCellNumberRenderer *self) {
    delete self;
}
wxGridCellNumberRenderer *wxGridCellNumberRenderer_new() {
    return new wxGridCellNumberRenderer();
}
// Mix-in(s) to wxGridCellNumberRenderer
wxRefCounter *wxGridCellNumberRenderer_AsRefCounter(wxGridCellNumberRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellRenderer
void wxGridCellRenderer_delete(wxGridCellRenderer *self) {
    delete self;
}
wxGridCellRenderer *wxGridCellRenderer_new() {
    return new wxGridCellRenderer();
}
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
void wxGridCellStringRenderer_delete(wxGridCellStringRenderer *self) {
    delete self;
}
wxGridCellStringRenderer *wxGridCellStringRenderer_new() {
    return new wxGridCellStringRenderer();
}
// Mix-in(s) to wxGridCellStringRenderer
wxRefCounter *wxGridCellStringRenderer_AsRefCounter(wxGridCellStringRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellTextEditor
void wxGridCellTextEditor_delete(wxGridCellTextEditor *self) {
    delete self;
}
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
wxGridCellCoords *wxGridRangeSelectEvent_GetBottomRightCoords(const wxGridRangeSelectEvent * self) {
    return new wxGridCellCoords(self->GetBottomRightCoords());
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
wxGridCellCoords *wxGridRangeSelectEvent_GetTopLeftCoords(const wxGridRangeSelectEvent * self) {
    return new wxGridCellCoords(self->GetTopLeftCoords());
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
wxGridTableBase *wxGridTableBase_new() {
    return new wxGridTableBase();
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

// CLASS: wxHScrolledWindow
wxClassInfo *wxHScrolledWindow_CLASSINFO() {
    return wxCLASSINFO(wxHScrolledWindow);
}
wxHScrolledWindow *wxHScrolledWindow_new() {
    return new wxHScrolledWindow();
}
wxHScrolledWindow *wxHScrolledWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxHScrolledWindow(parent, id, *pos, *size, style, *name);
}
bool wxHScrolledWindow_Create(wxHScrolledWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
// Mix-in(s) to wxHScrolledWindow
wxVarHScrollHelper *wxHScrolledWindow_AsVarHScrollHelper(wxHScrolledWindow* obj) {
    return static_cast<wxVarHScrollHelper*>(obj);
}
wxTrackable *wxHScrolledWindow_AsTrackable(wxHScrolledWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxHTMLDataObject
void wxHTMLDataObject_delete(wxHTMLDataObject *self) {
    delete self;
}
wxHTMLDataObject *wxHTMLDataObject_new(const wxString * html) {
    return new wxHTMLDataObject(*html);
}
wxString *wxHTMLDataObject_GetHTML(const wxHTMLDataObject * self) {
    return new wxString(self->GetHTML());
}
void wxHTMLDataObject_SetHTML(wxHTMLDataObject * self, const wxString * html) {
    return self->SetHTML(*html);
}

// CLASS: wxHVScrolledWindow
wxClassInfo *wxHVScrolledWindow_CLASSINFO() {
    return wxCLASSINFO(wxHVScrolledWindow);
}
wxHVScrolledWindow *wxHVScrolledWindow_new() {
    return new wxHVScrolledWindow();
}
wxHVScrolledWindow *wxHVScrolledWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxHVScrolledWindow(parent, id, *pos, *size, style, *name);
}
bool wxHVScrolledWindow_Create(wxHVScrolledWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
// Mix-in(s) to wxHVScrolledWindow
wxVarHVScrollHelper *wxHVScrolledWindow_AsVarHVScrollHelper(wxHVScrolledWindow* obj) {
    return static_cast<wxVarHVScrollHelper*>(obj);
}
wxTrackable *wxHVScrolledWindow_AsTrackable(wxHVScrolledWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxHeaderColumn
void wxHeaderColumn_delete(wxHeaderColumn *self) {
    delete self;
}
wxString *wxHeaderColumn_GetTitle(const wxHeaderColumn * self) {
    return new wxString(self->GetTitle());
}
wxBitmap *wxHeaderColumn_GetBitmap(const wxHeaderColumn * self) {
    return new wxBitmap(self->GetBitmap());
}
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxHeaderColumn_GetBitmapBundle(const wxHeaderColumn * self) {
    return new wxBitmapBundle(self->GetBitmapBundle());
}
#endif
int wxHeaderColumn_GetWidth(const wxHeaderColumn * self) {
    return self->GetWidth();
}
int wxHeaderColumn_GetMinWidth(const wxHeaderColumn * self) {
    return self->GetMinWidth();
}
wxAlignment wxHeaderColumn_GetAlignment(const wxHeaderColumn * self) {
    return self->GetAlignment();
}
int wxHeaderColumn_GetFlags(const wxHeaderColumn * self) {
    return self->GetFlags();
}
bool wxHeaderColumn_HasFlag(const wxHeaderColumn * self, int flag) {
    return self->HasFlag(flag);
}
bool wxHeaderColumn_IsResizeable(const wxHeaderColumn * self) {
    return self->IsResizeable();
}
bool wxHeaderColumn_IsSortable(const wxHeaderColumn * self) {
    return self->IsSortable();
}
bool wxHeaderColumn_IsReorderable(const wxHeaderColumn * self) {
    return self->IsReorderable();
}
bool wxHeaderColumn_IsHidden(const wxHeaderColumn * self) {
    return self->IsHidden();
}
bool wxHeaderColumn_IsShown(const wxHeaderColumn * self) {
    return self->IsShown();
}
bool wxHeaderColumn_IsSortKey(const wxHeaderColumn * self) {
    return self->IsSortKey();
}
bool wxHeaderColumn_IsSortOrderAscending(const wxHeaderColumn * self) {
    return self->IsSortOrderAscending();
}

// CLASS: wxHeaderColumnSimple
void wxHeaderColumnSimple_delete(wxHeaderColumnSimple *self) {
    delete self;
}
wxHeaderColumnSimple *wxHeaderColumnSimple_new(const wxString * title, int width, wxAlignment align, int flags) {
    return new wxHeaderColumnSimple(*title, width, align, flags);
}
wxHeaderColumnSimple *wxHeaderColumnSimple_new1(const wxBitmapBundle * bitmap, int width, wxAlignment align, int flags) {
    return new wxHeaderColumnSimple(*bitmap, width, align, flags);
}

// CLASS: wxHeaderCtrl
wxClassInfo *wxHeaderCtrl_CLASSINFO() {
    return wxCLASSINFO(wxHeaderCtrl);
}
bool wxHeaderCtrl_Create(wxHeaderCtrl * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, winid, *pos, *size, style, *name);
}
void wxHeaderCtrl_SetColumnCount(wxHeaderCtrl * self, unsigned int count) {
    return self->SetColumnCount(count);
}
unsigned int wxHeaderCtrl_GetColumnCount(const wxHeaderCtrl * self) {
    return self->GetColumnCount();
}
bool wxHeaderCtrl_IsEmpty(const wxHeaderCtrl * self) {
    return self->IsEmpty();
}
void wxHeaderCtrl_UpdateColumn(wxHeaderCtrl * self, unsigned int idx) {
    return self->UpdateColumn(idx);
}
void wxHeaderCtrl_SetColumnsOrder(wxHeaderCtrl * self, const wxArrayInt * order) {
    return self->SetColumnsOrder(*order);
}
wxArrayInt *wxHeaderCtrl_GetColumnsOrder(const wxHeaderCtrl * self) {
    return new wxArrayInt(self->GetColumnsOrder());
}
unsigned int wxHeaderCtrl_GetColumnAt(const wxHeaderCtrl * self, unsigned int pos) {
    return self->GetColumnAt(pos);
}
unsigned int wxHeaderCtrl_GetColumnPos(const wxHeaderCtrl * self, unsigned int idx) {
    return self->GetColumnPos(idx);
}
void wxHeaderCtrl_ResetColumnsOrder(wxHeaderCtrl * self) {
    return self->ResetColumnsOrder();
}
bool wxHeaderCtrl_ShowColumnsMenu(wxHeaderCtrl * self, const wxPoint * pt, const wxString * title) {
    return self->ShowColumnsMenu(*pt, *title);
}
void wxHeaderCtrl_AddColumnsItems(wxHeaderCtrl * self, wxMenu * menu, int id_columns_base) {
    return self->AddColumnsItems(*menu, id_columns_base);
}
bool wxHeaderCtrl_ShowCustomizeDialog(wxHeaderCtrl * self) {
    return self->ShowCustomizeDialog();
}
int wxHeaderCtrl_GetColumnTitleWidth(wxHeaderCtrl * self, const wxHeaderColumn * col) {
    return self->GetColumnTitleWidth(*col);
}
#if wxCHECK_VERSION(3, 1, 0)
int wxHeaderCtrl_GetColumnTitleWidth1(wxHeaderCtrl * self, unsigned int idx) {
    return self->GetColumnTitleWidth(idx);
}
#endif
void wxHeaderCtrl_MoveColumnInOrderArray(wxArrayInt * order, unsigned int idx, unsigned int pos) {
    return wxHeaderCtrl::MoveColumnInOrderArray(*order, idx, pos);
}
// Mix-in(s) to wxHeaderCtrl
wxTrackable *wxHeaderCtrl_AsTrackable(wxHeaderCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxHeaderCtrlEvent
wxClassInfo *wxHeaderCtrlEvent_CLASSINFO() {
    return wxCLASSINFO(wxHeaderCtrlEvent);
}
wxHeaderCtrlEvent *wxHeaderCtrlEvent_new1(const wxHeaderCtrlEvent * event) {
    return new wxHeaderCtrlEvent(*event);
}
int wxHeaderCtrlEvent_GetColumn(const wxHeaderCtrlEvent * self) {
    return self->GetColumn();
}
void wxHeaderCtrlEvent_SetColumn(wxHeaderCtrlEvent * self, int col) {
    return self->SetColumn(col);
}
int wxHeaderCtrlEvent_GetWidth(const wxHeaderCtrlEvent * self) {
    return self->GetWidth();
}
void wxHeaderCtrlEvent_SetWidth(wxHeaderCtrlEvent * self, int width) {
    return self->SetWidth(width);
}
unsigned int wxHeaderCtrlEvent_GetNewOrder(const wxHeaderCtrlEvent * self) {
    return self->GetNewOrder();
}
void wxHeaderCtrlEvent_SetNewOrder(wxHeaderCtrlEvent * self, unsigned int order) {
    return self->SetNewOrder(order);
}

// CLASS: wxHeaderCtrlSimple
wxClassInfo *wxHeaderCtrlSimple_CLASSINFO() {
    return wxCLASSINFO(wxHeaderCtrlSimple);
}
wxHeaderCtrlSimple *wxHeaderCtrlSimple_new() {
    return new wxHeaderCtrlSimple();
}
wxHeaderCtrlSimple *wxHeaderCtrlSimple_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxHeaderCtrlSimple(parent, winid, *pos, *size, style, *name);
}
void wxHeaderCtrlSimple_InsertColumn(wxHeaderCtrlSimple * self, const wxHeaderColumnSimple * col, unsigned int idx) {
    return self->InsertColumn(*col, idx);
}
void wxHeaderCtrlSimple_AppendColumn(wxHeaderCtrlSimple * self, const wxHeaderColumnSimple * col) {
    return self->AppendColumn(*col);
}
void wxHeaderCtrlSimple_DeleteColumn(wxHeaderCtrlSimple * self, unsigned int idx) {
    return self->DeleteColumn(idx);
}
void wxHeaderCtrlSimple_ShowColumn(wxHeaderCtrlSimple * self, unsigned int idx, bool show) {
    return self->ShowColumn(idx, show);
}
void wxHeaderCtrlSimple_HideColumn(wxHeaderCtrlSimple * self, unsigned int idx) {
    return self->HideColumn(idx);
}
void wxHeaderCtrlSimple_ShowSortIndicator(wxHeaderCtrlSimple * self, unsigned int idx, bool sort_order) {
    return self->ShowSortIndicator(idx, sort_order);
}
void wxHeaderCtrlSimple_RemoveSortIndicator(wxHeaderCtrlSimple * self) {
    return self->RemoveSortIndicator();
}
// Mix-in(s) to wxHeaderCtrlSimple
wxTrackable *wxHeaderCtrlSimple_AsTrackable(wxHeaderCtrlSimple* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxHelpController
wxClassInfo *wxHelpController_CLASSINFO() {
    return wxCLASSINFO(wxHelpController);
}
wxHelpController *wxHelpController_new(wxWindow * parent_window) {
    return new wxHelpController(parent_window);
}

// CLASS: wxHelpControllerBase
wxClassInfo *wxHelpControllerBase_CLASSINFO() {
    return wxCLASSINFO(wxHelpControllerBase);
}
wxHelpControllerBase *wxHelpControllerBase_new(wxWindow * parent_window) {
    return new wxHelpControllerBase(parent_window);
}
bool wxHelpControllerBase_DisplayBlock(wxHelpControllerBase * self, long block_no) {
    return self->DisplayBlock(block_no);
}
bool wxHelpControllerBase_DisplayContents(wxHelpControllerBase * self) {
    return self->DisplayContents();
}
bool wxHelpControllerBase_DisplayContextPopup(wxHelpControllerBase * self, int context_id) {
    return self->DisplayContextPopup(context_id);
}
bool wxHelpControllerBase_DisplaySection(wxHelpControllerBase * self, const wxString * section) {
    return self->DisplaySection(*section);
}
bool wxHelpControllerBase_DisplaySection1(wxHelpControllerBase * self, int section_no) {
    return self->DisplaySection(section_no);
}
bool wxHelpControllerBase_DisplayTextPopup(wxHelpControllerBase * self, const wxString * text, const wxPoint * pos) {
    return self->DisplayTextPopup(*text, *pos);
}
wxFrame * wxHelpControllerBase_GetFrameParameters(wxHelpControllerBase * self, wxSize * size, wxPoint * pos, bool * new_frame_each_time) {
    return self->GetFrameParameters(size, pos, new_frame_each_time);
}
wxWindow * wxHelpControllerBase_GetParentWindow(const wxHelpControllerBase * self) {
    return self->GetParentWindow();
}
bool wxHelpControllerBase_Initialize(wxHelpControllerBase * self, const wxString * file) {
    return self->Initialize(*file);
}
bool wxHelpControllerBase_LoadFile(wxHelpControllerBase * self, const wxString * file) {
    return self->LoadFile(*file);
}
void wxHelpControllerBase_OnQuit(wxHelpControllerBase * self) {
    return self->OnQuit();
}
bool wxHelpControllerBase_Quit(wxHelpControllerBase * self) {
    return self->Quit();
}
void wxHelpControllerBase_SetFrameParameters(wxHelpControllerBase * self, const wxString * title_format, const wxSize * size, const wxPoint * pos, bool new_frame_each_time) {
    return self->SetFrameParameters(*title_format, *size, *pos, new_frame_each_time);
}
void wxHelpControllerBase_SetParentWindow(wxHelpControllerBase * self, wxWindow * parent_window) {
    return self->SetParentWindow(parent_window);
}
void wxHelpControllerBase_SetViewer(wxHelpControllerBase * self, const wxString * viewer, long flags) {
    return self->SetViewer(*viewer, flags);
}

// CLASS: wxHelpControllerHelpProvider
void wxHelpControllerHelpProvider_delete(wxHelpControllerHelpProvider *self) {
    delete self;
}
wxHelpControllerHelpProvider *wxHelpControllerHelpProvider_new(wxHelpControllerBase * hc) {
    return new wxHelpControllerHelpProvider(hc);
}
wxHelpControllerBase * wxHelpControllerHelpProvider_GetHelpController(const wxHelpControllerHelpProvider * self) {
    return self->GetHelpController();
}
void wxHelpControllerHelpProvider_SetHelpController(wxHelpControllerHelpProvider * self, wxHelpControllerBase * hc) {
    return self->SetHelpController(hc);
}

// CLASS: wxHelpEvent
wxClassInfo *wxHelpEvent_CLASSINFO() {
    return wxCLASSINFO(wxHelpEvent);
}
wxPoint *wxHelpEvent_GetPosition(const wxHelpEvent * self) {
    return new wxPoint(self->GetPosition());
}
void wxHelpEvent_SetPosition(wxHelpEvent * self, const wxPoint * pt) {
    return self->SetPosition(*pt);
}

// CLASS: wxHelpProvider
void wxHelpProvider_delete(wxHelpProvider *self) {
    delete self;
}
void wxHelpProvider_AddHelp(wxHelpProvider * self, wxWindow * window, const wxString * text) {
    return self->AddHelp(window, *text);
}
void wxHelpProvider_AddHelp1(wxHelpProvider * self, wxWindowID id, const wxString * text) {
    return self->AddHelp(id, *text);
}
wxString *wxHelpProvider_GetHelp(wxHelpProvider * self, const wxWindow * window) {
    return new wxString(self->GetHelp(window));
}
void wxHelpProvider_RemoveHelp(wxHelpProvider * self, wxWindow * window) {
    return self->RemoveHelp(window);
}
bool wxHelpProvider_ShowHelp(wxHelpProvider * self, wxWindow * window) {
    return self->ShowHelp(window);
}
wxHelpProvider * wxHelpProvider_Get() {
    return wxHelpProvider::Get();
}
wxHelpProvider * wxHelpProvider_Set(wxHelpProvider * help_provider) {
    return wxHelpProvider::Set(help_provider);
}

// CLASS: wxHyperlinkCtrl
wxClassInfo *wxHyperlinkCtrl_CLASSINFO() {
    return wxCLASSINFO(wxHyperlinkCtrl);
}
wxHyperlinkCtrl *wxHyperlinkCtrl_new() {
    return new wxHyperlinkCtrl();
}
wxHyperlinkCtrl *wxHyperlinkCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxString * url, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxHyperlinkCtrl(parent, id, *label, *url, *pos, *size, style, *name);
}
bool wxHyperlinkCtrl_Create(wxHyperlinkCtrl * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxString * url, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *url, *pos, *size, style, *name);
}
wxColour *wxHyperlinkCtrl_GetHoverColour(const wxHyperlinkCtrl * self) {
    return new wxColour(self->GetHoverColour());
}
wxColour *wxHyperlinkCtrl_GetNormalColour(const wxHyperlinkCtrl * self) {
    return new wxColour(self->GetNormalColour());
}
wxString *wxHyperlinkCtrl_GetURL(const wxHyperlinkCtrl * self) {
    return new wxString(self->GetURL());
}
bool wxHyperlinkCtrl_GetVisited(const wxHyperlinkCtrl * self) {
    return self->GetVisited();
}
wxColour *wxHyperlinkCtrl_GetVisitedColour(const wxHyperlinkCtrl * self) {
    return new wxColour(self->GetVisitedColour());
}
void wxHyperlinkCtrl_SetHoverColour(wxHyperlinkCtrl * self, const wxColour * colour) {
    return self->SetHoverColour(*colour);
}
void wxHyperlinkCtrl_SetNormalColour(wxHyperlinkCtrl * self, const wxColour * colour) {
    return self->SetNormalColour(*colour);
}
void wxHyperlinkCtrl_SetURL(wxHyperlinkCtrl * self, const wxString * url) {
    return self->SetURL(*url);
}
void wxHyperlinkCtrl_SetVisited(wxHyperlinkCtrl * self, bool visited) {
    return self->SetVisited(visited);
}
void wxHyperlinkCtrl_SetVisitedColour(wxHyperlinkCtrl * self, const wxColour * colour) {
    return self->SetVisitedColour(*colour);
}
// Mix-in(s) to wxHyperlinkCtrl
wxTrackable *wxHyperlinkCtrl_AsTrackable(wxHyperlinkCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxHyperlinkEvent
wxClassInfo *wxHyperlinkEvent_CLASSINFO() {
    return wxCLASSINFO(wxHyperlinkEvent);
}
wxHyperlinkEvent *wxHyperlinkEvent_new(wxObject * generator, int id, const wxString * url) {
    return new wxHyperlinkEvent(generator, id, *url);
}
wxString *wxHyperlinkEvent_GetURL(const wxHyperlinkEvent * self) {
    return new wxString(self->GetURL());
}
void wxHyperlinkEvent_SetURL(wxHyperlinkEvent * self, const wxString * url) {
    return self->SetURL(*url);
}

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
bool wxIconizeEvent_Iconized(const wxIconizeEvent * self) {
    return self->Iconized();
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
void wxImage_ChangeSaturation(wxImage * self, double factor) {
    return self->ChangeSaturation(factor);
}
void wxImage_ChangeBrightness(wxImage * self, double factor) {
    return self->ChangeBrightness(factor);
}
void wxImage_ChangeHSV(wxImage * self, double angle_h, double factor_s, double factor_v) {
    return self->ChangeHSV(angle_h, factor_s, factor_v);
}
wxImage *wxImage_Size(const wxImage * self, const wxSize * size, const wxPoint * pos, int red, int green, int blue) {
    return new wxImage(self->Size(*size, *pos, red, green, blue));
}
wxImage *wxImage_ConvertToGreyscale(const wxImage * self, double weight_r, double weight_g, double weight_b) {
    return new wxImage(self->ConvertToGreyscale(weight_r, weight_g, weight_b));
}
wxImage *wxImage_ConvertToGreyscale1(const wxImage * self) {
    return new wxImage(self->ConvertToGreyscale());
}
wxImage *wxImage_ChangeLightness(const wxImage * self, int alpha) {
    return new wxImage(self->ChangeLightness(alpha));
}
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
wxPalette *wxImage_GetPalette(const wxImage * self) {
    return new wxPalette(self->GetPalette());
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
void wxImage_SetLoadFlags(wxImage * self, int flags) {
    return self->SetLoadFlags(flags);
}
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
void wxImage_SetDefaultLoadFlags(int flags) {
    return wxImage::SetDefaultLoadFlags(flags);
}
int wxImage_GetLoadFlags(const wxImage * self) {
    return self->GetLoadFlags();
}
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
wxList * wxImage_GetHandlers() {
    return wxImage::GetHandlers();
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
int wxImage_GetDefaultLoadFlags() {
    return wxImage::GetDefaultLoadFlags();
}
wxString *wxImage_GetImageExtWildcard() {
    return new wxString(wxImage::GetImageExtWildcard());
}
wxImage::HSVValue *wxImage_RGBtoHSV(const wxImage::RGBValue * rgb) {
    return new wxImage::HSVValue(wxImage::RGBtoHSV(*rgb));
}
wxImage::RGBValue *wxImage_HSVtoRGB(const wxImage::HSVValue * hsv) {
    return new wxImage::RGBValue(wxImage::HSVtoRGB(*hsv));
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

// CLASS: wxImageDataObject
void wxImageDataObject_delete(wxImageDataObject *self) {
    delete self;
}
wxImageDataObject *wxImageDataObject_new(const wxImage * image) {
    return new wxImageDataObject(*image);
}
wxImage *wxImageDataObject_GetImage(const wxImageDataObject * self) {
    return new wxImage(self->GetImage());
}
void wxImageDataObject_SetImage(wxImageDataObject * self, const wxImage * image) {
    return self->SetImage(*image);
}

// CLASS: wxImageHandler
wxClassInfo *wxImageHandler_CLASSINFO() {
    return wxCLASSINFO(wxImageHandler);
}
wxImageHandler *wxImageHandler_new() {
    return new wxImageHandler();
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
wxVersionInfo *wxImageHandler_GetLibraryVersionInfo() {
    return new wxVersionInfo(wxImageHandler::GetLibraryVersionInfo());
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
void wxImageList_Destroy(wxImageList * self) {
    return self->Destroy();
}
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
wxSize *wxImageList_GetSize1(const wxImageList * self) {
    return new wxSize(self->GetSize());
}
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
size_t wxInfoBar_GetButtonCount(const wxInfoBar * self) {
    return self->GetButtonCount();
}
wxWindowID wxInfoBar_GetButtonId(const wxInfoBar * self, size_t idx) {
    return self->GetButtonId(idx);
}
bool wxInfoBar_HasButtonId(const wxInfoBar * self, wxWindowID btnid) {
    return self->HasButtonId(btnid);
}
// Mix-in(s) to wxInfoBar
wxTrackable *wxInfoBar_AsTrackable(wxInfoBar* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxInitDialogEvent
wxClassInfo *wxInitDialogEvent_CLASSINFO() {
    return wxCLASSINFO(wxInitDialogEvent);
}
wxInitDialogEvent *wxInitDialogEvent_new(int id) {
    return new wxInitDialogEvent(id);
}

// CLASS: wxItemAttr
void wxItemAttr_delete(wxItemAttr *self) {
    delete self;
}
wxItemAttr *wxItemAttr_new() {
    return new wxItemAttr();
}
wxItemAttr *wxItemAttr_new1(const wxColour * col_text, const wxColour * col_back, const wxFont * font) {
    return new wxItemAttr(*col_text, *col_back, *font);
}
wxColour *wxItemAttr_GetBackgroundColour(const wxItemAttr * self) {
    return new wxColour(self->GetBackgroundColour());
}
wxFont *wxItemAttr_GetFont(const wxItemAttr * self) {
    return new wxFont(self->GetFont());
}
wxColour *wxItemAttr_GetTextColour(const wxItemAttr * self) {
    return new wxColour(self->GetTextColour());
}
bool wxItemAttr_HasBackgroundColour(const wxItemAttr * self) {
    return self->HasBackgroundColour();
}
bool wxItemAttr_HasColours(const wxItemAttr * self) {
    return self->HasColours();
}
bool wxItemAttr_HasFont(const wxItemAttr * self) {
    return self->HasFont();
}
bool wxItemAttr_HasTextColour(const wxItemAttr * self) {
    return self->HasTextColour();
}
bool wxItemAttr_IsDefault(const wxItemAttr * self) {
    return self->IsDefault();
}
void wxItemAttr_SetBackgroundColour(wxItemAttr * self, const wxColour * colour) {
    return self->SetBackgroundColour(*colour);
}
void wxItemAttr_SetFont(wxItemAttr * self, const wxFont * font) {
    return self->SetFont(*font);
}
void wxItemAttr_SetTextColour(wxItemAttr * self, const wxColour * colour) {
    return self->SetTextColour(*colour);
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

// CLASS: wxJPEGHandler
wxClassInfo *wxJPEGHandler_CLASSINFO() {
    return wxCLASSINFO(wxJPEGHandler);
}
wxJPEGHandler *wxJPEGHandler_new() {
    return new wxJPEGHandler();
}
wxVersionInfo *wxJPEGHandler_GetLibraryVersionInfo() {
    return new wxVersionInfo(wxJPEGHandler::GetLibraryVersionInfo());
}

// CLASS: wxJoystick
wxClassInfo *wxJoystick_CLASSINFO() {
    return wxCLASSINFO(wxJoystick);
}
wxJoystick *wxJoystick_new(int joystick) {
    return new wxJoystick(joystick);
}
int wxJoystick_GetButtonState(const wxJoystick * self) {
    return self->GetButtonState();
}
bool wxJoystick_GetButtonState1(const wxJoystick * self, unsigned int id) {
    return self->GetButtonState(id);
}
int wxJoystick_GetManufacturerId(const wxJoystick * self) {
    return self->GetManufacturerId();
}
int wxJoystick_GetMovementThreshold(const wxJoystick * self) {
    return self->GetMovementThreshold();
}
int wxJoystick_GetNumberAxes(const wxJoystick * self) {
    return self->GetNumberAxes();
}
int wxJoystick_GetNumberButtons(const wxJoystick * self) {
    return self->GetNumberButtons();
}
int wxJoystick_GetPOVCTSPosition(const wxJoystick * self) {
    return self->GetPOVCTSPosition();
}
int wxJoystick_GetPOVPosition(const wxJoystick * self) {
    return self->GetPOVPosition();
}
int wxJoystick_GetPollingMax(const wxJoystick * self) {
    return self->GetPollingMax();
}
int wxJoystick_GetPollingMin(const wxJoystick * self) {
    return self->GetPollingMin();
}
wxPoint *wxJoystick_GetPosition(const wxJoystick * self) {
    return new wxPoint(self->GetPosition());
}
int wxJoystick_GetPosition1(const wxJoystick * self, unsigned int axis) {
    return self->GetPosition(axis);
}
int wxJoystick_GetProductId(const wxJoystick * self) {
    return self->GetProductId();
}
wxString *wxJoystick_GetProductName(const wxJoystick * self) {
    return new wxString(self->GetProductName());
}
int wxJoystick_GetRudderMax(const wxJoystick * self) {
    return self->GetRudderMax();
}
int wxJoystick_GetRudderMin(const wxJoystick * self) {
    return self->GetRudderMin();
}
int wxJoystick_GetRudderPosition(const wxJoystick * self) {
    return self->GetRudderPosition();
}
int wxJoystick_GetUMax(const wxJoystick * self) {
    return self->GetUMax();
}
int wxJoystick_GetUMin(const wxJoystick * self) {
    return self->GetUMin();
}
int wxJoystick_GetUPosition(const wxJoystick * self) {
    return self->GetUPosition();
}
int wxJoystick_GetVMax(const wxJoystick * self) {
    return self->GetVMax();
}
int wxJoystick_GetVMin(const wxJoystick * self) {
    return self->GetVMin();
}
int wxJoystick_GetVPosition(const wxJoystick * self) {
    return self->GetVPosition();
}
int wxJoystick_GetXMax(const wxJoystick * self) {
    return self->GetXMax();
}
int wxJoystick_GetXMin(const wxJoystick * self) {
    return self->GetXMin();
}
int wxJoystick_GetYMax(const wxJoystick * self) {
    return self->GetYMax();
}
int wxJoystick_GetYMin(const wxJoystick * self) {
    return self->GetYMin();
}
int wxJoystick_GetZMax(const wxJoystick * self) {
    return self->GetZMax();
}
int wxJoystick_GetZMin(const wxJoystick * self) {
    return self->GetZMin();
}
int wxJoystick_GetZPosition(const wxJoystick * self) {
    return self->GetZPosition();
}
bool wxJoystick_HasPOV(const wxJoystick * self) {
    return self->HasPOV();
}
bool wxJoystick_HasPOV4Dir(const wxJoystick * self) {
    return self->HasPOV4Dir();
}
bool wxJoystick_HasPOVCTS(const wxJoystick * self) {
    return self->HasPOVCTS();
}
bool wxJoystick_HasRudder(const wxJoystick * self) {
    return self->HasRudder();
}
bool wxJoystick_HasU(const wxJoystick * self) {
    return self->HasU();
}
bool wxJoystick_HasV(const wxJoystick * self) {
    return self->HasV();
}
bool wxJoystick_HasZ(const wxJoystick * self) {
    return self->HasZ();
}
bool wxJoystick_IsOk(const wxJoystick * self) {
    return self->IsOk();
}
bool wxJoystick_ReleaseCapture(wxJoystick * self) {
    return self->ReleaseCapture();
}
bool wxJoystick_SetCapture(wxJoystick * self, wxWindow * win, int polling_freq) {
    return self->SetCapture(win, polling_freq);
}
void wxJoystick_SetMovementThreshold(wxJoystick * self, int threshold) {
    return self->SetMovementThreshold(threshold);
}
int wxJoystick_GetNumberJoysticks() {
    return wxJoystick::GetNumberJoysticks();
}

// CLASS: wxJoystickEvent
wxClassInfo *wxJoystickEvent_CLASSINFO() {
    return wxCLASSINFO(wxJoystickEvent);
}
bool wxJoystickEvent_ButtonDown(const wxJoystickEvent * self, int button) {
    return self->ButtonDown(button);
}
bool wxJoystickEvent_ButtonIsDown(const wxJoystickEvent * self, int button) {
    return self->ButtonIsDown(button);
}
bool wxJoystickEvent_ButtonUp(const wxJoystickEvent * self, int button) {
    return self->ButtonUp(button);
}
int wxJoystickEvent_GetButtonChange(const wxJoystickEvent * self) {
    return self->GetButtonChange();
}
int wxJoystickEvent_GetButtonOrdinal(const wxJoystickEvent * self) {
    return self->GetButtonOrdinal();
}
int wxJoystickEvent_GetButtonState(const wxJoystickEvent * self) {
    return self->GetButtonState();
}
int wxJoystickEvent_GetJoystick(const wxJoystickEvent * self) {
    return self->GetJoystick();
}
wxPoint *wxJoystickEvent_GetPosition(const wxJoystickEvent * self) {
    return new wxPoint(self->GetPosition());
}
int wxJoystickEvent_GetZPosition(const wxJoystickEvent * self) {
    return self->GetZPosition();
}
bool wxJoystickEvent_IsButton(const wxJoystickEvent * self) {
    return self->IsButton();
}
bool wxJoystickEvent_IsMove(const wxJoystickEvent * self) {
    return self->IsMove();
}
bool wxJoystickEvent_IsZMove(const wxJoystickEvent * self) {
    return self->IsZMove();
}

// CLASS: wxKeyEvent
wxClassInfo *wxKeyEvent_CLASSINFO() {
    return wxCLASSINFO(wxKeyEvent);
}
int wxKeyEvent_GetKeyCode(const wxKeyEvent * self) {
    return self->GetKeyCode();
}
bool wxKeyEvent_IsKeyInCategory(const wxKeyEvent * self, int category) {
    return self->IsKeyInCategory(category);
}
bool wxKeyEvent_IsAutoRepeat(const wxKeyEvent * self) {
    return self->IsAutoRepeat();
}
wxPoint *wxKeyEvent_GetPosition(const wxKeyEvent * self) {
    return new wxPoint(self->GetPosition());
}
void wxKeyEvent_GetPosition1(const wxKeyEvent * self, wxCoord * x, wxCoord * y) {
    return self->GetPosition(x, y);
}
wxCoord wxKeyEvent_GetX(const wxKeyEvent * self) {
    return self->GetX();
}
wxCoord wxKeyEvent_GetY(const wxKeyEvent * self) {
    return self->GetY();
}
void wxKeyEvent_DoAllowNextEvent(wxKeyEvent * self) {
    return self->DoAllowNextEvent();
}
bool wxKeyEvent_IsNextEventAllowed(const wxKeyEvent * self) {
    return self->IsNextEventAllowed();
}
// Mix-in(s) to wxKeyEvent
wxKeyboardState *wxKeyEvent_AsKeyboardState(wxKeyEvent* obj) {
    return static_cast<wxKeyboardState*>(obj);
}

// CLASS: wxLayoutAlgorithm
wxClassInfo *wxLayoutAlgorithm_CLASSINFO() {
    return wxCLASSINFO(wxLayoutAlgorithm);
}
wxLayoutAlgorithm *wxLayoutAlgorithm_new() {
    return new wxLayoutAlgorithm();
}
bool wxLayoutAlgorithm_LayoutFrame(wxLayoutAlgorithm * self, wxFrame * frame, wxWindow * main_window) {
    return self->LayoutFrame(frame, main_window);
}
bool wxLayoutAlgorithm_LayoutMDIFrame(wxLayoutAlgorithm * self, wxMDIParentFrame * frame, wxRect * rect) {
    return self->LayoutMDIFrame(frame, rect);
}
bool wxLayoutAlgorithm_LayoutWindow(wxLayoutAlgorithm * self, wxWindow * parent, wxWindow * main_window) {
    return self->LayoutWindow(parent, main_window);
}

// CLASS: wxListBox
wxClassInfo *wxListBox_CLASSINFO() {
    return wxCLASSINFO(wxListBox);
}
wxListBox *wxListBox_new() {
    return new wxListBox();
}
wxListBox *wxListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxListBox(parent, id, *pos, *size, *choices, style, *validator, *name);
}
bool wxListBox_Create1(wxListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *choices, style, *validator, *name);
}
void wxListBox_Deselect(wxListBox * self, int n) {
    return self->Deselect(n);
}
bool wxListBox_SetStringSelection(wxListBox * self, const wxString * s, bool select) {
    return self->SetStringSelection(*s, select);
}
bool wxListBox_SetStringSelection1(wxListBox * self, const wxString * s) {
    return self->SetStringSelection(*s);
}
int wxListBox_GetSelections(const wxListBox * self, wxArrayInt * selections) {
    return self->GetSelections(*selections);
}
int wxListBox_HitTest(const wxListBox * self, const wxPoint * point) {
    return self->HitTest(*point);
}
int wxListBox_HitTest1(const wxListBox * self, int x, int y) {
    return self->HitTest(x, y);
}
void wxListBox_InsertItems1(wxListBox * self, const wxArrayString * items, unsigned int pos) {
    return self->InsertItems(*items, pos);
}
bool wxListBox_IsSelected(const wxListBox * self, int n) {
    return self->IsSelected(n);
}
void wxListBox_SetFirstItem(wxListBox * self, int n) {
    return self->SetFirstItem(n);
}
void wxListBox_SetFirstItem1(wxListBox * self, const wxString * string) {
    return self->SetFirstItem(*string);
}
void wxListBox_EnsureVisible(wxListBox * self, int n) {
    return self->EnsureVisible(n);
}
bool wxListBox_IsSorted(const wxListBox * self) {
    return self->IsSorted();
}
#if wxCHECK_VERSION(3, 1, 0)
int wxListBox_GetCountPerPage(const wxListBox * self) {
    return self->GetCountPerPage();
}
int wxListBox_GetTopItem(const wxListBox * self) {
    return self->GetTopItem();
}
#endif
// Mix-in(s) to wxListBox
wxItemContainer *wxListBox_AsItemContainer(wxListBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTrackable *wxListBox_AsTrackable(wxListBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxListCtrl
wxClassInfo *wxListCtrl_CLASSINFO() {
    return wxCLASSINFO(wxListCtrl);
}
wxListCtrl *wxListCtrl_new() {
    return new wxListCtrl();
}
wxListCtrl *wxListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxListCtrl(parent, id, *pos, *size, style, *validator, *name);
}
bool wxListCtrl_Arrange(wxListCtrl * self, int flag) {
    return self->Arrange(flag);
}
void wxListCtrl_AssignImageList(wxListCtrl * self, wxImageList * image_list, int which) {
    return self->AssignImageList(image_list, which);
}
void wxListCtrl_ClearAll(wxListCtrl * self) {
    return self->ClearAll();
}
bool wxListCtrl_Create(wxListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
bool wxListCtrl_DeleteAllColumns(wxListCtrl * self) {
    return self->DeleteAllColumns();
}
bool wxListCtrl_DeleteAllItems(wxListCtrl * self) {
    return self->DeleteAllItems();
}
bool wxListCtrl_DeleteColumn(wxListCtrl * self, int col) {
    return self->DeleteColumn(col);
}
bool wxListCtrl_DeleteItem(wxListCtrl * self, long item) {
    return self->DeleteItem(item);
}
wxTextCtrl * wxListCtrl_EditLabel(wxListCtrl * self, long item, wxClassInfo * text_control_class) {
    return self->EditLabel(item, text_control_class);
}
void wxListCtrl_EnableAlternateRowColours(wxListCtrl * self, bool enable) {
    return self->EnableAlternateRowColours(enable);
}
void wxListCtrl_EnableBellOnNoMatch(wxListCtrl * self, bool on) {
    return self->EnableBellOnNoMatch(on);
}
bool wxListCtrl_EndEditLabel(wxListCtrl * self, bool cancel) {
    return self->EndEditLabel(cancel);
}
bool wxListCtrl_EnsureVisible(wxListCtrl * self, long item) {
    return self->EnsureVisible(item);
}
long wxListCtrl_FindItem(wxListCtrl * self, long start, const wxString * str, bool partial) {
    return self->FindItem(start, *str, partial);
}
long wxListCtrl_FindItem2(wxListCtrl * self, long start, const wxPoint * pt, int direction) {
    return self->FindItem(start, *pt, direction);
}
bool wxListCtrl_GetColumn(const wxListCtrl * self, int col, wxListItem * item) {
    return self->GetColumn(col, *item);
}
int wxListCtrl_GetColumnCount(const wxListCtrl * self) {
    return self->GetColumnCount();
}
int wxListCtrl_GetColumnIndexFromOrder(const wxListCtrl * self, int pos) {
    return self->GetColumnIndexFromOrder(pos);
}
int wxListCtrl_GetColumnOrder(const wxListCtrl * self, int col) {
    return self->GetColumnOrder(col);
}
int wxListCtrl_GetColumnWidth(const wxListCtrl * self, int col) {
    return self->GetColumnWidth(col);
}
wxArrayInt *wxListCtrl_GetColumnsOrder(const wxListCtrl * self) {
    return new wxArrayInt(self->GetColumnsOrder());
}
int wxListCtrl_GetCountPerPage(const wxListCtrl * self) {
    return self->GetCountPerPage();
}
wxTextCtrl * wxListCtrl_GetEditControl(const wxListCtrl * self) {
    return self->GetEditControl();
}
wxImageList * wxListCtrl_GetImageList(const wxListCtrl * self, int which) {
    return self->GetImageList(which);
}
bool wxListCtrl_GetItem(const wxListCtrl * self, wxListItem * info) {
    return self->GetItem(*info);
}
wxColour *wxListCtrl_GetItemBackgroundColour(const wxListCtrl * self, long item) {
    return new wxColour(self->GetItemBackgroundColour(item));
}
int wxListCtrl_GetItemCount(const wxListCtrl * self) {
    return self->GetItemCount();
}
wxFont *wxListCtrl_GetItemFont(const wxListCtrl * self, long item) {
    return new wxFont(self->GetItemFont(item));
}
bool wxListCtrl_GetItemPosition(const wxListCtrl * self, long item, wxPoint * pos) {
    return self->GetItemPosition(item, *pos);
}
bool wxListCtrl_GetItemRect(const wxListCtrl * self, long item, wxRect * rect, int code) {
    return self->GetItemRect(item, *rect, code);
}
wxSize *wxListCtrl_GetItemSpacing(const wxListCtrl * self) {
    return new wxSize(self->GetItemSpacing());
}
int wxListCtrl_GetItemState(const wxListCtrl * self, long item, long state_mask) {
    return self->GetItemState(item, state_mask);
}
wxString *wxListCtrl_GetItemText(const wxListCtrl * self, long item, int col) {
    return new wxString(self->GetItemText(item, col));
}
wxColour *wxListCtrl_GetItemTextColour(const wxListCtrl * self, long item) {
    return new wxColour(self->GetItemTextColour(item));
}
long wxListCtrl_GetNextItem(const wxListCtrl * self, long item, int geometry, int state) {
    return self->GetNextItem(item, geometry, state);
}
int wxListCtrl_GetSelectedItemCount(const wxListCtrl * self) {
    return self->GetSelectedItemCount();
}
bool wxListCtrl_GetSubItemRect(const wxListCtrl * self, long item, long sub_item, wxRect * rect, int code) {
    return self->GetSubItemRect(item, sub_item, *rect, code);
}
wxColour *wxListCtrl_GetTextColour(const wxListCtrl * self) {
    return new wxColour(self->GetTextColour());
}
long wxListCtrl_GetTopItem(const wxListCtrl * self) {
    return self->GetTopItem();
}
wxRect *wxListCtrl_GetViewRect(const wxListCtrl * self) {
    return new wxRect(self->GetViewRect());
}
void wxListCtrl_SetAlternateRowColour(wxListCtrl * self, const wxColour * colour) {
    return self->SetAlternateRowColour(*colour);
}
wxColour *wxListCtrl_GetAlternateRowColour(const wxListCtrl * self) {
    return new wxColour(self->GetAlternateRowColour());
}
long wxListCtrl_HitTest(const wxListCtrl * self, const wxPoint * point, int * flags, long * ptr_sub_item) {
    return self->HitTest(*point, *flags, ptr_sub_item);
}
bool wxListCtrl_InReportView(const wxListCtrl * self) {
    return self->InReportView();
}
long wxListCtrl_InsertColumn(wxListCtrl * self, long col, const wxListItem * info) {
    return self->InsertColumn(col, *info);
}
long wxListCtrl_InsertColumn1(wxListCtrl * self, long col, const wxString * heading, int format, int width) {
    return self->InsertColumn(col, *heading, format, width);
}
long wxListCtrl_InsertItem(wxListCtrl * self, wxListItem * info) {
    return self->InsertItem(*info);
}
long wxListCtrl_InsertItem1(wxListCtrl * self, long index, const wxString * label) {
    return self->InsertItem(index, *label);
}
long wxListCtrl_InsertItem2(wxListCtrl * self, long index, int image_index) {
    return self->InsertItem(index, image_index);
}
long wxListCtrl_InsertItem3(wxListCtrl * self, long index, const wxString * label, int image_index) {
    return self->InsertItem(index, *label, image_index);
}
bool wxListCtrl_IsEmpty(const wxListCtrl * self) {
    return self->IsEmpty();
}
bool wxListCtrl_IsVirtual(const wxListCtrl * self) {
    return self->IsVirtual();
}
void wxListCtrl_RefreshItem(wxListCtrl * self, long item) {
    return self->RefreshItem(item);
}
void wxListCtrl_RefreshItems(wxListCtrl * self, long item_from, long item_to) {
    return self->RefreshItems(item_from, item_to);
}
bool wxListCtrl_ScrollList(wxListCtrl * self, int dx, int dy) {
    return self->ScrollList(dx, dy);
}
bool wxListCtrl_SetColumn(wxListCtrl * self, int col, wxListItem * item) {
    return self->SetColumn(col, *item);
}
bool wxListCtrl_SetColumnWidth(wxListCtrl * self, int col, int width) {
    return self->SetColumnWidth(col, width);
}
bool wxListCtrl_SetColumnsOrder(wxListCtrl * self, const wxArrayInt * orders) {
    return self->SetColumnsOrder(*orders);
}
bool wxListCtrl_SetHeaderAttr(wxListCtrl * self, const wxItemAttr * attr) {
    return self->SetHeaderAttr(*attr);
}
void wxListCtrl_SetImageList(wxListCtrl * self, wxImageList * image_list, int which) {
    return self->SetImageList(image_list, which);
}
void wxListCtrl_SetNormalImages(wxListCtrl * self, const wxVector< wxBitmapBundle > * images) {
    return self->SetNormalImages(*images);
}
void wxListCtrl_SetSmallImages(wxListCtrl * self, const wxVector< wxBitmapBundle > * images) {
    return self->SetSmallImages(*images);
}
bool wxListCtrl_IsVisible(const wxListCtrl * self, long item) {
    return self->IsVisible(item);
}
bool wxListCtrl_SetItem(wxListCtrl * self, wxListItem * info) {
    return self->SetItem(*info);
}
bool wxListCtrl_SetItem1(wxListCtrl * self, long index, int column, const wxString * label, int image_id) {
    return self->SetItem(index, column, *label, image_id);
}
void wxListCtrl_SetItemBackgroundColour(wxListCtrl * self, long item, const wxColour * col) {
    return self->SetItemBackgroundColour(item, *col);
}
bool wxListCtrl_SetItemColumnImage(wxListCtrl * self, long item, long column, int image) {
    return self->SetItemColumnImage(item, column, image);
}
void wxListCtrl_SetItemCount(wxListCtrl * self, long count) {
    return self->SetItemCount(count);
}
bool wxListCtrl_SetItemData(wxListCtrl * self, long item, long data) {
    return self->SetItemData(item, data);
}
void wxListCtrl_SetItemFont(wxListCtrl * self, long item, const wxFont * font) {
    return self->SetItemFont(item, *font);
}
bool wxListCtrl_SetItemImage(wxListCtrl * self, long item, int image, int sel_image) {
    return self->SetItemImage(item, image, sel_image);
}
bool wxListCtrl_SetItemPosition(wxListCtrl * self, long item, const wxPoint * pos) {
    return self->SetItemPosition(item, *pos);
}
bool wxListCtrl_SetItemState(wxListCtrl * self, long item, long state, long state_mask) {
    return self->SetItemState(item, state, state_mask);
}
void wxListCtrl_SetItemText(wxListCtrl * self, long item, const wxString * text) {
    return self->SetItemText(item, *text);
}
void wxListCtrl_SetItemTextColour(wxListCtrl * self, long item, const wxColour * col) {
    return self->SetItemTextColour(item, *col);
}
void wxListCtrl_SetSingleStyle(wxListCtrl * self, long style, bool add) {
    return self->SetSingleStyle(style, add);
}
void wxListCtrl_SetTextColour(wxListCtrl * self, const wxColour * col) {
    return self->SetTextColour(*col);
}
bool wxListCtrl_HasCheckBoxes(const wxListCtrl * self) {
    return self->HasCheckBoxes();
}
bool wxListCtrl_EnableCheckBoxes(wxListCtrl * self, bool enable) {
    return self->EnableCheckBoxes(enable);
}
bool wxListCtrl_IsItemChecked(const wxListCtrl * self, long item) {
    return self->IsItemChecked(item);
}
void wxListCtrl_CheckItem(wxListCtrl * self, long item, bool check) {
    return self->CheckItem(item, check);
}
void wxListCtrl_ExtendRulesAndAlternateColour(wxListCtrl * self, bool extend) {
    return self->ExtendRulesAndAlternateColour(extend);
}
void wxListCtrl_ShowSortIndicator(wxListCtrl * self, int col, bool ascending) {
    return self->ShowSortIndicator(col, ascending);
}
void wxListCtrl_RemoveSortIndicator(wxListCtrl * self) {
    return self->RemoveSortIndicator();
}
int wxListCtrl_GetSortIndicator(const wxListCtrl * self) {
    return self->GetSortIndicator();
}
bool wxListCtrl_GetUpdatedAscendingSortIndicator(const wxListCtrl * self, int col) {
    return self->GetUpdatedAscendingSortIndicator(col);
}
bool wxListCtrl_IsAscendingSortIndicator(const wxListCtrl * self) {
    return self->IsAscendingSortIndicator();
}
// Mix-in(s) to wxListCtrl
wxTrackable *wxListCtrl_AsTrackable(wxListCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxListEvent
wxClassInfo *wxListEvent_CLASSINFO() {
    return wxCLASSINFO(wxListEvent);
}
long wxListEvent_GetCacheFrom(const wxListEvent * self) {
    return self->GetCacheFrom();
}
long wxListEvent_GetCacheTo(const wxListEvent * self) {
    return self->GetCacheTo();
}
int wxListEvent_GetColumn(const wxListEvent * self) {
    return self->GetColumn();
}
int wxListEvent_GetImage(const wxListEvent * self) {
    return self->GetImage();
}
long wxListEvent_GetIndex(const wxListEvent * self) {
    return self->GetIndex();
}
wxListItem *wxListEvent_GetItem(const wxListEvent * self) {
    return new wxListItem(self->GetItem());
}
int wxListEvent_GetKeyCode(const wxListEvent * self) {
    return self->GetKeyCode();
}
wxString *wxListEvent_GetLabel(const wxListEvent * self) {
    return new wxString(self->GetLabel());
}
long wxListEvent_GetMask(const wxListEvent * self) {
    return self->GetMask();
}
wxPoint *wxListEvent_GetPoint(const wxListEvent * self) {
    return new wxPoint(self->GetPoint());
}
wxString *wxListEvent_GetText(const wxListEvent * self) {
    return new wxString(self->GetText());
}
bool wxListEvent_IsEditCancelled(const wxListEvent * self) {
    return self->IsEditCancelled();
}
void wxListEvent_SetKeyCode(wxListEvent * self, int code) {
    return self->SetKeyCode(code);
}
void wxListEvent_SetIndex(wxListEvent * self, long index) {
    return self->SetIndex(index);
}
void wxListEvent_SetColumn(wxListEvent * self, int col) {
    return self->SetColumn(col);
}
void wxListEvent_SetPoint(wxListEvent * self, const wxPoint * point) {
    return self->SetPoint(*point);
}
void wxListEvent_SetItem(wxListEvent * self, const wxListItem * item) {
    return self->SetItem(*item);
}
void wxListEvent_SetCacheFrom(wxListEvent * self, long cache_from) {
    return self->SetCacheFrom(cache_from);
}
void wxListEvent_SetCacheTo(wxListEvent * self, long cache_to) {
    return self->SetCacheTo(cache_to);
}

// CLASS: wxListItem
wxClassInfo *wxListItem_CLASSINFO() {
    return wxCLASSINFO(wxListItem);
}
wxListItem *wxListItem_new() {
    return new wxListItem();
}
void wxListItem_Clear(wxListItem * self) {
    return self->Clear();
}
wxColour *wxListItem_GetBackgroundColour(const wxListItem * self) {
    return new wxColour(self->GetBackgroundColour());
}
int wxListItem_GetColumn(const wxListItem * self) {
    return self->GetColumn();
}
wxFont *wxListItem_GetFont(const wxListItem * self) {
    return new wxFont(self->GetFont());
}
long wxListItem_GetId(const wxListItem * self) {
    return self->GetId();
}
int wxListItem_GetImage(const wxListItem * self) {
    return self->GetImage();
}
long wxListItem_GetMask(const wxListItem * self) {
    return self->GetMask();
}
long wxListItem_GetState(const wxListItem * self) {
    return self->GetState();
}
wxString *wxListItem_GetText(const wxListItem * self) {
    return new wxString(self->GetText());
}
wxColour *wxListItem_GetTextColour(const wxListItem * self) {
    return new wxColour(self->GetTextColour());
}
int wxListItem_GetWidth(const wxListItem * self) {
    return self->GetWidth();
}
void wxListItem_SetBackgroundColour(wxListItem * self, const wxColour * col_back) {
    return self->SetBackgroundColour(*col_back);
}
void wxListItem_SetColumn(wxListItem * self, int col) {
    return self->SetColumn(col);
}
void wxListItem_SetData(wxListItem * self, long data) {
    return self->SetData(data);
}
void wxListItem_SetData1(wxListItem * self, void * data) {
    return self->SetData(data);
}
void wxListItem_SetFont(wxListItem * self, const wxFont * font) {
    return self->SetFont(*font);
}
void wxListItem_SetId(wxListItem * self, long id) {
    return self->SetId(id);
}
void wxListItem_SetImage(wxListItem * self, int image) {
    return self->SetImage(image);
}
void wxListItem_SetMask(wxListItem * self, long mask) {
    return self->SetMask(mask);
}
void wxListItem_SetState(wxListItem * self, long state) {
    return self->SetState(state);
}
void wxListItem_SetStateMask(wxListItem * self, long state_mask) {
    return self->SetStateMask(state_mask);
}
void wxListItem_SetText(wxListItem * self, const wxString * text) {
    return self->SetText(*text);
}
void wxListItem_SetTextColour(wxListItem * self, const wxColour * col_text) {
    return self->SetTextColour(*col_text);
}
void wxListItem_SetWidth(wxListItem * self, int width) {
    return self->SetWidth(width);
}

// CLASS: wxListView
wxClassInfo *wxListView_CLASSINFO() {
    return wxCLASSINFO(wxListView);
}
wxListView *wxListView_new() {
    return new wxListView();
}
wxListView *wxListView_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxListView(parent, winid, *pos, *size, style, *validator, *name);
}
void wxListView_ClearColumnImage(wxListView * self, int col) {
    return self->ClearColumnImage(col);
}
void wxListView_Focus(wxListView * self, long index) {
    return self->Focus(index);
}
long wxListView_GetFirstSelected(const wxListView * self) {
    return self->GetFirstSelected();
}
long wxListView_GetFocusedItem(const wxListView * self) {
    return self->GetFocusedItem();
}
long wxListView_GetNextSelected(const wxListView * self, long item) {
    return self->GetNextSelected(item);
}
bool wxListView_IsSelected(const wxListView * self, long index) {
    return self->IsSelected(index);
}
void wxListView_Select(wxListView * self, long n, bool on) {
    return self->Select(n, on);
}
void wxListView_SetColumnImage(wxListView * self, int col, int image) {
    return self->SetColumnImage(col, image);
}
// Mix-in(s) to wxListView
wxTrackable *wxListView_AsTrackable(wxListView* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxListbook
wxClassInfo *wxListbook_CLASSINFO() {
    return wxCLASSINFO(wxListbook);
}
wxListbook *wxListbook_new() {
    return new wxListbook();
}
wxListbook *wxListbook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxListbook(parent, id, *pos, *size, style, *name);
}
bool wxListbook_Create(wxListbook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
wxListView * wxListbook_GetListView(const wxListbook * self) {
    return self->GetListView();
}
// Mix-in(s) to wxListbook
wxWithImages *wxListbook_AsWithImages(wxListbook* obj) {
    return static_cast<wxWithImages*>(obj);
}
wxTrackable *wxListbook_AsTrackable(wxListbook* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxLogGui
void wxLogGui_delete(wxLogGui *self) {
    delete self;
}
wxLogGui *wxLogGui_new() {
    return new wxLogGui();
}

// CLASS: wxLogTextCtrl
void wxLogTextCtrl_delete(wxLogTextCtrl *self) {
    delete self;
}
wxLogTextCtrl *wxLogTextCtrl_new(wxTextCtrl * p_text_ctrl) {
    return new wxLogTextCtrl(p_text_ctrl);
}

// CLASS: wxLogWindow
void wxLogWindow_delete(wxLogWindow *self) {
    delete self;
}
wxLogWindow *wxLogWindow_new(wxWindow * p_parent, const wxString * sz_title, bool show, bool pass_to_old) {
    return new wxLogWindow(p_parent, *sz_title, show, pass_to_old);
}
wxFrame * wxLogWindow_GetFrame(const wxLogWindow * self) {
    return self->GetFrame();
}
bool wxLogWindow_OnFrameClose(wxLogWindow * self, wxFrame * frame) {
    return self->OnFrameClose(frame);
}
void wxLogWindow_OnFrameDelete(wxLogWindow * self, wxFrame * frame) {
    return self->OnFrameDelete(frame);
}
void wxLogWindow_Show(wxLogWindow * self, bool show) {
    return self->Show(show);
}

// CLASS: wxLongPressEvent
wxClassInfo *wxLongPressEvent_CLASSINFO() {
    return wxCLASSINFO(wxLongPressEvent);
}
wxLongPressEvent *wxLongPressEvent_new(wxWindowID windid) {
    return new wxLongPressEvent(windid);
}

// CLASS: wxMDIChildFrame
wxClassInfo *wxMDIChildFrame_CLASSINFO() {
    return wxCLASSINFO(wxMDIChildFrame);
}
wxMDIChildFrame *wxMDIChildFrame_new() {
    return new wxMDIChildFrame();
}
wxMDIChildFrame *wxMDIChildFrame_new1(wxMDIParentFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxMDIChildFrame(parent, id, *title, *pos, *size, style, *name);
}
void wxMDIChildFrame_Activate(wxMDIChildFrame * self) {
    return self->Activate();
}
bool wxMDIChildFrame_Create(wxMDIChildFrame * self, wxMDIParentFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxMDIParentFrame * wxMDIChildFrame_GetMDIParent(const wxMDIChildFrame * self) {
    return self->GetMDIParent();
}
void wxMDIChildFrame_Restore(wxMDIChildFrame * self) {
    return self->Restore();
}
// Mix-in(s) to wxMDIChildFrame
wxTrackable *wxMDIChildFrame_AsTrackable(wxMDIChildFrame* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxMDIClientWindow
wxClassInfo *wxMDIClientWindow_CLASSINFO() {
    return wxCLASSINFO(wxMDIClientWindow);
}
wxMDIClientWindow *wxMDIClientWindow_new() {
    return new wxMDIClientWindow();
}
bool wxMDIClientWindow_CreateClient(wxMDIClientWindow * self, wxMDIParentFrame * parent, long style) {
    return self->CreateClient(parent, style);
}
// Mix-in(s) to wxMDIClientWindow
wxTrackable *wxMDIClientWindow_AsTrackable(wxMDIClientWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxMDIParentFrame
wxClassInfo *wxMDIParentFrame_CLASSINFO() {
    return wxCLASSINFO(wxMDIParentFrame);
}
wxMDIParentFrame *wxMDIParentFrame_new() {
    return new wxMDIParentFrame();
}
wxMDIParentFrame *wxMDIParentFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxMDIParentFrame(parent, id, *title, *pos, *size, style, *name);
}
void wxMDIParentFrame_ActivateNext(wxMDIParentFrame * self) {
    return self->ActivateNext();
}
void wxMDIParentFrame_ActivatePrevious(wxMDIParentFrame * self) {
    return self->ActivatePrevious();
}
void wxMDIParentFrame_ArrangeIcons(wxMDIParentFrame * self) {
    return self->ArrangeIcons();
}
void wxMDIParentFrame_Cascade(wxMDIParentFrame * self) {
    return self->Cascade();
}
bool wxMDIParentFrame_Create(wxMDIParentFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxMDIChildFrame * wxMDIParentFrame_GetActiveChild(const wxMDIParentFrame * self) {
    return self->GetActiveChild();
}
wxMDIClientWindowBase * wxMDIParentFrame_GetClientWindow(const wxMDIParentFrame * self) {
    return self->GetClientWindow();
}
wxMenu * wxMDIParentFrame_GetWindowMenu(const wxMDIParentFrame * self) {
    return self->GetWindowMenu();
}
wxMDIClientWindow * wxMDIParentFrame_OnCreateClient(wxMDIParentFrame * self) {
    return self->OnCreateClient();
}
void wxMDIParentFrame_SetWindowMenu(wxMDIParentFrame * self, wxMenu * menu) {
    return self->SetWindowMenu(menu);
}
bool wxMDIParentFrame_IsTDI() {
    return wxMDIParentFrame::IsTDI();
}
// Mix-in(s) to wxMDIParentFrame
wxTrackable *wxMDIParentFrame_AsTrackable(wxMDIParentFrame* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxMask
wxClassInfo *wxMask_CLASSINFO() {
    return wxCLASSINFO(wxMask);
}
wxMask *wxMask_new() {
    return new wxMask();
}
wxMask *wxMask_new1(const wxBitmap * bitmap, int index) {
    return new wxMask(*bitmap, index);
}
wxMask *wxMask_new2(const wxBitmap * bitmap) {
    return new wxMask(*bitmap);
}
wxMask *wxMask_new3(const wxBitmap * bitmap, const wxColour * colour) {
    return new wxMask(*bitmap, *colour);
}
bool wxMask_Create(wxMask * self, const wxBitmap * bitmap, int index) {
    return self->Create(*bitmap, index);
}
bool wxMask_Create1(wxMask * self, const wxBitmap * bitmap) {
    return self->Create(*bitmap);
}
bool wxMask_Create2(wxMask * self, const wxBitmap * bitmap, const wxColour * colour) {
    return self->Create(*bitmap, *colour);
}
wxBitmap *wxMask_GetBitmap(const wxMask * self) {
    return new wxBitmap(self->GetBitmap());
}

// CLASS: wxMaximizeEvent
wxClassInfo *wxMaximizeEvent_CLASSINFO() {
    return wxCLASSINFO(wxMaximizeEvent);
}
wxMaximizeEvent *wxMaximizeEvent_new(int id) {
    return new wxMaximizeEvent(id);
}

// CLASS: wxMemoryDC
wxClassInfo *wxMemoryDC_CLASSINFO() {
    return wxCLASSINFO(wxMemoryDC);
}
wxMemoryDC *wxMemoryDC_new() {
    return new wxMemoryDC();
}
wxMemoryDC *wxMemoryDC_new1(wxDC * dc) {
    return new wxMemoryDC(dc);
}
wxMemoryDC *wxMemoryDC_new2(wxBitmap * bitmap) {
    return new wxMemoryDC(*bitmap);
}
void wxMemoryDC_SelectObject(wxMemoryDC * self, wxBitmap * bitmap) {
    return self->SelectObject(*bitmap);
}
void wxMemoryDC_SelectObjectAsSource(wxMemoryDC * self, const wxBitmap * bitmap) {
    return self->SelectObjectAsSource(*bitmap);
}
wxBitmap *wxMemoryDC_GetSelectedBitmap(const wxMemoryDC * self) {
    return new wxBitmap(self->GetSelectedBitmap());
}
wxBitmap * wxMemoryDC_GetSelectedBitmap1(wxMemoryDC * self) {
    return &(self->GetSelectedBitmap());
}

// CLASS: wxMenu
wxClassInfo *wxMenu_CLASSINFO() {
    return wxCLASSINFO(wxMenu);
}
wxMenu *wxMenu_new() {
    return new wxMenu();
}
wxMenu *wxMenu_new1(long style) {
    return new wxMenu(style);
}
wxMenu *wxMenu_new2(const wxString * title, long style) {
    return new wxMenu(*title, style);
}
wxMenuItem * wxMenu_Append(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind) {
    return self->Append(id, *item, *help_string, kind);
}
wxMenuItem * wxMenu_Append1(wxMenu * self, int id, const wxString * item, wxMenu * sub_menu, const wxString * help_string) {
    return self->Append(id, *item, sub_menu, *help_string);
}
wxMenuItem * wxMenu_Append2(wxMenu * self, wxMenuItem * menu_item) {
    return self->Append(menu_item);
}
wxMenuItem * wxMenu_AppendCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help) {
    return self->AppendCheckItem(id, *item, *help);
}
wxMenuItem * wxMenu_AppendRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help) {
    return self->AppendRadioItem(id, *item, *help);
}
wxMenuItem * wxMenu_AppendSeparator(wxMenu * self) {
    return self->AppendSeparator();
}
wxMenuItem * wxMenu_AppendSubMenu(wxMenu * self, wxMenu * submenu, const wxString * text, const wxString * help) {
    return self->AppendSubMenu(submenu, *text, *help);
}
void wxMenu_Break(wxMenu * self) {
    return self->Break();
}
void wxMenu_Check(wxMenu * self, int id, bool check) {
    return self->Check(id, check);
}
bool wxMenu_Delete(wxMenu * self, int id) {
    return self->Delete(id);
}
bool wxMenu_Delete1(wxMenu * self, wxMenuItem * item) {
    return self->Delete(item);
}
bool wxMenu_Destroy(wxMenu * self, int id) {
    return self->Destroy(id);
}
bool wxMenu_Destroy1(wxMenu * self, wxMenuItem * item) {
    return self->Destroy(item);
}
void wxMenu_Enable(wxMenu * self, int id, bool enable) {
    return self->Enable(id, enable);
}
wxMenuItem * wxMenu_FindChildItem(const wxMenu * self, int id, size_t * pos) {
    return self->FindChildItem(id, pos);
}
int wxMenu_FindItem(const wxMenu * self, const wxString * item_string) {
    return self->FindItem(*item_string);
}
wxMenuItem * wxMenu_FindItem1(const wxMenu * self, int id, wxMenu ** menu) {
    return self->FindItem(id, menu);
}
wxMenuItem * wxMenu_FindItemByPosition(const wxMenu * self, size_t position) {
    return self->FindItemByPosition(position);
}
wxString *wxMenu_GetHelpString(const wxMenu * self, int id) {
    return new wxString(self->GetHelpString(id));
}
wxString *wxMenu_GetLabel(const wxMenu * self, int id) {
    return new wxString(self->GetLabel(id));
}
wxString *wxMenu_GetLabelText(const wxMenu * self, int id) {
    return new wxString(self->GetLabelText(id));
}
size_t wxMenu_GetMenuItemCount(const wxMenu * self) {
    return self->GetMenuItemCount();
}
wxString *wxMenu_GetTitle(const wxMenu * self) {
    return new wxString(self->GetTitle());
}
wxMenuItem * wxMenu_Insert(wxMenu * self, size_t pos, wxMenuItem * menu_item) {
    return self->Insert(pos, menu_item);
}
wxMenuItem * wxMenu_Insert1(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string, wxItemKind kind) {
    return self->Insert(pos, id, *item, *help_string, kind);
}
wxMenuItem * wxMenu_Insert2(wxMenu * self, size_t pos, int id, const wxString * text, wxMenu * submenu, const wxString * help) {
    return self->Insert(pos, id, *text, submenu, *help);
}
wxMenuItem * wxMenu_InsertCheckItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string) {
    return self->InsertCheckItem(pos, id, *item, *help_string);
}
wxMenuItem * wxMenu_InsertRadioItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string) {
    return self->InsertRadioItem(pos, id, *item, *help_string);
}
wxMenuItem * wxMenu_InsertSeparator(wxMenu * self, size_t pos) {
    return self->InsertSeparator(pos);
}
bool wxMenu_IsChecked(const wxMenu * self, int id) {
    return self->IsChecked(id);
}
bool wxMenu_IsEnabled(const wxMenu * self, int id) {
    return self->IsEnabled(id);
}
wxMenuItem * wxMenu_Prepend(wxMenu * self, wxMenuItem * item) {
    return self->Prepend(item);
}
wxMenuItem * wxMenu_Prepend1(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind) {
    return self->Prepend(id, *item, *help_string, kind);
}
wxMenuItem * wxMenu_Prepend2(wxMenu * self, int id, const wxString * text, wxMenu * submenu, const wxString * help) {
    return self->Prepend(id, *text, submenu, *help);
}
wxMenuItem * wxMenu_PrependCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help_string) {
    return self->PrependCheckItem(id, *item, *help_string);
}
wxMenuItem * wxMenu_PrependRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help_string) {
    return self->PrependRadioItem(id, *item, *help_string);
}
wxMenuItem * wxMenu_PrependSeparator(wxMenu * self) {
    return self->PrependSeparator();
}
wxMenuItem * wxMenu_Remove(wxMenu * self, int id) {
    return self->Remove(id);
}
wxMenuItem * wxMenu_Remove1(wxMenu * self, wxMenuItem * item) {
    return self->Remove(item);
}
void wxMenu_SetHelpString(wxMenu * self, int id, const wxString * help_string) {
    return self->SetHelpString(id, *help_string);
}
void wxMenu_SetLabel(wxMenu * self, int id, const wxString * label) {
    return self->SetLabel(id, *label);
}
void wxMenu_SetTitle(wxMenu * self, const wxString * title) {
    return self->SetTitle(*title);
}
void wxMenu_UpdateUI(wxMenu * self, wxEvtHandler * source) {
    return self->UpdateUI(source);
}
void wxMenu_SetInvokingWindow(wxMenu * self, wxWindow * win) {
    return self->SetInvokingWindow(win);
}
wxWindow * wxMenu_GetInvokingWindow(const wxMenu * self) {
    return self->GetInvokingWindow();
}
wxWindow * wxMenu_GetWindow(const wxMenu * self) {
    return self->GetWindow();
}
long wxMenu_GetStyle(const wxMenu * self) {
    return self->GetStyle();
}
void wxMenu_SetParent(wxMenu * self, wxMenu * parent) {
    return self->SetParent(parent);
}
wxMenu * wxMenu_GetParent(const wxMenu * self) {
    return self->GetParent();
}
void wxMenu_Attach(wxMenu * self, wxMenuBar * menubar) {
    return self->Attach(menubar);
}
void wxMenu_Detach(wxMenu * self) {
    return self->Detach();
}
bool wxMenu_IsAttached(const wxMenu * self) {
    return self->IsAttached();
}
// Mix-in(s) to wxMenu
wxTrackable *wxMenu_AsTrackable(wxMenu* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxMenuBar
wxClassInfo *wxMenuBar_CLASSINFO() {
    return wxCLASSINFO(wxMenuBar);
}
wxMenuBar *wxMenuBar_new(long style) {
    return new wxMenuBar(style);
}
bool wxMenuBar_Append(wxMenuBar * self, wxMenu * menu, const wxString * title) {
    return self->Append(menu, *title);
}
void wxMenuBar_Check(wxMenuBar * self, int id, bool check) {
    return self->Check(id, check);
}
void wxMenuBar_Enable(wxMenuBar * self, int id, bool enable) {
    return self->Enable(id, enable);
}
bool wxMenuBar_IsEnabledTop(const wxMenuBar * self, size_t pos) {
    return self->IsEnabledTop(pos);
}
void wxMenuBar_EnableTop(wxMenuBar * self, size_t pos, bool enable) {
    return self->EnableTop(pos, enable);
}
wxMenuItem * wxMenuBar_FindItem(const wxMenuBar * self, int id, wxMenu ** menu) {
    return self->FindItem(id, menu);
}
int wxMenuBar_FindMenu(const wxMenuBar * self, const wxString * title) {
    return self->FindMenu(*title);
}
int wxMenuBar_FindMenuItem(const wxMenuBar * self, const wxString * menu_string, const wxString * item_string) {
    return self->FindMenuItem(*menu_string, *item_string);
}
wxString *wxMenuBar_GetHelpString(const wxMenuBar * self, int id) {
    return new wxString(self->GetHelpString(id));
}
wxString *wxMenuBar_GetLabel(const wxMenuBar * self, int id) {
    return new wxString(self->GetLabel(id));
}
wxMenu * wxMenuBar_GetMenu(const wxMenuBar * self, size_t menu_index) {
    return self->GetMenu(menu_index);
}
size_t wxMenuBar_GetMenuCount(const wxMenuBar * self) {
    return self->GetMenuCount();
}
wxString *wxMenuBar_GetMenuLabel(const wxMenuBar * self, size_t pos) {
    return new wxString(self->GetMenuLabel(pos));
}
wxString *wxMenuBar_GetMenuLabelText(const wxMenuBar * self, size_t pos) {
    return new wxString(self->GetMenuLabelText(pos));
}
bool wxMenuBar_Insert(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title) {
    return self->Insert(pos, menu, *title);
}
bool wxMenuBar_IsChecked(const wxMenuBar * self, int id) {
    return self->IsChecked(id);
}
bool wxMenuBar_IsEnabled(const wxMenuBar * self, int id) {
    return self->IsEnabled(id);
}
wxMenu * wxMenuBar_Remove(wxMenuBar * self, size_t pos) {
    return self->Remove(pos);
}
wxMenu * wxMenuBar_Replace(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title) {
    return self->Replace(pos, menu, *title);
}
void wxMenuBar_SetHelpString(wxMenuBar * self, int id, const wxString * help_string) {
    return self->SetHelpString(id, *help_string);
}
void wxMenuBar_SetLabel(wxMenuBar * self, int id, const wxString * label) {
    return self->SetLabel(id, *label);
}
void wxMenuBar_SetMenuLabel(wxMenuBar * self, size_t pos, const wxString * label) {
    return self->SetMenuLabel(pos, *label);
}
#ifdef __WXOSX__
wxMenu * wxMenuBar_OSXGetAppleMenu(const wxMenuBar * self) {
    return self->OSXGetAppleMenu();
}
#endif
wxFrame * wxMenuBar_GetFrame(const wxMenuBar * self) {
    return self->GetFrame();
}
bool wxMenuBar_IsAttached(const wxMenuBar * self) {
    return self->IsAttached();
}
void wxMenuBar_Attach(wxMenuBar * self, wxFrame * frame) {
    return self->Attach(frame);
}
void wxMenuBar_Detach(wxMenuBar * self) {
    return self->Detach();
}
#ifdef __WXOSX__
void wxMenuBar_MacSetCommonMenuBar(wxMenuBar * menubar) {
    return wxMenuBar::MacSetCommonMenuBar(menubar);
}
wxMenuBar * wxMenuBar_MacGetCommonMenuBar() {
    return wxMenuBar::MacGetCommonMenuBar();
}
#endif
// Mix-in(s) to wxMenuBar
wxTrackable *wxMenuBar_AsTrackable(wxMenuBar* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxMenuEvent
wxClassInfo *wxMenuEvent_CLASSINFO() {
    return wxCLASSINFO(wxMenuEvent);
}
wxMenu * wxMenuEvent_GetMenu(const wxMenuEvent * self) {
    return self->GetMenu();
}
int wxMenuEvent_GetMenuId(const wxMenuEvent * self) {
    return self->GetMenuId();
}
bool wxMenuEvent_IsPopup(const wxMenuEvent * self) {
    return self->IsPopup();
}

// CLASS: wxMenuItem
wxClassInfo *wxMenuItem_CLASSINFO() {
    return wxCLASSINFO(wxMenuItem);
}
wxBitmap *wxMenuItem_GetBitmap(const wxMenuItem * self) {
    return new wxBitmap(self->GetBitmap());
}
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetBitmap1(const wxMenuItem * self, bool checked) {
    return new wxBitmap(self->GetBitmap(checked));
}
#endif
#if wxCHECK_VERSION(3, 2, 0)
wxBitmapBundle *wxMenuItem_GetBitmapBundle(const wxMenuItem * self) {
    return new wxBitmapBundle(self->GetBitmapBundle());
}
#endif
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetDisabledBitmap(const wxMenuItem * self) {
    return new wxBitmap(self->GetDisabledBitmap());
}
#endif
wxString *wxMenuItem_GetHelp(const wxMenuItem * self) {
    return new wxString(self->GetHelp());
}
int wxMenuItem_GetId(const wxMenuItem * self) {
    return self->GetId();
}
wxString *wxMenuItem_GetItemLabel(const wxMenuItem * self) {
    return new wxString(self->GetItemLabel());
}
wxString *wxMenuItem_GetItemLabelText(const wxMenuItem * self) {
    return new wxString(self->GetItemLabelText());
}
wxItemKind wxMenuItem_GetKind(const wxMenuItem * self) {
    return self->GetKind();
}
#ifdef __WXMSW__
int wxMenuItem_GetMarginWidth(const wxMenuItem * self) {
    return self->GetMarginWidth();
}
#endif
wxMenu * wxMenuItem_GetMenu(const wxMenuItem * self) {
    return self->GetMenu();
}
wxMenu * wxMenuItem_GetSubMenu(const wxMenuItem * self) {
    return self->GetSubMenu();
}
wxAcceleratorEntry * wxMenuItem_GetAccel(const wxMenuItem * self) {
    return self->GetAccel();
}
bool wxMenuItem_IsCheck(const wxMenuItem * self) {
    return self->IsCheck();
}
bool wxMenuItem_IsCheckable(const wxMenuItem * self) {
    return self->IsCheckable();
}
bool wxMenuItem_IsChecked(const wxMenuItem * self) {
    return self->IsChecked();
}
bool wxMenuItem_IsEnabled(const wxMenuItem * self) {
    return self->IsEnabled();
}
bool wxMenuItem_IsRadio(const wxMenuItem * self) {
    return self->IsRadio();
}
bool wxMenuItem_IsSeparator(const wxMenuItem * self) {
    return self->IsSeparator();
}
bool wxMenuItem_IsSubMenu(const wxMenuItem * self) {
    return self->IsSubMenu();
}
#ifdef __WXMSW__
void wxMenuItem_SetBackgroundColour(wxMenuItem * self, const wxColour * colour) {
    return self->SetBackgroundColour(*colour);
}
#endif
void wxMenuItem_SetBitmap(wxMenuItem * self, const wxBitmapBundle * bmp) {
    return self->SetBitmap(*bmp);
}
#ifdef __WXMSW__
void wxMenuItem_SetBitmap1(wxMenuItem * self, const wxBitmapBundle * bmp, bool checked) {
    return self->SetBitmap(*bmp, checked);
}
void wxMenuItem_SetBitmaps(wxMenuItem * self, const wxBitmapBundle * checked, const wxBitmapBundle * unchecked) {
    return self->SetBitmaps(*checked, *unchecked);
}
void wxMenuItem_SetDisabledBitmap(wxMenuItem * self, const wxBitmapBundle * disabled) {
    return self->SetDisabledBitmap(*disabled);
}
void wxMenuItem_SetFont(wxMenuItem * self, const wxFont * font) {
    return self->SetFont(*font);
}
#endif
void wxMenuItem_SetHelp(wxMenuItem * self, const wxString * help_string) {
    return self->SetHelp(*help_string);
}
void wxMenuItem_SetItemLabel(wxMenuItem * self, const wxString * label) {
    return self->SetItemLabel(*label);
}
#ifdef __WXMSW__
void wxMenuItem_SetMarginWidth(wxMenuItem * self, int width) {
    return self->SetMarginWidth(width);
}
#endif
void wxMenuItem_SetMenu(wxMenuItem * self, wxMenu * menu) {
    return self->SetMenu(menu);
}
void wxMenuItem_SetSubMenu(wxMenuItem * self, wxMenu * menu) {
    return self->SetSubMenu(menu);
}
#ifdef __WXMSW__
void wxMenuItem_SetTextColour(wxMenuItem * self, const wxColour * colour) {
    return self->SetTextColour(*colour);
}
#endif
void wxMenuItem_SetAccel(wxMenuItem * self, wxAcceleratorEntry * accel) {
    return self->SetAccel(accel);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxMenuItem_AddExtraAccel(wxMenuItem * self, const wxAcceleratorEntry * accel) {
    return self->AddExtraAccel(*accel);
}
void wxMenuItem_ClearExtraAccels(wxMenuItem * self) {
    return self->ClearExtraAccels();
}
#endif
wxMenuItem *wxMenuItem_new(wxMenu * parent_menu, int id, const wxString * text, const wxString * help_string, wxItemKind kind, wxMenu * sub_menu) {
    return new wxMenuItem(parent_menu, id, *text, *help_string, kind, sub_menu);
}
void wxMenuItem_Check(wxMenuItem * self, bool check) {
    return self->Check(check);
}
void wxMenuItem_Enable(wxMenuItem * self, bool enable) {
    return self->Enable(enable);
}
wxString *wxMenuItem_GetLabelText(const wxString * text) {
    return new wxString(wxMenuItem::GetLabelText(*text));
}

// CLASS: wxMessageDialog
wxClassInfo *wxMessageDialog_CLASSINFO() {
    return wxCLASSINFO(wxMessageDialog);
}
wxMessageDialog *wxMessageDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, long style, const wxPoint * pos) {
    return new wxMessageDialog(parent, *message, *caption, style, *pos);
}
void wxMessageDialog_SetExtendedMessage(wxMessageDialog * self, const wxString * extended_message) {
    return self->SetExtendedMessage(*extended_message);
}
bool wxMessageDialog_SetHelpLabel(wxMessageDialog * self, const ButtonLabel * help) {
    return self->SetHelpLabel(*help);
}
void wxMessageDialog_SetMessage(wxMessageDialog * self, const wxString * message) {
    return self->SetMessage(*message);
}
bool wxMessageDialog_SetOKCancelLabels(wxMessageDialog * self, const ButtonLabel * ok, const ButtonLabel * cancel) {
    return self->SetOKCancelLabels(*ok, *cancel);
}
bool wxMessageDialog_SetOKLabel(wxMessageDialog * self, const ButtonLabel * ok) {
    return self->SetOKLabel(*ok);
}
bool wxMessageDialog_SetYesNoCancelLabels(wxMessageDialog * self, const ButtonLabel * yes, const ButtonLabel * no, const ButtonLabel * cancel) {
    return self->SetYesNoCancelLabels(*yes, *no, *cancel);
}
bool wxMessageDialog_SetYesNoLabels(wxMessageDialog * self, const ButtonLabel * yes, const ButtonLabel * no) {
    return self->SetYesNoLabels(*yes, *no);
}
wxString *wxMessageDialog_GetCaption(const wxMessageDialog * self) {
    return new wxString(self->GetCaption());
}
wxString *wxMessageDialog_GetMessage(const wxMessageDialog * self) {
    return new wxString(self->GetMessage());
}
wxString *wxMessageDialog_GetExtendedMessage(const wxMessageDialog * self) {
    return new wxString(self->GetExtendedMessage());
}
long wxMessageDialog_GetMessageDialogStyle(const wxMessageDialog * self) {
    return self->GetMessageDialogStyle();
}
bool wxMessageDialog_HasCustomLabels(const wxMessageDialog * self) {
    return self->HasCustomLabels();
}
wxString *wxMessageDialog_GetYesLabel(const wxMessageDialog * self) {
    return new wxString(self->GetYesLabel());
}
wxString *wxMessageDialog_GetNoLabel(const wxMessageDialog * self) {
    return new wxString(self->GetNoLabel());
}
wxString *wxMessageDialog_GetOKLabel(const wxMessageDialog * self) {
    return new wxString(self->GetOKLabel());
}
wxString *wxMessageDialog_GetCancelLabel(const wxMessageDialog * self) {
    return new wxString(self->GetCancelLabel());
}
wxString *wxMessageDialog_GetHelpLabel(const wxMessageDialog * self) {
    return new wxString(self->GetHelpLabel());
}
long wxMessageDialog_GetEffectiveIcon(const wxMessageDialog * self) {
    return self->GetEffectiveIcon();
}
// Mix-in(s) to wxMessageDialog
wxTrackable *wxMessageDialog_AsTrackable(wxMessageDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxMessageOutputMessageBox
void wxMessageOutputMessageBox_delete(wxMessageOutputMessageBox *self) {
    delete self;
}
wxMessageOutputMessageBox *wxMessageOutputMessageBox_new() {
    return new wxMessageOutputMessageBox();
}

// CLASS: wxMetafile
wxClassInfo *wxMetafile_CLASSINFO() {
    return wxCLASSINFO(wxMetafile);
}
wxMetafile *wxMetafile_new(const wxString * filename) {
    return new wxMetafile(*filename);
}
bool wxMetafile_IsOk(wxMetafile * self) {
    return self->IsOk();
}
bool wxMetafile_Play(wxMetafile * self, wxDC * dc) {
    return self->Play(dc);
}
bool wxMetafile_SetClipboard(wxMetafile * self, int width, int height) {
    return self->SetClipboard(width, height);
}

// CLASS: wxMetafileDC
wxClassInfo *wxMetafileDC_CLASSINFO() {
    return wxCLASSINFO(wxMetafileDC);
}
wxMetafileDC *wxMetafileDC_new(const wxString * filename) {
    return new wxMetafileDC(*filename);
}
wxMetafile * wxMetafileDC_Close(wxMetafileDC * self) {
    return self->Close();
}

// CLASS: wxMiniFrame
wxClassInfo *wxMiniFrame_CLASSINFO() {
    return wxCLASSINFO(wxMiniFrame);
}
wxMiniFrame *wxMiniFrame_new() {
    return new wxMiniFrame();
}
wxMiniFrame *wxMiniFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxMiniFrame(parent, id, *title, *pos, *size, style, *name);
}
bool wxMiniFrame_Create(wxMiniFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
// Mix-in(s) to wxMiniFrame
wxTrackable *wxMiniFrame_AsTrackable(wxMiniFrame* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxMirrorDC
wxClassInfo *wxMirrorDC_CLASSINFO() {
    return wxCLASSINFO(wxMirrorDC);
}
wxMirrorDC *wxMirrorDC_new(wxDC * dc, bool mirror) {
    return new wxMirrorDC(*dc, mirror);
}

// CLASS: wxMouseCaptureChangedEvent
wxClassInfo *wxMouseCaptureChangedEvent_CLASSINFO() {
    return wxCLASSINFO(wxMouseCaptureChangedEvent);
}
wxMouseCaptureChangedEvent *wxMouseCaptureChangedEvent_new(wxWindowID window_id, wxWindow * gained_capture) {
    return new wxMouseCaptureChangedEvent(window_id, gained_capture);
}
wxWindow * wxMouseCaptureChangedEvent_GetCapturedWindow(const wxMouseCaptureChangedEvent * self) {
    return self->GetCapturedWindow();
}

// CLASS: wxMouseCaptureLostEvent
wxClassInfo *wxMouseCaptureLostEvent_CLASSINFO() {
    return wxCLASSINFO(wxMouseCaptureLostEvent);
}
wxMouseCaptureLostEvent *wxMouseCaptureLostEvent_new(wxWindowID window_id) {
    return new wxMouseCaptureLostEvent(window_id);
}

// CLASS: wxMouseEvent
wxClassInfo *wxMouseEvent_CLASSINFO() {
    return wxCLASSINFO(wxMouseEvent);
}
bool wxMouseEvent_Aux1DClick(const wxMouseEvent * self) {
    return self->Aux1DClick();
}
bool wxMouseEvent_Aux1Down(const wxMouseEvent * self) {
    return self->Aux1Down();
}
bool wxMouseEvent_Aux1Up(const wxMouseEvent * self) {
    return self->Aux1Up();
}
bool wxMouseEvent_Aux2DClick(const wxMouseEvent * self) {
    return self->Aux2DClick();
}
bool wxMouseEvent_Aux2Down(const wxMouseEvent * self) {
    return self->Aux2Down();
}
bool wxMouseEvent_Aux2Up(const wxMouseEvent * self) {
    return self->Aux2Up();
}
bool wxMouseEvent_Dragging(const wxMouseEvent * self) {
    return self->Dragging();
}
bool wxMouseEvent_Entering(const wxMouseEvent * self) {
    return self->Entering();
}
int wxMouseEvent_GetButton(const wxMouseEvent * self) {
    return self->GetButton();
}
int wxMouseEvent_GetClickCount(const wxMouseEvent * self) {
    return self->GetClickCount();
}
int wxMouseEvent_GetLinesPerAction(const wxMouseEvent * self) {
    return self->GetLinesPerAction();
}
int wxMouseEvent_GetColumnsPerAction(const wxMouseEvent * self) {
    return self->GetColumnsPerAction();
}
wxPoint *wxMouseEvent_GetLogicalPosition(const wxMouseEvent * self, const wxDC * dc) {
    return new wxPoint(self->GetLogicalPosition(*dc));
}
int wxMouseEvent_GetWheelDelta(const wxMouseEvent * self) {
    return self->GetWheelDelta();
}
bool wxMouseEvent_IsWheelInverted(const wxMouseEvent * self) {
    return self->IsWheelInverted();
}
int wxMouseEvent_GetWheelRotation(const wxMouseEvent * self) {
    return self->GetWheelRotation();
}
bool wxMouseEvent_IsButton(const wxMouseEvent * self) {
    return self->IsButton();
}
bool wxMouseEvent_IsPageScroll(const wxMouseEvent * self) {
    return self->IsPageScroll();
}
bool wxMouseEvent_Leaving(const wxMouseEvent * self) {
    return self->Leaving();
}
bool wxMouseEvent_LeftDClick(const wxMouseEvent * self) {
    return self->LeftDClick();
}
bool wxMouseEvent_LeftDown(const wxMouseEvent * self) {
    return self->LeftDown();
}
bool wxMouseEvent_LeftUp(const wxMouseEvent * self) {
    return self->LeftUp();
}
bool wxMouseEvent_Magnify(const wxMouseEvent * self) {
    return self->Magnify();
}
bool wxMouseEvent_MetaDown(const wxMouseEvent * self) {
    return self->MetaDown();
}
bool wxMouseEvent_MiddleDClick(const wxMouseEvent * self) {
    return self->MiddleDClick();
}
bool wxMouseEvent_MiddleDown(const wxMouseEvent * self) {
    return self->MiddleDown();
}
bool wxMouseEvent_MiddleUp(const wxMouseEvent * self) {
    return self->MiddleUp();
}
bool wxMouseEvent_Moving(const wxMouseEvent * self) {
    return self->Moving();
}
bool wxMouseEvent_RightDClick(const wxMouseEvent * self) {
    return self->RightDClick();
}
bool wxMouseEvent_RightDown(const wxMouseEvent * self) {
    return self->RightDown();
}
bool wxMouseEvent_RightUp(const wxMouseEvent * self) {
    return self->RightUp();
}
// Mix-in(s) to wxMouseEvent
wxMouseState *wxMouseEvent_AsMouseState(wxMouseEvent* obj) {
    return static_cast<wxMouseState*>(obj);
}

// CLASS: wxMouseEventsManager
wxClassInfo *wxMouseEventsManager_CLASSINFO() {
    return wxCLASSINFO(wxMouseEventsManager);
}
wxMouseEventsManager *wxMouseEventsManager_new() {
    return new wxMouseEventsManager();
}
wxMouseEventsManager *wxMouseEventsManager_new1(wxWindow * win) {
    return new wxMouseEventsManager(win);
}
bool wxMouseEventsManager_Create(wxMouseEventsManager * self, wxWindow * win) {
    return self->Create(win);
}
// Mix-in(s) to wxMouseEventsManager
wxTrackable *wxMouseEventsManager_AsTrackable(wxMouseEventsManager* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxMoveEvent
wxClassInfo *wxMoveEvent_CLASSINFO() {
    return wxCLASSINFO(wxMoveEvent);
}
wxMoveEvent *wxMoveEvent_new(const wxPoint * pt, int id) {
    return new wxMoveEvent(*pt, id);
}
wxPoint *wxMoveEvent_GetPosition(const wxMoveEvent * self) {
    return new wxPoint(self->GetPosition());
}
wxRect *wxMoveEvent_GetRect(const wxMoveEvent * self) {
    return new wxRect(self->GetRect());
}
void wxMoveEvent_SetRect(wxMoveEvent * self, const wxRect * rect) {
    return self->SetRect(*rect);
}
void wxMoveEvent_SetPosition(wxMoveEvent * self, const wxPoint * pos) {
    return self->SetPosition(*pos);
}

// CLASS: wxNativeFontInfo
void wxNativeFontInfo_delete(wxNativeFontInfo *self) {
    delete self;
}
wxNativeFontInfo *wxNativeFontInfo_new() {
    return new wxNativeFontInfo();
}
wxNativeFontInfo *wxNativeFontInfo_new1(const wxNativeFontInfo * info) {
    return new wxNativeFontInfo(*info);
}
void wxNativeFontInfo_Init(wxNativeFontInfo * self) {
    return self->Init();
}
void wxNativeFontInfo_InitFromFont(wxNativeFontInfo * self, const wxFont * font) {
    return self->InitFromFont(*font);
}
int wxNativeFontInfo_GetPointSize(const wxNativeFontInfo * self) {
    return self->GetPointSize();
}
wxSize *wxNativeFontInfo_GetPixelSize(const wxNativeFontInfo * self) {
    return new wxSize(self->GetPixelSize());
}
int wxNativeFontInfo_GetNumericWeight(const wxNativeFontInfo * self) {
    return self->GetNumericWeight();
}
bool wxNativeFontInfo_GetUnderlined(const wxNativeFontInfo * self) {
    return self->GetUnderlined();
}
wxString *wxNativeFontInfo_GetFaceName(const wxNativeFontInfo * self) {
    return new wxString(self->GetFaceName());
}
void wxNativeFontInfo_SetPointSize(wxNativeFontInfo * self, int pointsize) {
    return self->SetPointSize(pointsize);
}
void wxNativeFontInfo_SetPixelSize(wxNativeFontInfo * self, const wxSize * pixel_size) {
    return self->SetPixelSize(*pixel_size);
}
void wxNativeFontInfo_SetNumericWeight(wxNativeFontInfo * self, int weight) {
    return self->SetNumericWeight(weight);
}
void wxNativeFontInfo_SetUnderlined(wxNativeFontInfo * self, bool underlined) {
    return self->SetUnderlined(underlined);
}
bool wxNativeFontInfo_SetFaceName(wxNativeFontInfo * self, const wxString * facename) {
    return self->SetFaceName(*facename);
}
void wxNativeFontInfo_SetFaceName1(wxNativeFontInfo * self, const wxArrayString * facenames) {
    return self->SetFaceName(*facenames);
}
bool wxNativeFontInfo_FromString(wxNativeFontInfo * self, const wxString * s) {
    return self->FromString(*s);
}
wxString *wxNativeFontInfo_ToString(const wxNativeFontInfo * self) {
    return new wxString(self->ToString());
}
bool wxNativeFontInfo_FromUserString(wxNativeFontInfo * self, const wxString * s) {
    return self->FromUserString(*s);
}
wxString *wxNativeFontInfo_ToUserString(const wxNativeFontInfo * self) {
    return new wxString(self->ToUserString());
}

// CLASS: wxNativeWindow
wxClassInfo *wxNativeWindow_CLASSINFO() {
    return wxCLASSINFO(wxNativeWindow);
}
wxNativeWindow *wxNativeWindow_new() {
    return new wxNativeWindow();
}
void wxNativeWindow_Disown(wxNativeWindow * self) {
    return self->Disown();
}
// Mix-in(s) to wxNativeWindow
wxTrackable *wxNativeWindow_AsTrackable(wxNativeWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxNavigationKeyEvent
wxClassInfo *wxNavigationKeyEvent_CLASSINFO() {
    return wxCLASSINFO(wxNavigationKeyEvent);
}
wxNavigationKeyEvent *wxNavigationKeyEvent_new() {
    return new wxNavigationKeyEvent();
}
wxNavigationKeyEvent *wxNavigationKeyEvent_new1(const wxNavigationKeyEvent * event) {
    return new wxNavigationKeyEvent(*event);
}
wxWindow * wxNavigationKeyEvent_GetCurrentFocus(const wxNavigationKeyEvent * self) {
    return self->GetCurrentFocus();
}
bool wxNavigationKeyEvent_GetDirection(const wxNavigationKeyEvent * self) {
    return self->GetDirection();
}
bool wxNavigationKeyEvent_IsFromTab(const wxNavigationKeyEvent * self) {
    return self->IsFromTab();
}
bool wxNavigationKeyEvent_IsWindowChange(const wxNavigationKeyEvent * self) {
    return self->IsWindowChange();
}
void wxNavigationKeyEvent_SetCurrentFocus(wxNavigationKeyEvent * self, wxWindow * current_focus) {
    return self->SetCurrentFocus(current_focus);
}
void wxNavigationKeyEvent_SetDirection(wxNavigationKeyEvent * self, bool direction) {
    return self->SetDirection(direction);
}
void wxNavigationKeyEvent_SetFlags(wxNavigationKeyEvent * self, long flags) {
    return self->SetFlags(flags);
}
void wxNavigationKeyEvent_SetFromTab(wxNavigationKeyEvent * self, bool from_tab) {
    return self->SetFromTab(from_tab);
}
void wxNavigationKeyEvent_SetWindowChange(wxNavigationKeyEvent * self, bool window_change) {
    return self->SetWindowChange(window_change);
}

// CLASS: wxNonOwnedWindow
wxClassInfo *wxNonOwnedWindow_CLASSINFO() {
    return wxCLASSINFO(wxNonOwnedWindow);
}
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region) {
    return self->SetShape(*region);
}
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path) {
    return self->SetShape(*path);
}
// Mix-in(s) to wxNonOwnedWindow
wxTrackable *wxNonOwnedWindow_AsTrackable(wxNonOwnedWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxNotebook
wxClassInfo *wxNotebook_CLASSINFO() {
    return wxCLASSINFO(wxNotebook);
}
wxNotebook *wxNotebook_new() {
    return new wxNotebook();
}
wxNotebook *wxNotebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxNotebook(parent, id, *pos, *size, style, *name);
}
int wxNotebook_GetRowCount(const wxNotebook * self) {
    return self->GetRowCount();
}
wxColour *wxNotebook_GetThemeBackgroundColour(const wxNotebook * self) {
    return new wxColour(self->GetThemeBackgroundColour());
}
void wxNotebook_SetPadding(wxNotebook * self, const wxSize * padding) {
    return self->SetPadding(*padding);
}
// Mix-in(s) to wxNotebook
wxWithImages *wxNotebook_AsWithImages(wxNotebook* obj) {
    return static_cast<wxWithImages*>(obj);
}
wxTrackable *wxNotebook_AsTrackable(wxNotebook* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxNotificationMessage
wxClassInfo *wxNotificationMessage_CLASSINFO() {
    return wxCLASSINFO(wxNotificationMessage);
}
wxNotificationMessage *wxNotificationMessage_new() {
    return new wxNotificationMessage();
}
wxNotificationMessage *wxNotificationMessage_new1(const wxString * title, const wxString * message, wxWindow * parent, int flags) {
    return new wxNotificationMessage(*title, *message, parent, flags);
}
bool wxNotificationMessage_AddAction(wxNotificationMessage * self, wxWindowID actionid, const wxString * label) {
    return self->AddAction(actionid, *label);
}
bool wxNotificationMessage_Close(wxNotificationMessage * self) {
    return self->Close();
}
void wxNotificationMessage_SetFlags(wxNotificationMessage * self, int flags) {
    return self->SetFlags(flags);
}
void wxNotificationMessage_SetIcon(wxNotificationMessage * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
void wxNotificationMessage_SetMessage(wxNotificationMessage * self, const wxString * message) {
    return self->SetMessage(*message);
}
void wxNotificationMessage_SetParent(wxNotificationMessage * self, wxWindow * parent) {
    return self->SetParent(parent);
}
void wxNotificationMessage_SetTitle(wxNotificationMessage * self, const wxString * title) {
    return self->SetTitle(*title);
}
bool wxNotificationMessage_Show(wxNotificationMessage * self, int timeout) {
    return self->Show(timeout);
}
wxTaskBarIcon * wxNotificationMessage_UseTaskBarIcon(wxTaskBarIcon * icon) {
    return wxNotificationMessage::UseTaskBarIcon(icon);
}
bool wxNotificationMessage_MSWUseToasts(const wxString * shortcut_path, const wxString * app_id) {
    return wxNotificationMessage::MSWUseToasts(*shortcut_path, *app_id);
}
// Mix-in(s) to wxNotificationMessage
wxTrackable *wxNotificationMessage_AsTrackable(wxNotificationMessage* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxNotifyEvent
wxClassInfo *wxNotifyEvent_CLASSINFO() {
    return wxCLASSINFO(wxNotifyEvent);
}
void wxNotifyEvent_Allow(wxNotifyEvent * self) {
    return self->Allow();
}
bool wxNotifyEvent_IsAllowed(const wxNotifyEvent * self) {
    return self->IsAllowed();
}
void wxNotifyEvent_Veto(wxNotifyEvent * self) {
    return self->Veto();
}

// CLASS: wxNumberEntryDialog
wxClassInfo *wxNumberEntryDialog_CLASSINFO() {
    return wxCLASSINFO(wxNumberEntryDialog);
}
wxNumberEntryDialog *wxNumberEntryDialog_new() {
    return new wxNumberEntryDialog();
}
wxNumberEntryDialog *wxNumberEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * prompt, const wxString * caption, long value, long min, long max, const wxPoint * pos) {
    return new wxNumberEntryDialog(parent, *message, *prompt, *caption, value, min, max, *pos);
}
bool wxNumberEntryDialog_Create(wxNumberEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * prompt, const wxString * caption, long value, long min, long max, const wxPoint * pos) {
    return self->Create(parent, *message, *prompt, *caption, value, min, max, *pos);
}
long wxNumberEntryDialog_GetValue(const wxNumberEntryDialog * self) {
    return self->GetValue();
}
// Mix-in(s) to wxNumberEntryDialog
wxTrackable *wxNumberEntryDialog_AsTrackable(wxNumberEntryDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxOverlay
void wxOverlay_delete(wxOverlay *self) {
    delete self;
}
wxOverlay *wxOverlay_new() {
    return new wxOverlay();
}
void wxOverlay_Reset(wxOverlay * self) {
    return self->Reset();
}

// CLASS: wxOwnerDrawnComboBox
wxClassInfo *wxOwnerDrawnComboBox_CLASSINFO() {
    return wxCLASSINFO(wxOwnerDrawnComboBox);
}
wxOwnerDrawnComboBox *wxOwnerDrawnComboBox_new() {
    return new wxOwnerDrawnComboBox();
}
wxOwnerDrawnComboBox *wxOwnerDrawnComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxOwnerDrawnComboBox(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
bool wxOwnerDrawnComboBox_Create(wxOwnerDrawnComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, *validator, *name);
}
bool wxOwnerDrawnComboBox_Create2(wxOwnerDrawnComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
bool wxOwnerDrawnComboBox_IsListEmpty(const wxOwnerDrawnComboBox * self) {
    return self->IsListEmpty();
}
bool wxOwnerDrawnComboBox_IsTextEmpty(const wxOwnerDrawnComboBox * self) {
    return self->IsTextEmpty();
}
int wxOwnerDrawnComboBox_GetWidestItem(wxOwnerDrawnComboBox * self) {
    return self->GetWidestItem();
}
int wxOwnerDrawnComboBox_GetWidestItemWidth(wxOwnerDrawnComboBox * self) {
    return self->GetWidestItemWidth();
}
// Mix-in(s) to wxOwnerDrawnComboBox
wxItemContainer *wxOwnerDrawnComboBox_AsItemContainer(wxOwnerDrawnComboBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTextEntryBase *wxOwnerDrawnComboBox_AsTextEntry(wxOwnerDrawnComboBox* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}
wxTrackable *wxOwnerDrawnComboBox_AsTrackable(wxOwnerDrawnComboBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxPCXHandler
wxClassInfo *wxPCXHandler_CLASSINFO() {
    return wxCLASSINFO(wxPCXHandler);
}
wxPCXHandler *wxPCXHandler_new() {
    return new wxPCXHandler();
}

// CLASS: wxPNGHandler
wxClassInfo *wxPNGHandler_CLASSINFO() {
    return wxCLASSINFO(wxPNGHandler);
}
wxPNGHandler *wxPNGHandler_new() {
    return new wxPNGHandler();
}

// CLASS: wxPNMHandler
wxClassInfo *wxPNMHandler_CLASSINFO() {
    return wxCLASSINFO(wxPNMHandler);
}
wxPNMHandler *wxPNMHandler_new() {
    return new wxPNMHandler();
}

// CLASS: wxPageSetupDialog
wxClassInfo *wxPageSetupDialog_CLASSINFO() {
    return wxCLASSINFO(wxPageSetupDialog);
}
wxPageSetupDialog *wxPageSetupDialog_new(wxWindow * parent, wxPageSetupDialogData * data) {
    return new wxPageSetupDialog(parent, data);
}
wxPageSetupDialogData * wxPageSetupDialog_GetPageSetupData(wxPageSetupDialog * self) {
    return &(self->GetPageSetupData());
}
int wxPageSetupDialog_ShowModal(wxPageSetupDialog * self) {
    return self->ShowModal();
}

// CLASS: wxPageSetupDialogData
wxClassInfo *wxPageSetupDialogData_CLASSINFO() {
    return wxCLASSINFO(wxPageSetupDialogData);
}
wxPageSetupDialogData *wxPageSetupDialogData_new() {
    return new wxPageSetupDialogData();
}
wxPageSetupDialogData *wxPageSetupDialogData_new1(const wxPageSetupDialogData * data) {
    return new wxPageSetupDialogData(*data);
}
wxPageSetupDialogData *wxPageSetupDialogData_new2(const wxPrintData * print_data) {
    return new wxPageSetupDialogData(*print_data);
}
void wxPageSetupDialogData_EnableHelp(wxPageSetupDialogData * self, bool flag) {
    return self->EnableHelp(flag);
}
void wxPageSetupDialogData_EnableMargins(wxPageSetupDialogData * self, bool flag) {
    return self->EnableMargins(flag);
}
void wxPageSetupDialogData_EnableOrientation(wxPageSetupDialogData * self, bool flag) {
    return self->EnableOrientation(flag);
}
void wxPageSetupDialogData_EnablePaper(wxPageSetupDialogData * self, bool flag) {
    return self->EnablePaper(flag);
}
void wxPageSetupDialogData_EnablePrinter(wxPageSetupDialogData * self, bool flag) {
    return self->EnablePrinter(flag);
}
bool wxPageSetupDialogData_GetDefaultInfo(const wxPageSetupDialogData * self) {
    return self->GetDefaultInfo();
}
bool wxPageSetupDialogData_GetDefaultMinMargins(const wxPageSetupDialogData * self) {
    return self->GetDefaultMinMargins();
}
bool wxPageSetupDialogData_GetEnableHelp(const wxPageSetupDialogData * self) {
    return self->GetEnableHelp();
}
bool wxPageSetupDialogData_GetEnableMargins(const wxPageSetupDialogData * self) {
    return self->GetEnableMargins();
}
bool wxPageSetupDialogData_GetEnableOrientation(const wxPageSetupDialogData * self) {
    return self->GetEnableOrientation();
}
bool wxPageSetupDialogData_GetEnablePaper(const wxPageSetupDialogData * self) {
    return self->GetEnablePaper();
}
bool wxPageSetupDialogData_GetEnablePrinter(const wxPageSetupDialogData * self) {
    return self->GetEnablePrinter();
}
wxPoint *wxPageSetupDialogData_GetMarginBottomRight(const wxPageSetupDialogData * self) {
    return new wxPoint(self->GetMarginBottomRight());
}
wxPoint *wxPageSetupDialogData_GetMarginTopLeft(const wxPageSetupDialogData * self) {
    return new wxPoint(self->GetMarginTopLeft());
}
wxPoint *wxPageSetupDialogData_GetMinMarginBottomRight(const wxPageSetupDialogData * self) {
    return new wxPoint(self->GetMinMarginBottomRight());
}
wxPoint *wxPageSetupDialogData_GetMinMarginTopLeft(const wxPageSetupDialogData * self) {
    return new wxPoint(self->GetMinMarginTopLeft());
}
wxSize *wxPageSetupDialogData_GetPaperSize(const wxPageSetupDialogData * self) {
    return new wxSize(self->GetPaperSize());
}
wxPrintData * wxPageSetupDialogData_GetPrintData(wxPageSetupDialogData * self) {
    return &(self->GetPrintData());
}
wxPrintData *wxPageSetupDialogData_GetPrintData1(const wxPageSetupDialogData * self) {
    return new wxPrintData(self->GetPrintData());
}
bool wxPageSetupDialogData_IsOk(const wxPageSetupDialogData * self) {
    return self->IsOk();
}
void wxPageSetupDialogData_SetDefaultInfo(wxPageSetupDialogData * self, bool flag) {
    return self->SetDefaultInfo(flag);
}
void wxPageSetupDialogData_SetDefaultMinMargins(wxPageSetupDialogData * self, bool flag) {
    return self->SetDefaultMinMargins(flag);
}
void wxPageSetupDialogData_SetMarginBottomRight(wxPageSetupDialogData * self, const wxPoint * pt) {
    return self->SetMarginBottomRight(*pt);
}
void wxPageSetupDialogData_SetMarginTopLeft(wxPageSetupDialogData * self, const wxPoint * pt) {
    return self->SetMarginTopLeft(*pt);
}
void wxPageSetupDialogData_SetMinMarginBottomRight(wxPageSetupDialogData * self, const wxPoint * pt) {
    return self->SetMinMarginBottomRight(*pt);
}
void wxPageSetupDialogData_SetMinMarginTopLeft(wxPageSetupDialogData * self, const wxPoint * pt) {
    return self->SetMinMarginTopLeft(*pt);
}
void wxPageSetupDialogData_SetPaperSize(wxPageSetupDialogData * self, const wxSize * size) {
    return self->SetPaperSize(*size);
}
void wxPageSetupDialogData_SetPrintData(wxPageSetupDialogData * self, const wxPrintData * print_data) {
    return self->SetPrintData(*print_data);
}

// CLASS: wxPaintDC
wxClassInfo *wxPaintDC_CLASSINFO() {
    return wxCLASSINFO(wxPaintDC);
}
wxPaintDC *wxPaintDC_new(wxWindow * window) {
    return new wxPaintDC(window);
}

// CLASS: wxPaintEvent
wxClassInfo *wxPaintEvent_CLASSINFO() {
    return wxCLASSINFO(wxPaintEvent);
}
wxPaintEvent *wxPaintEvent_new(wxWindow * window) {
    return new wxPaintEvent(window);
}

// CLASS: wxPalette
wxClassInfo *wxPalette_CLASSINFO() {
    return wxCLASSINFO(wxPalette);
}
wxPalette *wxPalette_new() {
    return new wxPalette();
}
wxPalette *wxPalette_new1(const wxPalette * palette) {
    return new wxPalette(*palette);
}
wxPalette *wxPalette_new2(int n, const unsigned char * red, const unsigned char * green, const unsigned char * blue) {
    return new wxPalette(n, red, green, blue);
}
bool wxPalette_Create(wxPalette * self, int n, const unsigned char * red, const unsigned char * green, const unsigned char * blue) {
    return self->Create(n, red, green, blue);
}
int wxPalette_GetColoursCount(const wxPalette * self) {
    return self->GetColoursCount();
}
bool wxPalette_GetRGB(const wxPalette * self, int pixel, unsigned char * red, unsigned char * green, unsigned char * blue) {
    return self->GetRGB(pixel, red, green, blue);
}
bool wxPalette_IsOk(const wxPalette * self) {
    return self->IsOk();
}

// CLASS: wxPanGestureEvent
wxClassInfo *wxPanGestureEvent_CLASSINFO() {
    return wxCLASSINFO(wxPanGestureEvent);
}
wxPanGestureEvent *wxPanGestureEvent_new(wxWindowID winid) {
    return new wxPanGestureEvent(winid);
}
wxPoint *wxPanGestureEvent_GetDelta(const wxPanGestureEvent * self) {
    return new wxPoint(self->GetDelta());
}
void wxPanGestureEvent_SetDelta(wxPanGestureEvent * self, const wxPoint * delta) {
    return self->SetDelta(*delta);
}

// CLASS: wxPanel
wxClassInfo *wxPanel_CLASSINFO() {
    return wxCLASSINFO(wxPanel);
}
wxPanel *wxPanel_new() {
    return new wxPanel();
}
wxPanel *wxPanel_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxPanel(parent, id, *pos, *size, style, *name);
}
bool wxPanel_Create(wxPanel * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
void wxPanel_OnSysColourChanged(wxPanel * self, wxSysColourChangedEvent * event) {
    return self->OnSysColourChanged(*event);
}
void wxPanel_SetFocusIgnoringChildren(wxPanel * self) {
    return self->SetFocusIgnoringChildren();
}
// Mix-in(s) to wxPanel
wxTrackable *wxPanel_AsTrackable(wxPanel* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxPasswordEntryDialog
wxClassInfo *wxPasswordEntryDialog_CLASSINFO() {
    return wxCLASSINFO(wxPasswordEntryDialog);
}
wxPasswordEntryDialog *wxPasswordEntryDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, const wxString * default_value, long style, const wxPoint * pos) {
    return new wxPasswordEntryDialog(parent, *message, *caption, *default_value, style, *pos);
}
// Mix-in(s) to wxPasswordEntryDialog
wxTrackable *wxPasswordEntryDialog_AsTrackable(wxPasswordEntryDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxPen
wxClassInfo *wxPen_CLASSINFO() {
    return wxCLASSINFO(wxPen);
}
wxPen *wxPen_new() {
    return new wxPen();
}
wxPen *wxPen_new1(const wxPenInfo * info) {
    return new wxPen(*info);
}
wxPen *wxPen_new3(const wxBitmap * stipple, int width) {
    return new wxPen(*stipple, width);
}
wxPen *wxPen_new4(const wxPen * pen) {
    return new wxPen(*pen);
}
wxColour *wxPen_GetColour(const wxPen * self) {
    return new wxColour(self->GetColour());
}
int wxPen_GetDashes(const wxPen * self, wxDash ** dashes) {
    return self->GetDashes(dashes);
}
wxBitmap * wxPen_GetStipple(const wxPen * self) {
    return self->GetStipple();
}
int wxPen_GetWidth(const wxPen * self) {
    return self->GetWidth();
}
bool wxPen_IsOk(const wxPen * self) {
    return self->IsOk();
}
bool wxPen_IsNonTransparent(const wxPen * self) {
    return self->IsNonTransparent();
}
bool wxPen_IsTransparent(const wxPen * self) {
    return self->IsTransparent();
}
void wxPen_SetColour(wxPen * self, wxColour * colour) {
    return self->SetColour(*colour);
}
void wxPen_SetDashes(wxPen * self, int n, const wxDash * dash) {
    return self->SetDashes(n, dash);
}
void wxPen_SetStipple(wxPen * self, const wxBitmap * stipple) {
    return self->SetStipple(*stipple);
}
void wxPen_SetWidth(wxPen * self, int width) {
    return self->SetWidth(width);
}

// CLASS: wxPenList
void wxPenList_delete(wxPenList *self) {
    delete self;
}
wxPenList *wxPenList_new() {
    return new wxPenList();
}

// CLASS: wxPersistenceManager
void wxPersistenceManager_delete(wxPersistenceManager *self) {
    delete self;
}
void wxPersistenceManager_Set(wxPersistenceManager * manager) {
    return wxPersistenceManager::Set(*manager);
}
wxPersistenceManager * wxPersistenceManager_Get() {
    return &(wxPersistenceManager::Get());
}
void wxPersistenceManager_DisableSaving(wxPersistenceManager * self) {
    return self->DisableSaving();
}
void wxPersistenceManager_DisableRestoring(wxPersistenceManager * self) {
    return self->DisableRestoring();
}
wxPersistentObject * wxPersistenceManager_Register(wxPersistenceManager * self, T * obj) {
    return self->Register(obj);
}
wxPersistentObject * wxPersistenceManager_Register1(wxPersistenceManager * self, void * obj, wxPersistentObject * po) {
    return self->Register(obj, po);
}
wxPersistentObject * wxPersistenceManager_Find(const wxPersistenceManager * self, void * obj) {
    return self->Find(obj);
}
void wxPersistenceManager_Unregister(wxPersistenceManager * self, void * obj) {
    return self->Unregister(obj);
}
void wxPersistenceManager_Save(wxPersistenceManager * self, void * obj) {
    return self->Save(obj);
}
bool wxPersistenceManager_Restore(wxPersistenceManager * self, void * obj) {
    return self->Restore(obj);
}
void wxPersistenceManager_SaveAndUnregister(wxPersistenceManager * self, void * obj) {
    return self->SaveAndUnregister(obj);
}
bool wxPersistenceManager_RegisterAndRestore(wxPersistenceManager * self, T * obj) {
    return self->RegisterAndRestore(obj);
}
bool wxPersistenceManager_RegisterAndRestore1(wxPersistenceManager * self, void * obj, wxPersistentObject * po) {
    return self->RegisterAndRestore(obj, po);
}

// CLASS: wxPickerBase
wxClassInfo *wxPickerBase_CLASSINFO() {
    return wxCLASSINFO(wxPickerBase);
}
bool wxPickerBase_CreateBase(wxPickerBase * self, wxWindow * parent, wxWindowID id, const wxString * text, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->CreateBase(parent, id, *text, *pos, *size, style, *validator, *name);
}
int wxPickerBase_GetInternalMargin(const wxPickerBase * self) {
    return self->GetInternalMargin();
}
int wxPickerBase_GetPickerCtrlProportion(const wxPickerBase * self) {
    return self->GetPickerCtrlProportion();
}
wxTextCtrl * wxPickerBase_GetTextCtrl(wxPickerBase * self) {
    return self->GetTextCtrl();
}
wxControl * wxPickerBase_GetPickerCtrl(wxPickerBase * self) {
    return self->GetPickerCtrl();
}
int wxPickerBase_GetTextCtrlProportion(const wxPickerBase * self) {
    return self->GetTextCtrlProportion();
}
bool wxPickerBase_HasTextCtrl(const wxPickerBase * self) {
    return self->HasTextCtrl();
}
bool wxPickerBase_IsPickerCtrlGrowable(const wxPickerBase * self) {
    return self->IsPickerCtrlGrowable();
}
bool wxPickerBase_IsTextCtrlGrowable(const wxPickerBase * self) {
    return self->IsTextCtrlGrowable();
}
void wxPickerBase_SetInternalMargin(wxPickerBase * self, int margin) {
    return self->SetInternalMargin(margin);
}
void wxPickerBase_SetPickerCtrlGrowable(wxPickerBase * self, bool grow) {
    return self->SetPickerCtrlGrowable(grow);
}
void wxPickerBase_SetPickerCtrlProportion(wxPickerBase * self, int prop) {
    return self->SetPickerCtrlProportion(prop);
}
void wxPickerBase_SetTextCtrlGrowable(wxPickerBase * self, bool grow) {
    return self->SetTextCtrlGrowable(grow);
}
void wxPickerBase_SetTextCtrlProportion(wxPickerBase * self, int prop) {
    return self->SetTextCtrlProportion(prop);
}
void wxPickerBase_SetTextCtrl(wxPickerBase * self, wxTextCtrl * text) {
    return self->SetTextCtrl(text);
}
void wxPickerBase_SetPickerCtrl(wxPickerBase * self, wxControl * picker) {
    return self->SetPickerCtrl(picker);
}
void wxPickerBase_UpdatePickerFromTextCtrl(wxPickerBase * self) {
    return self->UpdatePickerFromTextCtrl();
}
void wxPickerBase_UpdateTextCtrlFromPicker(wxPickerBase * self) {
    return self->UpdateTextCtrlFromPicker();
}
// Mix-in(s) to wxPickerBase
wxTrackable *wxPickerBase_AsTrackable(wxPickerBase* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self) {
    delete self;
}
bool wxPoint_IsFullySpecified(const wxPoint * self) {
    return self->IsFullySpecified();
}
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt) {
    return self->SetDefaults(*pt);
}
wxPoint *wxPoint_new() {
    return new wxPoint();
}
wxPoint *wxPoint_new1(int x, int y) {
    return new wxPoint(x, y);
}
wxPoint *wxPoint_new2(const wxRealPoint * pt) {
    return new wxPoint(*pt);
}

// CLASS: wxPopupTransientWindow
wxClassInfo *wxPopupTransientWindow_CLASSINFO() {
    return wxCLASSINFO(wxPopupTransientWindow);
}
wxPopupTransientWindow *wxPopupTransientWindow_new() {
    return new wxPopupTransientWindow();
}
wxPopupTransientWindow *wxPopupTransientWindow_new1(wxWindow * parent, int flags) {
    return new wxPopupTransientWindow(parent, flags);
}
void wxPopupTransientWindow_Popup(wxPopupTransientWindow * self, wxWindow * focus) {
    return self->Popup(focus);
}
void wxPopupTransientWindow_Dismiss(wxPopupTransientWindow * self) {
    return self->Dismiss();
}
bool wxPopupTransientWindow_ProcessLeftDown(wxPopupTransientWindow * self, wxMouseEvent * event) {
    return self->ProcessLeftDown(*event);
}
// Mix-in(s) to wxPopupTransientWindow
wxTrackable *wxPopupTransientWindow_AsTrackable(wxPopupTransientWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxPopupWindow
wxClassInfo *wxPopupWindow_CLASSINFO() {
    return wxCLASSINFO(wxPopupWindow);
}
wxPopupWindow *wxPopupWindow_new() {
    return new wxPopupWindow();
}
wxPopupWindow *wxPopupWindow_new1(wxWindow * parent, int flags) {
    return new wxPopupWindow(parent, flags);
}
bool wxPopupWindow_Create(wxPopupWindow * self, wxWindow * parent, int flags) {
    return self->Create(parent, flags);
}
void wxPopupWindow_Position(wxPopupWindow * self, const wxPoint * pt_origin, const wxSize * size_popup) {
    return self->Position(*pt_origin, *size_popup);
}
// Mix-in(s) to wxPopupWindow
wxTrackable *wxPopupWindow_AsTrackable(wxPopupWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxPreferencesEditor
void wxPreferencesEditor_delete(wxPreferencesEditor *self) {
    delete self;
}
wxPreferencesEditor *wxPreferencesEditor_new(const wxString * title) {
    return new wxPreferencesEditor(*title);
}
void wxPreferencesEditor_AddPage(wxPreferencesEditor * self, wxPreferencesPage * page) {
    return self->AddPage(page);
}
void wxPreferencesEditor_Show(wxPreferencesEditor * self, wxWindow * parent) {
    return self->Show(parent);
}
void wxPreferencesEditor_Dismiss(wxPreferencesEditor * self) {
    return self->Dismiss();
}
bool wxPreferencesEditor_ShouldApplyChangesImmediately() {
    return wxPreferencesEditor::ShouldApplyChangesImmediately();
}
bool wxPreferencesEditor_ShownModally() {
    return wxPreferencesEditor::ShownModally();
}

// CLASS: wxPreferencesPage
void wxPreferencesPage_delete(wxPreferencesPage *self) {
    delete self;
}
wxPreferencesPage *wxPreferencesPage_new() {
    return new wxPreferencesPage();
}
wxString *wxPreferencesPage_GetName(const wxPreferencesPage * self) {
    return new wxString(self->GetName());
}
wxBitmapBundle *wxPreferencesPage_GetIcon(const wxPreferencesPage * self) {
    return new wxBitmapBundle(self->GetIcon());
}
wxBitmap *wxPreferencesPage_GetLargeIcon(const wxPreferencesPage * self) {
    return new wxBitmap(self->GetLargeIcon());
}
wxWindow * wxPreferencesPage_CreateWindow(wxPreferencesPage * self, wxWindow * parent) {
    return self->CreateWindow(parent);
}

// CLASS: wxPressAndTapEvent
wxClassInfo *wxPressAndTapEvent_CLASSINFO() {
    return wxCLASSINFO(wxPressAndTapEvent);
}
wxPressAndTapEvent *wxPressAndTapEvent_new(wxWindowID windid) {
    return new wxPressAndTapEvent(windid);
}

// CLASS: wxPreviewControlBar
wxClassInfo *wxPreviewControlBar_CLASSINFO() {
    return wxCLASSINFO(wxPreviewControlBar);
}
wxPreviewControlBar *wxPreviewControlBar_new(wxPrintPreview * preview, long buttons, wxWindow * parent, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxPreviewControlBar(preview, buttons, parent, *pos, *size, style, *name);
}
void wxPreviewControlBar_CreateButtons(wxPreviewControlBar * self) {
    return self->CreateButtons();
}
wxPrintPreviewBase * wxPreviewControlBar_GetPrintPreview(const wxPreviewControlBar * self) {
    return self->GetPrintPreview();
}
int wxPreviewControlBar_GetZoomControl(wxPreviewControlBar * self) {
    return self->GetZoomControl();
}
void wxPreviewControlBar_SetZoomControl(wxPreviewControlBar * self, int percent) {
    return self->SetZoomControl(percent);
}
// Mix-in(s) to wxPreviewControlBar
wxTrackable *wxPreviewControlBar_AsTrackable(wxPreviewControlBar* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxPreviewFrame
wxClassInfo *wxPreviewFrame_CLASSINFO() {
    return wxCLASSINFO(wxPreviewFrame);
}
wxPreviewFrame *wxPreviewFrame_new(wxPrintPreviewBase * preview, wxWindow * parent, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxPreviewFrame(preview, parent, *title, *pos, *size, style, *name);
}
void wxPreviewFrame_CreateCanvas(wxPreviewFrame * self) {
    return self->CreateCanvas();
}
void wxPreviewFrame_CreateControlBar(wxPreviewFrame * self) {
    return self->CreateControlBar();
}
void wxPreviewFrame_Initialize(wxPreviewFrame * self) {
    return self->Initialize();
}
void wxPreviewFrame_OnCloseWindow(wxPreviewFrame * self, wxCloseEvent * event) {
    return self->OnCloseWindow(*event);
}
// Mix-in(s) to wxPreviewFrame
wxTrackable *wxPreviewFrame_AsTrackable(wxPreviewFrame* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxPrintData
wxClassInfo *wxPrintData_CLASSINFO() {
    return wxCLASSINFO(wxPrintData);
}
wxPrintData *wxPrintData_new() {
    return new wxPrintData();
}
wxPrintData *wxPrintData_new1(const wxPrintData * data) {
    return new wxPrintData(*data);
}
bool wxPrintData_GetCollate(const wxPrintData * self) {
    return self->GetCollate();
}
bool wxPrintData_GetColour(const wxPrintData * self) {
    return self->GetColour();
}
int wxPrintData_GetNoCopies(const wxPrintData * self) {
    return self->GetNoCopies();
}
wxString *wxPrintData_GetPrinterName(const wxPrintData * self) {
    return new wxString(self->GetPrinterName());
}
bool wxPrintData_IsOk(const wxPrintData * self) {
    return self->IsOk();
}
void wxPrintData_SetCollate(wxPrintData * self, bool flag) {
    return self->SetCollate(flag);
}
void wxPrintData_SetColour(wxPrintData * self, bool flag) {
    return self->SetColour(flag);
}
void wxPrintData_SetNoCopies(wxPrintData * self, int n) {
    return self->SetNoCopies(n);
}
void wxPrintData_SetPaperSize(wxPrintData * self, const wxSize * size) {
    return self->SetPaperSize(*size);
}
void wxPrintData_SetPrinterName(wxPrintData * self, const wxString * printer_name) {
    return self->SetPrinterName(*printer_name);
}
wxString *wxPrintData_GetFilename(const wxPrintData * self) {
    return new wxString(self->GetFilename());
}
void wxPrintData_SetFilename(wxPrintData * self, const wxString * filename) {
    return self->SetFilename(*filename);
}

// CLASS: wxPrintDialog
wxClassInfo *wxPrintDialog_CLASSINFO() {
    return wxCLASSINFO(wxPrintDialog);
}
wxPrintDialog *wxPrintDialog_new(wxWindow * parent, wxPrintDialogData * data) {
    return new wxPrintDialog(parent, data);
}
wxPrintDialog *wxPrintDialog_new1(wxWindow * parent, wxPrintData * data) {
    return new wxPrintDialog(parent, data);
}
wxDC * wxPrintDialog_GetPrintDC(wxPrintDialog * self) {
    return self->GetPrintDC();
}
wxPrintDialogData * wxPrintDialog_GetPrintDialogData(wxPrintDialog * self) {
    return &(self->GetPrintDialogData());
}
wxPrintData * wxPrintDialog_GetPrintData(wxPrintDialog * self) {
    return &(self->GetPrintData());
}
int wxPrintDialog_ShowModal(wxPrintDialog * self) {
    return self->ShowModal();
}

// CLASS: wxPrintDialogData
wxClassInfo *wxPrintDialogData_CLASSINFO() {
    return wxCLASSINFO(wxPrintDialogData);
}
wxPrintDialogData *wxPrintDialogData_new() {
    return new wxPrintDialogData();
}
wxPrintDialogData *wxPrintDialogData_new1(const wxPrintDialogData * dialog_data) {
    return new wxPrintDialogData(*dialog_data);
}
wxPrintDialogData *wxPrintDialogData_new2(const wxPrintData * print_data) {
    return new wxPrintDialogData(*print_data);
}
void wxPrintDialogData_EnableHelp(wxPrintDialogData * self, bool flag) {
    return self->EnableHelp(flag);
}
void wxPrintDialogData_EnablePageNumbers(wxPrintDialogData * self, bool flag) {
    return self->EnablePageNumbers(flag);
}
void wxPrintDialogData_EnablePrintToFile(wxPrintDialogData * self, bool flag) {
    return self->EnablePrintToFile(flag);
}
void wxPrintDialogData_EnableSelection(wxPrintDialogData * self, bool flag) {
    return self->EnableSelection(flag);
}
bool wxPrintDialogData_GetAllPages(const wxPrintDialogData * self) {
    return self->GetAllPages();
}
bool wxPrintDialogData_GetCollate(const wxPrintDialogData * self) {
    return self->GetCollate();
}
int wxPrintDialogData_GetFromPage(const wxPrintDialogData * self) {
    return self->GetFromPage();
}
int wxPrintDialogData_GetMaxPage(const wxPrintDialogData * self) {
    return self->GetMaxPage();
}
int wxPrintDialogData_GetMinPage(const wxPrintDialogData * self) {
    return self->GetMinPage();
}
int wxPrintDialogData_GetNoCopies(const wxPrintDialogData * self) {
    return self->GetNoCopies();
}
wxPrintData * wxPrintDialogData_GetPrintData(wxPrintDialogData * self) {
    return &(self->GetPrintData());
}
bool wxPrintDialogData_GetPrintToFile(const wxPrintDialogData * self) {
    return self->GetPrintToFile();
}
bool wxPrintDialogData_GetSelection(const wxPrintDialogData * self) {
    return self->GetSelection();
}
int wxPrintDialogData_GetToPage(const wxPrintDialogData * self) {
    return self->GetToPage();
}
bool wxPrintDialogData_IsOk(const wxPrintDialogData * self) {
    return self->IsOk();
}
void wxPrintDialogData_SetCollate(wxPrintDialogData * self, bool flag) {
    return self->SetCollate(flag);
}
void wxPrintDialogData_SetFromPage(wxPrintDialogData * self, int page) {
    return self->SetFromPage(page);
}
void wxPrintDialogData_SetMaxPage(wxPrintDialogData * self, int page) {
    return self->SetMaxPage(page);
}
void wxPrintDialogData_SetMinPage(wxPrintDialogData * self, int page) {
    return self->SetMinPage(page);
}
void wxPrintDialogData_SetNoCopies(wxPrintDialogData * self, int n) {
    return self->SetNoCopies(n);
}
void wxPrintDialogData_SetPrintData(wxPrintDialogData * self, const wxPrintData * print_data) {
    return self->SetPrintData(*print_data);
}
void wxPrintDialogData_SetPrintToFile(wxPrintDialogData * self, bool flag) {
    return self->SetPrintToFile(flag);
}
void wxPrintDialogData_SetSelection(wxPrintDialogData * self, bool flag) {
    return self->SetSelection(flag);
}
void wxPrintDialogData_SetSetupDialog(wxPrintDialogData * self, bool flag) {
    return self->SetSetupDialog(flag);
}
void wxPrintDialogData_SetToPage(wxPrintDialogData * self, int page) {
    return self->SetToPage(page);
}

// CLASS: wxPrintPreview
wxClassInfo *wxPrintPreview_CLASSINFO() {
    return wxCLASSINFO(wxPrintPreview);
}
wxPrintPreview *wxPrintPreview_new(wxPrintout * printout, wxPrintout * printout_for_printing, wxPrintDialogData * data) {
    return new wxPrintPreview(printout, printout_for_printing, data);
}
wxPrintPreview *wxPrintPreview_new1(wxPrintout * printout, wxPrintout * printout_for_printing, wxPrintData * data) {
    return new wxPrintPreview(printout, printout_for_printing, data);
}
wxPreviewCanvas * wxPrintPreview_GetCanvas(const wxPrintPreview * self) {
    return self->GetCanvas();
}
int wxPrintPreview_GetCurrentPage(const wxPrintPreview * self) {
    return self->GetCurrentPage();
}
wxFrame * wxPrintPreview_GetFrame(const wxPrintPreview * self) {
    return self->GetFrame();
}
int wxPrintPreview_GetMaxPage(const wxPrintPreview * self) {
    return self->GetMaxPage();
}
int wxPrintPreview_GetMinPage(const wxPrintPreview * self) {
    return self->GetMinPage();
}
wxPrintout * wxPrintPreview_GetPrintout(const wxPrintPreview * self) {
    return self->GetPrintout();
}
wxPrintout * wxPrintPreview_GetPrintoutForPrinting(const wxPrintPreview * self) {
    return self->GetPrintoutForPrinting();
}
bool wxPrintPreview_IsOk(const wxPrintPreview * self) {
    return self->IsOk();
}
bool wxPrintPreview_PaintPage(wxPrintPreview * self, wxPreviewCanvas * canvas, wxDC * dc) {
    return self->PaintPage(canvas, *dc);
}
bool wxPrintPreview_Print(wxPrintPreview * self, bool prompt) {
    return self->Print(prompt);
}
bool wxPrintPreview_RenderPage(wxPrintPreview * self, int page_num) {
    return self->RenderPage(page_num);
}
void wxPrintPreview_SetCanvas(wxPrintPreview * self, wxPreviewCanvas * window) {
    return self->SetCanvas(window);
}
bool wxPrintPreview_SetCurrentPage(wxPrintPreview * self, int page_num) {
    return self->SetCurrentPage(page_num);
}
void wxPrintPreview_SetFrame(wxPrintPreview * self, wxFrame * frame) {
    return self->SetFrame(frame);
}
void wxPrintPreview_SetPrintout(wxPrintPreview * self, wxPrintout * printout) {
    return self->SetPrintout(printout);
}
void wxPrintPreview_SetZoom(wxPrintPreview * self, int percent) {
    return self->SetZoom(percent);
}

// CLASS: wxPrinter
wxClassInfo *wxPrinter_CLASSINFO() {
    return wxCLASSINFO(wxPrinter);
}
wxPrinter *wxPrinter_new(wxPrintDialogData * data) {
    return new wxPrinter(data);
}
wxPrintAbortDialog * wxPrinter_CreateAbortWindow(wxPrinter * self, wxWindow * parent, wxPrintout * printout) {
    return self->CreateAbortWindow(parent, printout);
}
bool wxPrinter_GetAbort(const wxPrinter * self) {
    return self->GetAbort();
}
wxPrintDialogData * wxPrinter_GetPrintDialogData(const wxPrinter * self) {
    return &(self->GetPrintDialogData());
}
bool wxPrinter_Print(wxPrinter * self, wxWindow * parent, wxPrintout * printout, bool prompt) {
    return self->Print(parent, printout, prompt);
}
wxDC * wxPrinter_PrintDialog(wxPrinter * self, wxWindow * parent) {
    return self->PrintDialog(parent);
}
void wxPrinter_ReportError(wxPrinter * self, wxWindow * parent, wxPrintout * printout, const wxString * message) {
    return self->ReportError(parent, printout, *message);
}
bool wxPrinter_Setup(wxPrinter * self, wxWindow * parent) {
    return self->Setup(parent);
}

// CLASS: wxPrinterDC
wxClassInfo *wxPrinterDC_CLASSINFO() {
    return wxCLASSINFO(wxPrinterDC);
}
wxPrinterDC *wxPrinterDC_new(const wxPrintData * print_data) {
    return new wxPrinterDC(*print_data);
}
wxRect *wxPrinterDC_GetPaperRect(const wxPrinterDC * self) {
    return new wxRect(self->GetPaperRect());
}

// CLASS: wxPrintout
wxClassInfo *wxPrintout_CLASSINFO() {
    return wxCLASSINFO(wxPrintout);
}
wxPrintout *wxPrintout_new(const wxString * title) {
    return new wxPrintout(*title);
}
void wxPrintout_FitThisSizeToPage(wxPrintout * self, const wxSize * image_size) {
    return self->FitThisSizeToPage(*image_size);
}
void wxPrintout_FitThisSizeToPageMargins(wxPrintout * self, const wxSize * image_size, const wxPageSetupDialogData * page_setup_data) {
    return self->FitThisSizeToPageMargins(*image_size, *page_setup_data);
}
void wxPrintout_FitThisSizeToPaper(wxPrintout * self, const wxSize * image_size) {
    return self->FitThisSizeToPaper(*image_size);
}
wxDC * wxPrintout_GetDC(const wxPrintout * self) {
    return self->GetDC();
}
wxRect *wxPrintout_GetLogicalPageMarginsRect(const wxPrintout * self, const wxPageSetupDialogData * page_setup_data) {
    return new wxRect(self->GetLogicalPageMarginsRect(*page_setup_data));
}
wxRect *wxPrintout_GetLogicalPageRect(const wxPrintout * self) {
    return new wxRect(self->GetLogicalPageRect());
}
wxRect *wxPrintout_GetLogicalPaperRect(const wxPrintout * self) {
    return new wxRect(self->GetLogicalPaperRect());
}
void wxPrintout_GetPPIPrinter(const wxPrintout * self, int * w, int * h) {
    return self->GetPPIPrinter(w, h);
}
void wxPrintout_GetPPIScreen(const wxPrintout * self, int * w, int * h) {
    return self->GetPPIScreen(w, h);
}
void wxPrintout_GetPageInfo(wxPrintout * self, int * min_page, int * max_page, int * page_from, int * page_to) {
    return self->GetPageInfo(min_page, max_page, page_from, page_to);
}
void wxPrintout_GetPageSizeMM(const wxPrintout * self, int * w, int * h) {
    return self->GetPageSizeMM(w, h);
}
void wxPrintout_GetPageSizePixels(const wxPrintout * self, int * w, int * h) {
    return self->GetPageSizePixels(w, h);
}
wxRect *wxPrintout_GetPaperRectPixels(const wxPrintout * self) {
    return new wxRect(self->GetPaperRectPixels());
}
wxString *wxPrintout_GetTitle(const wxPrintout * self) {
    return new wxString(self->GetTitle());
}
bool wxPrintout_HasPage(wxPrintout * self, int page_num) {
    return self->HasPage(page_num);
}
bool wxPrintout_IsPreview(const wxPrintout * self) {
    return self->IsPreview();
}
wxPrintPreview * wxPrintout_GetPreview(const wxPrintout * self) {
    return self->GetPreview();
}
void wxPrintout_MapScreenSizeToDevice(wxPrintout * self) {
    return self->MapScreenSizeToDevice();
}
void wxPrintout_MapScreenSizeToPage(wxPrintout * self) {
    return self->MapScreenSizeToPage();
}
void wxPrintout_MapScreenSizeToPageMargins(wxPrintout * self, const wxPageSetupDialogData * page_setup_data) {
    return self->MapScreenSizeToPageMargins(*page_setup_data);
}
void wxPrintout_MapScreenSizeToPaper(wxPrintout * self) {
    return self->MapScreenSizeToPaper();
}
void wxPrintout_OffsetLogicalOrigin(wxPrintout * self, wxCoord xoff, wxCoord yoff) {
    return self->OffsetLogicalOrigin(xoff, yoff);
}
bool wxPrintout_OnBeginDocument(wxPrintout * self, int start_page, int end_page) {
    return self->OnBeginDocument(start_page, end_page);
}
void wxPrintout_OnBeginPrinting(wxPrintout * self) {
    return self->OnBeginPrinting();
}
void wxPrintout_OnEndDocument(wxPrintout * self) {
    return self->OnEndDocument();
}
void wxPrintout_OnEndPrinting(wxPrintout * self) {
    return self->OnEndPrinting();
}
void wxPrintout_OnPreparePrinting(wxPrintout * self) {
    return self->OnPreparePrinting();
}
bool wxPrintout_OnPrintPage(wxPrintout * self, int page_num) {
    return self->OnPrintPage(page_num);
}
void wxPrintout_SetLogicalOrigin(wxPrintout * self, wxCoord x, wxCoord y) {
    return self->SetLogicalOrigin(x, y);
}

// CLASS: wxPropertySheetDialog
wxClassInfo *wxPropertySheetDialog_CLASSINFO() {
    return wxCLASSINFO(wxPropertySheetDialog);
}
wxPropertySheetDialog *wxPropertySheetDialog_new() {
    return new wxPropertySheetDialog();
}
wxPropertySheetDialog *wxPropertySheetDialog_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxPropertySheetDialog(parent, id, *title, *pos, *size, style, *name);
}
void wxPropertySheetDialog_AddBookCtrl(wxPropertySheetDialog * self, wxSizer * sizer) {
    return self->AddBookCtrl(sizer);
}
bool wxPropertySheetDialog_Create(wxPropertySheetDialog * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxBookCtrlBase * wxPropertySheetDialog_CreateBookCtrl(wxPropertySheetDialog * self) {
    return self->CreateBookCtrl();
}
void wxPropertySheetDialog_CreateButtons(wxPropertySheetDialog * self, int flags) {
    return self->CreateButtons(flags);
}
wxBookCtrlBase * wxPropertySheetDialog_GetBookCtrl(const wxPropertySheetDialog * self) {
    return self->GetBookCtrl();
}
wxSizer * wxPropertySheetDialog_GetInnerSizer(const wxPropertySheetDialog * self) {
    return self->GetInnerSizer();
}
void wxPropertySheetDialog_SetInnerSizer(wxPropertySheetDialog * self, wxSizer * sizer) {
    return self->SetInnerSizer(sizer);
}
long wxPropertySheetDialog_GetSheetStyle(const wxPropertySheetDialog * self) {
    return self->GetSheetStyle();
}
void wxPropertySheetDialog_LayoutDialog(wxPropertySheetDialog * self, int centre_flags) {
    return self->LayoutDialog(centre_flags);
}
void wxPropertySheetDialog_SetBookCtrl(wxPropertySheetDialog * self, wxBookCtrlBase * book_ctrl) {
    return self->SetBookCtrl(book_ctrl);
}
void wxPropertySheetDialog_SetSheetStyle(wxPropertySheetDialog * self, long style) {
    return self->SetSheetStyle(style);
}
void wxPropertySheetDialog_SetSheetOuterBorder(wxPropertySheetDialog * self, int border) {
    return self->SetSheetOuterBorder(border);
}
int wxPropertySheetDialog_GetSheetOuterBorder(const wxPropertySheetDialog * self) {
    return self->GetSheetOuterBorder();
}
void wxPropertySheetDialog_SetSheetInnerBorder(wxPropertySheetDialog * self, int border) {
    return self->SetSheetInnerBorder(border);
}
int wxPropertySheetDialog_GetSheetInnerBorder(const wxPropertySheetDialog * self) {
    return self->GetSheetInnerBorder();
}
// Mix-in(s) to wxPropertySheetDialog
wxTrackable *wxPropertySheetDialog_AsTrackable(wxPropertySheetDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxQuantize
wxClassInfo *wxQuantize_CLASSINFO() {
    return wxCLASSINFO(wxQuantize);
}
wxQuantize *wxQuantize_new() {
    return new wxQuantize();
}
void wxQuantize_DoQuantize(unsigned int w, unsigned int h, unsigned char ** in_rows, unsigned char ** out_rows, unsigned char * palette, int desired_no_colours) {
    return wxQuantize::DoQuantize(w, h, in_rows, out_rows, palette, desired_no_colours);
}
bool wxQuantize_Quantize(const wxImage * src, wxImage * dest, wxPalette ** p_palette, int desired_no_colours, unsigned char ** eight_bit_data, int flags) {
    return wxQuantize::Quantize(*src, *dest, p_palette, desired_no_colours, eight_bit_data, flags);
}
bool wxQuantize_Quantize1(const wxImage * src, wxImage * dest, int desired_no_colours, unsigned char ** eight_bit_data, int flags) {
    return wxQuantize::Quantize(*src, *dest, desired_no_colours, eight_bit_data, flags);
}

// CLASS: wxQueryLayoutInfoEvent
wxClassInfo *wxQueryLayoutInfoEvent_CLASSINFO() {
    return wxCLASSINFO(wxQueryLayoutInfoEvent);
}
wxQueryLayoutInfoEvent *wxQueryLayoutInfoEvent_new(wxWindowID id) {
    return new wxQueryLayoutInfoEvent(id);
}
int wxQueryLayoutInfoEvent_GetFlags(const wxQueryLayoutInfoEvent * self) {
    return self->GetFlags();
}
int wxQueryLayoutInfoEvent_GetRequestedLength(const wxQueryLayoutInfoEvent * self) {
    return self->GetRequestedLength();
}
wxSize *wxQueryLayoutInfoEvent_GetSize(const wxQueryLayoutInfoEvent * self) {
    return new wxSize(self->GetSize());
}
void wxQueryLayoutInfoEvent_SetFlags(wxQueryLayoutInfoEvent * self, int flags) {
    return self->SetFlags(flags);
}
void wxQueryLayoutInfoEvent_SetRequestedLength(wxQueryLayoutInfoEvent * self, int length) {
    return self->SetRequestedLength(length);
}
void wxQueryLayoutInfoEvent_SetSize(wxQueryLayoutInfoEvent * self, const wxSize * size) {
    return self->SetSize(*size);
}

// CLASS: wxRadioBox
wxClassInfo *wxRadioBox_CLASSINFO() {
    return wxCLASSINFO(wxRadioBox);
}
wxRadioBox *wxRadioBox_new() {
    return new wxRadioBox();
}
wxRadioBox *wxRadioBox_new2(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name) {
    return new wxRadioBox(parent, id, *label, *pos, *size, *choices, major_dimension, style, *validator, *name);
}
bool wxRadioBox_Create1(wxRadioBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, *choices, major_dimension, style, *validator, *name);
}
bool wxRadioBox_Enable(wxRadioBox * self, unsigned int n, bool enable) {
    return self->Enable(n, enable);
}
unsigned int wxRadioBox_GetColumnCount(const wxRadioBox * self) {
    return self->GetColumnCount();
}
int wxRadioBox_GetItemFromPoint(const wxRadioBox * self, const wxPoint * pt) {
    return self->GetItemFromPoint(*pt);
}
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxRadioBox_GetItemHelpText(const wxRadioBox * self, unsigned int item) {
    return new wxString(self->GetItemHelpText(item));
}
#endif
wxToolTip * wxRadioBox_GetItemToolTip(const wxRadioBox * self, unsigned int item) {
    return self->GetItemToolTip(item);
}
unsigned int wxRadioBox_GetRowCount(const wxRadioBox * self) {
    return self->GetRowCount();
}
bool wxRadioBox_IsItemEnabled(const wxRadioBox * self, unsigned int n) {
    return self->IsItemEnabled(n);
}
bool wxRadioBox_IsItemShown(const wxRadioBox * self, unsigned int n) {
    return self->IsItemShown(n);
}
void wxRadioBox_SetItemHelpText(wxRadioBox * self, unsigned int item, const wxString * helptext) {
    return self->SetItemHelpText(item, *helptext);
}
void wxRadioBox_SetItemToolTip(wxRadioBox * self, unsigned int item, const wxString * text) {
    return self->SetItemToolTip(item, *text);
}
bool wxRadioBox_Show(wxRadioBox * self, unsigned int item, bool show) {
    return self->Show(item, show);
}
// Mix-in(s) to wxRadioBox
wxItemContainerImmutable *wxRadioBox_AsItemContainerImmutable(wxRadioBox* obj) {
    return static_cast<wxItemContainerImmutable*>(obj);
}
wxTrackable *wxRadioBox_AsTrackable(wxRadioBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxRadioButton
wxClassInfo *wxRadioButton_CLASSINFO() {
    return wxCLASSINFO(wxRadioButton);
}
wxRadioButton *wxRadioButton_new() {
    return new wxRadioButton();
}
wxRadioButton *wxRadioButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxRadioButton(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxRadioButton_Create(wxRadioButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxRadioButton_GetValue(const wxRadioButton * self) {
    return self->GetValue();
}
void wxRadioButton_SetValue(wxRadioButton * self, bool value) {
    return self->SetValue(value);
}
wxRadioButton * wxRadioButton_GetFirstInGroup(const wxRadioButton * self) {
    return self->GetFirstInGroup();
}
wxRadioButton * wxRadioButton_GetLastInGroup(const wxRadioButton * self) {
    return self->GetLastInGroup();
}
wxRadioButton * wxRadioButton_GetPreviousInGroup(const wxRadioButton * self) {
    return self->GetPreviousInGroup();
}
wxRadioButton * wxRadioButton_GetNextInGroup(const wxRadioButton * self) {
    return self->GetNextInGroup();
}
// Mix-in(s) to wxRadioButton
wxTrackable *wxRadioButton_AsTrackable(wxRadioButton* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxRealPoint
void wxRealPoint_delete(wxRealPoint *self) {
    delete self;
}
wxRealPoint *wxRealPoint_new() {
    return new wxRealPoint();
}
wxRealPoint *wxRealPoint_new1(double x, double y) {
    return new wxRealPoint(x, y);
}
wxRealPoint *wxRealPoint_new2(const wxPoint * pt) {
    return new wxRealPoint(*pt);
}

// CLASS: wxRearrangeCtrl
wxClassInfo *wxRearrangeCtrl_CLASSINFO() {
    return wxCLASSINFO(wxRearrangeCtrl);
}
wxRearrangeCtrl *wxRearrangeCtrl_new() {
    return new wxRearrangeCtrl();
}
wxRearrangeCtrl *wxRearrangeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name) {
    return new wxRearrangeCtrl(parent, id, *pos, *size, *order, *items, style, *validator, *name);
}
bool wxRearrangeCtrl_Create(wxRearrangeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *order, *items, style, *validator, *name);
}
wxRearrangeList * wxRearrangeCtrl_GetList(const wxRearrangeCtrl * self) {
    return self->GetList();
}
// Mix-in(s) to wxRearrangeCtrl
wxTrackable *wxRearrangeCtrl_AsTrackable(wxRearrangeCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxRearrangeDialog
wxClassInfo *wxRearrangeDialog_CLASSINFO() {
    return wxCLASSINFO(wxRearrangeDialog);
}
wxRearrangeDialog *wxRearrangeDialog_new() {
    return new wxRearrangeDialog();
}
wxRearrangeDialog *wxRearrangeDialog_new1(wxWindow * parent, const wxString * message, const wxString * title, const wxArrayInt * order, const wxArrayString * items, const wxPoint * pos, const wxString * name) {
    return new wxRearrangeDialog(parent, *message, *title, *order, *items, *pos, *name);
}
bool wxRearrangeDialog_Create(wxRearrangeDialog * self, wxWindow * parent, const wxString * message, const wxString * title, const wxArrayInt * order, const wxArrayString * items, const wxPoint * pos, const wxString * name) {
    return self->Create(parent, *message, *title, *order, *items, *pos, *name);
}
void wxRearrangeDialog_AddExtraControls(wxRearrangeDialog * self, wxWindow * win) {
    return self->AddExtraControls(win);
}
wxRearrangeList * wxRearrangeDialog_GetList(const wxRearrangeDialog * self) {
    return self->GetList();
}
wxArrayInt *wxRearrangeDialog_GetOrder(const wxRearrangeDialog * self) {
    return new wxArrayInt(self->GetOrder());
}
// Mix-in(s) to wxRearrangeDialog
wxTrackable *wxRearrangeDialog_AsTrackable(wxRearrangeDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxRearrangeList
wxClassInfo *wxRearrangeList_CLASSINFO() {
    return wxCLASSINFO(wxRearrangeList);
}
wxRearrangeList *wxRearrangeList_new() {
    return new wxRearrangeList();
}
wxRearrangeList *wxRearrangeList_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name) {
    return new wxRearrangeList(parent, id, *pos, *size, *order, *items, style, *validator, *name);
}
bool wxRearrangeList_Create(wxRearrangeList * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *order, *items, style, *validator, *name);
}
wxArrayInt *wxRearrangeList_GetCurrentOrder(const wxRearrangeList * self) {
    return new wxArrayInt(self->GetCurrentOrder());
}
bool wxRearrangeList_CanMoveCurrentUp(const wxRearrangeList * self) {
    return self->CanMoveCurrentUp();
}
bool wxRearrangeList_CanMoveCurrentDown(const wxRearrangeList * self) {
    return self->CanMoveCurrentDown();
}
bool wxRearrangeList_MoveCurrentUp(wxRearrangeList * self) {
    return self->MoveCurrentUp();
}
bool wxRearrangeList_MoveCurrentDown(wxRearrangeList * self) {
    return self->MoveCurrentDown();
}
// Mix-in(s) to wxRearrangeList
wxItemContainer *wxRearrangeList_AsItemContainer(wxRearrangeList* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTrackable *wxRearrangeList_AsTrackable(wxRearrangeList* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxRect
void wxRect_delete(wxRect *self) {
    delete self;
}
wxRect *wxRect_new() {
    return new wxRect();
}
wxRect *wxRect_new1(int x, int y, int width, int height) {
    return new wxRect(x, y, width, height);
}
wxRect *wxRect_new2(const wxPoint * top_left, const wxPoint * bottom_right) {
    return new wxRect(*top_left, *bottom_right);
}
wxRect *wxRect_new3(const wxPoint * pos, const wxSize * size) {
    return new wxRect(*pos, *size);
}
wxRect *wxRect_new4(const wxSize * size) {
    return new wxRect(*size);
}
wxRect *wxRect_CentreIn(const wxRect * self, const wxRect * r, int dir) {
    return new wxRect(self->CentreIn(*r, dir));
}
wxRect *wxRect_CenterIn(const wxRect * self, const wxRect * r, int dir) {
    return new wxRect(self->CenterIn(*r, dir));
}
bool wxRect_Contains(const wxRect * self, int x, int y) {
    return self->Contains(x, y);
}
bool wxRect_Contains1(const wxRect * self, const wxPoint * pt) {
    return self->Contains(*pt);
}
bool wxRect_Contains2(const wxRect * self, const wxRect * rect) {
    return self->Contains(*rect);
}
wxRect *wxRect_Deflate3(const wxRect * self, wxCoord dx, wxCoord dy) {
    return new wxRect(self->Deflate(dx, dy));
}
int wxRect_GetBottom(const wxRect * self) {
    return self->GetBottom();
}
wxPoint *wxRect_GetBottomLeft(const wxRect * self) {
    return new wxPoint(self->GetBottomLeft());
}
wxPoint *wxRect_GetBottomRight(const wxRect * self) {
    return new wxPoint(self->GetBottomRight());
}
int wxRect_GetHeight(const wxRect * self) {
    return self->GetHeight();
}
int wxRect_GetLeft(const wxRect * self) {
    return self->GetLeft();
}
wxPoint *wxRect_GetPosition(const wxRect * self) {
    return new wxPoint(self->GetPosition());
}
int wxRect_GetRight(const wxRect * self) {
    return self->GetRight();
}
wxSize *wxRect_GetSize(const wxRect * self) {
    return new wxSize(self->GetSize());
}
int wxRect_GetTop(const wxRect * self) {
    return self->GetTop();
}
wxPoint *wxRect_GetTopLeft(const wxRect * self) {
    return new wxPoint(self->GetTopLeft());
}
wxPoint *wxRect_GetTopRight(const wxRect * self) {
    return new wxPoint(self->GetTopRight());
}
int wxRect_GetWidth(const wxRect * self) {
    return self->GetWidth();
}
int wxRect_GetX(const wxRect * self) {
    return self->GetX();
}
int wxRect_GetY(const wxRect * self) {
    return self->GetY();
}
wxRect *wxRect_Inflate3(const wxRect * self, wxCoord dx, wxCoord dy) {
    return new wxRect(self->Inflate(dx, dy));
}
wxRect *wxRect_Intersect1(const wxRect * self, const wxRect * rect) {
    return new wxRect(self->Intersect(*rect));
}
bool wxRect_Intersects(const wxRect * self, const wxRect * rect) {
    return self->Intersects(*rect);
}
bool wxRect_IsEmpty(const wxRect * self) {
    return self->IsEmpty();
}
void wxRect_Offset(wxRect * self, wxCoord dx, wxCoord dy) {
    return self->Offset(dx, dy);
}
void wxRect_Offset1(wxRect * self, const wxPoint * pt) {
    return self->Offset(*pt);
}
void wxRect_SetHeight(wxRect * self, int height) {
    return self->SetHeight(height);
}
void wxRect_SetPosition(wxRect * self, const wxPoint * pos) {
    return self->SetPosition(*pos);
}
void wxRect_SetSize(wxRect * self, const wxSize * s) {
    return self->SetSize(*s);
}
void wxRect_SetWidth(wxRect * self, int width) {
    return self->SetWidth(width);
}
void wxRect_SetX(wxRect * self, int x) {
    return self->SetX(x);
}
void wxRect_SetY(wxRect * self, int y) {
    return self->SetY(y);
}
void wxRect_SetLeft(wxRect * self, int left) {
    return self->SetLeft(left);
}
void wxRect_SetRight(wxRect * self, int right) {
    return self->SetRight(right);
}
void wxRect_SetTop(wxRect * self, int top) {
    return self->SetTop(top);
}
void wxRect_SetBottom(wxRect * self, int bottom) {
    return self->SetBottom(bottom);
}
void wxRect_SetTopLeft(wxRect * self, const wxPoint * p) {
    return self->SetTopLeft(*p);
}
void wxRect_SetBottomRight(wxRect * self, const wxPoint * p) {
    return self->SetBottomRight(*p);
}
void wxRect_SetTopRight(wxRect * self, const wxPoint * p) {
    return self->SetTopRight(*p);
}
void wxRect_SetBottomLeft(wxRect * self, const wxPoint * p) {
    return self->SetBottomLeft(*p);
}
wxRect *wxRect_Union(const wxRect * self, const wxRect * rect) {
    return new wxRect(self->Union(*rect));
}

// CLASS: wxRegion
wxClassInfo *wxRegion_CLASSINFO() {
    return wxCLASSINFO(wxRegion);
}
wxRegion *wxRegion_new() {
    return new wxRegion();
}
wxRegion *wxRegion_new1(wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return new wxRegion(x, y, width, height);
}
wxRegion *wxRegion_new2(const wxPoint * top_left, const wxPoint * bottom_right) {
    return new wxRegion(*top_left, *bottom_right);
}
wxRegion *wxRegion_new3(const wxRect * rect) {
    return new wxRegion(*rect);
}
wxRegion *wxRegion_new4(const wxRegion * region) {
    return new wxRegion(*region);
}
wxRegion *wxRegion_new6(const wxBitmap * bmp) {
    return new wxRegion(*bmp);
}
wxRegion *wxRegion_new7(const wxBitmap * bmp, const wxColour * trans_colour, int tolerance) {
    return new wxRegion(*bmp, *trans_colour, tolerance);
}
void wxRegion_Clear(wxRegion * self) {
    return self->Clear();
}
wxBitmap *wxRegion_ConvertToBitmap(const wxRegion * self) {
    return new wxBitmap(self->ConvertToBitmap());
}
void wxRegion_GetBox(const wxRegion * self, wxCoord * x, wxCoord * y, wxCoord * width, wxCoord * height) {
    return self->GetBox(*x, *y, *width, *height);
}
wxRect *wxRegion_GetBox1(const wxRegion * self) {
    return new wxRect(self->GetBox());
}
bool wxRegion_Intersect(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->Intersect(x, y, width, height);
}
bool wxRegion_Intersect1(wxRegion * self, const wxRect * rect) {
    return self->Intersect(*rect);
}
bool wxRegion_Intersect2(wxRegion * self, const wxRegion * region) {
    return self->Intersect(*region);
}
bool wxRegion_IsEmpty(const wxRegion * self) {
    return self->IsEmpty();
}
bool wxRegion_IsEqual(const wxRegion * self, const wxRegion * region) {
    return self->IsEqual(*region);
}
bool wxRegion_Offset(wxRegion * self, wxCoord x, wxCoord y) {
    return self->Offset(x, y);
}
bool wxRegion_Offset1(wxRegion * self, const wxPoint * pt) {
    return self->Offset(*pt);
}
bool wxRegion_Subtract(wxRegion * self, const wxRect * rect) {
    return self->Subtract(*rect);
}
bool wxRegion_Subtract1(wxRegion * self, const wxRegion * region) {
    return self->Subtract(*region);
}
bool wxRegion_Union(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->Union(x, y, width, height);
}
bool wxRegion_Union1(wxRegion * self, const wxRect * rect) {
    return self->Union(*rect);
}
bool wxRegion_Union2(wxRegion * self, const wxRegion * region) {
    return self->Union(*region);
}
bool wxRegion_Union3(wxRegion * self, const wxBitmap * bmp) {
    return self->Union(*bmp);
}
bool wxRegion_Union4(wxRegion * self, const wxBitmap * bmp, const wxColour * trans_colour, int tolerance) {
    return self->Union(*bmp, *trans_colour, tolerance);
}
bool wxRegion_Xor(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->Xor(x, y, width, height);
}
bool wxRegion_Xor1(wxRegion * self, const wxRect * rect) {
    return self->Xor(*rect);
}
bool wxRegion_Xor2(wxRegion * self, const wxRegion * region) {
    return self->Xor(*region);
}

// CLASS: wxRegionIterator
wxClassInfo *wxRegionIterator_CLASSINFO() {
    return wxCLASSINFO(wxRegionIterator);
}
wxRegionIterator *wxRegionIterator_new() {
    return new wxRegionIterator();
}
wxRegionIterator *wxRegionIterator_new1(const wxRegion * region) {
    return new wxRegionIterator(*region);
}
wxCoord wxRegionIterator_GetH(const wxRegionIterator * self) {
    return self->GetH();
}
wxCoord wxRegionIterator_GetHeight(const wxRegionIterator * self) {
    return self->GetHeight();
}
wxRect *wxRegionIterator_GetRect(const wxRegionIterator * self) {
    return new wxRect(self->GetRect());
}
wxCoord wxRegionIterator_GetW(const wxRegionIterator * self) {
    return self->GetW();
}
wxCoord wxRegionIterator_GetWidth(const wxRegionIterator * self) {
    return self->GetWidth();
}
wxCoord wxRegionIterator_GetX(const wxRegionIterator * self) {
    return self->GetX();
}
wxCoord wxRegionIterator_GetY(const wxRegionIterator * self) {
    return self->GetY();
}
bool wxRegionIterator_HaveRects(const wxRegionIterator * self) {
    return self->HaveRects();
}
void wxRegionIterator_Reset(wxRegionIterator * self) {
    return self->Reset();
}
void wxRegionIterator_Reset1(wxRegionIterator * self, const wxRegion * region) {
    return self->Reset(*region);
}

// CLASS: wxRendererNative
void wxRendererNative_delete(wxRendererNative *self) {
    delete self;
}
void wxRendererNative_DrawCheckBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawCheckBox(win, *dc, *rect, flags);
}
void wxRendererNative_DrawComboBoxDropButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawComboBoxDropButton(win, *dc, *rect, flags);
}
void wxRendererNative_DrawDropArrow(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawDropArrow(win, *dc, *rect, flags);
}
void wxRendererNative_DrawFocusRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawFocusRect(win, *dc, *rect, flags);
}
void wxRendererNative_DrawGauge(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int value, int max, int flags) {
    return self->DrawGauge(win, *dc, *rect, value, max, flags);
}
void wxRendererNative_DrawItemSelectionRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawItemSelectionRect(win, *dc, *rect, flags);
}
void wxRendererNative_DrawItemText(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxString * text, const wxRect * rect, int align, int flags, wxEllipsizeMode ellipsize_mode) {
    return self->DrawItemText(win, *dc, *text, *rect, align, flags, ellipsize_mode);
}
void wxRendererNative_DrawPushButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawPushButton(win, *dc, *rect, flags);
}
void wxRendererNative_DrawCollapseButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawCollapseButton(win, *dc, *rect, flags);
}
wxSize *wxRendererNative_GetCollapseButtonSize(wxRendererNative * self, wxWindow * win, wxDC * dc) {
    return new wxSize(self->GetCollapseButtonSize(win, *dc));
}
void wxRendererNative_DrawSplitterBorder(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawSplitterBorder(win, *dc, *rect, flags);
}
void wxRendererNative_DrawTreeItemButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawTreeItemButton(win, *dc, *rect, flags);
}
void wxRendererNative_DrawChoice(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawChoice(win, *dc, *rect, flags);
}
void wxRendererNative_DrawComboBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawComboBox(win, *dc, *rect, flags);
}
void wxRendererNative_DrawTextCtrl(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawTextCtrl(win, *dc, *rect, flags);
}
void wxRendererNative_DrawRadioBitmap(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawRadioBitmap(win, *dc, *rect, flags);
}
void wxRendererNative_DrawCheckMark(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawCheckMark(win, *dc, *rect, flags);
}
wxSize *wxRendererNative_GetCheckBoxSize(wxRendererNative * self, wxWindow * win, int flags) {
    return new wxSize(self->GetCheckBoxSize(win, flags));
}
wxSize *wxRendererNative_GetCheckMarkSize(wxRendererNative * self, wxWindow * win) {
    return new wxSize(self->GetCheckMarkSize(win));
}
wxSize *wxRendererNative_GetExpanderSize(wxRendererNative * self, wxWindow * win) {
    return new wxSize(self->GetExpanderSize(win));
}
int wxRendererNative_GetHeaderButtonHeight(wxRendererNative * self, wxWindow * win) {
    return self->GetHeaderButtonHeight(win);
}
int wxRendererNative_GetHeaderButtonMargin(wxRendererNative * self, wxWindow * win) {
    return self->GetHeaderButtonMargin(win);
}
wxRendererNative * wxRendererNative_Get() {
    return &(wxRendererNative::Get());
}
wxRendererNative * wxRendererNative_GetDefault() {
    return &(wxRendererNative::GetDefault());
}
wxRendererNative * wxRendererNative_GetGeneric() {
    return &(wxRendererNative::GetGeneric());
}
wxRendererNative * wxRendererNative_Load(const wxString * name) {
    return wxRendererNative::Load(*name);
}
wxRendererNative * wxRendererNative_Set(wxRendererNative * renderer) {
    return wxRendererNative::Set(renderer);
}

// CLASS: wxRichToolTip
void wxRichToolTip_delete(wxRichToolTip *self) {
    delete self;
}
wxRichToolTip *wxRichToolTip_new(const wxString * title, const wxString * message) {
    return new wxRichToolTip(*title, *message);
}
void wxRichToolTip_SetBackgroundColour(wxRichToolTip * self, const wxColour * col, const wxColour * col_end) {
    return self->SetBackgroundColour(*col, *col_end);
}
void wxRichToolTip_SetIcon(wxRichToolTip * self, int icon) {
    return self->SetIcon(icon);
}
void wxRichToolTip_SetIcon1(wxRichToolTip * self, const wxBitmapBundle * icon) {
    return self->SetIcon(*icon);
}
void wxRichToolTip_SetTitleFont(wxRichToolTip * self, const wxFont * font) {
    return self->SetTitleFont(*font);
}
void wxRichToolTip_ShowFor(wxRichToolTip * self, wxWindow * win, const wxRect * rect) {
    return self->ShowFor(win, rect);
}

// CLASS: wxRotateGestureEvent
wxClassInfo *wxRotateGestureEvent_CLASSINFO() {
    return wxCLASSINFO(wxRotateGestureEvent);
}
wxRotateGestureEvent *wxRotateGestureEvent_new(wxWindowID windid) {
    return new wxRotateGestureEvent(windid);
}
double wxRotateGestureEvent_GetRotationAngle(const wxRotateGestureEvent * self) {
    return self->GetRotationAngle();
}
void wxRotateGestureEvent_SetRotationAngle(wxRotateGestureEvent * self, double rotation_angle) {
    return self->SetRotationAngle(rotation_angle);
}

// CLASS: wxSVGBitmapEmbedHandler
void wxSVGBitmapEmbedHandler_delete(wxSVGBitmapEmbedHandler *self) {
    delete self;
}

// CLASS: wxSVGBitmapFileHandler
void wxSVGBitmapFileHandler_delete(wxSVGBitmapFileHandler *self) {
    delete self;
}
wxSVGBitmapFileHandler *wxSVGBitmapFileHandler_new(const wxFileName * path) {
    return new wxSVGBitmapFileHandler(*path);
}

// CLASS: wxSVGBitmapHandler
void wxSVGBitmapHandler_delete(wxSVGBitmapHandler *self) {
    delete self;
}
bool wxSVGBitmapHandler_ProcessBitmap(const wxSVGBitmapHandler * self, const wxBitmap * bitmap, wxCoord x, wxCoord y, wxOutputStream * stream) {
    return self->ProcessBitmap(*bitmap, x, y, *stream);
}

// CLASS: wxSVGFileDC
wxClassInfo *wxSVGFileDC_CLASSINFO() {
    return wxCLASSINFO(wxSVGFileDC);
}
wxSVGFileDC *wxSVGFileDC_new(const wxString * filename, int width, int height, double dpi, const wxString * title) {
    return new wxSVGFileDC(*filename, width, height, dpi, *title);
}
void wxSVGFileDC_Clear(wxSVGFileDC * self) {
    return self->Clear();
}
void wxSVGFileDC_SetBitmapHandler(wxSVGFileDC * self, wxSVGBitmapHandler * handler) {
    return self->SetBitmapHandler(handler);
}
void wxSVGFileDC_DestroyClippingRegion(wxSVGFileDC * self) {
    return self->DestroyClippingRegion();
}
void wxSVGFileDC_CrossHair(wxSVGFileDC * self, wxCoord x, wxCoord y) {
    return self->CrossHair(x, y);
}
bool wxSVGFileDC_GetPixel(const wxSVGFileDC * self, wxCoord x, wxCoord y, wxColour * colour) {
    return self->GetPixel(x, y, colour);
}
void wxSVGFileDC_SetPalette(wxSVGFileDC * self, const wxPalette * palette) {
    return self->SetPalette(*palette);
}
int wxSVGFileDC_GetDepth(const wxSVGFileDC * self) {
    return self->GetDepth();
}
bool wxSVGFileDC_StartDoc(wxSVGFileDC * self, const wxString * message) {
    return self->StartDoc(*message);
}
void wxSVGFileDC_EndDoc(wxSVGFileDC * self) {
    return self->EndDoc();
}
void wxSVGFileDC_StartPage(wxSVGFileDC * self) {
    return self->StartPage();
}
void wxSVGFileDC_EndPage(wxSVGFileDC * self) {
    return self->EndPage();
}

// CLASS: wxSashEvent
wxClassInfo *wxSashEvent_CLASSINFO() {
    return wxCLASSINFO(wxSashEvent);
}
wxRect *wxSashEvent_GetDragRect(const wxSashEvent * self) {
    return new wxRect(self->GetDragRect());
}
void wxSashEvent_SetDragRect(wxSashEvent * self, const wxRect * rect) {
    return self->SetDragRect(*rect);
}

// CLASS: wxSashLayoutWindow
wxClassInfo *wxSashLayoutWindow_CLASSINFO() {
    return wxCLASSINFO(wxSashLayoutWindow);
}
wxSashLayoutWindow *wxSashLayoutWindow_new() {
    return new wxSashLayoutWindow();
}
wxSashLayoutWindow *wxSashLayoutWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxSashLayoutWindow(parent, id, *pos, *size, style, *name);
}
bool wxSashLayoutWindow_Create(wxSashLayoutWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
void wxSashLayoutWindow_OnCalculateLayout(wxSashLayoutWindow * self, wxCalculateLayoutEvent * event) {
    return self->OnCalculateLayout(*event);
}
void wxSashLayoutWindow_OnQueryLayoutInfo(wxSashLayoutWindow * self, wxQueryLayoutInfoEvent * event) {
    return self->OnQueryLayoutInfo(*event);
}
void wxSashLayoutWindow_SetDefaultSize(wxSashLayoutWindow * self, const wxSize * size) {
    return self->SetDefaultSize(*size);
}
// Mix-in(s) to wxSashLayoutWindow
wxTrackable *wxSashLayoutWindow_AsTrackable(wxSashLayoutWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxSashWindow
wxClassInfo *wxSashWindow_CLASSINFO() {
    return wxCLASSINFO(wxSashWindow);
}
wxSashWindow *wxSashWindow_new() {
    return new wxSashWindow();
}
wxSashWindow *wxSashWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxSashWindow(parent, id, *pos, *size, style, *name);
}
int wxSashWindow_GetMaximumSizeX(const wxSashWindow * self) {
    return self->GetMaximumSizeX();
}
int wxSashWindow_GetMaximumSizeY(const wxSashWindow * self) {
    return self->GetMaximumSizeY();
}
int wxSashWindow_GetMinimumSizeX(const wxSashWindow * self) {
    return self->GetMinimumSizeX();
}
int wxSashWindow_GetMinimumSizeY(const wxSashWindow * self) {
    return self->GetMinimumSizeY();
}
void wxSashWindow_SetMaximumSizeX(wxSashWindow * self, int min) {
    return self->SetMaximumSizeX(min);
}
void wxSashWindow_SetMaximumSizeY(wxSashWindow * self, int min) {
    return self->SetMaximumSizeY(min);
}
void wxSashWindow_SetMinimumSizeX(wxSashWindow * self, int min) {
    return self->SetMinimumSizeX(min);
}
void wxSashWindow_SetMinimumSizeY(wxSashWindow * self, int min) {
    return self->SetMinimumSizeY(min);
}
void wxSashWindow_SetDefaultBorderSize(wxSashWindow * self, int width) {
    return self->SetDefaultBorderSize(width);
}
int wxSashWindow_GetDefaultBorderSize(const wxSashWindow * self) {
    return self->GetDefaultBorderSize();
}
void wxSashWindow_SetExtraBorderSize(wxSashWindow * self, int width) {
    return self->SetExtraBorderSize(width);
}
int wxSashWindow_GetExtraBorderSize(const wxSashWindow * self) {
    return self->GetExtraBorderSize();
}
void wxSashWindow_SizeWindows(wxSashWindow * self) {
    return self->SizeWindows();
}
// Mix-in(s) to wxSashWindow
wxTrackable *wxSashWindow_AsTrackable(wxSashWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxScreenDC
wxClassInfo *wxScreenDC_CLASSINFO() {
    return wxCLASSINFO(wxScreenDC);
}
wxScreenDC *wxScreenDC_new() {
    return new wxScreenDC();
}
bool wxScreenDC_EndDrawingOnTop() {
    return wxScreenDC::EndDrawingOnTop();
}
bool wxScreenDC_StartDrawingOnTop(wxWindow * window) {
    return wxScreenDC::StartDrawingOnTop(window);
}
bool wxScreenDC_StartDrawingOnTop1(wxRect * rect) {
    return wxScreenDC::StartDrawingOnTop(rect);
}

// CLASS: wxScrollBar
wxClassInfo *wxScrollBar_CLASSINFO() {
    return wxCLASSINFO(wxScrollBar);
}
wxScrollBar *wxScrollBar_new() {
    return new wxScrollBar();
}
wxScrollBar *wxScrollBar_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxScrollBar(parent, id, *pos, *size, style, *validator, *name);
}
bool wxScrollBar_Create(wxScrollBar * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
int wxScrollBar_GetPageSize(const wxScrollBar * self) {
    return self->GetPageSize();
}
int wxScrollBar_GetRange(const wxScrollBar * self) {
    return self->GetRange();
}
int wxScrollBar_GetThumbPosition(const wxScrollBar * self) {
    return self->GetThumbPosition();
}
int wxScrollBar_GetThumbSize(const wxScrollBar * self) {
    return self->GetThumbSize();
}
void wxScrollBar_SetThumbPosition(wxScrollBar * self, int view_start) {
    return self->SetThumbPosition(view_start);
}
bool wxScrollBar_IsVertical(const wxScrollBar * self) {
    return self->IsVertical();
}
// Mix-in(s) to wxScrollBar
wxTrackable *wxScrollBar_AsTrackable(wxScrollBar* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxScrollEvent
wxClassInfo *wxScrollEvent_CLASSINFO() {
    return wxCLASSINFO(wxScrollEvent);
}
int wxScrollEvent_GetOrientation(const wxScrollEvent * self) {
    return self->GetOrientation();
}
int wxScrollEvent_GetPosition(const wxScrollEvent * self) {
    return self->GetPosition();
}
void wxScrollEvent_SetOrientation(wxScrollEvent * self, int orient) {
    return self->SetOrientation(orient);
}
void wxScrollEvent_SetPosition(wxScrollEvent * self, int pos) {
    return self->SetPosition(pos);
}

// CLASS: wxScrollWinEvent
wxClassInfo *wxScrollWinEvent_CLASSINFO() {
    return wxCLASSINFO(wxScrollWinEvent);
}
int wxScrollWinEvent_GetOrientation(const wxScrollWinEvent * self) {
    return self->GetOrientation();
}
int wxScrollWinEvent_GetPosition(const wxScrollWinEvent * self) {
    return self->GetPosition();
}
void wxScrollWinEvent_SetOrientation(wxScrollWinEvent * self, int orient) {
    return self->SetOrientation(orient);
}
void wxScrollWinEvent_SetPosition(wxScrollWinEvent * self, int pos) {
    return self->SetPosition(pos);
}

// CLASS: wxSearchCtrl
wxClassInfo *wxSearchCtrl_CLASSINFO() {
    return wxCLASSINFO(wxSearchCtrl);
}
wxSearchCtrl *wxSearchCtrl_new() {
    return new wxSearchCtrl();
}
wxSearchCtrl *wxSearchCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxSearchCtrl(parent, id, *value, *pos, *size, style, *validator, *name);
}
bool wxSearchCtrl_Create(wxSearchCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, *validator, *name);
}
wxMenu * wxSearchCtrl_GetMenu(wxSearchCtrl * self) {
    return self->GetMenu();
}
bool wxSearchCtrl_IsSearchButtonVisible(const wxSearchCtrl * self) {
    return self->IsSearchButtonVisible();
}
bool wxSearchCtrl_IsCancelButtonVisible(const wxSearchCtrl * self) {
    return self->IsCancelButtonVisible();
}
void wxSearchCtrl_SetMenu(wxSearchCtrl * self, wxMenu * menu) {
    return self->SetMenu(menu);
}
void wxSearchCtrl_ShowCancelButton(wxSearchCtrl * self, bool show) {
    return self->ShowCancelButton(show);
}
void wxSearchCtrl_ShowSearchButton(wxSearchCtrl * self, bool show) {
    return self->ShowSearchButton(show);
}
void wxSearchCtrl_SetDescriptiveText(wxSearchCtrl * self, const wxString * text) {
    return self->SetDescriptiveText(*text);
}
wxString *wxSearchCtrl_GetDescriptiveText(const wxSearchCtrl * self) {
    return new wxString(self->GetDescriptiveText());
}
// Mix-in(s) to wxSearchCtrl
wxTextEntryBase *wxSearchCtrl_AsTextEntry(wxSearchCtrl* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}
wxTrackable *wxSearchCtrl_AsTrackable(wxSearchCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxSetCursorEvent
wxClassInfo *wxSetCursorEvent_CLASSINFO() {
    return wxCLASSINFO(wxSetCursorEvent);
}
wxSetCursorEvent *wxSetCursorEvent_new(wxCoord x, wxCoord y) {
    return new wxSetCursorEvent(x, y);
}
wxCursor *wxSetCursorEvent_GetCursor(const wxSetCursorEvent * self) {
    return new wxCursor(self->GetCursor());
}
wxCoord wxSetCursorEvent_GetX(const wxSetCursorEvent * self) {
    return self->GetX();
}
wxCoord wxSetCursorEvent_GetY(const wxSetCursorEvent * self) {
    return self->GetY();
}
bool wxSetCursorEvent_HasCursor(const wxSetCursorEvent * self) {
    return self->HasCursor();
}
void wxSetCursorEvent_SetCursor(wxSetCursorEvent * self, const wxCursor * cursor) {
    return self->SetCursor(*cursor);
}

// CLASS: wxSettableHeaderColumn
void wxSettableHeaderColumn_delete(wxSettableHeaderColumn *self) {
    delete self;
}
void wxSettableHeaderColumn_SetTitle(wxSettableHeaderColumn * self, const wxString * title) {
    return self->SetTitle(*title);
}
void wxSettableHeaderColumn_SetBitmap(wxSettableHeaderColumn * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmap(*bitmap);
}
void wxSettableHeaderColumn_SetWidth(wxSettableHeaderColumn * self, int width) {
    return self->SetWidth(width);
}
void wxSettableHeaderColumn_SetMinWidth(wxSettableHeaderColumn * self, int min_width) {
    return self->SetMinWidth(min_width);
}
void wxSettableHeaderColumn_SetAlignment(wxSettableHeaderColumn * self, wxAlignment align) {
    return self->SetAlignment(align);
}
void wxSettableHeaderColumn_SetFlags(wxSettableHeaderColumn * self, int flags) {
    return self->SetFlags(flags);
}
void wxSettableHeaderColumn_ChangeFlag(wxSettableHeaderColumn * self, int flag, bool set) {
    return self->ChangeFlag(flag, set);
}
void wxSettableHeaderColumn_SetFlag(wxSettableHeaderColumn * self, int flag) {
    return self->SetFlag(flag);
}
void wxSettableHeaderColumn_ClearFlag(wxSettableHeaderColumn * self, int flag) {
    return self->ClearFlag(flag);
}
void wxSettableHeaderColumn_ToggleFlag(wxSettableHeaderColumn * self, int flag) {
    return self->ToggleFlag(flag);
}
void wxSettableHeaderColumn_SetResizeable(wxSettableHeaderColumn * self, bool resizable) {
    return self->SetResizeable(resizable);
}
void wxSettableHeaderColumn_SetSortable(wxSettableHeaderColumn * self, bool sortable) {
    return self->SetSortable(sortable);
}
void wxSettableHeaderColumn_SetReorderable(wxSettableHeaderColumn * self, bool reorderable) {
    return self->SetReorderable(reorderable);
}
void wxSettableHeaderColumn_SetHidden(wxSettableHeaderColumn * self, bool hidden) {
    return self->SetHidden(hidden);
}
void wxSettableHeaderColumn_UnsetAsSortKey(wxSettableHeaderColumn * self) {
    return self->UnsetAsSortKey();
}
void wxSettableHeaderColumn_SetSortOrder(wxSettableHeaderColumn * self, bool ascending) {
    return self->SetSortOrder(ascending);
}
void wxSettableHeaderColumn_ToggleSortOrder(wxSettableHeaderColumn * self) {
    return self->ToggleSortOrder();
}

// CLASS: wxShowEvent
wxClassInfo *wxShowEvent_CLASSINFO() {
    return wxCLASSINFO(wxShowEvent);
}
wxShowEvent *wxShowEvent_new(int winid, bool show) {
    return new wxShowEvent(winid, show);
}
void wxShowEvent_SetShow(wxShowEvent * self, bool show) {
    return self->SetShow(show);
}
bool wxShowEvent_IsShown(const wxShowEvent * self) {
    return self->IsShown();
}
bool wxShowEvent_GetShow(const wxShowEvent * self) {
    return self->GetShow();
}

// CLASS: wxSimpleHelpProvider
void wxSimpleHelpProvider_delete(wxSimpleHelpProvider *self) {
    delete self;
}

// CLASS: wxSimplebook
wxClassInfo *wxSimplebook_CLASSINFO() {
    return wxCLASSINFO(wxSimplebook);
}
wxSimplebook *wxSimplebook_new() {
    return new wxSimplebook();
}
wxSimplebook *wxSimplebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxSimplebook(parent, id, *pos, *size, style, *name);
}
bool wxSimplebook_Create(wxSimplebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
bool wxSimplebook_ShowNewPage(wxSimplebook * self, wxWindow * page) {
    return self->ShowNewPage(page);
}
// Mix-in(s) to wxSimplebook
wxWithImages *wxSimplebook_AsWithImages(wxSimplebook* obj) {
    return static_cast<wxWithImages*>(obj);
}
wxTrackable *wxSimplebook_AsTrackable(wxSimplebook* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxSize
void wxSize_delete(wxSize *self) {
    delete self;
}
wxSize *wxSize_new() {
    return new wxSize();
}
wxSize *wxSize_new1(int width, int height) {
    return new wxSize(width, height);
}
void wxSize_DecBy(wxSize * self, const wxPoint * pt) {
    return self->DecBy(*pt);
}
void wxSize_DecBy1(wxSize * self, const wxSize * size) {
    return self->DecBy(*size);
}
void wxSize_DecBy2(wxSize * self, int dx, int dy) {
    return self->DecBy(dx, dy);
}
void wxSize_DecBy3(wxSize * self, int d) {
    return self->DecBy(d);
}
void wxSize_DecTo(wxSize * self, const wxSize * size) {
    return self->DecTo(*size);
}
void wxSize_DecToIfSpecified(wxSize * self, const wxSize * size) {
    return self->DecToIfSpecified(*size);
}
int wxSize_GetHeight(const wxSize * self) {
    return self->GetHeight();
}
int wxSize_GetWidth(const wxSize * self) {
    return self->GetWidth();
}
void wxSize_IncBy(wxSize * self, const wxPoint * pt) {
    return self->IncBy(*pt);
}
void wxSize_IncBy1(wxSize * self, const wxSize * size) {
    return self->IncBy(*size);
}
void wxSize_IncBy2(wxSize * self, int dx, int dy) {
    return self->IncBy(dx, dy);
}
void wxSize_IncBy3(wxSize * self, int d) {
    return self->IncBy(d);
}
void wxSize_IncTo(wxSize * self, const wxSize * size) {
    return self->IncTo(*size);
}
bool wxSize_IsFullySpecified(const wxSize * self) {
    return self->IsFullySpecified();
}
void wxSize_Set(wxSize * self, int width, int height) {
    return self->Set(width, height);
}
void wxSize_SetDefaults(wxSize * self, const wxSize * size_default) {
    return self->SetDefaults(*size_default);
}
void wxSize_SetHeight(wxSize * self, int height) {
    return self->SetHeight(height);
}
void wxSize_SetWidth(wxSize * self, int width) {
    return self->SetWidth(width);
}

// CLASS: wxSizeEvent
wxClassInfo *wxSizeEvent_CLASSINFO() {
    return wxCLASSINFO(wxSizeEvent);
}
wxSizeEvent *wxSizeEvent_new(const wxSize * sz, int id) {
    return new wxSizeEvent(*sz, id);
}
wxSize *wxSizeEvent_GetSize(const wxSizeEvent * self) {
    return new wxSize(self->GetSize());
}
void wxSizeEvent_SetSize(wxSizeEvent * self, wxSize size) {
    return self->SetSize(size);
}
wxRect *wxSizeEvent_GetRect(const wxSizeEvent * self) {
    return new wxRect(self->GetRect());
}
void wxSizeEvent_SetRect(wxSizeEvent * self, wxRect rect) {
    return self->SetRect(rect);
}

// CLASS: wxSizer
wxClassInfo *wxSizer_CLASSINFO() {
    return wxCLASSINFO(wxSizer);
}
wxSizerItem * wxSizer_Add(wxSizer * self, wxWindow * window, const wxSizerFlags * flags) {
    return self->Add(window, *flags);
}
wxSizerItem * wxSizer_Add1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(window, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags) {
    return self->Add(sizer, *flags);
}
wxSizerItem * wxSizer_Add3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(sizer, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(width, height, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add5(wxSizer * self, int width, int height, const wxSizerFlags * flags) {
    return self->Add(width, height, *flags);
}
wxSizerItem * wxSizer_Add6(wxSizer * self, wxSizerItem * item) {
    return self->Add(item);
}
wxSizerItem * wxSizer_AddSpacer(wxSizer * self, int size) {
    return self->AddSpacer(size);
}
wxSizerItem * wxSizer_AddStretchSpacer(wxSizer * self, int prop) {
    return self->AddStretchSpacer(prop);
}
wxSize *wxSizer_CalcMin(wxSizer * self) {
    return new wxSize(self->CalcMin());
}
void wxSizer_Clear(wxSizer * self, bool delete_windows) {
    return self->Clear(delete_windows);
}
wxSize *wxSizer_ComputeFittingClientSize(wxSizer * self, wxWindow * window) {
    return new wxSize(self->ComputeFittingClientSize(window));
}
wxSize *wxSizer_ComputeFittingWindowSize(wxSizer * self, wxWindow * window) {
    return new wxSize(self->ComputeFittingWindowSize(window));
}
bool wxSizer_Detach(wxSizer * self, wxWindow * window) {
    return self->Detach(window);
}
bool wxSizer_Detach1(wxSizer * self, wxSizer * sizer) {
    return self->Detach(sizer);
}
bool wxSizer_Detach2(wxSizer * self, int index) {
    return self->Detach(index);
}
wxSize *wxSizer_Fit(wxSizer * self, wxWindow * window) {
    return new wxSize(self->Fit(window));
}
void wxSizer_FitInside(wxSizer * self, wxWindow * window) {
    return self->FitInside(window);
}
bool wxSizer_InformFirstDirection(wxSizer * self, int direction, int size, int available_other_dir) {
    return self->InformFirstDirection(direction, size, available_other_dir);
}
wxSizerItemList * wxSizer_GetChildren(wxSizer * self) {
    return &(self->GetChildren());
}
wxWindow * wxSizer_GetContainingWindow(const wxSizer * self) {
    return self->GetContainingWindow();
}
void wxSizer_SetContainingWindow(wxSizer * self, wxWindow * window) {
    return self->SetContainingWindow(window);
}
size_t wxSizer_GetItemCount(const wxSizer * self) {
    return self->GetItemCount();
}
wxSizerItem * wxSizer_GetItem(wxSizer * self, wxWindow * window, bool recursive) {
    return self->GetItem(window, recursive);
}
wxSizerItem * wxSizer_GetItem1(wxSizer * self, wxSizer * sizer, bool recursive) {
    return self->GetItem(sizer, recursive);
}
wxSizerItem * wxSizer_GetItem2(wxSizer * self, size_t index) {
    return self->GetItem(index);
}
wxSizerItem * wxSizer_GetItemById(wxSizer * self, int id, bool recursive) {
    return self->GetItemById(id, recursive);
}
wxSize *wxSizer_GetMinSize(wxSizer * self) {
    return new wxSize(self->GetMinSize());
}
wxPoint *wxSizer_GetPosition(const wxSizer * self) {
    return new wxPoint(self->GetPosition());
}
wxSize *wxSizer_GetSize(const wxSizer * self) {
    return new wxSize(self->GetSize());
}
bool wxSizer_Hide(wxSizer * self, wxWindow * window, bool recursive) {
    return self->Hide(window, recursive);
}
bool wxSizer_Hide1(wxSizer * self, wxSizer * sizer, bool recursive) {
    return self->Hide(sizer, recursive);
}
bool wxSizer_Hide2(wxSizer * self, size_t index) {
    return self->Hide(index);
}
wxSizerItem * wxSizer_Insert(wxSizer * self, size_t index, wxWindow * window, const wxSizerFlags * flags) {
    return self->Insert(index, window, *flags);
}
wxSizerItem * wxSizer_Insert1(wxSizer * self, size_t index, wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return self->Insert(index, window, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Insert2(wxSizer * self, size_t index, wxSizer * sizer, const wxSizerFlags * flags) {
    return self->Insert(index, sizer, *flags);
}
wxSizerItem * wxSizer_Insert3(wxSizer * self, size_t index, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return self->Insert(index, sizer, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Insert4(wxSizer * self, size_t index, int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return self->Insert(index, width, height, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Insert5(wxSizer * self, size_t index, int width, int height, const wxSizerFlags * flags) {
    return self->Insert(index, width, height, *flags);
}
wxSizerItem * wxSizer_Insert6(wxSizer * self, size_t index, wxSizerItem * item) {
    return self->Insert(index, item);
}
wxSizerItem * wxSizer_InsertSpacer(wxSizer * self, size_t index, int size) {
    return self->InsertSpacer(index, size);
}
wxSizerItem * wxSizer_InsertStretchSpacer(wxSizer * self, size_t index, int prop) {
    return self->InsertStretchSpacer(index, prop);
}
bool wxSizer_IsEmpty(const wxSizer * self) {
    return self->IsEmpty();
}
bool wxSizer_IsShown(const wxSizer * self, wxWindow * window) {
    return self->IsShown(window);
}
bool wxSizer_IsShown1(const wxSizer * self, wxSizer * sizer) {
    return self->IsShown(sizer);
}
bool wxSizer_IsShown2(const wxSizer * self, size_t index) {
    return self->IsShown(index);
}
void wxSizer_Layout(wxSizer * self) {
    return self->Layout();
}
wxSizerItem * wxSizer_Prepend(wxSizer * self, wxWindow * window, const wxSizerFlags * flags) {
    return self->Prepend(window, *flags);
}
wxSizerItem * wxSizer_Prepend1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(window, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags) {
    return self->Prepend(sizer, *flags);
}
wxSizerItem * wxSizer_Prepend3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(sizer, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(width, height, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend5(wxSizer * self, int width, int height, const wxSizerFlags * flags) {
    return self->Prepend(width, height, *flags);
}
wxSizerItem * wxSizer_Prepend6(wxSizer * self, wxSizerItem * item) {
    return self->Prepend(item);
}
wxSizerItem * wxSizer_PrependSpacer(wxSizer * self, int size) {
    return self->PrependSpacer(size);
}
wxSizerItem * wxSizer_PrependStretchSpacer(wxSizer * self, int prop) {
    return self->PrependStretchSpacer(prop);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxSizer_RepositionChildren(wxSizer * self, const wxSize * min_size) {
    return self->RepositionChildren(*min_size);
}
#endif
bool wxSizer_Remove1(wxSizer * self, wxSizer * sizer) {
    return self->Remove(sizer);
}
bool wxSizer_Remove2(wxSizer * self, int index) {
    return self->Remove(index);
}
bool wxSizer_Replace(wxSizer * self, wxWindow * oldwin, wxWindow * newwin, bool recursive) {
    return self->Replace(oldwin, newwin, recursive);
}
bool wxSizer_Replace1(wxSizer * self, wxSizer * oldsz, wxSizer * newsz, bool recursive) {
    return self->Replace(oldsz, newsz, recursive);
}
bool wxSizer_Replace2(wxSizer * self, size_t index, wxSizerItem * newitem) {
    return self->Replace(index, newitem);
}
void wxSizer_SetDimension(wxSizer * self, int x, int y, int width, int height) {
    return self->SetDimension(x, y, width, height);
}
void wxSizer_SetDimension1(wxSizer * self, const wxPoint * pos, const wxSize * size) {
    return self->SetDimension(*pos, *size);
}
bool wxSizer_SetItemMinSize(wxSizer * self, wxWindow * window, int width, int height) {
    return self->SetItemMinSize(window, width, height);
}
bool wxSizer_SetItemMinSize1(wxSizer * self, wxWindow * window, const wxSize * size) {
    return self->SetItemMinSize(window, *size);
}
bool wxSizer_SetItemMinSize2(wxSizer * self, wxSizer * sizer, int width, int height) {
    return self->SetItemMinSize(sizer, width, height);
}
bool wxSizer_SetItemMinSize3(wxSizer * self, wxSizer * sizer, const wxSize * size) {
    return self->SetItemMinSize(sizer, *size);
}
bool wxSizer_SetItemMinSize4(wxSizer * self, size_t index, int width, int height) {
    return self->SetItemMinSize(index, width, height);
}
bool wxSizer_SetItemMinSize5(wxSizer * self, size_t index, const wxSize * size) {
    return self->SetItemMinSize(index, *size);
}
void wxSizer_SetMinSize(wxSizer * self, const wxSize * size) {
    return self->SetMinSize(*size);
}
void wxSizer_SetMinSize1(wxSizer * self, int width, int height) {
    return self->SetMinSize(width, height);
}
void wxSizer_SetSizeHints(wxSizer * self, wxWindow * window) {
    return self->SetSizeHints(window);
}
bool wxSizer_Show(wxSizer * self, wxWindow * window, bool show, bool recursive) {
    return self->Show(window, show, recursive);
}
bool wxSizer_Show1(wxSizer * self, wxSizer * sizer, bool show, bool recursive) {
    return self->Show(sizer, show, recursive);
}
bool wxSizer_Show2(wxSizer * self, size_t index, bool show) {
    return self->Show(index, show);
}
void wxSizer_ShowItems(wxSizer * self, bool show) {
    return self->ShowItems(show);
}

// CLASS: wxSizerFlags
void wxSizerFlags_delete(wxSizerFlags *self) {
    delete self;
}
wxSizerFlags *wxSizerFlags_new(int proportion) {
    return new wxSizerFlags(proportion);
}
wxSizerFlags * wxSizerFlags_Align(wxSizerFlags * self, int alignment) {
    return &(self->Align(alignment));
}
wxSizerFlags * wxSizerFlags_Border(wxSizerFlags * self, int direction, int borderinpixels) {
    return &(self->Border(direction, borderinpixels));
}
wxSizerFlags * wxSizerFlags_Border1(wxSizerFlags * self, int direction) {
    return &(self->Border(direction));
}
wxSizerFlags * wxSizerFlags_Bottom(wxSizerFlags * self) {
    return &(self->Bottom());
}
wxSizerFlags * wxSizerFlags_Center(wxSizerFlags * self) {
    return &(self->Center());
}
wxSizerFlags * wxSizerFlags_Centre(wxSizerFlags * self) {
    return &(self->Centre());
}
#if wxCHECK_VERSION(3, 1, 0)
wxSizerFlags * wxSizerFlags_CenterHorizontal(wxSizerFlags * self) {
    return &(self->CenterHorizontal());
}
wxSizerFlags * wxSizerFlags_CenterVertical(wxSizerFlags * self) {
    return &(self->CenterVertical());
}
wxSizerFlags * wxSizerFlags_CentreHorizontal(wxSizerFlags * self) {
    return &(self->CentreHorizontal());
}
wxSizerFlags * wxSizerFlags_CentreVertical(wxSizerFlags * self) {
    return &(self->CentreVertical());
}
#endif
wxSizerFlags * wxSizerFlags_DoubleBorder(wxSizerFlags * self, int direction) {
    return &(self->DoubleBorder(direction));
}
wxSizerFlags * wxSizerFlags_DoubleHorzBorder(wxSizerFlags * self) {
    return &(self->DoubleHorzBorder());
}
wxSizerFlags * wxSizerFlags_Expand(wxSizerFlags * self) {
    return &(self->Expand());
}
wxSizerFlags * wxSizerFlags_FixedMinSize(wxSizerFlags * self) {
    return &(self->FixedMinSize());
}
wxSizerFlags * wxSizerFlags_ReserveSpaceEvenIfHidden(wxSizerFlags * self) {
    return &(self->ReserveSpaceEvenIfHidden());
}
wxSizerFlags * wxSizerFlags_Left(wxSizerFlags * self) {
    return &(self->Left());
}
wxSizerFlags * wxSizerFlags_Proportion(wxSizerFlags * self, int proportion) {
    return &(self->Proportion(proportion));
}
wxSizerFlags * wxSizerFlags_Right(wxSizerFlags * self) {
    return &(self->Right());
}
wxSizerFlags * wxSizerFlags_Shaped(wxSizerFlags * self) {
    return &(self->Shaped());
}
wxSizerFlags * wxSizerFlags_Top(wxSizerFlags * self) {
    return &(self->Top());
}
wxSizerFlags * wxSizerFlags_TripleBorder(wxSizerFlags * self, int direction) {
    return &(self->TripleBorder(direction));
}
#if wxCHECK_VERSION(3, 1, 0)
void wxSizerFlags_DisableConsistencyChecks() {
    return wxSizerFlags::DisableConsistencyChecks();
}
#endif
int wxSizerFlags_GetDefaultBorder() {
    return wxSizerFlags::GetDefaultBorder();
}

// CLASS: wxSizerItem
wxClassInfo *wxSizerItem_CLASSINFO() {
    return wxCLASSINFO(wxSizerItem);
}
wxSizerItem *wxSizerItem_new(int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return new wxSizerItem(width, height, proportion, flag, border, user_data);
}
wxSizerItem *wxSizerItem_new1(wxWindow * window, const wxSizerFlags * flags) {
    return new wxSizerItem(window, *flags);
}
wxSizerItem *wxSizerItem_new2(wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return new wxSizerItem(window, proportion, flag, border, user_data);
}
wxSizerItem *wxSizerItem_new3(wxSizer * sizer, const wxSizerFlags * flags) {
    return new wxSizerItem(sizer, *flags);
}
wxSizerItem *wxSizerItem_new4(wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return new wxSizerItem(sizer, proportion, flag, border, user_data);
}
void wxSizerItem_AssignWindow(wxSizerItem * self, wxWindow * window) {
    return self->AssignWindow(window);
}
void wxSizerItem_AssignSizer(wxSizerItem * self, wxSizer * sizer) {
    return self->AssignSizer(sizer);
}
void wxSizerItem_AssignSpacer(wxSizerItem * self, const wxSize * size) {
    return self->AssignSpacer(*size);
}
void wxSizerItem_AssignSpacer1(wxSizerItem * self, int w, int h) {
    return self->AssignSpacer(w, h);
}
wxSize *wxSizerItem_CalcMin(wxSizerItem * self) {
    return new wxSize(self->CalcMin());
}
void wxSizerItem_DeleteWindows(wxSizerItem * self) {
    return self->DeleteWindows();
}
void wxSizerItem_DetachSizer(wxSizerItem * self) {
    return self->DetachSizer();
}
int wxSizerItem_GetBorder(const wxSizerItem * self) {
    return self->GetBorder();
}
int wxSizerItem_GetFlag(const wxSizerItem * self) {
    return self->GetFlag();
}
int wxSizerItem_GetId(const wxSizerItem * self) {
    return self->GetId();
}
wxSize *wxSizerItem_GetMinSize(const wxSizerItem * self) {
    return new wxSize(self->GetMinSize());
}
void wxSizerItem_SetMinSize(wxSizerItem * self, const wxSize * size) {
    return self->SetMinSize(*size);
}
void wxSizerItem_SetMinSize1(wxSizerItem * self, int x, int y) {
    return self->SetMinSize(x, y);
}
wxPoint *wxSizerItem_GetPosition(const wxSizerItem * self) {
    return new wxPoint(self->GetPosition());
}
int wxSizerItem_GetProportion(const wxSizerItem * self) {
    return self->GetProportion();
}
wxRect *wxSizerItem_GetRect(wxSizerItem * self) {
    return new wxRect(self->GetRect());
}
wxSize *wxSizerItem_GetSize(const wxSizerItem * self) {
    return new wxSize(self->GetSize());
}
wxSizer * wxSizerItem_GetSizer(const wxSizerItem * self) {
    return self->GetSizer();
}
wxSize *wxSizerItem_GetSpacer(const wxSizerItem * self) {
    return new wxSize(self->GetSpacer());
}
wxObject * wxSizerItem_GetUserData(const wxSizerItem * self) {
    return self->GetUserData();
}
wxWindow * wxSizerItem_GetWindow(const wxSizerItem * self) {
    return self->GetWindow();
}
bool wxSizerItem_IsShown(const wxSizerItem * self) {
    return self->IsShown();
}
bool wxSizerItem_IsSizer(const wxSizerItem * self) {
    return self->IsSizer();
}
bool wxSizerItem_IsSpacer(const wxSizerItem * self) {
    return self->IsSpacer();
}
bool wxSizerItem_IsWindow(const wxSizerItem * self) {
    return self->IsWindow();
}
void wxSizerItem_SetBorder(wxSizerItem * self, int border) {
    return self->SetBorder(border);
}
void wxSizerItem_SetDimension(wxSizerItem * self, const wxPoint * pos, const wxSize * size) {
    return self->SetDimension(*pos, *size);
}
void wxSizerItem_SetFlag(wxSizerItem * self, int flag) {
    return self->SetFlag(flag);
}
void wxSizerItem_SetId(wxSizerItem * self, int id) {
    return self->SetId(id);
}
void wxSizerItem_SetInitSize(wxSizerItem * self, int x, int y) {
    return self->SetInitSize(x, y);
}
void wxSizerItem_SetProportion(wxSizerItem * self, int proportion) {
    return self->SetProportion(proportion);
}
void wxSizerItem_SetRatio(wxSizerItem * self, int width, int height) {
    return self->SetRatio(width, height);
}
void wxSizerItem_SetRatio1(wxSizerItem * self, wxSize size) {
    return self->SetRatio(size);
}
void wxSizerItem_SetSizer(wxSizerItem * self, wxSizer * sizer) {
    return self->SetSizer(sizer);
}
void wxSizerItem_SetSpacer(wxSizerItem * self, const wxSize * size) {
    return self->SetSpacer(*size);
}
void wxSizerItem_SetUserData(wxSizerItem * self, wxObject * user_data) {
    return self->SetUserData(user_data);
}
void wxSizerItem_SetWindow(wxSizerItem * self, wxWindow * window) {
    return self->SetWindow(window);
}
void wxSizerItem_Show(wxSizerItem * self, bool show) {
    return self->Show(show);
}

// CLASS: wxSlider
wxClassInfo *wxSlider_CLASSINFO() {
    return wxCLASSINFO(wxSlider);
}
wxSlider *wxSlider_new() {
    return new wxSlider();
}
wxSlider *wxSlider_new1(wxWindow * parent, wxWindowID id, int value, int min_value, int max_value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxSlider(parent, id, value, min_value, max_value, *pos, *size, style, *validator, *name);
}
void wxSlider_ClearSel(wxSlider * self) {
    return self->ClearSel();
}
void wxSlider_ClearTicks(wxSlider * self) {
    return self->ClearTicks();
}
bool wxSlider_Create(wxSlider * self, wxWindow * parent, wxWindowID id, int value, int min_value, int max_value, const wxPoint * point, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, value, min_value, max_value, *point, *size, style, *validator, *name);
}
int wxSlider_GetLineSize(const wxSlider * self) {
    return self->GetLineSize();
}
int wxSlider_GetMax(const wxSlider * self) {
    return self->GetMax();
}
int wxSlider_GetMin(const wxSlider * self) {
    return self->GetMin();
}
int wxSlider_GetPageSize(const wxSlider * self) {
    return self->GetPageSize();
}
int wxSlider_GetSelEnd(const wxSlider * self) {
    return self->GetSelEnd();
}
int wxSlider_GetSelStart(const wxSlider * self) {
    return self->GetSelStart();
}
int wxSlider_GetThumbLength(const wxSlider * self) {
    return self->GetThumbLength();
}
int wxSlider_GetTickFreq(const wxSlider * self) {
    return self->GetTickFreq();
}
int wxSlider_GetValue(const wxSlider * self) {
    return self->GetValue();
}
void wxSlider_SetLineSize(wxSlider * self, int line_size) {
    return self->SetLineSize(line_size);
}
void wxSlider_SetMin(wxSlider * self, int min_value) {
    return self->SetMin(min_value);
}
void wxSlider_SetMax(wxSlider * self, int max_value) {
    return self->SetMax(max_value);
}
void wxSlider_SetPageSize(wxSlider * self, int page_size) {
    return self->SetPageSize(page_size);
}
void wxSlider_SetRange(wxSlider * self, int min_value, int max_value) {
    return self->SetRange(min_value, max_value);
}
void wxSlider_SetSelection(wxSlider * self, int start_pos, int end_pos) {
    return self->SetSelection(start_pos, end_pos);
}
void wxSlider_SetThumbLength(wxSlider * self, int len) {
    return self->SetThumbLength(len);
}
void wxSlider_SetTick(wxSlider * self, int tick_pos) {
    return self->SetTick(tick_pos);
}
void wxSlider_SetTickFreq(wxSlider * self, int freq) {
    return self->SetTickFreq(freq);
}
void wxSlider_SetValue(wxSlider * self, int value) {
    return self->SetValue(value);
}
// Mix-in(s) to wxSlider
wxTrackable *wxSlider_AsTrackable(wxSlider* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxSound
wxClassInfo *wxSound_CLASSINFO() {
    return wxCLASSINFO(wxSound);
}
wxSound *wxSound_new() {
    return new wxSound();
}
wxSound *wxSound_new1(const wxString * file_name, bool is_resource) {
    return new wxSound(*file_name, is_resource);
}
wxSound *wxSound_new2(size_t size, const void * data) {
    return new wxSound(size, data);
}
bool wxSound_Create(wxSound * self, const wxString * file_name, bool is_resource) {
    return self->Create(*file_name, is_resource);
}
bool wxSound_Create1(wxSound * self, size_t size, const void * data) {
    return self->Create(size, data);
}
bool wxSound_IsOk(const wxSound * self) {
    return self->IsOk();
}
bool wxSound_IsPlaying() {
    return wxSound::IsPlaying();
}
void wxSound_Stop() {
    return wxSound::Stop();
}

// CLASS: wxSpinButton
wxClassInfo *wxSpinButton_CLASSINFO() {
    return wxCLASSINFO(wxSpinButton);
}
wxSpinButton *wxSpinButton_new() {
    return new wxSpinButton();
}
wxSpinButton *wxSpinButton_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxSpinButton(parent, id, *pos, *size, style, *name);
}
bool wxSpinButton_Create(wxSpinButton * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
#if wxCHECK_VERSION(3, 1, 0)
int wxSpinButton_GetIncrement(const wxSpinButton * self) {
    return self->GetIncrement();
}
#endif
int wxSpinButton_GetMax(const wxSpinButton * self) {
    return self->GetMax();
}
int wxSpinButton_GetMin(const wxSpinButton * self) {
    return self->GetMin();
}
int wxSpinButton_GetValue(const wxSpinButton * self) {
    return self->GetValue();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxSpinButton_SetIncrement(wxSpinButton * self, int value) {
    return self->SetIncrement(value);
}
#endif
void wxSpinButton_SetRange(wxSpinButton * self, int min, int max) {
    return self->SetRange(min, max);
}
void wxSpinButton_SetValue(wxSpinButton * self, int value) {
    return self->SetValue(value);
}
// Mix-in(s) to wxSpinButton
wxTrackable *wxSpinButton_AsTrackable(wxSpinButton* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxSpinCtrl
wxClassInfo *wxSpinCtrl_CLASSINFO() {
    return wxCLASSINFO(wxSpinCtrl);
}
wxSpinCtrl *wxSpinCtrl_new() {
    return new wxSpinCtrl();
}
wxSpinCtrl *wxSpinCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, int min, int max, int initial, const wxString * name) {
    return new wxSpinCtrl(parent, id, *value, *pos, *size, style, min, max, initial, *name);
}
bool wxSpinCtrl_Create(wxSpinCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, int min, int max, int initial, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, min, max, initial, *name);
}
int wxSpinCtrl_GetBase(const wxSpinCtrl * self) {
    return self->GetBase();
}
int wxSpinCtrl_GetMax(const wxSpinCtrl * self) {
    return self->GetMax();
}
int wxSpinCtrl_GetMin(const wxSpinCtrl * self) {
    return self->GetMin();
}
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxSpinCtrl_GetTextValue(const wxSpinCtrl * self) {
    return new wxString(self->GetTextValue());
}
#endif
int wxSpinCtrl_GetValue(const wxSpinCtrl * self) {
    return self->GetValue();
}
int wxSpinCtrl_GetIncrement(const wxSpinCtrl * self) {
    return self->GetIncrement();
}
bool wxSpinCtrl_SetBase(wxSpinCtrl * self, int base) {
    return self->SetBase(base);
}
void wxSpinCtrl_SetRange(wxSpinCtrl * self, int min_val, int max_val) {
    return self->SetRange(min_val, max_val);
}
void wxSpinCtrl_SetSelection(wxSpinCtrl * self, long from, long to) {
    return self->SetSelection(from, to);
}
void wxSpinCtrl_SetValue(wxSpinCtrl * self, const wxString * text) {
    return self->SetValue(*text);
}
void wxSpinCtrl_SetValue1(wxSpinCtrl * self, int value) {
    return self->SetValue(value);
}
void wxSpinCtrl_SetIncrement(wxSpinCtrl * self, int value) {
    return self->SetIncrement(value);
}
// Mix-in(s) to wxSpinCtrl
wxTrackable *wxSpinCtrl_AsTrackable(wxSpinCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxSpinCtrlDouble
wxClassInfo *wxSpinCtrlDouble_CLASSINFO() {
    return wxCLASSINFO(wxSpinCtrlDouble);
}
wxSpinCtrlDouble *wxSpinCtrlDouble_new() {
    return new wxSpinCtrlDouble();
}
wxSpinCtrlDouble *wxSpinCtrlDouble_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, double min, double max, double initial, double inc, const wxString * name) {
    return new wxSpinCtrlDouble(parent, id, *value, *pos, *size, style, min, max, initial, inc, *name);
}
bool wxSpinCtrlDouble_Create(wxSpinCtrlDouble * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, double min, double max, double initial, double inc, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, min, max, initial, inc, *name);
}
unsigned int wxSpinCtrlDouble_GetDigits(const wxSpinCtrlDouble * self) {
    return self->GetDigits();
}
double wxSpinCtrlDouble_GetIncrement(const wxSpinCtrlDouble * self) {
    return self->GetIncrement();
}
double wxSpinCtrlDouble_GetMax(const wxSpinCtrlDouble * self) {
    return self->GetMax();
}
double wxSpinCtrlDouble_GetMin(const wxSpinCtrlDouble * self) {
    return self->GetMin();
}
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxSpinCtrlDouble_GetTextValue(const wxSpinCtrlDouble * self) {
    return new wxString(self->GetTextValue());
}
#endif
double wxSpinCtrlDouble_GetValue(const wxSpinCtrlDouble * self) {
    return self->GetValue();
}
void wxSpinCtrlDouble_SetDigits(wxSpinCtrlDouble * self, unsigned int digits) {
    return self->SetDigits(digits);
}
void wxSpinCtrlDouble_SetIncrement(wxSpinCtrlDouble * self, double inc) {
    return self->SetIncrement(inc);
}
void wxSpinCtrlDouble_SetRange(wxSpinCtrlDouble * self, double min_val, double max_val) {
    return self->SetRange(min_val, max_val);
}
void wxSpinCtrlDouble_SetValue(wxSpinCtrlDouble * self, const wxString * text) {
    return self->SetValue(*text);
}
void wxSpinCtrlDouble_SetValue1(wxSpinCtrlDouble * self, double value) {
    return self->SetValue(value);
}
// Mix-in(s) to wxSpinCtrlDouble
wxTrackable *wxSpinCtrlDouble_AsTrackable(wxSpinCtrlDouble* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxSpinDoubleEvent
wxClassInfo *wxSpinDoubleEvent_CLASSINFO() {
    return wxCLASSINFO(wxSpinDoubleEvent);
}
wxSpinDoubleEvent *wxSpinDoubleEvent_new1(const wxSpinDoubleEvent * event) {
    return new wxSpinDoubleEvent(*event);
}
double wxSpinDoubleEvent_GetValue(const wxSpinDoubleEvent * self) {
    return self->GetValue();
}
void wxSpinDoubleEvent_SetValue(wxSpinDoubleEvent * self, double value) {
    return self->SetValue(value);
}

// CLASS: wxSpinEvent
wxClassInfo *wxSpinEvent_CLASSINFO() {
    return wxCLASSINFO(wxSpinEvent);
}
int wxSpinEvent_GetPosition(const wxSpinEvent * self) {
    return self->GetPosition();
}
void wxSpinEvent_SetPosition(wxSpinEvent * self, int pos) {
    return self->SetPosition(pos);
}

// CLASS: wxSplashScreen
wxClassInfo *wxSplashScreen_CLASSINFO() {
    return wxCLASSINFO(wxSplashScreen);
}
wxSplashScreen *wxSplashScreen_new(const wxBitmap * bitmap, long splash_style, int milliseconds, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style) {
    return new wxSplashScreen(*bitmap, splash_style, milliseconds, parent, id, *pos, *size, style);
}
long wxSplashScreen_GetSplashStyle(const wxSplashScreen * self) {
    return self->GetSplashStyle();
}
wxSplashScreenWindow * wxSplashScreen_GetSplashWindow(const wxSplashScreen * self) {
    return self->GetSplashWindow();
}
int wxSplashScreen_GetTimeout(const wxSplashScreen * self) {
    return self->GetTimeout();
}
void wxSplashScreen_OnCloseWindow(wxSplashScreen * self, wxCloseEvent * event) {
    return self->OnCloseWindow(*event);
}
// Mix-in(s) to wxSplashScreen
wxTrackable *wxSplashScreen_AsTrackable(wxSplashScreen* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxSplitterEvent
wxClassInfo *wxSplitterEvent_CLASSINFO() {
    return wxCLASSINFO(wxSplitterEvent);
}
int wxSplitterEvent_GetSashPosition(const wxSplitterEvent * self) {
    return self->GetSashPosition();
}
wxWindow * wxSplitterEvent_GetWindowBeingRemoved(const wxSplitterEvent * self) {
    return self->GetWindowBeingRemoved();
}
int wxSplitterEvent_GetX(const wxSplitterEvent * self) {
    return self->GetX();
}
int wxSplitterEvent_GetY(const wxSplitterEvent * self) {
    return self->GetY();
}
void wxSplitterEvent_SetSashPosition(wxSplitterEvent * self, int pos) {
    return self->SetSashPosition(pos);
}
void wxSplitterEvent_SetSize(wxSplitterEvent * self, int old_size, int new_size) {
    return self->SetSize(old_size, new_size);
}
int wxSplitterEvent_GetOldSize(const wxSplitterEvent * self) {
    return self->GetOldSize();
}

// CLASS: wxSplitterWindow
wxClassInfo *wxSplitterWindow_CLASSINFO() {
    return wxCLASSINFO(wxSplitterWindow);
}
wxSplitterWindow *wxSplitterWindow_new() {
    return new wxSplitterWindow();
}
wxSplitterWindow *wxSplitterWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxSplitterWindow(parent, id, *pos, *size, style, *name);
}
bool wxSplitterWindow_Create(wxSplitterWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * point, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *point, *size, style, *name);
}
int wxSplitterWindow_GetMinimumPaneSize(const wxSplitterWindow * self) {
    return self->GetMinimumPaneSize();
}
double wxSplitterWindow_GetSashGravity(const wxSplitterWindow * self) {
    return self->GetSashGravity();
}
int wxSplitterWindow_GetSashPosition(const wxSplitterWindow * self) {
    return self->GetSashPosition();
}
int wxSplitterWindow_GetSashSize(const wxSplitterWindow * self) {
    return self->GetSashSize();
}
int wxSplitterWindow_GetDefaultSashSize(const wxSplitterWindow * self) {
    return self->GetDefaultSashSize();
}
wxWindow * wxSplitterWindow_GetWindow1(const wxSplitterWindow * self) {
    return self->GetWindow1();
}
wxWindow * wxSplitterWindow_GetWindow2(const wxSplitterWindow * self) {
    return self->GetWindow2();
}
void wxSplitterWindow_Initialize(wxSplitterWindow * self, wxWindow * window) {
    return self->Initialize(window);
}
bool wxSplitterWindow_IsSashInvisible(const wxSplitterWindow * self) {
    return self->IsSashInvisible();
}
bool wxSplitterWindow_IsSplit(const wxSplitterWindow * self) {
    return self->IsSplit();
}
void wxSplitterWindow_OnDoubleClickSash(wxSplitterWindow * self, int x, int y) {
    return self->OnDoubleClickSash(x, y);
}
bool wxSplitterWindow_OnSashPositionChange(wxSplitterWindow * self, int new_sash_position) {
    return self->OnSashPositionChange(new_sash_position);
}
void wxSplitterWindow_OnUnsplit(wxSplitterWindow * self, wxWindow * removed) {
    return self->OnUnsplit(removed);
}
bool wxSplitterWindow_ReplaceWindow(wxSplitterWindow * self, wxWindow * win_old, wxWindow * win_new) {
    return self->ReplaceWindow(win_old, win_new);
}
void wxSplitterWindow_SetMinimumPaneSize(wxSplitterWindow * self, int pane_size) {
    return self->SetMinimumPaneSize(pane_size);
}
void wxSplitterWindow_SetSashGravity(wxSplitterWindow * self, double gravity) {
    return self->SetSashGravity(gravity);
}
void wxSplitterWindow_SetSashPosition(wxSplitterWindow * self, int position, bool redraw) {
    return self->SetSashPosition(position, redraw);
}
void wxSplitterWindow_SetSplitMode(wxSplitterWindow * self, int mode) {
    return self->SetSplitMode(mode);
}
void wxSplitterWindow_SetSashInvisible(wxSplitterWindow * self, bool invisible) {
    return self->SetSashInvisible(invisible);
}
bool wxSplitterWindow_SplitHorizontally(wxSplitterWindow * self, wxWindow * window1, wxWindow * window2, int sash_position) {
    return self->SplitHorizontally(window1, window2, sash_position);
}
bool wxSplitterWindow_SplitVertically(wxSplitterWindow * self, wxWindow * window1, wxWindow * window2, int sash_position) {
    return self->SplitVertically(window1, window2, sash_position);
}
bool wxSplitterWindow_Unsplit(wxSplitterWindow * self, wxWindow * to_remove) {
    return self->Unsplit(to_remove);
}
void wxSplitterWindow_UpdateSize(wxSplitterWindow * self) {
    return self->UpdateSize();
}
// Mix-in(s) to wxSplitterWindow
wxTrackable *wxSplitterWindow_AsTrackable(wxSplitterWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxStaticBitmap
wxClassInfo *wxStaticBitmap_CLASSINFO() {
    return wxCLASSINFO(wxStaticBitmap);
}
wxStaticBitmap *wxStaticBitmap_new() {
    return new wxStaticBitmap();
}
wxStaticBitmap *wxStaticBitmap_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxStaticBitmap(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticBitmap_Create(wxStaticBitmap * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}
wxBitmap *wxStaticBitmap_GetBitmap(const wxStaticBitmap * self) {
    return new wxBitmap(self->GetBitmap());
}
wxIcon *wxStaticBitmap_GetIcon(const wxStaticBitmap * self) {
    return new wxIcon(self->GetIcon());
}
void wxStaticBitmap_SetBitmap(wxStaticBitmap * self, const wxBitmapBundle * label) {
    return self->SetBitmap(*label);
}
void wxStaticBitmap_SetIcon(wxStaticBitmap * self, const wxIcon * label) {
    return self->SetIcon(*label);
}
// Mix-in(s) to wxStaticBitmap
wxTrackable *wxStaticBitmap_AsTrackable(wxStaticBitmap* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxStaticBox
wxClassInfo *wxStaticBox_CLASSINFO() {
    return wxCLASSINFO(wxStaticBox);
}
wxStaticBox *wxStaticBox_new() {
    return new wxStaticBox();
}
wxStaticBox *wxStaticBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxStaticBox(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticBox_Create(wxStaticBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}
// Mix-in(s) to wxStaticBox
wxTrackable *wxStaticBox_AsTrackable(wxStaticBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxStaticBoxSizer
wxClassInfo *wxStaticBoxSizer_CLASSINFO() {
    return wxCLASSINFO(wxStaticBoxSizer);
}
wxStaticBoxSizer *wxStaticBoxSizer_new(wxStaticBox * box_, int orient) {
    return new wxStaticBoxSizer(box_, orient);
}
wxStaticBoxSizer *wxStaticBoxSizer_new1(int orient, wxWindow * parent, const wxString * label) {
    return new wxStaticBoxSizer(orient, parent, *label);
}
wxStaticBox * wxStaticBoxSizer_GetStaticBox(const wxStaticBoxSizer * self) {
    return self->GetStaticBox();
}

// CLASS: wxStaticLine
wxClassInfo *wxStaticLine_CLASSINFO() {
    return wxCLASSINFO(wxStaticLine);
}
wxStaticLine *wxStaticLine_new() {
    return new wxStaticLine();
}
wxStaticLine *wxStaticLine_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxStaticLine(parent, id, *pos, *size, style, *name);
}
bool wxStaticLine_Create(wxStaticLine * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
bool wxStaticLine_IsVertical(const wxStaticLine * self) {
    return self->IsVertical();
}
int wxStaticLine_GetDefaultSize() {
    return wxStaticLine::GetDefaultSize();
}
// Mix-in(s) to wxStaticLine
wxTrackable *wxStaticLine_AsTrackable(wxStaticLine* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxStaticText
wxClassInfo *wxStaticText_CLASSINFO() {
    return wxCLASSINFO(wxStaticText);
}
wxStaticText *wxStaticText_new() {
    return new wxStaticText();
}
wxStaticText *wxStaticText_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxStaticText(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticText_Create(wxStaticText * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticText_IsEllipsized(const wxStaticText * self) {
    return self->IsEllipsized();
}
void wxStaticText_Wrap(wxStaticText * self, int width) {
    return self->Wrap(width);
}
// Mix-in(s) to wxStaticText
wxTrackable *wxStaticText_AsTrackable(wxStaticText* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxStatusBar
wxClassInfo *wxStatusBar_CLASSINFO() {
    return wxCLASSINFO(wxStatusBar);
}
wxStatusBar *wxStatusBar_new() {
    return new wxStatusBar();
}
wxStatusBar *wxStatusBar_new1(wxWindow * parent, wxWindowID id, long style, const wxString * name) {
    return new wxStatusBar(parent, id, style, *name);
}
bool wxStatusBar_Create(wxStatusBar * self, wxWindow * parent, wxWindowID id, long style, const wxString * name) {
    return self->Create(parent, id, style, *name);
}
bool wxStatusBar_GetFieldRect(const wxStatusBar * self, int i, wxRect * rect) {
    return self->GetFieldRect(i, *rect);
}
int wxStatusBar_GetFieldsCount(const wxStatusBar * self) {
    return self->GetFieldsCount();
}
wxStatusBarPane *wxStatusBar_GetField(const wxStatusBar * self, int n) {
    return new wxStatusBarPane(self->GetField(n));
}
wxSize *wxStatusBar_GetBorders(const wxStatusBar * self) {
    return new wxSize(self->GetBorders());
}
wxString *wxStatusBar_GetStatusText(const wxStatusBar * self, int i) {
    return new wxString(self->GetStatusText(i));
}
int wxStatusBar_GetStatusWidth(const wxStatusBar * self, int n) {
    return self->GetStatusWidth(n);
}
int wxStatusBar_GetStatusStyle(const wxStatusBar * self, int n) {
    return self->GetStatusStyle(n);
}
void wxStatusBar_PopStatusText(wxStatusBar * self, int field) {
    return self->PopStatusText(field);
}
void wxStatusBar_PushStatusText(wxStatusBar * self, const wxString * string, int field) {
    return self->PushStatusText(*string, field);
}
void wxStatusBar_SetFieldsCount(wxStatusBar * self, int number, const int * widths) {
    return self->SetFieldsCount(number, widths);
}
void wxStatusBar_SetMinHeight(wxStatusBar * self, int height) {
    return self->SetMinHeight(height);
}
void wxStatusBar_SetStatusStyles(wxStatusBar * self, int n, const int * styles) {
    return self->SetStatusStyles(n, styles);
}
void wxStatusBar_SetStatusText(wxStatusBar * self, const wxString * text, int i) {
    return self->SetStatusText(*text, i);
}
void wxStatusBar_SetStatusWidths(wxStatusBar * self, int n, const int * widths_field) {
    return self->SetStatusWidths(n, widths_field);
}
// Mix-in(s) to wxStatusBar
wxTrackable *wxStatusBar_AsTrackable(wxStatusBar* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxStatusBarPane
void wxStatusBarPane_delete(wxStatusBarPane *self) {
    delete self;
}
wxStatusBarPane *wxStatusBarPane_new(int style, int width) {
    return new wxStatusBarPane(style, width);
}
int wxStatusBarPane_GetWidth(const wxStatusBarPane * self) {
    return self->GetWidth();
}
int wxStatusBarPane_GetStyle(const wxStatusBarPane * self) {
    return self->GetStyle();
}
wxString *wxStatusBarPane_GetText(const wxStatusBarPane * self) {
    return new wxString(self->GetText());
}

// CLASS: wxStdDialogButtonSizer
wxClassInfo *wxStdDialogButtonSizer_CLASSINFO() {
    return wxCLASSINFO(wxStdDialogButtonSizer);
}
wxStdDialogButtonSizer *wxStdDialogButtonSizer_new() {
    return new wxStdDialogButtonSizer();
}
void wxStdDialogButtonSizer_AddButton(wxStdDialogButtonSizer * self, wxButton * button) {
    return self->AddButton(button);
}
void wxStdDialogButtonSizer_Realize(wxStdDialogButtonSizer * self) {
    return self->Realize();
}
void wxStdDialogButtonSizer_SetAffirmativeButton(wxStdDialogButtonSizer * self, wxButton * button) {
    return self->SetAffirmativeButton(button);
}
void wxStdDialogButtonSizer_SetCancelButton(wxStdDialogButtonSizer * self, wxButton * button) {
    return self->SetCancelButton(button);
}
void wxStdDialogButtonSizer_SetNegativeButton(wxStdDialogButtonSizer * self, wxButton * button) {
    return self->SetNegativeButton(button);
}

// CLASS: wxStockPreferencesPage
void wxStockPreferencesPage_delete(wxStockPreferencesPage *self) {
    delete self;
}

// CLASS: wxStreamToTextRedirector
void wxStreamToTextRedirector_delete(wxStreamToTextRedirector *self) {
    delete self;
}
wxStreamToTextRedirector *wxStreamToTextRedirector_new(wxTextCtrl * text, ostream * ostr) {
    return new wxStreamToTextRedirector(text, ostr);
}

// CLASS: wxSysColourChangedEvent
wxClassInfo *wxSysColourChangedEvent_CLASSINFO() {
    return wxCLASSINFO(wxSysColourChangedEvent);
}
wxSysColourChangedEvent *wxSysColourChangedEvent_new() {
    return new wxSysColourChangedEvent();
}

// CLASS: wxSystemSettings
void wxSystemSettings_delete(wxSystemSettings *self) {
    delete self;
}
wxSystemSettings *wxSystemSettings_new() {
    return new wxSystemSettings();
}
wxSystemAppearance *wxSystemSettings_GetAppearance() {
    return new wxSystemAppearance(wxSystemSettings::GetAppearance());
}

// CLASS: wxTGAHandler
wxClassInfo *wxTGAHandler_CLASSINFO() {
    return wxCLASSINFO(wxTGAHandler);
}
wxTGAHandler *wxTGAHandler_new() {
    return new wxTGAHandler();
}

// CLASS: wxTIFFHandler
wxClassInfo *wxTIFFHandler_CLASSINFO() {
    return wxCLASSINFO(wxTIFFHandler);
}
wxTIFFHandler *wxTIFFHandler_new() {
    return new wxTIFFHandler();
}
wxVersionInfo *wxTIFFHandler_GetLibraryVersionInfo() {
    return new wxVersionInfo(wxTIFFHandler::GetLibraryVersionInfo());
}

// CLASS: wxTaskBarButton
void wxTaskBarButton_delete(wxTaskBarButton *self) {
    delete self;
}
void wxTaskBarButton_SetProgressRange(wxTaskBarButton * self, int range) {
    return self->SetProgressRange(range);
}
void wxTaskBarButton_SetProgressValue(wxTaskBarButton * self, int value) {
    return self->SetProgressValue(value);
}
void wxTaskBarButton_PulseProgress(wxTaskBarButton * self) {
    return self->PulseProgress();
}
void wxTaskBarButton_Show(wxTaskBarButton * self, bool show) {
    return self->Show(show);
}
void wxTaskBarButton_Hide(wxTaskBarButton * self) {
    return self->Hide();
}
void wxTaskBarButton_SetThumbnailTooltip(wxTaskBarButton * self, const wxString * tooltip) {
    return self->SetThumbnailTooltip(*tooltip);
}
void wxTaskBarButton_SetOverlayIcon(wxTaskBarButton * self, const wxIcon * icon, const wxString * description) {
    return self->SetOverlayIcon(*icon, *description);
}
void wxTaskBarButton_SetThumbnailClip(wxTaskBarButton * self, const wxRect * rect) {
    return self->SetThumbnailClip(*rect);
}
void wxTaskBarButton_SetThumbnailContents(wxTaskBarButton * self, const wxWindow * child) {
    return self->SetThumbnailContents(child);
}
bool wxTaskBarButton_InsertThumbBarButton(wxTaskBarButton * self, size_t pos, wxThumbBarButton * button) {
    return self->InsertThumbBarButton(pos, button);
}
bool wxTaskBarButton_AppendThumbBarButton(wxTaskBarButton * self, wxThumbBarButton * button) {
    return self->AppendThumbBarButton(button);
}
bool wxTaskBarButton_AppendSeparatorInThumbBar(wxTaskBarButton * self) {
    return self->AppendSeparatorInThumbBar();
}
wxThumbBarButton * wxTaskBarButton_RemoveThumbBarButton(wxTaskBarButton * self, wxThumbBarButton * button) {
    return self->RemoveThumbBarButton(button);
}
wxThumbBarButton * wxTaskBarButton_RemoveThumbBarButton1(wxTaskBarButton * self, int id) {
    return self->RemoveThumbBarButton(id);
}

// CLASS: wxTaskBarIcon
wxClassInfo *wxTaskBarIcon_CLASSINFO() {
    return wxCLASSINFO(wxTaskBarIcon);
}
void wxTaskBarIcon_Destroy(wxTaskBarIcon * self) {
    return self->Destroy();
}
bool wxTaskBarIcon_IsIconInstalled(const wxTaskBarIcon * self) {
    return self->IsIconInstalled();
}
bool wxTaskBarIcon_IsOk(const wxTaskBarIcon * self) {
    return self->IsOk();
}
bool wxTaskBarIcon_PopupMenu(wxTaskBarIcon * self, wxMenu * menu) {
    return self->PopupMenu(menu);
}
bool wxTaskBarIcon_RemoveIcon(wxTaskBarIcon * self) {
    return self->RemoveIcon();
}
bool wxTaskBarIcon_SetIcon(wxTaskBarIcon * self, const wxBitmapBundle * icon, const wxString * tooltip) {
    return self->SetIcon(*icon, *tooltip);
}
bool wxTaskBarIcon_IsAvailable() {
    return wxTaskBarIcon::IsAvailable();
}
// Mix-in(s) to wxTaskBarIcon
wxTrackable *wxTaskBarIcon_AsTrackable(wxTaskBarIcon* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTaskBarIconEvent
wxClassInfo *wxTaskBarIconEvent_CLASSINFO() {
    return wxCLASSINFO(wxTaskBarIconEvent);
}

// CLASS: wxTaskBarJumpList
void wxTaskBarJumpList_delete(wxTaskBarJumpList *self) {
    delete self;
}
wxTaskBarJumpList *wxTaskBarJumpList_new(const wxString * app_id) {
    return new wxTaskBarJumpList(*app_id);
}
void wxTaskBarJumpList_ShowRecentCategory(wxTaskBarJumpList * self, bool shown) {
    return self->ShowRecentCategory(shown);
}
void wxTaskBarJumpList_HideRecentCategory(wxTaskBarJumpList * self) {
    return self->HideRecentCategory();
}
void wxTaskBarJumpList_ShowFrequentCategory(wxTaskBarJumpList * self, bool shown) {
    return self->ShowFrequentCategory(shown);
}
void wxTaskBarJumpList_HideFrequentCategory(wxTaskBarJumpList * self) {
    return self->HideFrequentCategory();
}
wxTaskBarJumpListCategory * wxTaskBarJumpList_GetTasks(const wxTaskBarJumpList * self) {
    return &(self->GetTasks());
}
wxTaskBarJumpListCategory *wxTaskBarJumpList_GetFrequentCategory(const wxTaskBarJumpList * self) {
    return new wxTaskBarJumpListCategory(self->GetFrequentCategory());
}
wxTaskBarJumpListCategory *wxTaskBarJumpList_GetRecentCategory(const wxTaskBarJumpList * self) {
    return new wxTaskBarJumpListCategory(self->GetRecentCategory());
}
const wxTaskBarJumpListCategories * wxTaskBarJumpList_GetCustomCategories(const wxTaskBarJumpList * self) {
    return self->GetCustomCategories();
}
void wxTaskBarJumpList_AddCustomCategory(wxTaskBarJumpList * self, wxTaskBarJumpListCategory * category) {
    return self->AddCustomCategory(category);
}
wxTaskBarJumpListCategory * wxTaskBarJumpList_RemoveCustomCategory(wxTaskBarJumpList * self, const wxString * title) {
    return self->RemoveCustomCategory(*title);
}
void wxTaskBarJumpList_DeleteCustomCategory(wxTaskBarJumpList * self, const wxString * title) {
    return self->DeleteCustomCategory(*title);
}

// CLASS: wxTaskBarJumpListCategory
void wxTaskBarJumpListCategory_delete(wxTaskBarJumpListCategory *self) {
    delete self;
}
wxTaskBarJumpListCategory *wxTaskBarJumpListCategory_new(wxTaskBarJumpList * parent, const wxString * title) {
    return new wxTaskBarJumpListCategory(parent, *title);
}
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_Append(wxTaskBarJumpListCategory * self, wxTaskBarJumpListItem * item) {
    return self->Append(item);
}
void wxTaskBarJumpListCategory_Delete(wxTaskBarJumpListCategory * self, wxTaskBarJumpListItem * item) {
    return self->Delete(item);
}
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_Remove(wxTaskBarJumpListCategory * self, wxTaskBarJumpListItem * item) {
    return self->Remove(item);
}
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_FindItemByPosition(const wxTaskBarJumpListCategory * self, size_t pos) {
    return self->FindItemByPosition(pos);
}
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_Insert(wxTaskBarJumpListCategory * self, size_t pos, wxTaskBarJumpListItem * item) {
    return self->Insert(pos, item);
}
wxTaskBarJumpListItem * wxTaskBarJumpListCategory_Prepend(wxTaskBarJumpListCategory * self, wxTaskBarJumpListItem * item) {
    return self->Prepend(item);
}
void wxTaskBarJumpListCategory_SetTitle(wxTaskBarJumpListCategory * self, const wxString * title) {
    return self->SetTitle(*title);
}
wxString *wxTaskBarJumpListCategory_GetTitle(const wxTaskBarJumpListCategory * self) {
    return new wxString(self->GetTitle());
}
const wxTaskBarJumpListItems * wxTaskBarJumpListCategory_GetItems(const wxTaskBarJumpListCategory * self) {
    return self->GetItems();
}

// CLASS: wxTaskBarJumpListItem
void wxTaskBarJumpListItem_delete(wxTaskBarJumpListItem *self) {
    delete self;
}
wxString *wxTaskBarJumpListItem_GetTitle(const wxTaskBarJumpListItem * self) {
    return new wxString(self->GetTitle());
}
void wxTaskBarJumpListItem_SetTitle(wxTaskBarJumpListItem * self, const wxString * title) {
    return self->SetTitle(*title);
}
wxString *wxTaskBarJumpListItem_GetFilePath(const wxTaskBarJumpListItem * self) {
    return new wxString(self->GetFilePath());
}
void wxTaskBarJumpListItem_SetFilePath(wxTaskBarJumpListItem * self, const wxString * file_path) {
    return self->SetFilePath(*file_path);
}
wxString *wxTaskBarJumpListItem_GetArguments(const wxTaskBarJumpListItem * self) {
    return new wxString(self->GetArguments());
}
void wxTaskBarJumpListItem_SetArguments(wxTaskBarJumpListItem * self, const wxString * arguments) {
    return self->SetArguments(*arguments);
}
wxString *wxTaskBarJumpListItem_GetTooltip(const wxTaskBarJumpListItem * self) {
    return new wxString(self->GetTooltip());
}
void wxTaskBarJumpListItem_SetTooltip(wxTaskBarJumpListItem * self, const wxString * tooltip) {
    return self->SetTooltip(*tooltip);
}
wxString *wxTaskBarJumpListItem_GetIconPath(const wxTaskBarJumpListItem * self) {
    return new wxString(self->GetIconPath());
}
void wxTaskBarJumpListItem_SetIconPath(wxTaskBarJumpListItem * self, const wxString * icon_path) {
    return self->SetIconPath(*icon_path);
}
int wxTaskBarJumpListItem_GetIconIndex(const wxTaskBarJumpListItem * self) {
    return self->GetIconIndex();
}
void wxTaskBarJumpListItem_SetIconIndex(wxTaskBarJumpListItem * self, int icon_index) {
    return self->SetIconIndex(icon_index);
}
wxTaskBarJumpListCategory * wxTaskBarJumpListItem_GetCategory(const wxTaskBarJumpListItem * self) {
    return self->GetCategory();
}
void wxTaskBarJumpListItem_SetCategory(wxTaskBarJumpListItem * self, wxTaskBarJumpListCategory * category) {
    return self->SetCategory(category);
}

// CLASS: wxTextAttr
void wxTextAttr_delete(wxTextAttr *self) {
    delete self;
}
wxColour *wxTextAttr_GetBackgroundColour(const wxTextAttr * self) {
    return new wxColour(self->GetBackgroundColour());
}
wxString *wxTextAttr_GetBulletFont(const wxTextAttr * self) {
    return new wxString(self->GetBulletFont());
}
wxString *wxTextAttr_GetBulletName(const wxTextAttr * self) {
    return new wxString(self->GetBulletName());
}
int wxTextAttr_GetBulletNumber(const wxTextAttr * self) {
    return self->GetBulletNumber();
}
int wxTextAttr_GetBulletStyle(const wxTextAttr * self) {
    return self->GetBulletStyle();
}
wxString *wxTextAttr_GetBulletText(const wxTextAttr * self) {
    return new wxString(self->GetBulletText());
}
wxString *wxTextAttr_GetCharacterStyleName(const wxTextAttr * self) {
    return new wxString(self->GetCharacterStyleName());
}
long wxTextAttr_GetFlags(const wxTextAttr * self) {
    return self->GetFlags();
}
wxFont *wxTextAttr_GetFont(const wxTextAttr * self) {
    return new wxFont(self->GetFont());
}
bool wxTextAttr_GetFontAttributes(wxTextAttr * self, const wxFont * font, int flags) {
    return self->GetFontAttributes(*font, flags);
}
wxString *wxTextAttr_GetFontFaceName(const wxTextAttr * self) {
    return new wxString(self->GetFontFaceName());
}
int wxTextAttr_GetFontSize(const wxTextAttr * self) {
    return self->GetFontSize();
}
bool wxTextAttr_GetFontUnderlined(const wxTextAttr * self) {
    return self->GetFontUnderlined();
}
#if wxCHECK_VERSION(3, 1, 0)
wxColour *wxTextAttr_GetUnderlineColour(const wxTextAttr * self) {
    return new wxColour(self->GetUnderlineColour());
}
#endif
long wxTextAttr_GetLeftIndent(const wxTextAttr * self) {
    return self->GetLeftIndent();
}
long wxTextAttr_GetLeftSubIndent(const wxTextAttr * self) {
    return self->GetLeftSubIndent();
}
int wxTextAttr_GetLineSpacing(const wxTextAttr * self) {
    return self->GetLineSpacing();
}
wxString *wxTextAttr_GetListStyleName(const wxTextAttr * self) {
    return new wxString(self->GetListStyleName());
}
int wxTextAttr_GetOutlineLevel(const wxTextAttr * self) {
    return self->GetOutlineLevel();
}
int wxTextAttr_GetParagraphSpacingAfter(const wxTextAttr * self) {
    return self->GetParagraphSpacingAfter();
}
int wxTextAttr_GetParagraphSpacingBefore(const wxTextAttr * self) {
    return self->GetParagraphSpacingBefore();
}
wxString *wxTextAttr_GetParagraphStyleName(const wxTextAttr * self) {
    return new wxString(self->GetParagraphStyleName());
}
long wxTextAttr_GetRightIndent(const wxTextAttr * self) {
    return self->GetRightIndent();
}
wxArrayInt *wxTextAttr_GetTabs(const wxTextAttr * self) {
    return new wxArrayInt(self->GetTabs());
}
wxColour *wxTextAttr_GetTextColour(const wxTextAttr * self) {
    return new wxColour(self->GetTextColour());
}
int wxTextAttr_GetTextEffectFlags(const wxTextAttr * self) {
    return self->GetTextEffectFlags();
}
int wxTextAttr_GetTextEffects(const wxTextAttr * self) {
    return self->GetTextEffects();
}
wxString *wxTextAttr_GetURL(const wxTextAttr * self) {
    return new wxString(self->GetURL());
}
bool wxTextAttr_HasAlignment(const wxTextAttr * self) {
    return self->HasAlignment();
}
bool wxTextAttr_HasBackgroundColour(const wxTextAttr * self) {
    return self->HasBackgroundColour();
}
bool wxTextAttr_HasBulletName(const wxTextAttr * self) {
    return self->HasBulletName();
}
bool wxTextAttr_HasBulletNumber(const wxTextAttr * self) {
    return self->HasBulletNumber();
}
bool wxTextAttr_HasBulletStyle(const wxTextAttr * self) {
    return self->HasBulletStyle();
}
bool wxTextAttr_HasBulletText(const wxTextAttr * self) {
    return self->HasBulletText();
}
bool wxTextAttr_HasCharacterStyleName(const wxTextAttr * self) {
    return self->HasCharacterStyleName();
}
bool wxTextAttr_HasFlag(const wxTextAttr * self, long flag) {
    return self->HasFlag(flag);
}
bool wxTextAttr_HasFont(const wxTextAttr * self) {
    return self->HasFont();
}
bool wxTextAttr_HasFontEncoding(const wxTextAttr * self) {
    return self->HasFontEncoding();
}
bool wxTextAttr_HasFontFaceName(const wxTextAttr * self) {
    return self->HasFontFaceName();
}
bool wxTextAttr_HasFontFamily(const wxTextAttr * self) {
    return self->HasFontFamily();
}
bool wxTextAttr_HasFontItalic(const wxTextAttr * self) {
    return self->HasFontItalic();
}
bool wxTextAttr_HasFontSize(const wxTextAttr * self) {
    return self->HasFontSize();
}
bool wxTextAttr_HasFontPointSize(const wxTextAttr * self) {
    return self->HasFontPointSize();
}
bool wxTextAttr_HasFontPixelSize(const wxTextAttr * self) {
    return self->HasFontPixelSize();
}
bool wxTextAttr_HasFontUnderlined(const wxTextAttr * self) {
    return self->HasFontUnderlined();
}
bool wxTextAttr_HasFontWeight(const wxTextAttr * self) {
    return self->HasFontWeight();
}
bool wxTextAttr_HasLeftIndent(const wxTextAttr * self) {
    return self->HasLeftIndent();
}
bool wxTextAttr_HasLineSpacing(const wxTextAttr * self) {
    return self->HasLineSpacing();
}
bool wxTextAttr_HasListStyleName(const wxTextAttr * self) {
    return self->HasListStyleName();
}
bool wxTextAttr_HasOutlineLevel(const wxTextAttr * self) {
    return self->HasOutlineLevel();
}
bool wxTextAttr_HasPageBreak(const wxTextAttr * self) {
    return self->HasPageBreak();
}
bool wxTextAttr_HasParagraphSpacingAfter(const wxTextAttr * self) {
    return self->HasParagraphSpacingAfter();
}
bool wxTextAttr_HasParagraphSpacingBefore(const wxTextAttr * self) {
    return self->HasParagraphSpacingBefore();
}
bool wxTextAttr_HasParagraphStyleName(const wxTextAttr * self) {
    return self->HasParagraphStyleName();
}
bool wxTextAttr_HasRightIndent(const wxTextAttr * self) {
    return self->HasRightIndent();
}
bool wxTextAttr_HasTabs(const wxTextAttr * self) {
    return self->HasTabs();
}
bool wxTextAttr_HasTextColour(const wxTextAttr * self) {
    return self->HasTextColour();
}
bool wxTextAttr_HasTextEffects(const wxTextAttr * self) {
    return self->HasTextEffects();
}
bool wxTextAttr_HasURL(const wxTextAttr * self) {
    return self->HasURL();
}
bool wxTextAttr_IsCharacterStyle(const wxTextAttr * self) {
    return self->IsCharacterStyle();
}
bool wxTextAttr_IsDefault(const wxTextAttr * self) {
    return self->IsDefault();
}
bool wxTextAttr_IsParagraphStyle(const wxTextAttr * self) {
    return self->IsParagraphStyle();
}
void wxTextAttr_SetBackgroundColour(wxTextAttr * self, const wxColour * col_back) {
    return self->SetBackgroundColour(*col_back);
}
void wxTextAttr_SetBulletFont(wxTextAttr * self, const wxString * font) {
    return self->SetBulletFont(*font);
}
void wxTextAttr_SetBulletName(wxTextAttr * self, const wxString * name) {
    return self->SetBulletName(*name);
}
void wxTextAttr_SetBulletNumber(wxTextAttr * self, int n) {
    return self->SetBulletNumber(n);
}
void wxTextAttr_SetBulletStyle(wxTextAttr * self, int style) {
    return self->SetBulletStyle(style);
}
void wxTextAttr_SetBulletText(wxTextAttr * self, const wxString * text) {
    return self->SetBulletText(*text);
}
void wxTextAttr_SetCharacterStyleName(wxTextAttr * self, const wxString * name) {
    return self->SetCharacterStyleName(*name);
}
void wxTextAttr_SetFlags(wxTextAttr * self, long flags) {
    return self->SetFlags(flags);
}
void wxTextAttr_SetFont(wxTextAttr * self, const wxFont * font, int flags) {
    return self->SetFont(*font, flags);
}
void wxTextAttr_SetFontFaceName(wxTextAttr * self, const wxString * face_name) {
    return self->SetFontFaceName(*face_name);
}
void wxTextAttr_SetFontSize(wxTextAttr * self, int point_size) {
    return self->SetFontSize(point_size);
}
void wxTextAttr_SetFontPointSize(wxTextAttr * self, int point_size) {
    return self->SetFontPointSize(point_size);
}
void wxTextAttr_SetFontPixelSize(wxTextAttr * self, int pixel_size) {
    return self->SetFontPixelSize(pixel_size);
}
void wxTextAttr_SetFontUnderlined(wxTextAttr * self, bool underlined) {
    return self->SetFontUnderlined(underlined);
}
void wxTextAttr_SetLeftIndent(wxTextAttr * self, int indent, int sub_indent) {
    return self->SetLeftIndent(indent, sub_indent);
}
void wxTextAttr_SetLineSpacing(wxTextAttr * self, int spacing) {
    return self->SetLineSpacing(spacing);
}
void wxTextAttr_SetListStyleName(wxTextAttr * self, const wxString * name) {
    return self->SetListStyleName(*name);
}
void wxTextAttr_SetOutlineLevel(wxTextAttr * self, int level) {
    return self->SetOutlineLevel(level);
}
void wxTextAttr_SetPageBreak(wxTextAttr * self, bool page_break) {
    return self->SetPageBreak(page_break);
}
void wxTextAttr_SetParagraphSpacingAfter(wxTextAttr * self, int spacing) {
    return self->SetParagraphSpacingAfter(spacing);
}
void wxTextAttr_SetParagraphSpacingBefore(wxTextAttr * self, int spacing) {
    return self->SetParagraphSpacingBefore(spacing);
}
void wxTextAttr_SetParagraphStyleName(wxTextAttr * self, const wxString * name) {
    return self->SetParagraphStyleName(*name);
}
void wxTextAttr_SetRightIndent(wxTextAttr * self, int indent) {
    return self->SetRightIndent(indent);
}
void wxTextAttr_SetTabs(wxTextAttr * self, const wxArrayInt * tabs) {
    return self->SetTabs(*tabs);
}
void wxTextAttr_SetTextColour(wxTextAttr * self, const wxColour * col_text) {
    return self->SetTextColour(*col_text);
}
void wxTextAttr_SetTextEffectFlags(wxTextAttr * self, int flags) {
    return self->SetTextEffectFlags(flags);
}
void wxTextAttr_SetTextEffects(wxTextAttr * self, int effects) {
    return self->SetTextEffects(effects);
}
void wxTextAttr_SetURL(wxTextAttr * self, const wxString * url) {
    return self->SetURL(*url);
}
wxTextAttr *wxTextAttr_new() {
    return new wxTextAttr();
}
wxTextAttr *wxTextAttr_new2(const wxTextAttr * attr) {
    return new wxTextAttr(*attr);
}
bool wxTextAttr_Apply(wxTextAttr * self, const wxTextAttr * style, const wxTextAttr * compare_with) {
    return self->Apply(*style, compare_with);
}
void wxTextAttr_Merge(wxTextAttr * self, const wxTextAttr * overlay) {
    return self->Merge(*overlay);
}
bool wxTextAttr_EqPartial(const wxTextAttr * self, const wxTextAttr * attr, bool weak_test) {
    return self->EqPartial(*attr, weak_test);
}
wxTextAttr *wxTextAttr_Merge1(const wxTextAttr * base, const wxTextAttr * overlay) {
    return new wxTextAttr(wxTextAttr::Merge(*base, *overlay));
}

// CLASS: wxTextCompleterSimple
void wxTextCompleterSimple_delete(wxTextCompleterSimple *self) {
    delete self;
}
void wxTextCompleterSimple_GetCompletions(wxTextCompleterSimple * self, const wxString * prefix, wxArrayString * res) {
    return self->GetCompletions(*prefix, *res);
}

// CLASS: wxTextCtrl
wxClassInfo *wxTextCtrl_CLASSINFO() {
    return wxCLASSINFO(wxTextCtrl);
}
#ifdef __WXOSX__
void wxTextCtrl_OSXEnableNewLineReplacement(wxTextCtrl * self, bool enable) {
    return self->OSXEnableNewLineReplacement(enable);
}
#endif
wxTextCtrl *wxTextCtrl_new() {
    return new wxTextCtrl();
}
wxTextCtrl *wxTextCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxTextCtrl(parent, id, *value, *pos, *size, style, *validator, *name);
}
bool wxTextCtrl_Create(wxTextCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, *validator, *name);
}
void wxTextCtrl_DiscardEdits(wxTextCtrl * self) {
    return self->DiscardEdits();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxTextCtrl_EmptyUndoBuffer(wxTextCtrl * self) {
    return self->EmptyUndoBuffer();
}
#endif
bool wxTextCtrl_EmulateKeyPress(wxTextCtrl * self, const wxKeyEvent * event) {
    return self->EmulateKeyPress(*event);
}
#ifndef __WXGTK__
bool wxTextCtrl_EnableProofCheck(wxTextCtrl * self, const wxTextProofOptions * options) {
    return self->EnableProofCheck(*options);
}
#endif
wxTextAttr *wxTextCtrl_GetDefaultStyle(const wxTextCtrl * self) {
    return new wxTextAttr(self->GetDefaultStyle());
}
int wxTextCtrl_GetLineLength(const wxTextCtrl * self, long line_no) {
    return self->GetLineLength(line_no);
}
wxString *wxTextCtrl_GetLineText(const wxTextCtrl * self, long line_no) {
    return new wxString(self->GetLineText(line_no));
}
int wxTextCtrl_GetNumberOfLines(const wxTextCtrl * self) {
    return self->GetNumberOfLines();
}
bool wxTextCtrl_GetStyle(wxTextCtrl * self, long position, wxTextAttr * style) {
    return self->GetStyle(position, *style);
}
bool wxTextCtrl_IsModified(const wxTextCtrl * self) {
    return self->IsModified();
}
bool wxTextCtrl_IsMultiLine(const wxTextCtrl * self) {
    return self->IsMultiLine();
}
bool wxTextCtrl_IsSingleLine(const wxTextCtrl * self) {
    return self->IsSingleLine();
}
wxTextProofOptions *wxTextCtrl_GetProofCheckOptions(wxTextCtrl * self) {
    return new wxTextProofOptions(self->GetProofCheckOptions());
}
bool wxTextCtrl_LoadFile(wxTextCtrl * self, const wxString * filename, int file_type) {
    return self->LoadFile(*filename, file_type);
}
void wxTextCtrl_MarkDirty(wxTextCtrl * self) {
    return self->MarkDirty();
}
void wxTextCtrl_OnDropFiles(wxTextCtrl * self, wxDropFilesEvent * event) {
    return self->OnDropFiles(*event);
}
bool wxTextCtrl_PositionToXY(const wxTextCtrl * self, long pos, long * x, long * y) {
    return self->PositionToXY(pos, x, y);
}
wxPoint *wxTextCtrl_PositionToCoords(const wxTextCtrl * self, long pos) {
    return new wxPoint(self->PositionToCoords(pos));
}
bool wxTextCtrl_SaveFile(wxTextCtrl * self, const wxString * filename, int file_type) {
    return self->SaveFile(*filename, file_type);
}
bool wxTextCtrl_SetDefaultStyle(wxTextCtrl * self, const wxTextAttr * style) {
    return self->SetDefaultStyle(*style);
}
void wxTextCtrl_SetModified(wxTextCtrl * self, bool modified) {
    return self->SetModified(modified);
}
bool wxTextCtrl_SetStyle(wxTextCtrl * self, long start, long end, const wxTextAttr * style) {
    return self->SetStyle(start, end, *style);
}
void wxTextCtrl_ShowPosition(wxTextCtrl * self, long pos) {
    return self->ShowPosition(pos);
}
long wxTextCtrl_XYToPosition(const wxTextCtrl * self, long x, long y) {
    return self->XYToPosition(x, y);
}
// Mix-in(s) to wxTextCtrl
wxTextEntryBase *wxTextCtrl_AsTextEntry(wxTextCtrl* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}
wxTrackable *wxTextCtrl_AsTrackable(wxTextCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTextDataObject
void wxTextDataObject_delete(wxTextDataObject *self) {
    delete self;
}
wxTextDataObject *wxTextDataObject_new(const wxString * text) {
    return new wxTextDataObject(*text);
}
wxString *wxTextDataObject_GetText(const wxTextDataObject * self) {
    return new wxString(self->GetText());
}
size_t wxTextDataObject_GetTextLength(const wxTextDataObject * self) {
    return self->GetTextLength();
}
const wxDataFormat * wxTextDataObject_GetFormat(const wxTextDataObject * self) {
    return self->GetFormat();
}
void wxTextDataObject_SetText(wxTextDataObject * self, const wxString * str_text) {
    return self->SetText(*str_text);
}

// CLASS: wxTextDropTarget
void wxTextDropTarget_delete(wxTextDropTarget *self) {
    delete self;
}
wxTextDropTarget *wxTextDropTarget_new() {
    return new wxTextDropTarget();
}
bool wxTextDropTarget_OnDropText(wxTextDropTarget * self, wxCoord x, wxCoord y, const wxString * data) {
    return self->OnDropText(x, y, *data);
}

// CLASS: wxTextEntry
void wxTextEntry_delete(wxTextEntry *self) {
    delete self;
}
void wxTextEntry_AppendText(wxTextEntry * self, const wxString * text) {
    return self->AppendText(*text);
}
bool wxTextEntry_AutoComplete(wxTextEntry * self, const wxArrayString * choices) {
    return self->AutoComplete(*choices);
}
bool wxTextEntry_AutoComplete1(wxTextEntry * self, wxTextCompleter * completer) {
    return self->AutoComplete(completer);
}
bool wxTextEntry_AutoCompleteFileNames(wxTextEntry * self) {
    return self->AutoCompleteFileNames();
}
bool wxTextEntry_AutoCompleteDirectories(wxTextEntry * self) {
    return self->AutoCompleteDirectories();
}
bool wxTextEntry_CanCopy(const wxTextEntry * self) {
    return self->CanCopy();
}
bool wxTextEntry_CanCut(const wxTextEntry * self) {
    return self->CanCut();
}
bool wxTextEntry_CanPaste(const wxTextEntry * self) {
    return self->CanPaste();
}
bool wxTextEntry_CanRedo(const wxTextEntry * self) {
    return self->CanRedo();
}
bool wxTextEntry_CanUndo(const wxTextEntry * self) {
    return self->CanUndo();
}
void wxTextEntry_ChangeValue(wxTextEntry * self, const wxString * value) {
    return self->ChangeValue(*value);
}
void wxTextEntry_Clear(wxTextEntry * self) {
    return self->Clear();
}
void wxTextEntry_Copy(wxTextEntry * self) {
    return self->Copy();
}
void wxTextEntry_Cut(wxTextEntry * self) {
    return self->Cut();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxTextEntry_ForceUpper(wxTextEntry * self) {
    return self->ForceUpper();
}
#endif
long wxTextEntry_GetInsertionPoint(const wxTextEntry * self) {
    return self->GetInsertionPoint();
}
wxString *wxTextEntry_GetRange(const wxTextEntry * self, long from, long to) {
    return new wxString(self->GetRange(from, to));
}
void wxTextEntry_GetSelection(const wxTextEntry * self, long * from, long * to) {
    return self->GetSelection(from, to);
}
wxString *wxTextEntry_GetStringSelection(const wxTextEntry * self) {
    return new wxString(self->GetStringSelection());
}
wxString *wxTextEntry_GetValue(const wxTextEntry * self) {
    return new wxString(self->GetValue());
}
bool wxTextEntry_IsEditable(const wxTextEntry * self) {
    return self->IsEditable();
}
bool wxTextEntry_IsEmpty(const wxTextEntry * self) {
    return self->IsEmpty();
}
void wxTextEntry_Paste(wxTextEntry * self) {
    return self->Paste();
}
void wxTextEntry_Redo(wxTextEntry * self) {
    return self->Redo();
}
void wxTextEntry_Remove(wxTextEntry * self, long from, long to) {
    return self->Remove(from, to);
}
void wxTextEntry_Replace(wxTextEntry * self, long from, long to, const wxString * value) {
    return self->Replace(from, to, *value);
}
void wxTextEntry_SetEditable(wxTextEntry * self, bool editable) {
    return self->SetEditable(editable);
}
void wxTextEntry_SetInsertionPoint(wxTextEntry * self, long pos) {
    return self->SetInsertionPoint(pos);
}
void wxTextEntry_SetInsertionPointEnd(wxTextEntry * self) {
    return self->SetInsertionPointEnd();
}
void wxTextEntry_SetSelection(wxTextEntry * self, long from, long to) {
    return self->SetSelection(from, to);
}
void wxTextEntry_SelectAll(wxTextEntry * self) {
    return self->SelectAll();
}
void wxTextEntry_SelectNone(wxTextEntry * self) {
    return self->SelectNone();
}
bool wxTextEntry_SetHint(wxTextEntry * self, const wxString * hint) {
    return self->SetHint(*hint);
}
wxString *wxTextEntry_GetHint(const wxTextEntry * self) {
    return new wxString(self->GetHint());
}
bool wxTextEntry_SetMargins(wxTextEntry * self, const wxPoint * pt) {
    return self->SetMargins(*pt);
}
bool wxTextEntry_SetMargins1(wxTextEntry * self, wxCoord left, wxCoord top) {
    return self->SetMargins(left, top);
}
wxPoint *wxTextEntry_GetMargins(const wxTextEntry * self) {
    return new wxPoint(self->GetMargins());
}
void wxTextEntry_SetValue(wxTextEntry * self, const wxString * value) {
    return self->SetValue(*value);
}
void wxTextEntry_Undo(wxTextEntry * self) {
    return self->Undo();
}
void wxTextEntry_WriteText(wxTextEntry * self, const wxString * text) {
    return self->WriteText(*text);
}

// CLASS: wxTextEntryDialog
wxClassInfo *wxTextEntryDialog_CLASSINFO() {
    return wxCLASSINFO(wxTextEntryDialog);
}
wxTextEntryDialog *wxTextEntryDialog_new() {
    return new wxTextEntryDialog();
}
wxTextEntryDialog *wxTextEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * caption, const wxString * value, long style, const wxPoint * pos) {
    return new wxTextEntryDialog(parent, *message, *caption, *value, style, *pos);
}
bool wxTextEntryDialog_Create(wxTextEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * caption, const wxString * value, long style, const wxPoint * pos) {
    return self->Create(parent, *message, *caption, *value, style, *pos);
}
wxString *wxTextEntryDialog_GetValue(const wxTextEntryDialog * self) {
    return new wxString(self->GetValue());
}
void wxTextEntryDialog_SetTextValidator(wxTextEntryDialog * self, const wxTextValidator * validator) {
    return self->SetTextValidator(*validator);
}
void wxTextEntryDialog_SetValue(wxTextEntryDialog * self, const wxString * value) {
    return self->SetValue(*value);
}
void wxTextEntryDialog_ForceUpper(wxTextEntryDialog * self) {
    return self->ForceUpper();
}
// Mix-in(s) to wxTextEntryDialog
wxTrackable *wxTextEntryDialog_AsTrackable(wxTextEntryDialog* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTextValidator
wxClassInfo *wxTextValidator_CLASSINFO() {
    return wxCLASSINFO(wxTextValidator);
}
wxTextValidator *wxTextValidator_new(const wxTextValidator * validator) {
    return new wxTextValidator(*validator);
}
wxTextValidator *wxTextValidator_new1(long style, wxString * val_ptr) {
    return new wxTextValidator(style, val_ptr);
}
wxString *wxTextValidator_GetCharExcludes(const wxTextValidator * self) {
    return new wxString(self->GetCharExcludes());
}
wxString *wxTextValidator_GetCharIncludes(const wxTextValidator * self) {
    return new wxString(self->GetCharIncludes());
}
wxArrayString *wxTextValidator_GetExcludes(const wxTextValidator * self) {
    return new wxArrayString(self->GetExcludes());
}
wxArrayString *wxTextValidator_GetIncludes(const wxTextValidator * self) {
    return new wxArrayString(self->GetIncludes());
}
long wxTextValidator_GetStyle(const wxTextValidator * self) {
    return self->GetStyle();
}
void wxTextValidator_OnChar(wxTextValidator * self, wxKeyEvent * event) {
    return self->OnChar(*event);
}
void wxTextValidator_SetExcludes(wxTextValidator * self, const wxArrayString * string_list) {
    return self->SetExcludes(*string_list);
}
void wxTextValidator_SetCharExcludes(wxTextValidator * self, const wxString * chars) {
    return self->SetCharExcludes(*chars);
}
void wxTextValidator_SetIncludes(wxTextValidator * self, const wxArrayString * string_list) {
    return self->SetIncludes(*string_list);
}
void wxTextValidator_SetCharIncludes(wxTextValidator * self, const wxString * chars) {
    return self->SetCharIncludes(*chars);
}
void wxTextValidator_AddExclude(wxTextValidator * self, const wxString * exclude) {
    return self->AddExclude(*exclude);
}
void wxTextValidator_AddInclude(wxTextValidator * self, const wxString * include) {
    return self->AddInclude(*include);
}
void wxTextValidator_AddCharExcludes(wxTextValidator * self, const wxString * chars) {
    return self->AddCharExcludes(*chars);
}
void wxTextValidator_AddCharIncludes(wxTextValidator * self, const wxString * chars) {
    return self->AddCharIncludes(*chars);
}
void wxTextValidator_SetStyle(wxTextValidator * self, long style) {
    return self->SetStyle(style);
}
wxString *wxTextValidator_IsValid(const wxTextValidator * self, const wxString * val) {
    return new wxString(self->IsValid(*val));
}
// Mix-in(s) to wxTextValidator
wxTrackable *wxTextValidator_AsTrackable(wxTextValidator* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxThreadEvent
wxClassInfo *wxThreadEvent_CLASSINFO() {
    return wxCLASSINFO(wxThreadEvent);
}
void wxThreadEvent_SetPayload(wxThreadEvent * self, const T * payload) {
    return self->SetPayload(*payload);
}
long wxThreadEvent_GetExtraLong(const wxThreadEvent * self) {
    return self->GetExtraLong();
}
int wxThreadEvent_GetInt(const wxThreadEvent * self) {
    return self->GetInt();
}
wxString *wxThreadEvent_GetString(const wxThreadEvent * self) {
    return new wxString(self->GetString());
}
void wxThreadEvent_SetExtraLong(wxThreadEvent * self, long extra_long) {
    return self->SetExtraLong(extra_long);
}
void wxThreadEvent_SetInt(wxThreadEvent * self, int int_command) {
    return self->SetInt(int_command);
}
void wxThreadEvent_SetString(wxThreadEvent * self, const wxString * string) {
    return self->SetString(*string);
}

// CLASS: wxThumbBarButton
void wxThumbBarButton_delete(wxThumbBarButton *self) {
    delete self;
}
wxThumbBarButton *wxThumbBarButton_new() {
    return new wxThumbBarButton();
}
wxThumbBarButton *wxThumbBarButton_new1(int id, const wxIcon * icon, const wxString * tooltip, bool enable, bool dismiss_on_click, bool has_background, bool shown, bool interactive) {
    return new wxThumbBarButton(id, *icon, *tooltip, enable, dismiss_on_click, has_background, shown, interactive);
}
bool wxThumbBarButton_Create(wxThumbBarButton * self, int id, const wxIcon * icon, const wxString * tooltip, bool enable, bool dismiss_on_click, bool has_background, bool shown, bool interactive) {
    return self->Create(id, *icon, *tooltip, enable, dismiss_on_click, has_background, shown, interactive);
}
int wxThumbBarButton_GetID(const wxThumbBarButton * self) {
    return self->GetID();
}
wxIcon *wxThumbBarButton_GetIcon(const wxThumbBarButton * self) {
    return new wxIcon(self->GetIcon());
}
wxString *wxThumbBarButton_GetTooltip(const wxThumbBarButton * self) {
    return new wxString(self->GetTooltip());
}
bool wxThumbBarButton_IsEnable(const wxThumbBarButton * self) {
    return self->IsEnable();
}
void wxThumbBarButton_Enable(wxThumbBarButton * self, bool enable) {
    return self->Enable(enable);
}
void wxThumbBarButton_Disable(wxThumbBarButton * self) {
    return self->Disable();
}
bool wxThumbBarButton_IsDismissOnClick(const wxThumbBarButton * self) {
    return self->IsDismissOnClick();
}
void wxThumbBarButton_EnableDismissOnClick(wxThumbBarButton * self, bool enable) {
    return self->EnableDismissOnClick(enable);
}
void wxThumbBarButton_DisableDimissOnClick(wxThumbBarButton * self) {
    return self->DisableDimissOnClick();
}
bool wxThumbBarButton_HasBackground(const wxThumbBarButton * self) {
    return self->HasBackground();
}
void wxThumbBarButton_SetHasBackground(wxThumbBarButton * self, bool has) {
    return self->SetHasBackground(has);
}
bool wxThumbBarButton_IsShown(const wxThumbBarButton * self) {
    return self->IsShown();
}
void wxThumbBarButton_Show(wxThumbBarButton * self, bool shown) {
    return self->Show(shown);
}
void wxThumbBarButton_Hide(wxThumbBarButton * self) {
    return self->Hide();
}
bool wxThumbBarButton_IsInteractive(const wxThumbBarButton * self) {
    return self->IsInteractive();
}
void wxThumbBarButton_SetInteractive(wxThumbBarButton * self, bool interactive) {
    return self->SetInteractive(interactive);
}

// CLASS: wxTimePickerCtrl
wxClassInfo *wxTimePickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxTimePickerCtrl);
}
wxTimePickerCtrl *wxTimePickerCtrl_new() {
    return new wxTimePickerCtrl();
}
wxTimePickerCtrl *wxTimePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxTimePickerCtrl(parent, id, *dt, *pos, *size, style, *validator, *name);
}
bool wxTimePickerCtrl_Create(wxTimePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *dt, *pos, *size, style, *validator, *name);
}
bool wxTimePickerCtrl_GetTime(const wxTimePickerCtrl * self, int * hour, int * min, int * sec) {
    return self->GetTime(hour, min, sec);
}
wxDateTime *wxTimePickerCtrl_GetValue(const wxTimePickerCtrl * self) {
    return new wxDateTime(self->GetValue());
}
bool wxTimePickerCtrl_SetTime(wxTimePickerCtrl * self, int hour, int min, int sec) {
    return self->SetTime(hour, min, sec);
}
void wxTimePickerCtrl_SetValue(wxTimePickerCtrl * self, const wxDateTime * dt) {
    return self->SetValue(*dt);
}
// Mix-in(s) to wxTimePickerCtrl
wxTrackable *wxTimePickerCtrl_AsTrackable(wxTimePickerCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTipProvider
void wxTipProvider_delete(wxTipProvider *self) {
    delete self;
}
wxTipProvider *wxTipProvider_new(size_t current_tip) {
    return new wxTipProvider(current_tip);
}
size_t wxTipProvider_GetCurrentTip(const wxTipProvider * self) {
    return self->GetCurrentTip();
}
wxString *wxTipProvider_GetTip(wxTipProvider * self) {
    return new wxString(self->GetTip());
}

// CLASS: wxTipWindow
wxClassInfo *wxTipWindow_CLASSINFO() {
    return wxCLASSINFO(wxTipWindow);
}
wxTipWindow *wxTipWindow_new(wxWindow * parent, const wxString * text, wxCoord max_length, wxTipWindow ** window_ptr, wxRect * rect_bounds) {
    return new wxTipWindow(parent, *text, max_length, window_ptr, rect_bounds);
}
void wxTipWindow_SetBoundingRect(wxTipWindow * self, const wxRect * rect_bound) {
    return self->SetBoundingRect(*rect_bound);
}
void wxTipWindow_SetTipWindowPtr(wxTipWindow * self, wxTipWindow ** window_ptr) {
    return self->SetTipWindowPtr(window_ptr);
}
// Mix-in(s) to wxTipWindow
wxTrackable *wxTipWindow_AsTrackable(wxTipWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxToggleButton
wxClassInfo *wxToggleButton_CLASSINFO() {
    return wxCLASSINFO(wxToggleButton);
}
wxToggleButton *wxToggleButton_new() {
    return new wxToggleButton();
}
wxToggleButton *wxToggleButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name) {
    return new wxToggleButton(parent, id, *label, *pos, *size, style, *val, *name);
}
bool wxToggleButton_Create(wxToggleButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *val, *name);
}
bool wxToggleButton_GetValue(const wxToggleButton * self) {
    return self->GetValue();
}
void wxToggleButton_SetValue(wxToggleButton * self, bool state) {
    return self->SetValue(state);
}
// Mix-in(s) to wxToggleButton
wxTrackable *wxToggleButton_AsTrackable(wxToggleButton* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxToolBar
wxClassInfo *wxToolBar_CLASSINFO() {
    return wxCLASSINFO(wxToolBar);
}
wxToolBar *wxToolBar_new() {
    return new wxToolBar();
}
wxToolBar *wxToolBar_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxToolBar(parent, id, *pos, *size, style, *name);
}
wxToolBarToolBase * wxToolBar_AddCheckTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap1, const wxBitmapBundle * bmp_disabled, const wxString * short_help, const wxString * long_help, wxObject * client_data) {
    return self->AddCheckTool(tool_id, *label, *bitmap1, *bmp_disabled, *short_help, *long_help, client_data);
}
wxToolBarToolBase * wxToolBar_AddControl(wxToolBar * self, wxControl * control, const wxString * label) {
    return self->AddControl(control, *label);
}
wxToolBarToolBase * wxToolBar_AddRadioTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap1, const wxBitmapBundle * bmp_disabled, const wxString * short_help, const wxString * long_help, wxObject * client_data) {
    return self->AddRadioTool(tool_id, *label, *bitmap1, *bmp_disabled, *short_help, *long_help, client_data);
}
wxToolBarToolBase * wxToolBar_AddSeparator(wxToolBar * self) {
    return self->AddSeparator();
}
wxToolBarToolBase * wxToolBar_AddStretchableSpace(wxToolBar * self) {
    return self->AddStretchableSpace();
}
wxToolBarToolBase * wxToolBar_AddTool(wxToolBar * self, wxToolBarToolBase * tool) {
    return self->AddTool(tool);
}
wxToolBarToolBase * wxToolBar_AddTool1(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxString * short_help, wxItemKind kind) {
    return self->AddTool(tool_id, *label, *bitmap, *short_help, kind);
}
wxToolBarToolBase * wxToolBar_AddTool2(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxBitmapBundle * bmp_disabled, wxItemKind kind, const wxString * short_help, const wxString * long_help, wxObject * client_data) {
    return self->AddTool(tool_id, *label, *bitmap, *bmp_disabled, kind, *short_help, *long_help, client_data);
}
void wxToolBar_ClearTools(wxToolBar * self) {
    return self->ClearTools();
}
bool wxToolBar_DeleteTool(wxToolBar * self, int tool_id) {
    return self->DeleteTool(tool_id);
}
bool wxToolBar_DeleteToolByPos(wxToolBar * self, size_t pos) {
    return self->DeleteToolByPos(pos);
}
void wxToolBar_EnableTool(wxToolBar * self, int tool_id, bool enable) {
    return self->EnableTool(tool_id, enable);
}
wxToolBarToolBase * wxToolBar_FindById(const wxToolBar * self, int id) {
    return self->FindById(id);
}
wxControl * wxToolBar_FindControl(wxToolBar * self, int id) {
    return self->FindControl(id);
}
wxToolBarToolBase * wxToolBar_FindToolForPosition(const wxToolBar * self, wxCoord x, wxCoord y) {
    return self->FindToolForPosition(x, y);
}
wxSize *wxToolBar_GetMargins(const wxToolBar * self) {
    return new wxSize(self->GetMargins());
}
wxSize *wxToolBar_GetToolBitmapSize(const wxToolBar * self) {
    return new wxSize(self->GetToolBitmapSize());
}
const wxToolBarToolBase * wxToolBar_GetToolByPos1(const wxToolBar * self, int pos) {
    return self->GetToolByPos(pos);
}
wxObject * wxToolBar_GetToolClientData(const wxToolBar * self, int tool_id) {
    return self->GetToolClientData(tool_id);
}
bool wxToolBar_GetToolEnabled(const wxToolBar * self, int tool_id) {
    return self->GetToolEnabled(tool_id);
}
wxString *wxToolBar_GetToolLongHelp(const wxToolBar * self, int tool_id) {
    return new wxString(self->GetToolLongHelp(tool_id));
}
int wxToolBar_GetToolPacking(const wxToolBar * self) {
    return self->GetToolPacking();
}
int wxToolBar_GetToolPos(const wxToolBar * self, int tool_id) {
    return self->GetToolPos(tool_id);
}
int wxToolBar_GetToolSeparation(const wxToolBar * self) {
    return self->GetToolSeparation();
}
wxString *wxToolBar_GetToolShortHelp(const wxToolBar * self, int tool_id) {
    return new wxString(self->GetToolShortHelp(tool_id));
}
wxSize *wxToolBar_GetToolSize(const wxToolBar * self) {
    return new wxSize(self->GetToolSize());
}
bool wxToolBar_GetToolState(const wxToolBar * self, int tool_id) {
    return self->GetToolState(tool_id);
}
size_t wxToolBar_GetToolsCount(const wxToolBar * self) {
    return self->GetToolsCount();
}
wxToolBarToolBase * wxToolBar_InsertControl(wxToolBar * self, size_t pos, wxControl * control, const wxString * label) {
    return self->InsertControl(pos, control, *label);
}
wxToolBarToolBase * wxToolBar_InsertSeparator(wxToolBar * self, size_t pos) {
    return self->InsertSeparator(pos);
}
wxToolBarToolBase * wxToolBar_InsertStretchableSpace(wxToolBar * self, size_t pos) {
    return self->InsertStretchableSpace(pos);
}
wxToolBarToolBase * wxToolBar_InsertTool(wxToolBar * self, size_t pos, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxBitmapBundle * bmp_disabled, wxItemKind kind, const wxString * short_help, const wxString * long_help, wxObject * client_data) {
    return self->InsertTool(pos, tool_id, *label, *bitmap, *bmp_disabled, kind, *short_help, *long_help, client_data);
}
wxToolBarToolBase * wxToolBar_InsertTool1(wxToolBar * self, size_t pos, wxToolBarToolBase * tool) {
    return self->InsertTool(pos, tool);
}
bool wxToolBar_OnLeftClick(wxToolBar * self, int tool_id, bool toggle_down) {
    return self->OnLeftClick(tool_id, toggle_down);
}
void wxToolBar_OnMouseEnter(wxToolBar * self, int tool_id) {
    return self->OnMouseEnter(tool_id);
}
void wxToolBar_OnRightClick(wxToolBar * self, int tool_id, long x, long y) {
    return self->OnRightClick(tool_id, x, y);
}
bool wxToolBar_Realize(wxToolBar * self) {
    return self->Realize();
}
wxToolBarToolBase * wxToolBar_RemoveTool(wxToolBar * self, int id) {
    return self->RemoveTool(id);
}
bool wxToolBar_SetDropdownMenu(wxToolBar * self, int id, wxMenu * menu) {
    return self->SetDropdownMenu(id, menu);
}
void wxToolBar_SetMargins(wxToolBar * self, int x, int y) {
    return self->SetMargins(x, y);
}
void wxToolBar_SetMargins1(wxToolBar * self, const wxSize * size) {
    return self->SetMargins(*size);
}
void wxToolBar_SetToolBitmapSize(wxToolBar * self, const wxSize * size) {
    return self->SetToolBitmapSize(*size);
}
void wxToolBar_SetToolClientData(wxToolBar * self, int id, wxObject * client_data) {
    return self->SetToolClientData(id, client_data);
}
void wxToolBar_SetToolDisabledBitmap(wxToolBar * self, int id, const wxBitmapBundle * bitmap) {
    return self->SetToolDisabledBitmap(id, *bitmap);
}
void wxToolBar_SetToolLongHelp(wxToolBar * self, int tool_id, const wxString * help_string) {
    return self->SetToolLongHelp(tool_id, *help_string);
}
void wxToolBar_SetToolNormalBitmap(wxToolBar * self, int id, const wxBitmapBundle * bitmap) {
    return self->SetToolNormalBitmap(id, *bitmap);
}
void wxToolBar_SetToolPacking(wxToolBar * self, int packing) {
    return self->SetToolPacking(packing);
}
void wxToolBar_SetToolSeparation(wxToolBar * self, int separation) {
    return self->SetToolSeparation(separation);
}
void wxToolBar_SetToolShortHelp(wxToolBar * self, int tool_id, const wxString * help_string) {
    return self->SetToolShortHelp(tool_id, *help_string);
}
void wxToolBar_ToggleTool(wxToolBar * self, int tool_id, bool toggle) {
    return self->ToggleTool(tool_id, toggle);
}
wxToolBarToolBase * wxToolBar_CreateTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bmp_normal, const wxBitmapBundle * bmp_disabled, wxItemKind kind, wxObject * client_data, const wxString * short_help, const wxString * long_help) {
    return self->CreateTool(tool_id, *label, *bmp_normal, *bmp_disabled, kind, client_data, *short_help, *long_help);
}
wxToolBarToolBase * wxToolBar_CreateTool1(wxToolBar * self, wxControl * control, const wxString * label) {
    return self->CreateTool(control, *label);
}
wxToolBarToolBase * wxToolBar_CreateSeparator(wxToolBar * self) {
    return self->CreateSeparator();
}
// Mix-in(s) to wxToolBar
wxTrackable *wxToolBar_AsTrackable(wxToolBar* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxToolTip
wxClassInfo *wxToolTip_CLASSINFO() {
    return wxCLASSINFO(wxToolTip);
}
wxToolTip *wxToolTip_new(const wxString * tip) {
    return new wxToolTip(*tip);
}
wxString *wxToolTip_GetTip(const wxToolTip * self) {
    return new wxString(self->GetTip());
}
wxWindow * wxToolTip_GetWindow(const wxToolTip * self) {
    return self->GetWindow();
}
void wxToolTip_SetTip(wxToolTip * self, const wxString * tip) {
    return self->SetTip(*tip);
}
void wxToolTip_Enable(bool flag) {
    return wxToolTip::Enable(flag);
}
void wxToolTip_SetAutoPop(long msecs) {
    return wxToolTip::SetAutoPop(msecs);
}
void wxToolTip_SetDelay(long msecs) {
    return wxToolTip::SetDelay(msecs);
}
void wxToolTip_SetMaxWidth(int width) {
    return wxToolTip::SetMaxWidth(width);
}
void wxToolTip_SetReshow(long msecs) {
    return wxToolTip::SetReshow(msecs);
}

// CLASS: wxToolbook
wxClassInfo *wxToolbook_CLASSINFO() {
    return wxCLASSINFO(wxToolbook);
}
wxToolbook *wxToolbook_new() {
    return new wxToolbook();
}
wxToolbook *wxToolbook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxToolbook(parent, id, *pos, *size, style, *name);
}
bool wxToolbook_Create(wxToolbook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
wxToolBarBase * wxToolbook_GetToolBar(const wxToolbook * self) {
    return self->GetToolBar();
}
bool wxToolbook_EnablePage(wxToolbook * self, size_t page, bool enable) {
    return self->EnablePage(page, enable);
}
bool wxToolbook_EnablePage1(wxToolbook * self, wxWindow * page, bool enable) {
    return self->EnablePage(page, enable);
}
// Mix-in(s) to wxToolbook
wxWithImages *wxToolbook_AsWithImages(wxToolbook* obj) {
    return static_cast<wxWithImages*>(obj);
}
wxTrackable *wxToolbook_AsTrackable(wxToolbook* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTopLevelWindow
wxClassInfo *wxTopLevelWindow_CLASSINFO() {
    return wxCLASSINFO(wxTopLevelWindow);
}
wxTopLevelWindow *wxTopLevelWindow_new() {
    return new wxTopLevelWindow();
}
wxTopLevelWindow *wxTopLevelWindow_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxTopLevelWindow(parent, id, *title, *pos, *size, style, *name);
}
bool wxTopLevelWindow_Create(wxTopLevelWindow * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
void wxTopLevelWindow_CenterOnScreen(wxTopLevelWindow * self, int direction) {
    return self->CenterOnScreen(direction);
}
void wxTopLevelWindow_CentreOnScreen(wxTopLevelWindow * self, int direction) {
    return self->CentreOnScreen(direction);
}
bool wxTopLevelWindow_EnableCloseButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableCloseButton(enable);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableMaximizeButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableMaximizeButton(enable);
}
bool wxTopLevelWindow_EnableMinimizeButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableMinimizeButton(enable);
}
#endif
wxWindow * wxTopLevelWindow_GetDefaultItem(const wxTopLevelWindow * self) {
    return self->GetDefaultItem();
}
wxIcon *wxTopLevelWindow_GetIcon(const wxTopLevelWindow * self) {
    return new wxIcon(self->GetIcon());
}
wxString *wxTopLevelWindow_GetTitle(const wxTopLevelWindow * self) {
    return new wxString(self->GetTitle());
}
void wxTopLevelWindow_Iconize(wxTopLevelWindow * self, bool iconize) {
    return self->Iconize(iconize);
}
bool wxTopLevelWindow_IsActive(wxTopLevelWindow * self) {
    return self->IsActive();
}
bool wxTopLevelWindow_IsAlwaysMaximized(const wxTopLevelWindow * self) {
    return self->IsAlwaysMaximized();
}
bool wxTopLevelWindow_IsFullScreen(const wxTopLevelWindow * self) {
    return self->IsFullScreen();
}
bool wxTopLevelWindow_IsIconized(const wxTopLevelWindow * self) {
    return self->IsIconized();
}
bool wxTopLevelWindow_IsMaximized(const wxTopLevelWindow * self) {
    return self->IsMaximized();
}
void wxTopLevelWindow_Maximize(wxTopLevelWindow * self, bool maximize) {
    return self->Maximize(maximize);
}
#ifdef __WXMSW__
wxMenu * wxTopLevelWindow_MSWGetSystemMenu(const wxTopLevelWindow * self) {
    return self->MSWGetSystemMenu();
}
#endif
void wxTopLevelWindow_RequestUserAttention(wxTopLevelWindow * self, int flags) {
    return self->RequestUserAttention(flags);
}
void wxTopLevelWindow_Restore(wxTopLevelWindow * self) {
    return self->Restore();
}
wxWindow * wxTopLevelWindow_SetDefaultItem(wxTopLevelWindow * self, wxWindow * win) {
    return self->SetDefaultItem(win);
}
wxWindow * wxTopLevelWindow_SetTmpDefaultItem(wxTopLevelWindow * self, wxWindow * win) {
    return self->SetTmpDefaultItem(win);
}
wxWindow * wxTopLevelWindow_GetTmpDefaultItem(const wxTopLevelWindow * self) {
    return self->GetTmpDefaultItem();
}
void wxTopLevelWindow_SetIcon(wxTopLevelWindow * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
void wxTopLevelWindow_SetIcons(wxTopLevelWindow * self, const wxIconBundle * icons) {
    return self->SetIcons(*icons);
}
void wxTopLevelWindow_SetTitle(wxTopLevelWindow * self, const wxString * title) {
    return self->SetTitle(*title);
}
bool wxTopLevelWindow_ShouldPreventAppExit(const wxTopLevelWindow * self) {
    return self->ShouldPreventAppExit();
}
void wxTopLevelWindow_OSXSetModified(wxTopLevelWindow * self, bool modified) {
    return self->OSXSetModified(modified);
}
bool wxTopLevelWindow_OSXIsModified(const wxTopLevelWindow * self) {
    return self->OSXIsModified();
}
void wxTopLevelWindow_SetRepresentedFilename(wxTopLevelWindow * self, const wxString * filename) {
    return self->SetRepresentedFilename(*filename);
}
void wxTopLevelWindow_ShowWithoutActivating(wxTopLevelWindow * self) {
    return self->ShowWithoutActivating();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableFullScreenView(wxTopLevelWindow * self, bool enable, long style) {
    return self->EnableFullScreenView(enable, style);
}
#endif
bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow * self, bool show, long style) {
    return self->ShowFullScreen(show, style);
}
wxSize *wxTopLevelWindow_GetDefaultSize() {
    return new wxSize(wxTopLevelWindow::GetDefaultSize());
}
// Mix-in(s) to wxTopLevelWindow
wxTrackable *wxTopLevelWindow_AsTrackable(wxTopLevelWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTreeCtrl
wxClassInfo *wxTreeCtrl_CLASSINFO() {
    return wxCLASSINFO(wxTreeCtrl);
}
wxTreeCtrl *wxTreeCtrl_new() {
    return new wxTreeCtrl();
}
wxTreeCtrl *wxTreeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxTreeCtrl(parent, id, *pos, *size, style, *validator, *name);
}
wxTreeItemId *wxTreeCtrl_AddRoot(wxTreeCtrl * self, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->AddRoot(*text, image, sel_image, data));
}
wxTreeItemId *wxTreeCtrl_AppendItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->AppendItem(*parent, *text, image, sel_image, data));
}
void wxTreeCtrl_AssignButtonsImageList(wxTreeCtrl * self, wxImageList * image_list) {
    return self->AssignButtonsImageList(image_list);
}
void wxTreeCtrl_AssignStateImageList(wxTreeCtrl * self, wxImageList * image_list) {
    return self->AssignStateImageList(image_list);
}
void wxTreeCtrl_Collapse(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->Collapse(*item);
}
void wxTreeCtrl_CollapseAll(wxTreeCtrl * self) {
    return self->CollapseAll();
}
void wxTreeCtrl_CollapseAllChildren(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->CollapseAllChildren(*item);
}
void wxTreeCtrl_CollapseAndReset(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->CollapseAndReset(*item);
}
bool wxTreeCtrl_Create(wxTreeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
void wxTreeCtrl_Delete(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->Delete(*item);
}
void wxTreeCtrl_DeleteAllItems(wxTreeCtrl * self) {
    return self->DeleteAllItems();
}
void wxTreeCtrl_DeleteChildren(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->DeleteChildren(*item);
}
wxTextCtrl * wxTreeCtrl_EditLabel(wxTreeCtrl * self, const wxTreeItemId * item, wxClassInfo * text_ctrl_class) {
    return self->EditLabel(*item, text_ctrl_class);
}
void wxTreeCtrl_EnableBellOnNoMatch(wxTreeCtrl * self, bool on) {
    return self->EnableBellOnNoMatch(on);
}
void wxTreeCtrl_EndEditLabel(wxTreeCtrl * self, const wxTreeItemId * item, bool discard_changes) {
    return self->EndEditLabel(*item, discard_changes);
}
void wxTreeCtrl_EnsureVisible(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->EnsureVisible(*item);
}
void wxTreeCtrl_Expand(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->Expand(*item);
}
void wxTreeCtrl_ExpandAll(wxTreeCtrl * self) {
    return self->ExpandAll();
}
void wxTreeCtrl_ExpandAllChildren(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->ExpandAllChildren(*item);
}
bool wxTreeCtrl_GetBoundingRect(const wxTreeCtrl * self, const wxTreeItemId * item, wxRect * rect, bool text_only) {
    return self->GetBoundingRect(*item, *rect, text_only);
}
wxImageList * wxTreeCtrl_GetButtonsImageList(const wxTreeCtrl * self) {
    return self->GetButtonsImageList();
}
size_t wxTreeCtrl_GetChildrenCount(const wxTreeCtrl * self, const wxTreeItemId * item, bool recursively) {
    return self->GetChildrenCount(*item, recursively);
}
unsigned int wxTreeCtrl_GetCount(const wxTreeCtrl * self) {
    return self->GetCount();
}
wxTextCtrl * wxTreeCtrl_GetEditControl(const wxTreeCtrl * self) {
    return self->GetEditControl();
}
wxTreeItemId *wxTreeCtrl_GetFirstChild(const wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemIdValue * cookie) {
    return new wxTreeItemId(self->GetFirstChild(*item, *cookie));
}
wxTreeItemId *wxTreeCtrl_GetFirstVisibleItem(const wxTreeCtrl * self) {
    return new wxTreeItemId(self->GetFirstVisibleItem());
}
wxTreeItemId *wxTreeCtrl_GetFocusedItem(const wxTreeCtrl * self) {
    return new wxTreeItemId(self->GetFocusedItem());
}
void wxTreeCtrl_ClearFocusedItem(wxTreeCtrl * self) {
    return self->ClearFocusedItem();
}
void wxTreeCtrl_SetFocusedItem(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->SetFocusedItem(*item);
}
unsigned int wxTreeCtrl_GetIndent(const wxTreeCtrl * self) {
    return self->GetIndent();
}
unsigned int wxTreeCtrl_GetSpacing(const wxTreeCtrl * self) {
    return self->GetSpacing();
}
wxColour *wxTreeCtrl_GetItemBackgroundColour(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxColour(self->GetItemBackgroundColour(*item));
}
wxTreeItemData * wxTreeCtrl_GetItemData(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->GetItemData(*item);
}
wxFont *wxTreeCtrl_GetItemFont(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxFont(self->GetItemFont(*item));
}
wxTreeItemId *wxTreeCtrl_GetItemParent(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetItemParent(*item));
}
int wxTreeCtrl_GetItemState(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->GetItemState(*item);
}
wxString *wxTreeCtrl_GetItemText(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxString(self->GetItemText(*item));
}
wxColour *wxTreeCtrl_GetItemTextColour(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxColour(self->GetItemTextColour(*item));
}
wxTreeItemId *wxTreeCtrl_GetLastChild(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetLastChild(*item));
}
wxTreeItemId *wxTreeCtrl_GetNextChild(const wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemIdValue * cookie) {
    return new wxTreeItemId(self->GetNextChild(*item, *cookie));
}
wxTreeItemId *wxTreeCtrl_GetNextSibling(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetNextSibling(*item));
}
wxTreeItemId *wxTreeCtrl_GetNextVisible(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetNextVisible(*item));
}
wxTreeItemId *wxTreeCtrl_GetPrevSibling(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetPrevSibling(*item));
}
wxTreeItemId *wxTreeCtrl_GetPrevVisible(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetPrevVisible(*item));
}
bool wxTreeCtrl_GetQuickBestSize(const wxTreeCtrl * self) {
    return self->GetQuickBestSize();
}
wxTreeItemId *wxTreeCtrl_GetRootItem(const wxTreeCtrl * self) {
    return new wxTreeItemId(self->GetRootItem());
}
wxTreeItemId *wxTreeCtrl_GetSelection(const wxTreeCtrl * self) {
    return new wxTreeItemId(self->GetSelection());
}
size_t wxTreeCtrl_GetSelections(const wxTreeCtrl * self, wxArrayTreeItemIds * selection) {
    return self->GetSelections(*selection);
}
wxImageList * wxTreeCtrl_GetStateImageList(const wxTreeCtrl * self) {
    return self->GetStateImageList();
}
wxTreeItemId *wxTreeCtrl_HitTest(const wxTreeCtrl * self, const wxPoint * point, int * flags) {
    return new wxTreeItemId(self->HitTest(*point, *flags));
}
wxTreeItemId *wxTreeCtrl_InsertItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxTreeItemId * previous, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->InsertItem(*parent, *previous, *text, image, sel_image, data));
}
wxTreeItemId *wxTreeCtrl_InsertItem1(wxTreeCtrl * self, const wxTreeItemId * parent, size_t pos, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->InsertItem(*parent, pos, *text, image, sel_image, data));
}
bool wxTreeCtrl_IsBold(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->IsBold(*item);
}
bool wxTreeCtrl_IsEmpty(const wxTreeCtrl * self) {
    return self->IsEmpty();
}
bool wxTreeCtrl_IsExpanded(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->IsExpanded(*item);
}
bool wxTreeCtrl_IsSelected(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->IsSelected(*item);
}
bool wxTreeCtrl_IsVisible(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->IsVisible(*item);
}
bool wxTreeCtrl_ItemHasChildren(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->ItemHasChildren(*item);
}
int wxTreeCtrl_OnCompareItems(wxTreeCtrl * self, const wxTreeItemId * item1, const wxTreeItemId * item2) {
    return self->OnCompareItems(*item1, *item2);
}
wxTreeItemId *wxTreeCtrl_PrependItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->PrependItem(*parent, *text, image, sel_image, data));
}
void wxTreeCtrl_ScrollTo(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->ScrollTo(*item);
}
void wxTreeCtrl_SelectItem(wxTreeCtrl * self, const wxTreeItemId * item, bool select) {
    return self->SelectItem(*item, select);
}
void wxTreeCtrl_SetButtonsImageList(wxTreeCtrl * self, wxImageList * image_list) {
    return self->SetButtonsImageList(image_list);
}
void wxTreeCtrl_SetIndent(wxTreeCtrl * self, unsigned int indent) {
    return self->SetIndent(indent);
}
void wxTreeCtrl_SetSpacing(wxTreeCtrl * self, unsigned int spacing) {
    return self->SetSpacing(spacing);
}
void wxTreeCtrl_SetItemBackgroundColour(wxTreeCtrl * self, const wxTreeItemId * item, const wxColour * col) {
    return self->SetItemBackgroundColour(*item, *col);
}
void wxTreeCtrl_SetItemBold(wxTreeCtrl * self, const wxTreeItemId * item, bool bold) {
    return self->SetItemBold(*item, bold);
}
void wxTreeCtrl_SetItemData(wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemData * data) {
    return self->SetItemData(*item, data);
}
void wxTreeCtrl_SetItemDropHighlight(wxTreeCtrl * self, const wxTreeItemId * item, bool highlight) {
    return self->SetItemDropHighlight(*item, highlight);
}
void wxTreeCtrl_SetItemFont(wxTreeCtrl * self, const wxTreeItemId * item, const wxFont * font) {
    return self->SetItemFont(*item, *font);
}
void wxTreeCtrl_SetItemHasChildren(wxTreeCtrl * self, const wxTreeItemId * item, bool has_children) {
    return self->SetItemHasChildren(*item, has_children);
}
void wxTreeCtrl_SetItemState(wxTreeCtrl * self, const wxTreeItemId * item, int state) {
    return self->SetItemState(*item, state);
}
void wxTreeCtrl_SetItemText(wxTreeCtrl * self, const wxTreeItemId * item, const wxString * text) {
    return self->SetItemText(*item, *text);
}
void wxTreeCtrl_SetItemTextColour(wxTreeCtrl * self, const wxTreeItemId * item, const wxColour * col) {
    return self->SetItemTextColour(*item, *col);
}
void wxTreeCtrl_SetQuickBestSize(wxTreeCtrl * self, bool quick_best_size) {
    return self->SetQuickBestSize(quick_best_size);
}
void wxTreeCtrl_SetStateImageList(wxTreeCtrl * self, wxImageList * image_list) {
    return self->SetStateImageList(image_list);
}
void wxTreeCtrl_SetWindowStyle(wxTreeCtrl * self, long styles) {
    return self->SetWindowStyle(styles);
}
void wxTreeCtrl_SortChildren(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->SortChildren(*item);
}
void wxTreeCtrl_Toggle(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->Toggle(*item);
}
void wxTreeCtrl_ToggleItemSelection(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->ToggleItemSelection(*item);
}
void wxTreeCtrl_Unselect(wxTreeCtrl * self) {
    return self->Unselect();
}
void wxTreeCtrl_UnselectAll(wxTreeCtrl * self) {
    return self->UnselectAll();
}
void wxTreeCtrl_UnselectItem(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->UnselectItem(*item);
}
void wxTreeCtrl_SelectChildren(wxTreeCtrl * self, const wxTreeItemId * parent) {
    return self->SelectChildren(*parent);
}
// Mix-in(s) to wxTreeCtrl
wxWithImages *wxTreeCtrl_AsWithImages(wxTreeCtrl* obj) {
    return static_cast<wxWithImages*>(obj);
}
wxTrackable *wxTreeCtrl_AsTrackable(wxTreeCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTreeEvent
wxClassInfo *wxTreeEvent_CLASSINFO() {
    return wxCLASSINFO(wxTreeEvent);
}
wxTreeItemId *wxTreeEvent_GetItem(const wxTreeEvent * self) {
    return new wxTreeItemId(self->GetItem());
}
int wxTreeEvent_GetKeyCode(const wxTreeEvent * self) {
    return self->GetKeyCode();
}
wxKeyEvent *wxTreeEvent_GetKeyEvent(const wxTreeEvent * self) {
    return new wxKeyEvent(self->GetKeyEvent());
}
wxString *wxTreeEvent_GetLabel(const wxTreeEvent * self) {
    return new wxString(self->GetLabel());
}
wxTreeItemId *wxTreeEvent_GetOldItem(const wxTreeEvent * self) {
    return new wxTreeItemId(self->GetOldItem());
}
wxPoint *wxTreeEvent_GetPoint(const wxTreeEvent * self) {
    return new wxPoint(self->GetPoint());
}
bool wxTreeEvent_IsEditCancelled(const wxTreeEvent * self) {
    return self->IsEditCancelled();
}
void wxTreeEvent_SetToolTip(wxTreeEvent * self, const wxString * tooltip) {
    return self->SetToolTip(*tooltip);
}

// CLASS: wxTreeItemData
void wxTreeItemData_delete(wxTreeItemData *self) {
    delete self;
}
wxTreeItemData *wxTreeItemData_new() {
    return new wxTreeItemData();
}
wxTreeItemId *wxTreeItemData_GetId(const wxTreeItemData * self) {
    return new wxTreeItemId(self->GetId());
}
void wxTreeItemData_SetId(wxTreeItemData * self, const wxTreeItemId * id) {
    return self->SetId(*id);
}

// CLASS: wxTreeItemId
void wxTreeItemId_delete(wxTreeItemId *self) {
    delete self;
}
wxTreeItemId *wxTreeItemId_new() {
    return new wxTreeItemId();
}
bool wxTreeItemId_IsOk(const wxTreeItemId * self) {
    return self->IsOk();
}
void * wxTreeItemId_GetID(const wxTreeItemId * self) {
    return self->GetID();
}
void wxTreeItemId_Unset(wxTreeItemId * self) {
    return self->Unset();
}

// CLASS: wxTreeListCtrl
wxClassInfo *wxTreeListCtrl_CLASSINFO() {
    return wxCLASSINFO(wxTreeListCtrl);
}
void wxTreeListCtrl_AssignImageList(wxTreeListCtrl * self, wxImageList * image_list) {
    return self->AssignImageList(image_list);
}
void wxTreeListCtrl_SetImageList(wxTreeListCtrl * self, wxImageList * image_list) {
    return self->SetImageList(image_list);
}
int wxTreeListCtrl_AppendColumn(wxTreeListCtrl * self, const wxString * title, int width, wxAlignment align, int flags) {
    return self->AppendColumn(*title, width, align, flags);
}
void wxTreeListCtrl_ClearColumns(wxTreeListCtrl * self) {
    return self->ClearColumns();
}
int wxTreeListCtrl_WidthFor(const wxTreeListCtrl * self, const wxString * text) {
    return self->WidthFor(*text);
}
wxTreeListItem *wxTreeListCtrl_AppendItem(wxTreeListCtrl * self, wxTreeListItem parent, const wxString * text, int image_closed, int image_opened, wxClientData * data) {
    return new wxTreeListItem(self->AppendItem(parent, *text, image_closed, image_opened, data));
}
wxTreeListItem *wxTreeListCtrl_InsertItem(wxTreeListCtrl * self, wxTreeListItem parent, wxTreeListItem previous, const wxString * text, int image_closed, int image_opened, wxClientData * data) {
    return new wxTreeListItem(self->InsertItem(parent, previous, *text, image_closed, image_opened, data));
}
wxTreeListItem *wxTreeListCtrl_PrependItem(wxTreeListCtrl * self, wxTreeListItem parent, const wxString * text, int image_closed, int image_opened, wxClientData * data) {
    return new wxTreeListItem(self->PrependItem(parent, *text, image_closed, image_opened, data));
}
void wxTreeListCtrl_DeleteItem(wxTreeListCtrl * self, wxTreeListItem item) {
    return self->DeleteItem(item);
}
void wxTreeListCtrl_DeleteAllItems(wxTreeListCtrl * self) {
    return self->DeleteAllItems();
}
wxTreeListItem *wxTreeListCtrl_GetRootItem(const wxTreeListCtrl * self) {
    return new wxTreeListItem(self->GetRootItem());
}
wxTreeListItem *wxTreeListCtrl_GetItemParent(const wxTreeListCtrl * self, wxTreeListItem item) {
    return new wxTreeListItem(self->GetItemParent(item));
}
wxTreeListItem *wxTreeListCtrl_GetFirstChild(const wxTreeListCtrl * self, wxTreeListItem item) {
    return new wxTreeListItem(self->GetFirstChild(item));
}
wxTreeListItem *wxTreeListCtrl_GetNextSibling(const wxTreeListCtrl * self, wxTreeListItem item) {
    return new wxTreeListItem(self->GetNextSibling(item));
}
wxTreeListItem *wxTreeListCtrl_GetFirstItem(const wxTreeListCtrl * self) {
    return new wxTreeListItem(self->GetFirstItem());
}
wxTreeListItem *wxTreeListCtrl_GetNextItem(const wxTreeListCtrl * self, wxTreeListItem item) {
    return new wxTreeListItem(self->GetNextItem(item));
}
void wxTreeListCtrl_SetItemText1(wxTreeListCtrl * self, wxTreeListItem item, const wxString * text) {
    return self->SetItemText(item, *text);
}
void wxTreeListCtrl_SetItemImage(wxTreeListCtrl * self, wxTreeListItem item, int closed, int opened) {
    return self->SetItemImage(item, closed, opened);
}
wxClientData * wxTreeListCtrl_GetItemData(const wxTreeListCtrl * self, wxTreeListItem item) {
    return self->GetItemData(item);
}
void wxTreeListCtrl_SetItemData(wxTreeListCtrl * self, wxTreeListItem item, wxClientData * data) {
    return self->SetItemData(item, data);
}
void wxTreeListCtrl_Expand(wxTreeListCtrl * self, wxTreeListItem item) {
    return self->Expand(item);
}
void wxTreeListCtrl_Collapse(wxTreeListCtrl * self, wxTreeListItem item) {
    return self->Collapse(item);
}
bool wxTreeListCtrl_IsExpanded(const wxTreeListCtrl * self, wxTreeListItem item) {
    return self->IsExpanded(item);
}
wxTreeListItem *wxTreeListCtrl_GetSelection(const wxTreeListCtrl * self) {
    return new wxTreeListItem(self->GetSelection());
}
void wxTreeListCtrl_Select(wxTreeListCtrl * self, wxTreeListItem item) {
    return self->Select(item);
}
void wxTreeListCtrl_Unselect(wxTreeListCtrl * self, wxTreeListItem item) {
    return self->Unselect(item);
}
bool wxTreeListCtrl_IsSelected(const wxTreeListCtrl * self, wxTreeListItem item) {
    return self->IsSelected(item);
}
void wxTreeListCtrl_SelectAll(wxTreeListCtrl * self) {
    return self->SelectAll();
}
void wxTreeListCtrl_UnselectAll(wxTreeListCtrl * self) {
    return self->UnselectAll();
}
void wxTreeListCtrl_EnsureVisible(wxTreeListCtrl * self, wxTreeListItem item) {
    return self->EnsureVisible(item);
}
void wxTreeListCtrl_CheckItem(wxTreeListCtrl * self, wxTreeListItem item, wxCheckBoxState state) {
    return self->CheckItem(item, state);
}
void wxTreeListCtrl_CheckItemRecursively(wxTreeListCtrl * self, wxTreeListItem item, wxCheckBoxState state) {
    return self->CheckItemRecursively(item, state);
}
void wxTreeListCtrl_UncheckItem(wxTreeListCtrl * self, wxTreeListItem item) {
    return self->UncheckItem(item);
}
void wxTreeListCtrl_UpdateItemParentStateRecursively(wxTreeListCtrl * self, wxTreeListItem item) {
    return self->UpdateItemParentStateRecursively(item);
}
wxCheckBoxState wxTreeListCtrl_GetCheckedState(const wxTreeListCtrl * self, wxTreeListItem item) {
    return self->GetCheckedState(item);
}
bool wxTreeListCtrl_AreAllChildrenInState(const wxTreeListCtrl * self, wxTreeListItem item, wxCheckBoxState state) {
    return self->AreAllChildrenInState(item, state);
}
bool wxTreeListCtrl_GetSortColumn(wxTreeListCtrl * self, unsigned * col, bool * ascending_order) {
    return self->GetSortColumn(col, ascending_order);
}
void wxTreeListCtrl_SetItemComparator(wxTreeListCtrl * self, wxTreeListItemComparator * comparator) {
    return self->SetItemComparator(comparator);
}
wxWindow * wxTreeListCtrl_GetView(const wxTreeListCtrl * self) {
    return self->GetView();
}
wxDataViewCtrl * wxTreeListCtrl_GetDataView(const wxTreeListCtrl * self) {
    return self->GetDataView();
}
wxTreeListCtrl *wxTreeListCtrl_new() {
    return new wxTreeListCtrl();
}
wxTreeListCtrl *wxTreeListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxTreeListCtrl(parent, id, *pos, *size, style, *name);
}
bool wxTreeListCtrl_Create(wxTreeListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
// Mix-in(s) to wxTreeListCtrl
wxTrackable *wxTreeListCtrl_AsTrackable(wxTreeListCtrl* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTreeListItem
void wxTreeListItem_delete(wxTreeListItem *self) {
    delete self;
}
wxTreeListItem *wxTreeListItem_new() {
    return new wxTreeListItem();
}
bool wxTreeListItem_IsOk(const wxTreeListItem * self) {
    return self->IsOk();
}

// CLASS: wxTreeListItemComparator
void wxTreeListItemComparator_delete(wxTreeListItemComparator *self) {
    delete self;
}
wxTreeListItemComparator *wxTreeListItemComparator_new() {
    return new wxTreeListItemComparator();
}

// CLASS: wxTreebook
wxClassInfo *wxTreebook_CLASSINFO() {
    return wxCLASSINFO(wxTreebook);
}
wxTreebook *wxTreebook_new() {
    return new wxTreebook();
}
wxTreebook *wxTreebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxTreebook(parent, id, *pos, *size, style, *name);
}
bool wxTreebook_AddSubPage(wxTreebook * self, wxWindow * page, const wxString * text, bool b_select, int image_id) {
    return self->AddSubPage(page, *text, b_select, image_id);
}
bool wxTreebook_CollapseNode(wxTreebook * self, size_t page_id) {
    return self->CollapseNode(page_id);
}
bool wxTreebook_Create(wxTreebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
bool wxTreebook_ExpandNode(wxTreebook * self, size_t page_id, bool expand) {
    return self->ExpandNode(page_id, expand);
}
int wxTreebook_GetPageParent(const wxTreebook * self, size_t page) {
    return self->GetPageParent(page);
}
bool wxTreebook_InsertSubPage(wxTreebook * self, size_t page_pos, wxWindow * page, const wxString * text, bool b_select, int image_id) {
    return self->InsertSubPage(page_pos, page, *text, b_select, image_id);
}
bool wxTreebook_IsNodeExpanded(const wxTreebook * self, size_t page_id) {
    return self->IsNodeExpanded(page_id);
}
// Mix-in(s) to wxTreebook
wxWithImages *wxTreebook_AsWithImages(wxTreebook* obj) {
    return static_cast<wxWithImages*>(obj);
}
wxTrackable *wxTreebook_AsTrackable(wxTreebook* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTwoFingerTapEvent
wxClassInfo *wxTwoFingerTapEvent_CLASSINFO() {
    return wxCLASSINFO(wxTwoFingerTapEvent);
}
wxTwoFingerTapEvent *wxTwoFingerTapEvent_new(wxWindowID windid) {
    return new wxTwoFingerTapEvent(windid);
}

// CLASS: wxUIActionSimulator
void wxUIActionSimulator_delete(wxUIActionSimulator *self) {
    delete self;
}
wxUIActionSimulator *wxUIActionSimulator_new() {
    return new wxUIActionSimulator();
}
bool wxUIActionSimulator_MouseMove(wxUIActionSimulator * self, long x, long y) {
    return self->MouseMove(x, y);
}
bool wxUIActionSimulator_MouseMove1(wxUIActionSimulator * self, const wxPoint * point) {
    return self->MouseMove(*point);
}
bool wxUIActionSimulator_MouseDown(wxUIActionSimulator * self, int button) {
    return self->MouseDown(button);
}
bool wxUIActionSimulator_MouseUp(wxUIActionSimulator * self, int button) {
    return self->MouseUp(button);
}
bool wxUIActionSimulator_MouseClick(wxUIActionSimulator * self, int button) {
    return self->MouseClick(button);
}
bool wxUIActionSimulator_MouseDblClick(wxUIActionSimulator * self, int button) {
    return self->MouseDblClick(button);
}
bool wxUIActionSimulator_MouseDragDrop(wxUIActionSimulator * self, long x1, long y1, long x2, long y2, int button) {
    return self->MouseDragDrop(x1, y1, x2, y2, button);
}
bool wxUIActionSimulator_KeyDown(wxUIActionSimulator * self, int keycode, int modifiers) {
    return self->KeyDown(keycode, modifiers);
}
bool wxUIActionSimulator_KeyUp(wxUIActionSimulator * self, int keycode, int modifiers) {
    return self->KeyUp(keycode, modifiers);
}
bool wxUIActionSimulator_Char(wxUIActionSimulator * self, int keycode, int modifiers) {
    return self->Char(keycode, modifiers);
}
bool wxUIActionSimulator_Select(wxUIActionSimulator * self, const wxString * text) {
    return self->Select(*text);
}
bool wxUIActionSimulator_Text(wxUIActionSimulator * self, const char * text) {
    return self->Text(text);
}

// CLASS: wxURLDataObject
void wxURLDataObject_delete(wxURLDataObject *self) {
    delete self;
}
wxURLDataObject *wxURLDataObject_new(const wxString * url) {
    return new wxURLDataObject(*url);
}
wxString *wxURLDataObject_GetURL(const wxURLDataObject * self) {
    return new wxString(self->GetURL());
}
void wxURLDataObject_SetURL(wxURLDataObject * self, const wxString * url) {
    return self->SetURL(*url);
}

// CLASS: wxUpdateUIEvent
wxClassInfo *wxUpdateUIEvent_CLASSINFO() {
    return wxCLASSINFO(wxUpdateUIEvent);
}
wxUpdateUIEvent *wxUpdateUIEvent_new(wxWindowID command_id) {
    return new wxUpdateUIEvent(command_id);
}
void wxUpdateUIEvent_Check(wxUpdateUIEvent * self, bool check) {
    return self->Check(check);
}
void wxUpdateUIEvent_Enable(wxUpdateUIEvent * self, bool enable) {
    return self->Enable(enable);
}
bool wxUpdateUIEvent_GetChecked(const wxUpdateUIEvent * self) {
    return self->GetChecked();
}
bool wxUpdateUIEvent_GetEnabled(const wxUpdateUIEvent * self) {
    return self->GetEnabled();
}
bool wxUpdateUIEvent_IsCheckable(const wxUpdateUIEvent * self) {
    return self->IsCheckable();
}
bool wxUpdateUIEvent_GetSetChecked(const wxUpdateUIEvent * self) {
    return self->GetSetChecked();
}
bool wxUpdateUIEvent_GetSetEnabled(const wxUpdateUIEvent * self) {
    return self->GetSetEnabled();
}
bool wxUpdateUIEvent_GetSetShown(const wxUpdateUIEvent * self) {
    return self->GetSetShown();
}
bool wxUpdateUIEvent_GetSetText(const wxUpdateUIEvent * self) {
    return self->GetSetText();
}
bool wxUpdateUIEvent_GetShown(const wxUpdateUIEvent * self) {
    return self->GetShown();
}
wxString *wxUpdateUIEvent_GetText(const wxUpdateUIEvent * self) {
    return new wxString(self->GetText());
}
void wxUpdateUIEvent_SetText(wxUpdateUIEvent * self, const wxString * text) {
    return self->SetText(*text);
}
void wxUpdateUIEvent_Show(wxUpdateUIEvent * self, bool show) {
    return self->Show(show);
}
bool wxUpdateUIEvent_CanUpdate(wxWindow * window) {
    return wxUpdateUIEvent::CanUpdate(window);
}
long wxUpdateUIEvent_GetUpdateInterval() {
    return wxUpdateUIEvent::GetUpdateInterval();
}
void wxUpdateUIEvent_ResetUpdateTime() {
    return wxUpdateUIEvent::ResetUpdateTime();
}
void wxUpdateUIEvent_SetUpdateInterval(long update_interval) {
    return wxUpdateUIEvent::SetUpdateInterval(update_interval);
}

// CLASS: wxVListBox
wxClassInfo *wxVListBox_CLASSINFO() {
    return wxCLASSINFO(wxVListBox);
}
wxVListBox *wxVListBox_new() {
    return new wxVListBox();
}
wxVListBox *wxVListBox_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxVListBox(parent, id, *pos, *size, style, *name);
}
void wxVListBox_Clear(wxVListBox * self) {
    return self->Clear();
}
bool wxVListBox_Create(wxVListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
bool wxVListBox_DeselectAll(wxVListBox * self) {
    return self->DeselectAll();
}
int wxVListBox_GetFirstSelected(const wxVListBox * self, unsigned long * cookie) {
    return self->GetFirstSelected(*cookie);
}
size_t wxVListBox_GetItemCount(const wxVListBox * self) {
    return self->GetItemCount();
}
wxPoint *wxVListBox_GetMargins(const wxVListBox * self) {
    return new wxPoint(self->GetMargins());
}
wxRect *wxVListBox_GetItemRect(const wxVListBox * self, size_t item) {
    return new wxRect(self->GetItemRect(item));
}
int wxVListBox_GetNextSelected(const wxVListBox * self, unsigned long * cookie) {
    return self->GetNextSelected(*cookie);
}
size_t wxVListBox_GetSelectedCount(const wxVListBox * self) {
    return self->GetSelectedCount();
}
int wxVListBox_GetSelection(const wxVListBox * self) {
    return self->GetSelection();
}
wxColour *wxVListBox_GetSelectionBackground(const wxVListBox * self) {
    return new wxColour(self->GetSelectionBackground());
}
bool wxVListBox_HasMultipleSelection(const wxVListBox * self) {
    return self->HasMultipleSelection();
}
bool wxVListBox_IsCurrent(const wxVListBox * self, size_t item) {
    return self->IsCurrent(item);
}
bool wxVListBox_IsSelected(const wxVListBox * self, size_t item) {
    return self->IsSelected(item);
}
bool wxVListBox_Select(wxVListBox * self, size_t item, bool select) {
    return self->Select(item, select);
}
bool wxVListBox_SelectAll(wxVListBox * self) {
    return self->SelectAll();
}
bool wxVListBox_SelectRange(wxVListBox * self, size_t from, size_t to) {
    return self->SelectRange(from, to);
}
void wxVListBox_SetItemCount(wxVListBox * self, size_t count) {
    return self->SetItemCount(count);
}
void wxVListBox_SetMargins(wxVListBox * self, const wxPoint * pt) {
    return self->SetMargins(*pt);
}
void wxVListBox_SetMargins1(wxVListBox * self, wxCoord x, wxCoord y) {
    return self->SetMargins(x, y);
}
void wxVListBox_SetSelection(wxVListBox * self, int selection) {
    return self->SetSelection(selection);
}
void wxVListBox_SetSelectionBackground(wxVListBox * self, const wxColour * col) {
    return self->SetSelectionBackground(*col);
}
void wxVListBox_Toggle(wxVListBox * self, size_t item) {
    return self->Toggle(item);
}
// Mix-in(s) to wxVListBox
wxVarVScrollHelper *wxVListBox_AsVarVScrollHelper(wxVListBox* obj) {
    return static_cast<wxVarVScrollHelper*>(obj);
}
wxTrackable *wxVListBox_AsTrackable(wxVListBox* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxVScrolledWindow
wxClassInfo *wxVScrolledWindow_CLASSINFO() {
    return wxCLASSINFO(wxVScrolledWindow);
}
wxVScrolledWindow *wxVScrolledWindow_new() {
    return new wxVScrolledWindow();
}
wxVScrolledWindow *wxVScrolledWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxVScrolledWindow(parent, id, *pos, *size, style, *name);
}
bool wxVScrolledWindow_Create(wxVScrolledWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
// Mix-in(s) to wxVScrolledWindow
wxVarVScrollHelper *wxVScrolledWindow_AsVarVScrollHelper(wxVScrolledWindow* obj) {
    return static_cast<wxVarVScrollHelper*>(obj);
}
wxTrackable *wxVScrolledWindow_AsTrackable(wxVScrolledWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxValidator
wxClassInfo *wxValidator_CLASSINFO() {
    return wxCLASSINFO(wxValidator);
}
wxValidator *wxValidator_new() {
    return new wxValidator();
}
wxObject * wxValidator_Clone(const wxValidator * self) {
    return self->Clone();
}
wxWindow * wxValidator_GetWindow(const wxValidator * self) {
    return self->GetWindow();
}
void wxValidator_SetWindow(wxValidator * self, wxWindow * window) {
    return self->SetWindow(window);
}
bool wxValidator_TransferFromWindow(wxValidator * self) {
    return self->TransferFromWindow();
}
bool wxValidator_TransferToWindow(wxValidator * self) {
    return self->TransferToWindow();
}
bool wxValidator_Validate(wxValidator * self, wxWindow * parent) {
    return self->Validate(parent);
}
void wxValidator_SuppressBellOnError(bool suppress) {
    return wxValidator::SuppressBellOnError(suppress);
}
bool wxValidator_IsSilent() {
    return wxValidator::IsSilent();
}
// Mix-in(s) to wxValidator
wxTrackable *wxValidator_AsTrackable(wxValidator* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxVarHScrollHelper
void wxVarHScrollHelper_delete(wxVarHScrollHelper *self) {
    delete self;
}
wxVarHScrollHelper *wxVarHScrollHelper_new(wxWindow * win_to_scroll) {
    return new wxVarHScrollHelper(win_to_scroll);
}
size_t wxVarHScrollHelper_GetColumnCount(const wxVarHScrollHelper * self) {
    return self->GetColumnCount();
}
size_t wxVarHScrollHelper_GetVisibleColumnsBegin(const wxVarHScrollHelper * self) {
    return self->GetVisibleColumnsBegin();
}
size_t wxVarHScrollHelper_GetVisibleColumnsEnd(const wxVarHScrollHelper * self) {
    return self->GetVisibleColumnsEnd();
}
bool wxVarHScrollHelper_IsColumnVisible(const wxVarHScrollHelper * self, size_t column) {
    return self->IsColumnVisible(column);
}
void wxVarHScrollHelper_RefreshColumn(wxVarHScrollHelper * self, size_t column) {
    return self->RefreshColumn(column);
}
void wxVarHScrollHelper_RefreshColumns(wxVarHScrollHelper * self, size_t from, size_t to) {
    return self->RefreshColumns(from, to);
}
bool wxVarHScrollHelper_ScrollColumnPages(wxVarHScrollHelper * self, int pages) {
    return self->ScrollColumnPages(pages);
}
bool wxVarHScrollHelper_ScrollColumns(wxVarHScrollHelper * self, int columns) {
    return self->ScrollColumns(columns);
}
bool wxVarHScrollHelper_ScrollToColumn(wxVarHScrollHelper * self, size_t column) {
    return self->ScrollToColumn(column);
}
void wxVarHScrollHelper_SetColumnCount(wxVarHScrollHelper * self, size_t column_count) {
    return self->SetColumnCount(column_count);
}

// CLASS: wxVarHVScrollHelper
void wxVarHVScrollHelper_delete(wxVarHVScrollHelper *self) {
    delete self;
}
wxVarHVScrollHelper *wxVarHVScrollHelper_new(wxWindow * win_to_scroll) {
    return new wxVarHVScrollHelper(win_to_scroll);
}
void wxVarHVScrollHelper_EnablePhysicalScrolling(wxVarHVScrollHelper * self, bool vscrolling, bool hscrolling) {
    return self->EnablePhysicalScrolling(vscrolling, hscrolling);
}
wxSize *wxVarHVScrollHelper_GetRowColumnCount(const wxVarHVScrollHelper * self) {
    return new wxSize(self->GetRowColumnCount());
}
wxPosition *wxVarHVScrollHelper_GetVisibleBegin(const wxVarHVScrollHelper * self) {
    return new wxPosition(self->GetVisibleBegin());
}
wxPosition *wxVarHVScrollHelper_GetVisibleEnd(const wxVarHVScrollHelper * self) {
    return new wxPosition(self->GetVisibleEnd());
}
bool wxVarHVScrollHelper_IsVisible(const wxVarHVScrollHelper * self, size_t row, size_t column) {
    return self->IsVisible(row, column);
}
bool wxVarHVScrollHelper_IsVisible1(const wxVarHVScrollHelper * self, const wxPosition * pos) {
    return self->IsVisible(*pos);
}
void wxVarHVScrollHelper_RefreshRowColumn(wxVarHVScrollHelper * self, size_t row, size_t column) {
    return self->RefreshRowColumn(row, column);
}
void wxVarHVScrollHelper_RefreshRowColumn1(wxVarHVScrollHelper * self, const wxPosition * pos) {
    return self->RefreshRowColumn(*pos);
}
void wxVarHVScrollHelper_RefreshRowsColumns(wxVarHVScrollHelper * self, size_t from_row, size_t to_row, size_t from_column, size_t to_column) {
    return self->RefreshRowsColumns(from_row, to_row, from_column, to_column);
}
void wxVarHVScrollHelper_RefreshRowsColumns1(wxVarHVScrollHelper * self, const wxPosition * from, const wxPosition * to) {
    return self->RefreshRowsColumns(*from, *to);
}
bool wxVarHVScrollHelper_ScrollToRowColumn(wxVarHVScrollHelper * self, size_t row, size_t column) {
    return self->ScrollToRowColumn(row, column);
}
bool wxVarHVScrollHelper_ScrollToRowColumn1(wxVarHVScrollHelper * self, const wxPosition * pos) {
    return self->ScrollToRowColumn(*pos);
}
void wxVarHVScrollHelper_SetRowColumnCount(wxVarHVScrollHelper * self, size_t row_count, size_t column_count) {
    return self->SetRowColumnCount(row_count, column_count);
}
wxPosition *wxVarHVScrollHelper_VirtualHitTest(const wxVarHVScrollHelper * self, wxCoord x, wxCoord y) {
    return new wxPosition(self->VirtualHitTest(x, y));
}
wxPosition *wxVarHVScrollHelper_VirtualHitTest1(const wxVarHVScrollHelper * self, const wxPoint * pos) {
    return new wxPosition(self->VirtualHitTest(*pos));
}
// Mix-in(s) to wxVarHVScrollHelper
wxVarHScrollHelper *wxVarHVScrollHelper_AsVarHScrollHelper(wxVarHVScrollHelper* obj) {
    return static_cast<wxVarHScrollHelper*>(obj);
}

// CLASS: wxVarScrollHelperBase
void wxVarScrollHelperBase_delete(wxVarScrollHelperBase *self) {
    delete self;
}
wxVarScrollHelperBase *wxVarScrollHelperBase_new(wxWindow * win_to_scroll) {
    return new wxVarScrollHelperBase(win_to_scroll);
}
int wxVarScrollHelperBase_CalcScrolledPosition(const wxVarScrollHelperBase * self, int coord) {
    return self->CalcScrolledPosition(coord);
}
int wxVarScrollHelperBase_CalcUnscrolledPosition(const wxVarScrollHelperBase * self, int coord) {
    return self->CalcUnscrolledPosition(coord);
}
void wxVarScrollHelperBase_EnablePhysicalScrolling(wxVarScrollHelperBase * self, bool scrolling) {
    return self->EnablePhysicalScrolling(scrolling);
}
int wxVarScrollHelperBase_GetNonOrientationTargetSize(const wxVarScrollHelperBase * self) {
    return self->GetNonOrientationTargetSize();
}
int wxVarScrollHelperBase_GetOrientationTargetSize(const wxVarScrollHelperBase * self) {
    return self->GetOrientationTargetSize();
}
wxWindow * wxVarScrollHelperBase_GetTargetWindow(const wxVarScrollHelperBase * self) {
    return self->GetTargetWindow();
}
size_t wxVarScrollHelperBase_GetVisibleBegin(const wxVarScrollHelperBase * self) {
    return self->GetVisibleBegin();
}
size_t wxVarScrollHelperBase_GetVisibleEnd(const wxVarScrollHelperBase * self) {
    return self->GetVisibleEnd();
}
bool wxVarScrollHelperBase_IsVisible(const wxVarScrollHelperBase * self, size_t unit) {
    return self->IsVisible(unit);
}
void wxVarScrollHelperBase_RefreshAll(wxVarScrollHelperBase * self) {
    return self->RefreshAll();
}
void wxVarScrollHelperBase_SetTargetWindow(wxVarScrollHelperBase * self, wxWindow * target) {
    return self->SetTargetWindow(target);
}
void wxVarScrollHelperBase_UpdateScrollbar(wxVarScrollHelperBase * self) {
    return self->UpdateScrollbar();
}
int wxVarScrollHelperBase_VirtualHitTest(const wxVarScrollHelperBase * self, wxCoord coord) {
    return self->VirtualHitTest(coord);
}

// CLASS: wxVarVScrollHelper
void wxVarVScrollHelper_delete(wxVarVScrollHelper *self) {
    delete self;
}
wxVarVScrollHelper *wxVarVScrollHelper_new(wxWindow * win_to_scroll) {
    return new wxVarVScrollHelper(win_to_scroll);
}
size_t wxVarVScrollHelper_GetRowCount(const wxVarVScrollHelper * self) {
    return self->GetRowCount();
}
size_t wxVarVScrollHelper_GetVisibleRowsBegin(const wxVarVScrollHelper * self) {
    return self->GetVisibleRowsBegin();
}
size_t wxVarVScrollHelper_GetVisibleRowsEnd(const wxVarVScrollHelper * self) {
    return self->GetVisibleRowsEnd();
}
bool wxVarVScrollHelper_IsRowVisible(const wxVarVScrollHelper * self, size_t row) {
    return self->IsRowVisible(row);
}
void wxVarVScrollHelper_RefreshRow(wxVarVScrollHelper * self, size_t row) {
    return self->RefreshRow(row);
}
void wxVarVScrollHelper_RefreshRows(wxVarVScrollHelper * self, size_t from, size_t to) {
    return self->RefreshRows(from, to);
}
bool wxVarVScrollHelper_ScrollRowPages(wxVarVScrollHelper * self, int pages) {
    return self->ScrollRowPages(pages);
}
bool wxVarVScrollHelper_ScrollRows(wxVarVScrollHelper * self, int rows) {
    return self->ScrollRows(rows);
}
bool wxVarVScrollHelper_ScrollToRow(wxVarVScrollHelper * self, size_t row) {
    return self->ScrollToRow(row);
}
void wxVarVScrollHelper_SetRowCount(wxVarVScrollHelper * self, size_t row_count) {
    return self->SetRowCount(row_count);
}

// CLASS: wxVariantDataCurrency
void wxVariantDataCurrency_delete(wxVariantDataCurrency *self) {
    delete self;
}
wxVariantDataCurrency *wxVariantDataCurrency_new() {
    return new wxVariantDataCurrency();
}
bool wxVariantDataCurrency_GetAsAny(const wxVariantDataCurrency * self, wxAny * any) {
    return self->GetAsAny(any);
}

// CLASS: wxVariantDataErrorCode
void wxVariantDataErrorCode_delete(wxVariantDataErrorCode *self) {
    delete self;
}
bool wxVariantDataErrorCode_GetAsAny(const wxVariantDataErrorCode * self, wxAny * any) {
    return self->GetAsAny(any);
}

// CLASS: wxVariantDataSafeArray
void wxVariantDataSafeArray_delete(wxVariantDataSafeArray *self) {
    delete self;
}
wxVariantDataSafeArray *wxVariantDataSafeArray_new(SAFEARRAY * value) {
    return new wxVariantDataSafeArray(value);
}
SAFEARRAY * wxVariantDataSafeArray_GetValue(const wxVariantDataSafeArray * self) {
    return self->GetValue();
}
void wxVariantDataSafeArray_SetValue(wxVariantDataSafeArray * self, SAFEARRAY * value) {
    return self->SetValue(value);
}
bool wxVariantDataSafeArray_GetAsAny(const wxVariantDataSafeArray * self, wxAny * any) {
    return self->GetAsAny(any);
}

// CLASS: wxView
wxClassInfo *wxView_CLASSINFO() {
    return wxCLASSINFO(wxView);
}
wxView *wxView_new() {
    return new wxView();
}
void wxView_Activate(wxView * self, bool activate) {
    return self->Activate(activate);
}
bool wxView_Close(wxView * self, bool delete_window) {
    return self->Close(delete_window);
}
wxDocument * wxView_GetDocument(const wxView * self) {
    return self->GetDocument();
}
wxDocManager * wxView_GetDocumentManager(const wxView * self) {
    return self->GetDocumentManager();
}
wxWindow * wxView_GetFrame(const wxView * self) {
    return self->GetFrame();
}
wxString *wxView_GetViewName(const wxView * self) {
    return new wxString(self->GetViewName());
}
void wxView_OnActivateView(wxView * self, bool activate, wxView * active_view, wxView * deactive_view) {
    return self->OnActivateView(activate, active_view, deactive_view);
}
void wxView_OnChangeFilename(wxView * self) {
    return self->OnChangeFilename();
}
bool wxView_OnClose(wxView * self, bool delete_window) {
    return self->OnClose(delete_window);
}
void wxView_OnClosingDocument(wxView * self) {
    return self->OnClosingDocument();
}
bool wxView_OnCreate(wxView * self, wxDocument * doc, long flags) {
    return self->OnCreate(doc, flags);
}
wxPrintout * wxView_OnCreatePrintout(wxView * self) {
    return self->OnCreatePrintout();
}
void wxView_OnDraw(wxView * self, wxDC * dc) {
    return self->OnDraw(dc);
}
void wxView_OnUpdate(wxView * self, wxView * sender, wxObject * hint) {
    return self->OnUpdate(sender, hint);
}
void wxView_SetDocument(wxView * self, wxDocument * doc) {
    return self->SetDocument(doc);
}
void wxView_SetFrame(wxView * self, wxWindow * frame) {
    return self->SetFrame(frame);
}
void wxView_SetViewName(wxView * self, const wxString * name) {
    return self->SetViewName(*name);
}
// Mix-in(s) to wxView
wxTrackable *wxView_AsTrackable(wxView* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxWindow
wxClassInfo *wxWindow_CLASSINFO() {
    return wxCLASSINFO(wxWindow);
}
bool wxWindow_AcceptsFocus(const wxWindow * self) {
    return self->AcceptsFocus();
}
bool wxWindow_AcceptsFocusFromKeyboard(const wxWindow * self) {
    return self->AcceptsFocusFromKeyboard();
}
bool wxWindow_AcceptsFocusRecursively(const wxWindow * self) {
    return self->AcceptsFocusRecursively();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_DisableFocusFromKeyboard(wxWindow * self) {
    return self->DisableFocusFromKeyboard();
}
#endif
bool wxWindow_IsFocusable(const wxWindow * self) {
    return self->IsFocusable();
}
bool wxWindow_CanAcceptFocus(const wxWindow * self) {
    return self->CanAcceptFocus();
}
bool wxWindow_CanAcceptFocusFromKeyboard(const wxWindow * self) {
    return self->CanAcceptFocusFromKeyboard();
}
bool wxWindow_HasFocus(const wxWindow * self) {
    return self->HasFocus();
}
void wxWindow_SetCanFocus(wxWindow * self, bool can_focus) {
    return self->SetCanFocus(can_focus);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_EnableVisibleFocus(wxWindow * self, bool enable) {
    return self->EnableVisibleFocus(enable);
}
#endif
void wxWindow_SetFocus(wxWindow * self) {
    return self->SetFocus();
}
void wxWindow_SetFocusFromKbd(wxWindow * self) {
    return self->SetFocusFromKbd();
}
void wxWindow_AddChild(wxWindow * self, wxWindow * child) {
    return self->AddChild(child);
}
bool wxWindow_DestroyChildren(wxWindow * self) {
    return self->DestroyChildren();
}
wxWindow * wxWindow_FindWindow(const wxWindow * self, long id) {
    return self->FindWindow(id);
}
wxWindow * wxWindow_FindWindow1(const wxWindow * self, const wxString * name) {
    return self->FindWindow(*name);
}
wxWindowList *wxWindow_GetChildren1(const wxWindow * self) {
    return new wxWindowList(self->GetChildren());
}
void wxWindow_RemoveChild(wxWindow * self, wxWindow * child) {
    return self->RemoveChild(child);
}
wxWindow * wxWindow_GetGrandParent(const wxWindow * self) {
    return self->GetGrandParent();
}
wxWindow * wxWindow_GetNextSibling(const wxWindow * self) {
    return self->GetNextSibling();
}
wxWindow * wxWindow_GetParent(const wxWindow * self) {
    return self->GetParent();
}
wxWindow * wxWindow_GetPrevSibling(const wxWindow * self) {
    return self->GetPrevSibling();
}
bool wxWindow_IsDescendant(const wxWindow * self, wxWindow * win) {
    return self->IsDescendant(win);
}
bool wxWindow_Reparent(wxWindow * self, wxWindow * new_parent) {
    return self->Reparent(new_parent);
}
void wxWindow_AlwaysShowScrollbars(wxWindow * self, bool hflag, bool vflag) {
    return self->AlwaysShowScrollbars(hflag, vflag);
}
int wxWindow_GetScrollPos(const wxWindow * self, int orientation) {
    return self->GetScrollPos(orientation);
}
int wxWindow_GetScrollRange(const wxWindow * self, int orientation) {
    return self->GetScrollRange(orientation);
}
int wxWindow_GetScrollThumb(const wxWindow * self, int orientation) {
    return self->GetScrollThumb(orientation);
}
bool wxWindow_CanScroll(const wxWindow * self, int orient) {
    return self->CanScroll(orient);
}
bool wxWindow_HasScrollbar(const wxWindow * self, int orient) {
    return self->HasScrollbar(orient);
}
bool wxWindow_IsScrollbarAlwaysShown(const wxWindow * self, int orient) {
    return self->IsScrollbarAlwaysShown(orient);
}
bool wxWindow_ScrollLines(wxWindow * self, int lines) {
    return self->ScrollLines(lines);
}
bool wxWindow_ScrollPages(wxWindow * self, int pages) {
    return self->ScrollPages(pages);
}
void wxWindow_ScrollWindow(wxWindow * self, int dx, int dy, const wxRect * rect) {
    return self->ScrollWindow(dx, dy, rect);
}
bool wxWindow_LineUp(wxWindow * self) {
    return self->LineUp();
}
bool wxWindow_LineDown(wxWindow * self) {
    return self->LineDown();
}
bool wxWindow_PageUp(wxWindow * self) {
    return self->PageUp();
}
bool wxWindow_PageDown(wxWindow * self) {
    return self->PageDown();
}
void wxWindow_SetScrollPos(wxWindow * self, int orientation, int pos, bool refresh) {
    return self->SetScrollPos(orientation, pos, refresh);
}
void wxWindow_SetScrollbar(wxWindow * self, int orientation, int position, int thumb_size, int range, bool refresh) {
    return self->SetScrollbar(orientation, position, thumb_size, range, refresh);
}
bool wxWindow_BeginRepositioningChildren(wxWindow * self) {
    return self->BeginRepositioningChildren();
}
void wxWindow_EndRepositioningChildren(wxWindow * self) {
    return self->EndRepositioningChildren();
}
void wxWindow_CacheBestSize(const wxWindow * self, const wxSize * size) {
    return self->CacheBestSize(*size);
}
wxSize *wxWindow_ClientToWindowSize(const wxWindow * self, const wxSize * size) {
    return new wxSize(self->ClientToWindowSize(*size));
}
wxSize *wxWindow_WindowToClientSize(const wxWindow * self, const wxSize * size) {
    return new wxSize(self->WindowToClientSize(*size));
}
void wxWindow_Fit(wxWindow * self) {
    return self->Fit();
}
void wxWindow_FitInside(wxWindow * self) {
    return self->FitInside();
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->FromDIP(*sz));
}
wxPoint *wxWindow_FromDIP1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->FromDIP(*pt));
}
int wxWindow_FromDIP2(const wxWindow * self, int d) {
    return self->FromDIP(d);
}
wxSize *wxWindow_ToDIP(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->ToDIP(*sz));
}
wxPoint *wxWindow_ToDIP1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ToDIP(*pt));
}
int wxWindow_ToDIP2(const wxWindow * self, int d) {
    return self->ToDIP(d);
}
wxSize *wxWindow_FromPhys(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->FromPhys(*sz));
}
wxPoint *wxWindow_FromPhys1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->FromPhys(*pt));
}
int wxWindow_FromPhys2(const wxWindow * self, int d) {
    return self->FromPhys(d);
}
wxSize *wxWindow_ToPhys(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->ToPhys(*sz));
}
wxPoint *wxWindow_ToPhys1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ToPhys(*pt));
}
int wxWindow_ToPhys2(const wxWindow * self, int d) {
    return self->ToPhys(d);
}
#endif
wxSize *wxWindow_GetBestSize(const wxWindow * self) {
    return new wxSize(self->GetBestSize());
}
int wxWindow_GetBestHeight(const wxWindow * self, int width) {
    return self->GetBestHeight(width);
}
int wxWindow_GetBestWidth(const wxWindow * self, int height) {
    return self->GetBestWidth(height);
}
void wxWindow_GetClientSize(const wxWindow * self, int * width, int * height) {
    return self->GetClientSize(width, height);
}
wxSize *wxWindow_GetClientSize1(const wxWindow * self) {
    return new wxSize(self->GetClientSize());
}
wxSize *wxWindow_GetEffectiveMinSize(const wxWindow * self) {
    return new wxSize(self->GetEffectiveMinSize());
}
wxSize *wxWindow_GetMaxClientSize(const wxWindow * self) {
    return new wxSize(self->GetMaxClientSize());
}
wxSize *wxWindow_GetMaxSize(const wxWindow * self) {
    return new wxSize(self->GetMaxSize());
}
wxSize *wxWindow_GetMinClientSize(const wxWindow * self) {
    return new wxSize(self->GetMinClientSize());
}
wxSize *wxWindow_GetMinSize(const wxWindow * self) {
    return new wxSize(self->GetMinSize());
}
int wxWindow_GetMinWidth(const wxWindow * self) {
    return self->GetMinWidth();
}
int wxWindow_GetMinHeight(const wxWindow * self) {
    return self->GetMinHeight();
}
int wxWindow_GetMaxWidth(const wxWindow * self) {
    return self->GetMaxWidth();
}
int wxWindow_GetMaxHeight(const wxWindow * self) {
    return self->GetMaxHeight();
}
void wxWindow_GetSize(const wxWindow * self, int * width, int * height) {
    return self->GetSize(width, height);
}
wxSize *wxWindow_GetSize1(const wxWindow * self) {
    return new wxSize(self->GetSize());
}
wxSize *wxWindow_GetVirtualSize(const wxWindow * self) {
    return new wxSize(self->GetVirtualSize());
}
void wxWindow_GetVirtualSize1(const wxWindow * self, int * width, int * height) {
    return self->GetVirtualSize(width, height);
}
wxSize *wxWindow_GetBestVirtualSize(const wxWindow * self) {
    return new wxSize(self->GetBestVirtualSize());
}
double wxWindow_GetContentScaleFactor(const wxWindow * self) {
    return self->GetContentScaleFactor();
}
#if wxCHECK_VERSION(3, 1, 0)
double wxWindow_GetDPIScaleFactor(const wxWindow * self) {
    return self->GetDPIScaleFactor();
}
#endif
wxSize *wxWindow_GetWindowBorderSize(const wxWindow * self) {
    return new wxSize(self->GetWindowBorderSize());
}
bool wxWindow_InformFirstDirection(wxWindow * self, int direction, int size, int available_other_dir) {
    return self->InformFirstDirection(direction, size, available_other_dir);
}
void wxWindow_InvalidateBestSize(wxWindow * self) {
    return self->InvalidateBestSize();
}
void wxWindow_PostSizeEvent(wxWindow * self) {
    return self->PostSizeEvent();
}
void wxWindow_PostSizeEventToParent(wxWindow * self) {
    return self->PostSizeEventToParent();
}
void wxWindow_SendSizeEvent(wxWindow * self, int flags) {
    return self->SendSizeEvent(flags);
}
void wxWindow_SendSizeEventToParent(wxWindow * self, int flags) {
    return self->SendSizeEventToParent(flags);
}
void wxWindow_SetClientSize(wxWindow * self, int width, int height) {
    return self->SetClientSize(width, height);
}
void wxWindow_SetClientSize1(wxWindow * self, const wxSize * size) {
    return self->SetClientSize(*size);
}
void wxWindow_SetClientSize2(wxWindow * self, const wxRect * rect) {
    return self->SetClientSize(*rect);
}
void wxWindow_SetContainingSizer(wxWindow * self, wxSizer * sizer) {
    return self->SetContainingSizer(sizer);
}
void wxWindow_SetInitialSize(wxWindow * self, const wxSize * size) {
    return self->SetInitialSize(*size);
}
void wxWindow_SetMaxClientSize(wxWindow * self, const wxSize * size) {
    return self->SetMaxClientSize(*size);
}
void wxWindow_SetMaxSize(wxWindow * self, const wxSize * size) {
    return self->SetMaxSize(*size);
}
void wxWindow_SetMinClientSize(wxWindow * self, const wxSize * size) {
    return self->SetMinClientSize(*size);
}
void wxWindow_SetMinSize(wxWindow * self, const wxSize * size) {
    return self->SetMinSize(*size);
}
void wxWindow_SetSize(wxWindow * self, int x, int y, int width, int height, int size_flags) {
    return self->SetSize(x, y, width, height, size_flags);
}
void wxWindow_SetSize1(wxWindow * self, const wxRect * rect) {
    return self->SetSize(*rect);
}
void wxWindow_SetSize2(wxWindow * self, const wxSize * size) {
    return self->SetSize(*size);
}
void wxWindow_SetSize3(wxWindow * self, int width, int height) {
    return self->SetSize(width, height);
}
void wxWindow_SetSizeHints(wxWindow * self, const wxSize * min_size, const wxSize * max_size, const wxSize * inc_size) {
    return self->SetSizeHints(*min_size, *max_size, *inc_size);
}
void wxWindow_SetSizeHints1(wxWindow * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h) {
    return self->SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h);
}
void wxWindow_SetVirtualSize(wxWindow * self, int width, int height) {
    return self->SetVirtualSize(width, height);
}
void wxWindow_SetVirtualSize1(wxWindow * self, const wxSize * size) {
    return self->SetVirtualSize(*size);
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::FromDIP(*sz, w));
}
wxPoint *wxWindow_FromDIP4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::FromDIP(*pt, w));
}
int wxWindow_FromDIP5(int d, const wxWindow * w) {
    return wxWindow::FromDIP(d, w);
}
wxSize *wxWindow_ToDIP3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::ToDIP(*sz, w));
}
wxPoint *wxWindow_ToDIP4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::ToDIP(*pt, w));
}
int wxWindow_ToDIP5(int d, const wxWindow * w) {
    return wxWindow::ToDIP(d, w);
}
wxSize *wxWindow_FromPhys3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::FromPhys(*sz, w));
}
wxPoint *wxWindow_FromPhys4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::FromPhys(*pt, w));
}
int wxWindow_FromPhys5(int d, const wxWindow * w) {
    return wxWindow::FromPhys(d, w);
}
wxSize *wxWindow_ToPhys3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::ToPhys(*sz, w));
}
wxPoint *wxWindow_ToPhys4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::ToPhys(*pt, w));
}
int wxWindow_ToPhys5(int d, const wxWindow * w) {
    return wxWindow::ToPhys(d, w);
}
#endif
void wxWindow_Center(wxWindow * self, int dir) {
    return self->Center(dir);
}
void wxWindow_CenterOnParent(wxWindow * self, int dir) {
    return self->CenterOnParent(dir);
}
void wxWindow_Centre(wxWindow * self, int direction) {
    return self->Centre(direction);
}
void wxWindow_CentreOnParent(wxWindow * self, int direction) {
    return self->CentreOnParent(direction);
}
void wxWindow_GetPosition(const wxWindow * self, int * x, int * y) {
    return self->GetPosition(x, y);
}
wxPoint *wxWindow_GetPosition1(const wxWindow * self) {
    return new wxPoint(self->GetPosition());
}
wxRect *wxWindow_GetRect(const wxWindow * self) {
    return new wxRect(self->GetRect());
}
void wxWindow_GetScreenPosition(const wxWindow * self, int * x, int * y) {
    return self->GetScreenPosition(x, y);
}
wxPoint *wxWindow_GetScreenPosition1(const wxWindow * self) {
    return new wxPoint(self->GetScreenPosition());
}
wxRect *wxWindow_GetScreenRect(const wxWindow * self) {
    return new wxRect(self->GetScreenRect());
}
wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow * self) {
    return new wxPoint(self->GetClientAreaOrigin());
}
wxRect *wxWindow_GetClientRect(const wxWindow * self) {
    return new wxRect(self->GetClientRect());
}
void wxWindow_Move(wxWindow * self, int x, int y, int flags) {
    return self->Move(x, y, flags);
}
void wxWindow_Move1(wxWindow * self, const wxPoint * pt, int flags) {
    return self->Move(*pt, flags);
}
void wxWindow_SetPosition(wxWindow * self, const wxPoint * pt) {
    return self->SetPosition(*pt);
}
void wxWindow_ClientToScreen(const wxWindow * self, int * x, int * y) {
    return self->ClientToScreen(x, y);
}
wxPoint *wxWindow_ClientToScreen1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ClientToScreen(*pt));
}
wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ConvertDialogToPixels(*pt));
}
wxSize *wxWindow_ConvertDialogToPixels1(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->ConvertDialogToPixels(*sz));
}
wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ConvertPixelsToDialog(*pt));
}
wxSize *wxWindow_ConvertPixelsToDialog1(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->ConvertPixelsToDialog(*sz));
}
void wxWindow_ScreenToClient(const wxWindow * self, int * x, int * y) {
    return self->ScreenToClient(x, y);
}
wxPoint *wxWindow_ScreenToClient1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ScreenToClient(*pt));
}
void wxWindow_ClearBackground(wxWindow * self) {
    return self->ClearBackground();
}
void wxWindow_Freeze(wxWindow * self) {
    return self->Freeze();
}
void wxWindow_Thaw(wxWindow * self) {
    return self->Thaw();
}
bool wxWindow_IsFrozen(const wxWindow * self) {
    return self->IsFrozen();
}
wxColour *wxWindow_GetBackgroundColour(const wxWindow * self) {
    return new wxColour(self->GetBackgroundColour());
}
int wxWindow_GetCharHeight(const wxWindow * self) {
    return self->GetCharHeight();
}
int wxWindow_GetCharWidth(const wxWindow * self) {
    return self->GetCharWidth();
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_GetDPI(const wxWindow * self) {
    return new wxSize(self->GetDPI());
}
#endif
wxFont *wxWindow_GetFont(const wxWindow * self) {
    return new wxFont(self->GetFont());
}
wxColour *wxWindow_GetForegroundColour(const wxWindow * self) {
    return new wxColour(self->GetForegroundColour());
}
void wxWindow_GetTextExtent(const wxWindow * self, const wxString * string, int * w, int * h, int * descent, int * external_leading, const wxFont * font) {
    return self->GetTextExtent(*string, w, h, descent, external_leading, font);
}
wxSize *wxWindow_GetTextExtent1(const wxWindow * self, const wxString * string) {
    return new wxSize(self->GetTextExtent(*string));
}
wxRect *wxWindow_GetUpdateClientRect(const wxWindow * self) {
    return new wxRect(self->GetUpdateClientRect());
}
bool wxWindow_HasTransparentBackground(wxWindow * self) {
    return self->HasTransparentBackground();
}
void wxWindow_Refresh(wxWindow * self, bool erase_background, const wxRect * rect) {
    return self->Refresh(erase_background, rect);
}
void wxWindow_RefreshRect(wxWindow * self, const wxRect * rect, bool erase_background) {
    return self->RefreshRect(*rect, erase_background);
}
void wxWindow_Update(wxWindow * self) {
    return self->Update();
}
bool wxWindow_SetBackgroundColour(wxWindow * self, const wxColour * colour) {
    return self->SetBackgroundColour(*colour);
}
bool wxWindow_IsTransparentBackgroundSupported(const wxWindow * self, wxString * reason) {
    return self->IsTransparentBackgroundSupported(reason);
}
bool wxWindow_SetFont(wxWindow * self, const wxFont * font) {
    return self->SetFont(*font);
}
bool wxWindow_SetForegroundColour(wxWindow * self, const wxColour * colour) {
    return self->SetForegroundColour(*colour);
}
void wxWindow_SetOwnBackgroundColour(wxWindow * self, const wxColour * colour) {
    return self->SetOwnBackgroundColour(*colour);
}
bool wxWindow_InheritsBackgroundColour(const wxWindow * self) {
    return self->InheritsBackgroundColour();
}
bool wxWindow_UseBgCol(const wxWindow * self) {
    return self->UseBgCol();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseBackgroundColour(const wxWindow * self) {
    return self->UseBackgroundColour();
}
#endif
void wxWindow_SetOwnFont(wxWindow * self, const wxFont * font) {
    return self->SetOwnFont(*font);
}
void wxWindow_SetOwnForegroundColour(wxWindow * self, const wxColour * colour) {
    return self->SetOwnForegroundColour(*colour);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseForegroundColour(const wxWindow * self) {
    return self->UseForegroundColour();
}
bool wxWindow_InheritsForegroundColour(const wxWindow * self) {
    return self->InheritsForegroundColour();
}
#endif
void wxWindow_SetPalette(wxWindow * self, const wxPalette * pal) {
    return self->SetPalette(*pal);
}
bool wxWindow_ShouldInheritColours(const wxWindow * self) {
    return self->ShouldInheritColours();
}
void wxWindow_SetThemeEnabled(wxWindow * self, bool enable) {
    return self->SetThemeEnabled(enable);
}
bool wxWindow_GetThemeEnabled(const wxWindow * self) {
    return self->GetThemeEnabled();
}
bool wxWindow_CanSetTransparent(wxWindow * self) {
    return self->CanSetTransparent();
}
bool wxWindow_SetTransparent(wxWindow * self, wxByte alpha) {
    return self->SetTransparent(alpha);
}
wxEvtHandler * wxWindow_GetEventHandler(const wxWindow * self) {
    return self->GetEventHandler();
}
bool wxWindow_HandleAsNavigationKey(wxWindow * self, const wxKeyEvent * event) {
    return self->HandleAsNavigationKey(*event);
}
bool wxWindow_HandleWindowEvent(const wxWindow * self, wxEvent * event) {
    return self->HandleWindowEvent(*event);
}
bool wxWindow_ProcessWindowEvent(wxWindow * self, wxEvent * event) {
    return self->ProcessWindowEvent(*event);
}
bool wxWindow_ProcessWindowEventLocally(wxWindow * self, wxEvent * event) {
    return self->ProcessWindowEventLocally(*event);
}
wxEvtHandler * wxWindow_PopEventHandler(wxWindow * self, bool delete_handler) {
    return self->PopEventHandler(delete_handler);
}
void wxWindow_PushEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return self->PushEventHandler(handler);
}
bool wxWindow_RemoveEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return self->RemoveEventHandler(handler);
}
void wxWindow_SetEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return self->SetEventHandler(handler);
}
long wxWindow_GetExtraStyle(const wxWindow * self) {
    return self->GetExtraStyle();
}
long wxWindow_GetWindowStyleFlag(const wxWindow * self) {
    return self->GetWindowStyleFlag();
}
long wxWindow_GetWindowStyle(const wxWindow * self) {
    return self->GetWindowStyle();
}
bool wxWindow_HasExtraStyle(const wxWindow * self, int ex_flag) {
    return self->HasExtraStyle(ex_flag);
}
bool wxWindow_HasFlag(const wxWindow * self, int flag) {
    return self->HasFlag(flag);
}
void wxWindow_SetExtraStyle(wxWindow * self, long ex_style) {
    return self->SetExtraStyle(ex_style);
}
void wxWindow_SetWindowStyleFlag(wxWindow * self, long style) {
    return self->SetWindowStyleFlag(style);
}
void wxWindow_SetWindowStyle(wxWindow * self, long style) {
    return self->SetWindowStyle(style);
}
bool wxWindow_ToggleWindowStyle(wxWindow * self, int flag) {
    return self->ToggleWindowStyle(flag);
}
void wxWindow_MoveAfterInTabOrder(wxWindow * self, wxWindow * win) {
    return self->MoveAfterInTabOrder(win);
}
void wxWindow_MoveBeforeInTabOrder(wxWindow * self, wxWindow * win) {
    return self->MoveBeforeInTabOrder(win);
}
bool wxWindow_Navigate(wxWindow * self, int flags) {
    return self->Navigate(flags);
}
bool wxWindow_NavigateIn(wxWindow * self, int flags) {
    return self->NavigateIn(flags);
}
void wxWindow_Lower(wxWindow * self) {
    return self->Lower();
}
void wxWindow_Raise(wxWindow * self) {
    return self->Raise();
}
bool wxWindow_Hide(wxWindow * self) {
    return self->Hide();
}
bool wxWindow_IsEnabled(const wxWindow * self) {
    return self->IsEnabled();
}
bool wxWindow_IsExposed(const wxWindow * self, int x, int y) {
    return self->IsExposed(x, y);
}
bool wxWindow_IsExposed1(const wxWindow * self, wxPoint * pt) {
    return self->IsExposed(*pt);
}
bool wxWindow_IsExposed2(const wxWindow * self, int x, int y, int w, int h) {
    return self->IsExposed(x, y, w, h);
}
bool wxWindow_IsExposed3(const wxWindow * self, wxRect * rect) {
    return self->IsExposed(*rect);
}
bool wxWindow_IsShown(const wxWindow * self) {
    return self->IsShown();
}
bool wxWindow_IsShownOnScreen(const wxWindow * self) {
    return self->IsShownOnScreen();
}
bool wxWindow_Disable(wxWindow * self) {
    return self->Disable();
}
bool wxWindow_Enable(wxWindow * self, bool enable) {
    return self->Enable(enable);
}
bool wxWindow_Show(wxWindow * self, bool show) {
    return self->Show(show);
}
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxWindow_GetHelpText(const wxWindow * self) {
    return new wxString(self->GetHelpText());
}
#endif
void wxWindow_SetHelpText(wxWindow * self, const wxString * help_text) {
    return self->SetHelpText(*help_text);
}
wxToolTip * wxWindow_GetToolTip(const wxWindow * self) {
    return self->GetToolTip();
}
wxString *wxWindow_GetToolTipText(const wxWindow * self) {
    return new wxString(self->GetToolTipText());
}
void wxWindow_SetToolTip(wxWindow * self, const wxString * tip_string) {
    return self->SetToolTip(*tip_string);
}
void wxWindow_SetToolTip1(wxWindow * self, wxToolTip * tip) {
    return self->SetToolTip(tip);
}
void wxWindow_UnsetToolTip(wxWindow * self) {
    return self->UnsetToolTip();
}
int wxWindow_GetPopupMenuSelectionFromUser(wxWindow * self, wxMenu * menu, const wxPoint * pos) {
    return self->GetPopupMenuSelectionFromUser(*menu, *pos);
}
int wxWindow_GetPopupMenuSelectionFromUser1(wxWindow * self, wxMenu * menu, int x, int y) {
    return self->GetPopupMenuSelectionFromUser(*menu, x, y);
}
bool wxWindow_PopupMenu(wxWindow * self, wxMenu * menu, const wxPoint * pos) {
    return self->PopupMenu(menu, *pos);
}
bool wxWindow_PopupMenu1(wxWindow * self, wxMenu * menu, int x, int y) {
    return self->PopupMenu(menu, x, y);
}
wxValidator * wxWindow_GetValidator(wxWindow * self) {
    return self->GetValidator();
}
void wxWindow_SetValidator(wxWindow * self, const wxValidator * validator) {
    return self->SetValidator(*validator);
}
bool wxWindow_TransferDataFromWindow(wxWindow * self) {
    return self->TransferDataFromWindow();
}
bool wxWindow_TransferDataToWindow(wxWindow * self) {
    return self->TransferDataToWindow();
}
bool wxWindow_Validate(wxWindow * self) {
    return self->Validate();
}
wxWindowID wxWindow_GetId(const wxWindow * self) {
    return self->GetId();
}
wxString *wxWindow_GetLabel(const wxWindow * self) {
    return new wxString(self->GetLabel());
}
wxLayoutDirection wxWindow_GetLayoutDirection(const wxWindow * self) {
    return self->GetLayoutDirection();
}
wxCoord wxWindow_AdjustForLayoutDirection(const wxWindow * self, wxCoord x, wxCoord width, wxCoord width_total) {
    return self->AdjustForLayoutDirection(x, width, width_total);
}
wxString *wxWindow_GetName(const wxWindow * self) {
    return new wxString(self->GetName());
}
void wxWindow_SetId(wxWindow * self, wxWindowID winid) {
    return self->SetId(winid);
}
void wxWindow_SetLabel(wxWindow * self, const wxString * label) {
    return self->SetLabel(*label);
}
void wxWindow_SetLayoutDirection(wxWindow * self, wxLayoutDirection dir) {
    return self->SetLayoutDirection(dir);
}
void wxWindow_SetName(wxWindow * self, const wxString * name) {
    return self->SetName(*name);
}
wxAcceleratorTable * wxWindow_GetAcceleratorTable(wxWindow * self) {
    return self->GetAcceleratorTable();
}
void wxWindow_SetAcceleratorTable(wxWindow * self, const wxAcceleratorTable * accel) {
    return self->SetAcceleratorTable(*accel);
}
bool wxWindow_Close(wxWindow * self, bool force) {
    return self->Close(force);
}
bool wxWindow_Destroy(wxWindow * self) {
    return self->Destroy();
}
bool wxWindow_IsBeingDeleted(const wxWindow * self) {
    return self->IsBeingDeleted();
}
wxDropTarget * wxWindow_GetDropTarget(const wxWindow * self) {
    return self->GetDropTarget();
}
void wxWindow_SetDropTarget(wxWindow * self, wxDropTarget * target) {
    return self->SetDropTarget(target);
}
void wxWindow_DragAcceptFiles(wxWindow * self, bool accept) {
    return self->DragAcceptFiles(accept);
}
wxSizer * wxWindow_GetContainingSizer(const wxWindow * self) {
    return self->GetContainingSizer();
}
wxSizer * wxWindow_GetSizer(const wxWindow * self) {
    return self->GetSizer();
}
void wxWindow_SetSizer(wxWindow * self, wxSizer * sizer, bool delete_old) {
    return self->SetSizer(sizer, delete_old);
}
void wxWindow_SetSizerAndFit(wxWindow * self, wxSizer * sizer, bool delete_old) {
    return self->SetSizerAndFit(sizer, delete_old);
}
wxLayoutConstraints * wxWindow_GetConstraints(const wxWindow * self) {
    return self->GetConstraints();
}
void wxWindow_SetConstraints(wxWindow * self, wxLayoutConstraints * constraints) {
    return self->SetConstraints(constraints);
}
bool wxWindow_Layout(wxWindow * self) {
    return self->Layout();
}
void wxWindow_SetAutoLayout(wxWindow * self, bool auto_layout) {
    return self->SetAutoLayout(auto_layout);
}
bool wxWindow_GetAutoLayout(const wxWindow * self) {
    return self->GetAutoLayout();
}
void wxWindow_CaptureMouse(wxWindow * self) {
    return self->CaptureMouse();
}
wxCaret * wxWindow_GetCaret(const wxWindow * self) {
    return self->GetCaret();
}
bool wxWindow_HasCapture(const wxWindow * self) {
    return self->HasCapture();
}
void wxWindow_ReleaseMouse(wxWindow * self) {
    return self->ReleaseMouse();
}
void wxWindow_SetCaret(wxWindow * self, wxCaret * caret) {
    return self->SetCaret(caret);
}
bool wxWindow_SetCursor(wxWindow * self, const wxCursor * cursor) {
    return self->SetCursor(*cursor);
}
void wxWindow_WarpPointer(wxWindow * self, int x, int y) {
    return self->WarpPointer(x, y);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_EnableTouchEvents(wxWindow * self, int events_mask) {
    return self->EnableTouchEvents(events_mask);
}
#endif
void wxWindow_DoUpdateWindowUI(wxWindow * self, wxUpdateUIEvent * event) {
    return self->DoUpdateWindowUI(*event);
}
bool wxWindow_HasMultiplePages(const wxWindow * self) {
    return self->HasMultiplePages();
}
void wxWindow_InheritAttributes(wxWindow * self) {
    return self->InheritAttributes();
}
void wxWindow_InitDialog(wxWindow * self) {
    return self->InitDialog();
}
bool wxWindow_IsDoubleBuffered(const wxWindow * self) {
    return self->IsDoubleBuffered();
}
void wxWindow_SetDoubleBuffered(wxWindow * self, bool on) {
    return self->SetDoubleBuffered(on);
}
bool wxWindow_IsRetained(const wxWindow * self) {
    return self->IsRetained();
}
bool wxWindow_IsThisEnabled(const wxWindow * self) {
    return self->IsThisEnabled();
}
bool wxWindow_IsTopLevel(const wxWindow * self) {
    return self->IsTopLevel();
}
void wxWindow_OnInternalIdle(wxWindow * self) {
    return self->OnInternalIdle();
}
bool wxWindow_SendIdleEvents(wxWindow * self, wxIdleEvent * event) {
    return self->SendIdleEvents(*event);
}
#ifndef __WXGTK__
bool wxWindow_RegisterHotKey(wxWindow * self, int hotkey_id, int modifiers, int virtual_key_code) {
    return self->RegisterHotKey(hotkey_id, modifiers, virtual_key_code);
}
bool wxWindow_UnregisterHotKey(wxWindow * self, int hotkey_id) {
    return self->UnregisterHotKey(hotkey_id);
}
#endif
void wxWindow_UpdateWindowUI(wxWindow * self, long flags) {
    return self->UpdateWindowUI(flags);
}
wxWindow * wxWindow_FindFocus() {
    return wxWindow::FindFocus();
}
wxWindow * wxWindow_FindWindowById(long id, const wxWindow * parent) {
    return wxWindow::FindWindowById(id, parent);
}
wxWindow * wxWindow_FindWindowByLabel(const wxString * label, const wxWindow * parent) {
    return wxWindow::FindWindowByLabel(*label, parent);
}
wxWindow * wxWindow_FindWindowByName(const wxString * name, const wxWindow * parent) {
    return wxWindow::FindWindowByName(*name, parent);
}
wxWindow * wxWindow_GetCapture() {
    return wxWindow::GetCapture();
}
wxWindowID wxWindow_NewControlId(int count) {
    return wxWindow::NewControlId(count);
}
void wxWindow_UnreserveControlId(wxWindowID id, int count) {
    return wxWindow::UnreserveControlId(id, count);
}
wxWindow *wxWindow_new() {
    return new wxWindow();
}
wxWindow *wxWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxWindow(parent, id, *pos, *size, style, *name);
}
bool wxWindow_Create(wxWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
// Mix-in(s) to wxWindow
wxTrackable *wxWindow_AsTrackable(wxWindow* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxWindowCreateEvent
wxClassInfo *wxWindowCreateEvent_CLASSINFO() {
    return wxCLASSINFO(wxWindowCreateEvent);
}
wxWindowCreateEvent *wxWindowCreateEvent_new(wxWindow * win) {
    return new wxWindowCreateEvent(win);
}
wxWindow * wxWindowCreateEvent_GetWindow(const wxWindowCreateEvent * self) {
    return self->GetWindow();
}

// CLASS: wxWindowDC
wxClassInfo *wxWindowDC_CLASSINFO() {
    return wxCLASSINFO(wxWindowDC);
}
wxWindowDC *wxWindowDC_new(wxWindow * window) {
    return new wxWindowDC(window);
}

// CLASS: wxWindowDestroyEvent
wxClassInfo *wxWindowDestroyEvent_CLASSINFO() {
    return wxCLASSINFO(wxWindowDestroyEvent);
}
wxWindowDestroyEvent *wxWindowDestroyEvent_new(wxWindow * win) {
    return new wxWindowDestroyEvent(win);
}
wxWindow * wxWindowDestroyEvent_GetWindow(const wxWindowDestroyEvent * self) {
    return self->GetWindow();
}

// CLASS: wxWindowDisabler
void wxWindowDisabler_delete(wxWindowDisabler *self) {
    delete self;
}
wxWindowDisabler *wxWindowDisabler_new(bool disable) {
    return new wxWindowDisabler(disable);
}
wxWindowDisabler *wxWindowDisabler_new1(wxWindow * win_to_skip, wxWindow * win_to_skip2) {
    return new wxWindowDisabler(win_to_skip, win_to_skip2);
}

// CLASS: wxWizard
wxClassInfo *wxWizard_CLASSINFO() {
    return wxCLASSINFO(wxWizard);
}
wxWizard *wxWizard_new() {
    return new wxWizard();
}
wxWizard *wxWizard_new1(wxWindow * parent, int id, const wxString * title, const wxBitmapBundle * bitmap, const wxPoint * pos, long style) {
    return new wxWizard(parent, id, *title, *bitmap, *pos, style);
}
bool wxWizard_Create(wxWizard * self, wxWindow * parent, int id, const wxString * title, const wxBitmapBundle * bitmap, const wxPoint * pos, long style) {
    return self->Create(parent, id, *title, *bitmap, *pos, style);
}
void wxWizard_FitToPage(wxWizard * self, const wxWizardPage * first_page) {
    return self->FitToPage(first_page);
}
wxBitmap *wxWizard_GetBitmap(const wxWizard * self) {
    return new wxBitmap(self->GetBitmap());
}
wxColour *wxWizard_GetBitmapBackgroundColour(const wxWizard * self) {
    return new wxColour(self->GetBitmapBackgroundColour());
}
int wxWizard_GetBitmapPlacement(const wxWizard * self) {
    return self->GetBitmapPlacement();
}
wxWizardPage * wxWizard_GetCurrentPage(const wxWizard * self) {
    return self->GetCurrentPage();
}
int wxWizard_GetMinimumBitmapWidth(const wxWizard * self) {
    return self->GetMinimumBitmapWidth();
}
wxSizer * wxWizard_GetPageAreaSizer(const wxWizard * self) {
    return self->GetPageAreaSizer();
}
wxSize *wxWizard_GetPageSize(const wxWizard * self) {
    return new wxSize(self->GetPageSize());
}
bool wxWizard_HasNextPage(wxWizard * self, wxWizardPage * page) {
    return self->HasNextPage(page);
}
bool wxWizard_HasPrevPage(wxWizard * self, wxWizardPage * page) {
    return self->HasPrevPage(page);
}
bool wxWizard_RunWizard(wxWizard * self, wxWizardPage * first_page) {
    return self->RunWizard(first_page);
}
void wxWizard_SetBitmap(wxWizard * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmap(*bitmap);
}
void wxWizard_SetBitmapBackgroundColour(wxWizard * self, const wxColour * colour) {
    return self->SetBitmapBackgroundColour(*colour);
}
void wxWizard_SetBitmapPlacement(wxWizard * self, int placement) {
    return self->SetBitmapPlacement(placement);
}
void wxWizard_SetBorder(wxWizard * self, int border) {
    return self->SetBorder(border);
}
void wxWizard_SetMinimumBitmapWidth(wxWizard * self, int width) {
    return self->SetMinimumBitmapWidth(width);
}
void wxWizard_SetPageSize(wxWizard * self, const wxSize * size_page) {
    return self->SetPageSize(*size_page);
}
// Mix-in(s) to wxWizard
wxTrackable *wxWizard_AsTrackable(wxWizard* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxWizardEvent
wxClassInfo *wxWizardEvent_CLASSINFO() {
    return wxCLASSINFO(wxWizardEvent);
}
bool wxWizardEvent_GetDirection(const wxWizardEvent * self) {
    return self->GetDirection();
}
wxWizardPage * wxWizardEvent_GetPage(const wxWizardEvent * self) {
    return self->GetPage();
}

// CLASS: wxWizardPage
wxClassInfo *wxWizardPage_CLASSINFO() {
    return wxCLASSINFO(wxWizardPage);
}
wxWizardPage *wxWizardPage_new() {
    return new wxWizardPage();
}
wxWizardPage *wxWizardPage_new1(wxWizard * parent, const wxBitmapBundle * bitmap) {
    return new wxWizardPage(parent, *bitmap);
}
bool wxWizardPage_Create(wxWizardPage * self, wxWizard * parent, const wxBitmapBundle * bitmap) {
    return self->Create(parent, *bitmap);
}
wxBitmap *wxWizardPage_GetBitmap(const wxWizardPage * self) {
    return new wxBitmap(self->GetBitmap());
}
wxWizardPage * wxWizardPage_GetNext(const wxWizardPage * self) {
    return self->GetNext();
}
wxWizardPage * wxWizardPage_GetPrev(const wxWizardPage * self) {
    return self->GetPrev();
}
// Mix-in(s) to wxWizardPage
wxTrackable *wxWizardPage_AsTrackable(wxWizardPage* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxWizardPageSimple
wxClassInfo *wxWizardPageSimple_CLASSINFO() {
    return wxCLASSINFO(wxWizardPageSimple);
}
wxWizardPageSimple *wxWizardPageSimple_new() {
    return new wxWizardPageSimple();
}
wxWizardPageSimple *wxWizardPageSimple_new1(wxWizard * parent, wxWizardPage * prev, wxWizardPage * next, const wxBitmapBundle * bitmap) {
    return new wxWizardPageSimple(parent, prev, next, *bitmap);
}
bool wxWizardPageSimple_Create(wxWizardPageSimple * self, wxWizard * parent, wxWizardPage * prev, wxWizardPage * next, const wxBitmapBundle * bitmap) {
    return self->Create(parent, prev, next, *bitmap);
}
wxWizardPageSimple * wxWizardPageSimple_Chain(wxWizardPageSimple * self, wxWizardPageSimple * next) {
    return &(self->Chain(next));
}
void wxWizardPageSimple_SetNext(wxWizardPageSimple * self, wxWizardPage * next) {
    return self->SetNext(next);
}
void wxWizardPageSimple_SetPrev(wxWizardPageSimple * self, wxWizardPage * prev) {
    return self->SetPrev(prev);
}
void wxWizardPageSimple_Chain1(wxWizardPageSimple * first, wxWizardPageSimple * second) {
    return wxWizardPageSimple::Chain(first, second);
}
// Mix-in(s) to wxWizardPageSimple
wxTrackable *wxWizardPageSimple_AsTrackable(wxWizardPageSimple* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxWrapSizer
wxClassInfo *wxWrapSizer_CLASSINFO() {
    return wxCLASSINFO(wxWrapSizer);
}
wxWrapSizer *wxWrapSizer_new(int orient, int flags) {
    return new wxWrapSizer(orient, flags);
}

// CLASS: wxXPMHandler
wxClassInfo *wxXPMHandler_CLASSINFO() {
    return wxCLASSINFO(wxXPMHandler);
}
wxXPMHandler *wxXPMHandler_new() {
    return new wxXPMHandler();
}

// CLASS: wxZoomGestureEvent
wxClassInfo *wxZoomGestureEvent_CLASSINFO() {
    return wxCLASSINFO(wxZoomGestureEvent);
}
wxZoomGestureEvent *wxZoomGestureEvent_new(wxWindowID windid) {
    return new wxZoomGestureEvent(windid);
}
double wxZoomGestureEvent_GetZoomFactor(const wxZoomGestureEvent * self) {
    return self->GetZoomFactor();
}
void wxZoomGestureEvent_SetZoomFactor(wxZoomGestureEvent * self, double zoom_factor) {
    return self->SetZoomFactor(zoom_factor);
}

} // extern "C"

