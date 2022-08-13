use super::*;

extern "C" {

    // wxAboutDialogInfo
    pub fn wxAboutDialogInfo_delete(self_: *mut c_void);
    pub fn wxAboutDialogInfo_new() -> *mut c_void;
    pub fn wxAboutDialogInfo_AddArtist(self_: *mut c_void, artist: *const c_void);
    pub fn wxAboutDialogInfo_AddDeveloper(self_: *mut c_void, developer: *const c_void);
    pub fn wxAboutDialogInfo_AddDocWriter(self_: *mut c_void, docwriter: *const c_void);
    pub fn wxAboutDialogInfo_AddTranslator(self_: *mut c_void, translator: *const c_void);
    pub fn wxAboutDialogInfo_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_HasDescription(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetDescription(self_: *mut c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_HasCopyright(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetCopyright(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_SetArtists(self_: *mut c_void, artists: *const c_void);
    pub fn wxAboutDialogInfo_SetCopyright(self_: *mut c_void, copyright: *const c_void);
    pub fn wxAboutDialogInfo_SetDescription(self_: *mut c_void, desc: *const c_void);
    pub fn wxAboutDialogInfo_SetDevelopers(self_: *mut c_void, developers: *const c_void);
    pub fn wxAboutDialogInfo_SetDocWriters(self_: *mut c_void, docwriters: *const c_void);
    pub fn wxAboutDialogInfo_HasIcon(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetIcon(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxAboutDialogInfo_HasLicence(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetLicence(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_SetLicence(self_: *mut c_void, licence: *const c_void);
    pub fn wxAboutDialogInfo_SetLicense(self_: *mut c_void, licence: *const c_void);
    pub fn wxAboutDialogInfo_SetName(self_: *mut c_void, name: *const c_void);
    pub fn wxAboutDialogInfo_SetTranslators(self_: *mut c_void, translators: *const c_void);
    pub fn wxAboutDialogInfo_SetVersion(
        self_: *mut c_void,
        version: *const c_void,
        long_version: *const c_void,
    );
    pub fn wxAboutDialogInfo_GetVersion(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_GetLongVersion(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_HasWebSite(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetWebSiteURL(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_GetWebSiteDescription(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_SetWebSite(
        self_: *mut c_void,
        url: *const c_void,
        desc: *const c_void,
    );
    pub fn wxAboutDialogInfo_HasDevelopers(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetDevelopers(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_HasDocWriters(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetDocWriters(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_HasArtists(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetArtists(self_: *const c_void) -> *mut c_void;
    pub fn wxAboutDialogInfo_HasTranslators(self_: *const c_void) -> bool;
    pub fn wxAboutDialogInfo_GetTranslators(self_: *const c_void) -> *mut c_void;

    // wxAcceleratorEntry
    pub fn wxAcceleratorEntry_delete(self_: *mut c_void);
    pub fn wxAcceleratorEntry_new(
        flags: c_int,
        key_code: c_int,
        cmd: c_int,
        item: *mut c_void,
    ) -> *mut c_void;
    pub fn wxAcceleratorEntry_new1(entry: *const c_void) -> *mut c_void;
    pub fn wxAcceleratorEntry_GetCommand(self_: *const c_void) -> c_int;
    pub fn wxAcceleratorEntry_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxAcceleratorEntry_GetKeyCode(self_: *const c_void) -> c_int;
    pub fn wxAcceleratorEntry_GetMenuItem(self_: *const c_void) -> *mut c_void;
    pub fn wxAcceleratorEntry_Set(
        self_: *mut c_void,
        flags: c_int,
        key_code: c_int,
        cmd: c_int,
        item: *mut c_void,
    );
    pub fn wxAcceleratorEntry_IsOk(self_: *const c_void) -> bool;
    pub fn wxAcceleratorEntry_ToString(self_: *const c_void) -> *mut c_void;
    pub fn wxAcceleratorEntry_ToRawString(self_: *const c_void) -> *mut c_void;
    pub fn wxAcceleratorEntry_FromString(self_: *mut c_void, str: *const c_void) -> bool;
    // BLOCKED: pub fn wxAcceleratorEntry_operator=(self_: *mut c_void, entry: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxAcceleratorEntry_operator==(self_: *const c_void, entry: *const c_void) -> bool;
    // BLOCKED: pub fn wxAcceleratorEntry_operator!=(self_: *const c_void, entry: *const c_void) -> bool;

    // wxAcceleratorTable
    pub fn wxAcceleratorTable_CLASSINFO() -> *mut c_void;
    pub fn wxAcceleratorTable_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxAcceleratorTable_new1(n: c_int, entries: wxAcceleratorEntry) -> *mut c_void;
    // BLOCKED: pub fn wxAcceleratorTable_new2(resource: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxAcceleratorTable_~wxAcceleratorTable(self_: *mut c_void);
    pub fn wxAcceleratorTable_IsOk(self_: *const c_void) -> bool;

    // wxActivateEvent
    pub fn wxActivateEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxActivateEvent_new(event_type: wxEventType, active: bool, id: c_int, activation_reason: Reason) -> *mut c_void;
    pub fn wxActivateEvent_GetActive(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxActivateEvent_GetActivationReason(self_: *const c_void) -> Reason;

    // wxAddRemoveAdaptor
    pub fn wxAddRemoveAdaptor_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxAddRemoveAdaptor_new() -> *mut c_void;
    // DTOR: pub fn wxAddRemoveAdaptor_~wxAddRemoveAdaptor(self_: *mut c_void);
    pub fn wxAddRemoveAdaptor_GetItemsCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxAddRemoveAdaptor_CanAdd(self_: *const c_void) -> bool;
    pub fn wxAddRemoveAdaptor_CanRemove(self_: *const c_void) -> bool;
    pub fn wxAddRemoveAdaptor_OnAdd(self_: *mut c_void);
    pub fn wxAddRemoveAdaptor_OnRemove(self_: *mut c_void);

    // wxAddRemoveCtrl
    pub fn wxAddRemoveCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxAddRemoveCtrl_new() -> *mut c_void;
    pub fn wxAddRemoveCtrl_new1(
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxAddRemoveCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxAddRemoveCtrl_SetAdaptor(self_: *mut c_void, adaptor: *mut c_void);
    pub fn wxAddRemoveCtrl_SetButtonsToolTips(
        self_: *mut c_void,
        addtip: *const c_void,
        removetip: *const c_void,
    );

    // wxAffineMatrix2D
    pub fn wxAffineMatrix2D_delete(self_: *mut c_void);
    pub fn wxAffineMatrix2D_new() -> *mut c_void;
    // BLOCKED: pub fn wxAffineMatrix2D_IsEqual(self_: *mut c_void, t: *const c_void);
    // BLOCKED: pub fn wxAffineMatrix2D_operator==(self_: *const c_void, t: *const c_void) -> bool;
    // BLOCKED: pub fn wxAffineMatrix2D_operator!=(self_: *const c_void, t: *const c_void) -> bool;
    pub fn wxAffineMatrix2D_Mirror(self_: *mut c_void, direction: c_int);
    // NOT_SUPPORTED: pub fn wxAffineMatrix2D_TransformPoint(self_: *const c_void, p: *const c_void) -> wxPoint2DDouble;
    pub fn wxAffineMatrix2D_TransformPoint1(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    // NOT_SUPPORTED: pub fn wxAffineMatrix2D_TransformDistance(self_: *const c_void, p: *const c_void) -> wxPoint2DDouble;
    pub fn wxAffineMatrix2D_TransformDistance1(
        self_: *const c_void,
        dx: *mut c_void,
        dy: *mut c_void,
    );

    // wxAffineMatrix2DBase
    pub fn wxAffineMatrix2DBase_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxAffineMatrix2DBase_new() -> *mut c_void;
    // DTOR: pub fn wxAffineMatrix2DBase_~wxAffineMatrix2DBase(self_: *mut c_void);
    pub fn wxAffineMatrix2DBase_Set(self_: *mut c_void, mat2_d: *const c_void, tr: *const c_void);
    pub fn wxAffineMatrix2DBase_Get(self_: *const c_void, mat2_d: *mut c_void, tr: *mut c_void);
    pub fn wxAffineMatrix2DBase_Concat(self_: *mut c_void, t: *const c_void);
    pub fn wxAffineMatrix2DBase_Invert(self_: *mut c_void) -> bool;
    pub fn wxAffineMatrix2DBase_IsIdentity(self_: *const c_void) -> bool;
    pub fn wxAffineMatrix2DBase_IsEqual(self_: *const c_void, t: *const c_void) -> bool;
    // BLOCKED: pub fn wxAffineMatrix2DBase_operator==(self_: *const c_void, t: *const c_void) -> bool;
    // BLOCKED: pub fn wxAffineMatrix2DBase_operator!=(self_: *const c_void, t: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxAffineMatrix2DBase_Translate(self_: *mut c_void, dx: wxDouble, dy: wxDouble);
    // NOT_SUPPORTED: pub fn wxAffineMatrix2DBase_Scale(self_: *mut c_void, x_scale: wxDouble, y_scale: wxDouble);
    // NOT_SUPPORTED: pub fn wxAffineMatrix2DBase_Rotate(self_: *mut c_void, c_radians: wxDouble);
    pub fn wxAffineMatrix2DBase_Mirror(self_: *mut c_void, direction: c_int);
    // NOT_SUPPORTED: pub fn wxAffineMatrix2DBase_TransformPoint(self_: *const c_void, p: *const c_void) -> wxPoint2DDouble;
    pub fn wxAffineMatrix2DBase_TransformPoint1(
        self_: *const c_void,
        x: *mut c_void,
        y: *mut c_void,
    );
    // NOT_SUPPORTED: pub fn wxAffineMatrix2DBase_TransformDistance(self_: *const c_void, p: *const c_void) -> wxPoint2DDouble;
    pub fn wxAffineMatrix2DBase_TransformDistance1(
        self_: *const c_void,
        dx: *mut c_void,
        dy: *mut c_void,
    );

    // wxAnimationCtrl
    pub fn wxAnimationCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxAnimationCtrl_new(
        parent: *mut c_void,
        id: c_int,
        anim: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxAnimationCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        anim: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxAnimationCtrl_CreateAnimation(self_: *const c_void) -> wxAnimation;
    // NOT_SUPPORTED: pub fn wxAnimationCtrl_GetAnimation(self_: *const c_void) -> wxAnimation;
    pub fn wxAnimationCtrl_GetInactiveBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxAnimationCtrl_IsPlaying(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxAnimationCtrl_LoadFile(self_: *mut c_void, file: *const c_void, anim_type: wxAnimationType) -> bool;
    // NOT_SUPPORTED: pub fn wxAnimationCtrl_Load(self_: *mut c_void, file: *mut c_void, anim_type: wxAnimationType) -> bool;
    pub fn wxAnimationCtrl_Play(self_: *mut c_void) -> bool;
    pub fn wxAnimationCtrl_SetAnimation(self_: *mut c_void, anim: *const c_void);
    pub fn wxAnimationCtrl_SetInactiveBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxAnimationCtrl_Stop(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxAnimationCtrl_CreateCompatibleAnimation() -> wxAnimation;

    // wxAnyButton
    pub fn wxAnyButton_CLASSINFO() -> *mut c_void;
    pub fn wxAnyButton_new() -> *mut c_void;
    // DTOR: pub fn wxAnyButton_~wxAnyButton(self_: *mut c_void);
    pub fn wxAnyButton_GetBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapCurrent(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapDisabled(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapFocus(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapPressed(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_SetBitmap(self_: *mut c_void, bitmap: *const c_void, dir: c_int);
    pub fn wxAnyButton_SetBitmapCurrent(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_SetBitmapDisabled(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_SetBitmapFocus(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_SetBitmapLabel(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_SetBitmapPressed(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_GetBitmapMargins(self_: *mut c_void) -> *mut c_void;
    pub fn wxAnyButton_SetBitmapMargins(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxAnyButton_SetBitmapMargins1(self_: *mut c_void, sz: *const c_void);
    pub fn wxAnyButton_SetBitmapPosition(self_: *mut c_void, dir: c_int);

    // wxAppProgressIndicator
    pub fn wxAppProgressIndicator_delete(self_: *mut c_void);
    pub fn wxAppProgressIndicator_new(parent: *mut c_void, max_value: c_int) -> *mut c_void;
    // DTOR: pub fn wxAppProgressIndicator_~wxAppProgressIndicator(self_: *mut c_void);
    pub fn wxAppProgressIndicator_IsAvailable(self_: *const c_void) -> bool;
    pub fn wxAppProgressIndicator_SetValue(self_: *mut c_void, value: c_int);
    pub fn wxAppProgressIndicator_SetRange(self_: *mut c_void, range: c_int);
    // BLOCKED: pub fn wxAppProgressIndicator_Pulse(self_: *mut c_void) -> bool;

    // wxArtProvider
    pub fn wxArtProvider_CLASSINFO() -> *mut c_void;
    // DTOR: pub fn wxArtProvider_~wxArtProvider(self_: *mut c_void);
    pub fn wxArtProvider_Delete(provider: *mut c_void) -> bool;
    pub fn wxArtProvider_GetBitmap(
        id: *const c_void,
        client: *const c_void,
        size: *const c_void,
    ) -> *mut c_void;
    pub fn wxArtProvider_GetBitmapBundle(
        id: *const c_void,
        client: *const c_void,
        size: *const c_void,
    ) -> *mut c_void;
    pub fn wxArtProvider_GetIcon(
        id: *const c_void,
        client: *const c_void,
        size: *const c_void,
    ) -> *mut c_void;
    pub fn wxArtProvider_GetNativeDIPSizeHint(client: *const c_void) -> *mut c_void;
    pub fn wxArtProvider_GetNativeSizeHint(client: *const c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxArtProvider_GetDIPSizeHint(client: *const c_void) -> *mut c_void;
    pub fn wxArtProvider_GetSizeHint(client: *const c_void, win: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxArtProvider_GetIconBundle(id: *const c_void, client: *const c_void) -> wxIconBundle;
    pub fn wxArtProvider_HasNativeProvider() -> bool;
    // BLOCKED: pub fn wxArtProvider_Insert(provider: *mut c_void);
    pub fn wxArtProvider_Pop() -> bool;
    pub fn wxArtProvider_Push(provider: *mut c_void);
    pub fn wxArtProvider_PushBack(provider: *mut c_void);
    pub fn wxArtProvider_Remove(provider: *mut c_void) -> bool;
    pub fn wxArtProvider_GetMessageBoxIconId(flags: c_int) -> *mut c_void;
    pub fn wxArtProvider_GetMessageBoxIcon(flags: c_int) -> *mut c_void;

}
