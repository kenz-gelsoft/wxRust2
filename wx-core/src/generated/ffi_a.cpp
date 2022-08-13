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
bool wxAcceleratorTable_IsOk(const wxAcceleratorTable * self) {
    return self->IsOk();
}

// CLASS: wxActivateEvent
wxClassInfo *wxActivateEvent_CLASSINFO() {
    return wxCLASSINFO(wxActivateEvent);
}
bool wxActivateEvent_GetActive(const wxActivateEvent * self) {
    return self->GetActive();
}

// CLASS: wxAffineMatrix2D
void wxAffineMatrix2D_delete(wxAffineMatrix2D *self) {
    delete self;
}
wxAffineMatrix2D *wxAffineMatrix2D_new() {
    return new wxAffineMatrix2D();
}
void wxAffineMatrix2D_Mirror(wxAffineMatrix2D * self, int direction) {
    return self->Mirror(direction);
}
void wxAffineMatrix2D_TransformPoint1(const wxAffineMatrix2D * self, wxDouble * x, wxDouble * y) {
    return self->TransformPoint(x, y);
}
void wxAffineMatrix2D_TransformDistance1(const wxAffineMatrix2D * self, wxDouble * dx, wxDouble * dy) {
    return self->TransformDistance(dx, dy);
}

// CLASS: wxAffineMatrix2DBase
void wxAffineMatrix2DBase_delete(wxAffineMatrix2DBase *self) {
    delete self;
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
void wxAffineMatrix2DBase_TransformPoint1(const wxAffineMatrix2DBase * self, wxDouble * x, wxDouble * y) {
    return self->TransformPoint(x, y);
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

