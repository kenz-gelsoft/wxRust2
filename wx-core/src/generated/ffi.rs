use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

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

    // wxActivateEvent
    pub fn wxActivateEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxActivateEvent_new(event_type: wxEventType, active: bool, id: c_int, activation_reason: Reason) -> *mut c_void;
    pub fn wxActivateEvent_GetActive(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxActivateEvent_GetActivationReason(self_: *const c_void) -> Reason;

    // wxActivityIndicator
    pub fn wxActivityIndicator_CLASSINFO() -> *mut c_void;
    pub fn wxActivityIndicator_new() -> *mut c_void;
    pub fn wxActivityIndicator_new1(
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxActivityIndicator_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxActivityIndicator_Start(self_: *mut c_void);
    pub fn wxActivityIndicator_Stop(self_: *mut c_void);
    pub fn wxActivityIndicator_IsRunning(self_: *const c_void) -> bool;

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
    pub fn wxAnimationCtrl_CreateAnimation(self_: *const c_void) -> *mut c_void;
    pub fn wxAnimationCtrl_GetAnimation(self_: *const c_void) -> *mut c_void;
    pub fn wxAnimationCtrl_GetInactiveBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxAnimationCtrl_IsPlaying(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxAnimationCtrl_LoadFile(self_: *mut c_void, file: *const c_void, anim_type: wxAnimationType) -> bool;
    // NOT_SUPPORTED: pub fn wxAnimationCtrl_Load(self_: *mut c_void, file: *mut c_void, anim_type: wxAnimationType) -> bool;
    pub fn wxAnimationCtrl_Play(self_: *mut c_void) -> bool;
    pub fn wxAnimationCtrl_SetAnimation(self_: *mut c_void, anim: *const c_void);
    pub fn wxAnimationCtrl_SetInactiveBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxAnimationCtrl_Stop(self_: *mut c_void);
    pub fn wxAnimationCtrl_CreateCompatibleAnimation() -> *mut c_void;

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
    pub fn wxArtProvider_GetIconBundle(id: *const c_void, client: *const c_void) -> *mut c_void;
    pub fn wxArtProvider_HasNativeProvider() -> bool;
    // BLOCKED: pub fn wxArtProvider_Insert(provider: *mut c_void);
    pub fn wxArtProvider_Pop() -> bool;
    pub fn wxArtProvider_Push(provider: *mut c_void);
    pub fn wxArtProvider_PushBack(provider: *mut c_void);
    pub fn wxArtProvider_Remove(provider: *mut c_void) -> bool;
    pub fn wxArtProvider_GetMessageBoxIconId(flags: c_int) -> *mut c_void;
    pub fn wxArtProvider_GetMessageBoxIcon(flags: c_int) -> *mut c_void;

    // wxAutoBufferedPaintDC
    pub fn wxAutoBufferedPaintDC_CLASSINFO() -> *mut c_void;
    pub fn wxAutoBufferedPaintDC_new(window: *mut c_void) -> *mut c_void;

    // wxBannerWindow
    pub fn wxBannerWindow_CLASSINFO() -> *mut c_void;
    pub fn wxBannerWindow_new() -> *mut c_void;
    pub fn wxBannerWindow_new1(parent: *mut c_void, dir: c_int) -> *mut c_void;
    pub fn wxBannerWindow_new2(
        parent: *mut c_void,
        winid: c_int,
        dir: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxBannerWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        dir: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxBannerWindow_SetBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxBannerWindow_SetText(self_: *mut c_void, title: *const c_void, message: *const c_void);
    pub fn wxBannerWindow_SetGradient(self_: *mut c_void, start: *const c_void, end: *const c_void);

    // wxBitmap
    pub fn wxBitmap_CLASSINFO() -> *mut c_void;
    pub fn wxBitmap_new() -> *mut c_void;
    pub fn wxBitmap_new1(bitmap: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_new2(bits: char, width: c_int, height: c_int, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new3(width: c_int, height: c_int, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new4(sz: *const c_void, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new5(width: c_int, height: c_int, dc: *const c_void) -> *mut c_void;
    pub fn wxBitmap_new6(bits: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_new7(name: *const c_void, type_: wxBitmapType) -> *mut c_void;
    pub fn wxBitmap_new8(img: *const c_void, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new9(img: *const c_void, dc: *const c_void) -> *mut c_void;
    pub fn wxBitmap_new10(cursor: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxBitmap_~wxBitmap(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBitmap_ConvertToDisabled(self_: *const c_void, brightness: unsigned char) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_ConvertToImage(self_: *const c_void) -> wxImage;
    pub fn wxBitmap_CopyFromIcon(self_: *mut c_void, icon: *const c_void) -> bool;
    pub fn wxBitmap_Create(self_: *mut c_void, width: c_int, height: c_int, depth: c_int) -> bool;
    pub fn wxBitmap_Create1(self_: *mut c_void, sz: *const c_void, depth: c_int) -> bool;
    pub fn wxBitmap_Create2(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        dc: *const c_void,
    ) -> bool;
    pub fn wxBitmap_CreateWithDIPSize(
        self_: *mut c_void,
        size: *const c_void,
        scale: c_double,
        depth: c_int,
    ) -> bool;
    pub fn wxBitmap_CreateWithDIPSize1(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        scale: c_double,
        depth: c_int,
    ) -> bool;
    pub fn wxBitmap_CreateScaled(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        depth: c_int,
        logical_scale: c_double,
    ) -> bool;
    pub fn wxBitmap_GetDepth(self_: *const c_void) -> c_int;
    pub fn wxBitmap_GetDIPSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxBitmap_GetLogicalHeight(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetLogicalSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetLogicalWidth(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetMask(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetPalette(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetSubBitmap(self_: *const c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetScaledHeight(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetScaledSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetScaledWidth(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxBitmap_HasAlpha(self_: *const c_void) -> bool;
    pub fn wxBitmap_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxBitmap_LoadFile(self_: *mut c_void, name: *const c_void, type_: wxBitmapType) -> bool;
    // BLOCKED: pub fn wxBitmap_ResetAlpha(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBitmap_SaveFile(self_: *const c_void, name: *const c_void, type_: wxBitmapType, palette: *const c_void) -> bool;
    pub fn wxBitmap_SetDepth(self_: *mut c_void, depth: c_int);
    pub fn wxBitmap_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxBitmap_SetScaleFactor(self_: *mut c_void, scale: c_double);
    pub fn wxBitmap_SetMask(self_: *mut c_void, mask: *mut c_void);
    pub fn wxBitmap_SetPalette(self_: *mut c_void, palette: *const c_void);
    pub fn wxBitmap_SetWidth(self_: *mut c_void, width: c_int);
    // BLOCKED: pub fn wxBitmap_UseAlpha(self_: *mut c_void, use_: bool) -> bool;
    pub fn wxBitmap_AddHandler(handler: *mut c_void);
    pub fn wxBitmap_CleanUpHandlers();
    pub fn wxBitmap_FindHandler(name: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_FindHandler1(extension: *const c_void, bitmap_type: wxBitmapType) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_FindHandler2(bitmap_type: wxBitmapType) -> *mut c_void;
    // BLOCKED: pub fn wxBitmap_GetHandlers() -> *mut c_void;
    pub fn wxBitmap_InitStandardHandlers();
    pub fn wxBitmap_InsertHandler(handler: *mut c_void);
    pub fn wxBitmap_NewFromPNGData(data: *const c_void, size: usize) -> *mut c_void;
    pub fn wxBitmap_RemoveHandler(name: *const c_void) -> bool;
    pub fn wxBitmap_Rescale(bmp: *mut c_void, size_needed: *const c_void);

    // wxBitmapBundle
    pub fn wxBitmapBundle_delete(self_: *mut c_void);
    pub fn wxBitmapBundle_new() -> *mut c_void;
    pub fn wxBitmapBundle_new1(bitmap: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_new2(icon: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_new3(image: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_new4(xpm: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_new5(other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxBitmapBundle_operator=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_Clear(self_: *mut c_void);
    pub fn wxBitmapBundle_IsOk(self_: *const c_void) -> bool;
    pub fn wxBitmapBundle_GetDefaultSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_GetPreferredBitmapSizeAtScale(
        self_: *const c_void,
        scale: c_double,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_GetPreferredBitmapSizeFor(
        self_: *const c_void,
        window: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_GetPreferredLogicalSizeFor(
        self_: *const c_void,
        window: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_GetBitmap(self_: *const c_void, size: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_GetBitmapFor(self_: *const c_void, window: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_GetIcon(self_: *const c_void, size: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_GetIconFor(self_: *const c_void, window: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_IsSameAs(self_: *const c_void, other: *const c_void) -> bool;
    // BLOCKED: pub fn wxBitmapBundle_FromBitmaps(bitmaps: *const c_void) -> wxBitmapBundle;
    pub fn wxBitmapBundle_FromBitmaps1(
        bitmap1: *const c_void,
        bitmap2: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_FromBitmap(bitmap: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromIconBundle(icon_bundle: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromImage(image: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromImpl(impl_: *mut c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromResources(name: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromFiles(
        path: *const c_void,
        filename: *const c_void,
        extension: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_FromFiles1(fullpathname: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxBitmapBundle_FromSVG(data: *mut c_void, size_def: *const c_void) -> wxBitmapBundle;
    pub fn wxBitmapBundle_FromSVG1(data: *const c_void, size_def: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromSVGFile(path: *const c_void, size_def: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromSVGResource(
        name: *const c_void,
        size_def: *const c_void,
    ) -> *mut c_void;

    // wxBitmapButton
    pub fn wxBitmapButton_CLASSINFO() -> *mut c_void;
    pub fn wxBitmapButton_new() -> *mut c_void;
    pub fn wxBitmapButton_new1(
        parent: *mut c_void,
        id: c_int,
        bitmap: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        bitmap: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxBitmapButton_CreateCloseButton(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        name: *const c_void,
    ) -> bool;
    pub fn wxBitmapButton_NewCloseButton(
        parent: *mut c_void,
        winid: c_int,
        name: *const c_void,
    ) -> *mut c_void;

    // wxBitmapComboBox
    pub fn wxBitmapComboBox_CLASSINFO() -> *mut c_void;
    pub fn wxBitmapComboBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmapComboBox_new1(parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxBitmapComboBox_new2(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxBitmapComboBox_~wxBitmapComboBox(self_: *mut c_void);
    pub fn wxBitmapComboBox_Append(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
    ) -> c_int;
    pub fn wxBitmapComboBox_Append1(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxBitmapComboBox_Append2(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    // NOT_SUPPORTED: pub fn wxBitmapComboBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxBitmapComboBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxBitmapComboBox_GetBitmapSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmapComboBox_GetItemBitmap(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxBitmapComboBox_Insert(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        pos: c_uint,
    ) -> c_int;
    pub fn wxBitmapComboBox_Insert1(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxBitmapComboBox_Insert2(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxBitmapComboBox_SetItemBitmap(self_: *mut c_void, n: c_uint, bitmap: *const c_void);
    // Mix-in(s) to wxBitmapComboBox
    pub fn wxBitmapComboBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;
    pub fn wxBitmapComboBox_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxBitmapHandler
    pub fn wxBitmapHandler_CLASSINFO() -> *mut c_void;
    pub fn wxBitmapHandler_new() -> *mut c_void;
    // DTOR: pub fn wxBitmapHandler_~wxBitmapHandler(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBitmapHandler_Create(self_: *mut c_void, bitmap: *mut c_void, data: *const c_void, type_: wxBitmapType, width: c_int, height: c_int, depth: c_int) -> bool;
    pub fn wxBitmapHandler_GetExtension(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmapHandler_GetName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmapHandler_GetType(self_: *const c_void) -> wxBitmapType;
    // NOT_SUPPORTED: pub fn wxBitmapHandler_LoadFile(self_: *mut c_void, bitmap: *mut c_void, name: *const c_void, type_: wxBitmapType, desired_width: c_int, desired_height: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxBitmapHandler_SaveFile(self_: *const c_void, bitmap: *const c_void, name: *const c_void, type_: wxBitmapType, palette: *const c_void) -> bool;
    pub fn wxBitmapHandler_SetExtension(self_: *mut c_void, extension: *const c_void);
    pub fn wxBitmapHandler_SetName(self_: *mut c_void, name: *const c_void);
    // NOT_SUPPORTED: pub fn wxBitmapHandler_SetType(self_: *mut c_void, type_: wxBitmapType);

    // wxBitmapToggleButton
    pub fn wxBitmapToggleButton_CLASSINFO() -> *mut c_void;
    pub fn wxBitmapToggleButton_new() -> *mut c_void;
    pub fn wxBitmapToggleButton_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        val: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapToggleButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        val: *const c_void,
        name: *const c_void,
    ) -> bool;

    // wxBookCtrlBase
    pub fn wxBookCtrlBase_CLASSINFO() -> *mut c_void;
    pub fn wxBookCtrlBase_GetPageImage(self_: *const c_void, n_page: usize) -> c_int;
    pub fn wxBookCtrlBase_SetPageImage(self_: *mut c_void, page: usize, image: c_int) -> bool;
    pub fn wxBookCtrlBase_GetPageText(self_: *const c_void, n_page: usize) -> *mut c_void;
    pub fn wxBookCtrlBase_SetPageText(self_: *mut c_void, page: usize, text: *const c_void)
        -> bool;
    pub fn wxBookCtrlBase_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxBookCtrlBase_GetCurrentPage(self_: *const c_void) -> *mut c_void;
    pub fn wxBookCtrlBase_SetSelection(self_: *mut c_void, page: usize) -> c_int;
    pub fn wxBookCtrlBase_AdvanceSelection(self_: *mut c_void, forward: bool);
    pub fn wxBookCtrlBase_ChangeSelection(self_: *mut c_void, page: usize) -> c_int;
    pub fn wxBookCtrlBase_FindPage(self_: *const c_void, page: *const c_void) -> c_int;
    pub fn wxBookCtrlBase_SetPageSize(self_: *mut c_void, size: *const c_void);
    pub fn wxBookCtrlBase_HitTest(
        self_: *const c_void,
        pt: *const c_void,
        flags: *mut c_void,
    ) -> c_int;
    pub fn wxBookCtrlBase_AddPage(
        self_: *mut c_void,
        page: *mut c_void,
        text: *const c_void,
        select: bool,
        image_id: c_int,
    ) -> bool;
    pub fn wxBookCtrlBase_DeleteAllPages(self_: *mut c_void) -> bool;
    pub fn wxBookCtrlBase_DeletePage(self_: *mut c_void, page: usize) -> bool;
    pub fn wxBookCtrlBase_InsertPage(
        self_: *mut c_void,
        index: usize,
        page: *mut c_void,
        text: *const c_void,
        select: bool,
        image_id: c_int,
    ) -> bool;
    pub fn wxBookCtrlBase_RemovePage(self_: *mut c_void, page: usize) -> bool;
    pub fn wxBookCtrlBase_GetPageCount(self_: *const c_void) -> usize;
    pub fn wxBookCtrlBase_GetPage(self_: *const c_void, page: usize) -> *mut c_void;
    // BLOCKED: pub fn wxBookCtrlBase_new() -> *mut c_void;
    // BLOCKED: pub fn wxBookCtrlBase_new1(parent: *mut c_void, winid: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    pub fn wxBookCtrlBase_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    // Mix-in(s) to wxBookCtrlBase
    pub fn wxBookCtrlBase_AsWithImages(obj: *mut c_void) -> *mut c_void;

    // wxBookCtrlEvent
    pub fn wxBookCtrlEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBookCtrlEvent_new(event_type: wxEventType, id: c_int, sel: c_int, old_sel: c_int) -> *mut c_void;
    pub fn wxBookCtrlEvent_GetOldSelection(self_: *const c_void) -> c_int;
    pub fn wxBookCtrlEvent_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxBookCtrlEvent_SetOldSelection(self_: *mut c_void, page: c_int);
    pub fn wxBookCtrlEvent_SetSelection(self_: *mut c_void, page: c_int);

    // wxBoxSizer
    pub fn wxBoxSizer_CLASSINFO() -> *mut c_void;
    pub fn wxBoxSizer_new(orient: c_int) -> *mut c_void;
    pub fn wxBoxSizer_GetOrientation(self_: *const c_void) -> c_int;
    pub fn wxBoxSizer_SetOrientation(self_: *mut c_void, orient: c_int);

    // wxBrush
    pub fn wxBrush_CLASSINFO() -> *mut c_void;
    pub fn wxBrush_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBrush_new1(colour: *const c_void, style: wxBrushStyle) -> *mut c_void;
    pub fn wxBrush_new2(stipple_bitmap: *const c_void) -> *mut c_void;
    pub fn wxBrush_new3(brush: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxBrush_~wxBrush(self_: *mut c_void);
    pub fn wxBrush_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxBrush_GetStipple(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBrush_GetStyle(self_: *const c_void) -> wxBrushStyle;
    pub fn wxBrush_IsHatch(self_: *const c_void) -> bool;
    pub fn wxBrush_IsOk(self_: *const c_void) -> bool;
    pub fn wxBrush_IsNonTransparent(self_: *const c_void) -> bool;
    pub fn wxBrush_IsTransparent(self_: *const c_void) -> bool;
    pub fn wxBrush_SetColour(self_: *mut c_void, colour: *const c_void);
    // NOT_SUPPORTED: pub fn wxBrush_SetColour1(self_: *mut c_void, red: unsigned char, green: unsigned char, blue: unsigned char);
    pub fn wxBrush_SetStipple(self_: *mut c_void, bitmap: *const c_void);
    // NOT_SUPPORTED: pub fn wxBrush_SetStyle(self_: *mut c_void, style: wxBrushStyle);
    // BLOCKED: pub fn wxBrush_operator!=(self_: *const c_void, brush: *const c_void) -> bool;
    // BLOCKED: pub fn wxBrush_operator==(self_: *const c_void, brush: *const c_void) -> bool;

    // wxBrushList
    pub fn wxBrushList_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBrushList_FindOrCreateBrush(self_: *mut c_void, colour: *const c_void, style: wxBrushStyle) -> *mut c_void;

    // wxBufferedDC
    pub fn wxBufferedDC_CLASSINFO() -> *mut c_void;
    pub fn wxBufferedDC_new() -> *mut c_void;
    pub fn wxBufferedDC_new1(dc: *mut c_void, area: *const c_void, style: c_int) -> *mut c_void;
    pub fn wxBufferedDC_new2(dc: *mut c_void, buffer: *mut c_void, style: c_int) -> *mut c_void;
    // DTOR: pub fn wxBufferedDC_~wxBufferedDC(self_: *mut c_void);
    pub fn wxBufferedDC_Init(
        self_: *mut c_void,
        dc: *mut c_void,
        area: *const c_void,
        style: c_int,
    );
    pub fn wxBufferedDC_Init1(
        self_: *mut c_void,
        dc: *mut c_void,
        buffer: *mut c_void,
        style: c_int,
    );
    pub fn wxBufferedDC_UnMask(self_: *mut c_void);
    pub fn wxBufferedDC_SetStyle(self_: *mut c_void, style: c_int);
    pub fn wxBufferedDC_GetStyle(self_: *const c_void) -> c_int;

    // wxBufferedPaintDC
    pub fn wxBufferedPaintDC_CLASSINFO() -> *mut c_void;
    pub fn wxBufferedPaintDC_new(
        window: *mut c_void,
        buffer: *mut c_void,
        style: c_int,
    ) -> *mut c_void;
    pub fn wxBufferedPaintDC_new1(window: *mut c_void, style: c_int) -> *mut c_void;
    // DTOR: pub fn wxBufferedPaintDC_~wxBufferedPaintDC(self_: *mut c_void);

    // wxBusyCursor
    pub fn wxBusyCursor_delete(self_: *mut c_void);
    pub fn wxBusyCursor_new(cursor: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxBusyCursor_~wxBusyCursor(self_: *mut c_void);

    // wxBusyInfo
    pub fn wxBusyInfo_delete(self_: *mut c_void);
    pub fn wxBusyInfo_new(flags: *const c_void) -> *mut c_void;
    pub fn wxBusyInfo_new1(msg: *const c_void, parent: *mut c_void) -> *mut c_void;
    pub fn wxBusyInfo_UpdateText(self_: *mut c_void, str: *const c_void);
    pub fn wxBusyInfo_UpdateLabel(self_: *mut c_void, str: *const c_void);
    // DTOR: pub fn wxBusyInfo_~wxBusyInfo(self_: *mut c_void);

    // wxButton
    pub fn wxButton_CLASSINFO() -> *mut c_void;
    pub fn wxButton_new() -> *mut c_void;
    pub fn wxButton_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxButton_GetAuthNeeded(self_: *const c_void) -> bool;
    pub fn wxButton_SetAuthNeeded(self_: *mut c_void, needed: bool);
    pub fn wxButton_SetDefault(self_: *mut c_void) -> *mut c_void;
    pub fn wxButton_GetDefaultSize(win: *mut c_void) -> *mut c_void;

    // wxCalculateLayoutEvent
    pub fn wxCalculateLayoutEvent_CLASSINFO() -> *mut c_void;
    pub fn wxCalculateLayoutEvent_new(id: c_int) -> *mut c_void;
    pub fn wxCalculateLayoutEvent_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxCalculateLayoutEvent_GetRect(self_: *const c_void) -> *mut c_void;
    pub fn wxCalculateLayoutEvent_SetFlags(self_: *mut c_void, flags: c_int);
    pub fn wxCalculateLayoutEvent_SetRect(self_: *mut c_void, rect: *const c_void);

    // wxCalendarCtrl
    pub fn wxCalendarCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxCalendarCtrl_SetDateRange(
        self_: *mut c_void,
        lowerdate: *const c_void,
        upperdate: *const c_void,
    ) -> bool;
    pub fn wxCalendarCtrl_GetDateRange(
        self_: *const c_void,
        lowerdate: *mut c_void,
        upperdate: *mut c_void,
    ) -> bool;
    pub fn wxCalendarCtrl_new() -> *mut c_void;
    pub fn wxCalendarCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        date: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxCalendarCtrl_~wxCalendarCtrl(self_: *mut c_void);
    pub fn wxCalendarCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        date: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxCalendarCtrl_EnableHolidayDisplay(self_: *mut c_void, display: bool);
    pub fn wxCalendarCtrl_EnableMonthChange(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxCalendarCtrl_EnableYearChange(self_: *mut c_void, enable: bool);
    pub fn wxCalendarCtrl_GetAttr(self_: *const c_void, day: usize) -> *mut c_void;
    pub fn wxCalendarCtrl_GetDate(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHeaderColourBg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHeaderColourFg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHighlightColourBg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHighlightColourFg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHolidayColourBg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHolidayColourFg(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarCtrl_HitTest(self_: *mut c_void, pos: *const c_void, date: *mut c_void, wd: *mut c_void) -> wxCalendarHitTestResult;
    pub fn wxCalendarCtrl_ResetAttr(self_: *mut c_void, day: usize);
    pub fn wxCalendarCtrl_SetAttr(self_: *mut c_void, day: usize, attr: *mut c_void);
    pub fn wxCalendarCtrl_SetDate(self_: *mut c_void, date: *const c_void) -> bool;
    pub fn wxCalendarCtrl_SetHeaderColours(
        self_: *mut c_void,
        col_fg: *const c_void,
        col_bg: *const c_void,
    );
    pub fn wxCalendarCtrl_SetHighlightColours(
        self_: *mut c_void,
        col_fg: *const c_void,
        col_bg: *const c_void,
    );
    pub fn wxCalendarCtrl_SetHoliday(self_: *mut c_void, day: usize);
    pub fn wxCalendarCtrl_SetHolidayColours(
        self_: *mut c_void,
        col_fg: *const c_void,
        col_bg: *const c_void,
    );
    pub fn wxCalendarCtrl_Mark(self_: *mut c_void, day: usize, mark: bool);

    // wxCalendarDateAttr
    pub fn wxCalendarDateAttr_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxCalendarDateAttr_new(col_text: *const c_void, col_back: *const c_void, col_border: *const c_void, font: *const c_void, border: wxCalendarDateBorder) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarDateAttr_new1(border: wxCalendarDateBorder, col_border: *const c_void) -> *mut c_void;
    pub fn wxCalendarDateAttr_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarDateAttr_GetBorder(self_: *const c_void) -> wxCalendarDateBorder;
    pub fn wxCalendarDateAttr_GetBorderColour(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarDateAttr_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarDateAttr_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarDateAttr_HasBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_HasBorder(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_HasBorderColour(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_HasFont(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_HasTextColour(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_IsHoliday(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_SetBackgroundColour(self_: *mut c_void, col_back: *const c_void);
    // NOT_SUPPORTED: pub fn wxCalendarDateAttr_SetBorder(self_: *mut c_void, border: wxCalendarDateBorder);
    pub fn wxCalendarDateAttr_SetBorderColour(self_: *mut c_void, col: *const c_void);
    pub fn wxCalendarDateAttr_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxCalendarDateAttr_SetHoliday(self_: *mut c_void, holiday: bool);
    pub fn wxCalendarDateAttr_SetTextColour(self_: *mut c_void, col_text: *const c_void);
    pub fn wxCalendarDateAttr_GetMark() -> *mut c_void;
    pub fn wxCalendarDateAttr_SetMark(m: *const c_void);

    // wxCalendarEvent
    pub fn wxCalendarEvent_CLASSINFO() -> *mut c_void;
    pub fn wxCalendarEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarEvent_new1(win: *mut c_void, dt: *const c_void, type_: wxEventType) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarEvent_GetWeekDay(self_: *const c_void) -> wxDateTime::WeekDay;
    // NOT_SUPPORTED: pub fn wxCalendarEvent_SetWeekDay(self_: *mut c_void, day: wxDateTime::WeekDay);

    // wxCaret
    pub fn wxCaret_delete(self_: *mut c_void);
    pub fn wxCaret_new() -> *mut c_void;
    pub fn wxCaret_new1(window: *mut c_void, width: c_int, height: c_int) -> *mut c_void;
    pub fn wxCaret_new2(window: *mut c_void, size: *const c_void) -> *mut c_void;
    pub fn wxCaret_Create(
        self_: *mut c_void,
        window: *mut c_void,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxCaret_Create1(self_: *mut c_void, window: *mut c_void, size: *const c_void) -> bool;
    pub fn wxCaret_GetPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxCaret_GetPosition1(self_: *const c_void) -> *mut c_void;
    pub fn wxCaret_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxCaret_GetSize1(self_: *const c_void) -> *mut c_void;
    pub fn wxCaret_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxCaret_Hide(self_: *mut c_void);
    pub fn wxCaret_IsOk(self_: *const c_void) -> bool;
    pub fn wxCaret_IsVisible(self_: *const c_void) -> bool;
    pub fn wxCaret_Move(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxCaret_Move1(self_: *mut c_void, pt: *const c_void);
    pub fn wxCaret_SetSize(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxCaret_SetSize1(self_: *mut c_void, size: *const c_void);
    pub fn wxCaret_Show(self_: *mut c_void, show: bool);
    pub fn wxCaret_GetBlinkTime() -> c_int;
    pub fn wxCaret_SetBlinkTime(milliseconds: c_int);

    // wxCheckBox
    pub fn wxCheckBox_CLASSINFO() -> *mut c_void;
    pub fn wxCheckBox_new() -> *mut c_void;
    pub fn wxCheckBox_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxCheckBox_~wxCheckBox(self_: *mut c_void);
    pub fn wxCheckBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxCheckBox_GetValue(self_: *const c_void) -> bool;
    pub fn wxCheckBox_Get3StateValue(self_: *const c_void) -> c_int;
    pub fn wxCheckBox_Is3State(self_: *const c_void) -> bool;
    pub fn wxCheckBox_Is3rdStateAllowedForUser(self_: *const c_void) -> bool;
    pub fn wxCheckBox_IsChecked(self_: *const c_void) -> bool;
    pub fn wxCheckBox_SetValue(self_: *mut c_void, state: bool);
    pub fn wxCheckBox_Set3StateValue(self_: *mut c_void, state: c_int);

    // wxCheckListBox
    pub fn wxCheckListBox_CLASSINFO() -> *mut c_void;
    pub fn wxCheckListBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCheckListBox_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxCheckListBox_new2(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCheckListBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n_strings: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxCheckListBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    // DTOR: pub fn wxCheckListBox_~wxCheckListBox(self_: *mut c_void);
    pub fn wxCheckListBox_Check(self_: *mut c_void, item: c_uint, check: bool);
    pub fn wxCheckListBox_IsChecked(self_: *const c_void, item: c_uint) -> bool;
    pub fn wxCheckListBox_GetCheckedItems(
        self_: *const c_void,
        checked_items: *mut c_void,
    ) -> c_uint;
    // Mix-in(s) to wxCheckListBox
    pub fn wxCheckListBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxChildFocusEvent
    pub fn wxChildFocusEvent_CLASSINFO() -> *mut c_void;
    pub fn wxChildFocusEvent_new(win: *mut c_void) -> *mut c_void;
    pub fn wxChildFocusEvent_GetWindow(self_: *const c_void) -> *mut c_void;

    // wxChoice
    pub fn wxChoice_CLASSINFO() -> *mut c_void;
    pub fn wxChoice_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxChoice_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxChoice_new2(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxChoice_~wxChoice(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxChoice_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxChoice_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxChoice_GetColumns(self_: *const c_void) -> c_int;
    pub fn wxChoice_GetCurrentSelection(self_: *const c_void) -> c_int;
    pub fn wxChoice_SetColumns(self_: *mut c_void, n: c_int);
    pub fn wxChoice_IsSorted(self_: *const c_void) -> bool;
    // Mix-in(s) to wxChoice
    pub fn wxChoice_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxChoicebook
    pub fn wxChoicebook_CLASSINFO() -> *mut c_void;
    pub fn wxChoicebook_new() -> *mut c_void;
    pub fn wxChoicebook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxChoicebook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxChoicebook_GetChoiceCtrl(self_: *const c_void) -> *mut c_void;
    // Mix-in(s) to wxChoicebook
    pub fn wxChoicebook_AsWithImages(obj: *mut c_void) -> *mut c_void;

    // wxClientDC
    pub fn wxClientDC_CLASSINFO() -> *mut c_void;
    pub fn wxClientDC_new(window: *mut c_void) -> *mut c_void;

    // wxClipboard
    pub fn wxClipboard_CLASSINFO() -> *mut c_void;
    pub fn wxClipboard_new() -> *mut c_void;
    // DTOR: pub fn wxClipboard_~wxClipboard(self_: *mut c_void);
    pub fn wxClipboard_AddData(self_: *mut c_void, data: *mut c_void) -> bool;
    pub fn wxClipboard_Clear(self_: *mut c_void);
    pub fn wxClipboard_Close(self_: *mut c_void);
    pub fn wxClipboard_Flush(self_: *mut c_void) -> bool;
    pub fn wxClipboard_GetData(self_: *mut c_void, data: *mut c_void) -> bool;
    pub fn wxClipboard_IsOpened(self_: *const c_void) -> bool;
    pub fn wxClipboard_IsSupported(self_: *mut c_void, format: *const c_void) -> bool;
    pub fn wxClipboard_IsUsingPrimarySelection(self_: *const c_void) -> bool;
    pub fn wxClipboard_Open(self_: *mut c_void) -> bool;
    pub fn wxClipboard_SetData(self_: *mut c_void, data: *mut c_void) -> bool;
    pub fn wxClipboard_UsePrimarySelection(self_: *mut c_void, primary: bool);
    pub fn wxClipboard_Get() -> *mut c_void;

    // wxClipboardTextEvent
    pub fn wxClipboardTextEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxClipboardTextEvent_new(command_type: wxEventType, id: c_int) -> *mut c_void;

    // wxCloseEvent
    pub fn wxCloseEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCloseEvent_new(command_event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxCloseEvent_CanVeto(self_: *const c_void) -> bool;
    pub fn wxCloseEvent_GetLoggingOff(self_: *const c_void) -> bool;
    pub fn wxCloseEvent_SetCanVeto(self_: *mut c_void, can_veto: bool);
    pub fn wxCloseEvent_SetLoggingOff(self_: *mut c_void, logging_off: bool);
    pub fn wxCloseEvent_Veto(self_: *mut c_void, veto: bool);
    pub fn wxCloseEvent_GetVeto(self_: *const c_void) -> bool;

    // wxCollapsibleHeaderCtrl
    pub fn wxCollapsibleHeaderCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxCollapsibleHeaderCtrl_new() -> *mut c_void;
    pub fn wxCollapsibleHeaderCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxCollapsibleHeaderCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxCollapsibleHeaderCtrl_SetCollapsed(self_: *mut c_void, collapsed: bool);
    pub fn wxCollapsibleHeaderCtrl_IsCollapsed(self_: *const c_void) -> bool;

    // wxCollapsiblePane
    pub fn wxCollapsiblePane_CLASSINFO() -> *mut c_void;
    pub fn wxCollapsiblePane_new() -> *mut c_void;
    pub fn wxCollapsiblePane_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxCollapsiblePane_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxCollapsiblePane_Collapse(self_: *mut c_void, collapse: bool);
    pub fn wxCollapsiblePane_Expand(self_: *mut c_void);
    pub fn wxCollapsiblePane_GetPane(self_: *const c_void) -> *mut c_void;
    pub fn wxCollapsiblePane_IsCollapsed(self_: *const c_void) -> bool;
    pub fn wxCollapsiblePane_IsExpanded(self_: *const c_void) -> bool;

    // wxCollapsiblePaneEvent
    pub fn wxCollapsiblePaneEvent_CLASSINFO() -> *mut c_void;
    pub fn wxCollapsiblePaneEvent_new(
        generator: *mut c_void,
        id: c_int,
        collapsed: bool,
    ) -> *mut c_void;
    pub fn wxCollapsiblePaneEvent_GetCollapsed(self_: *const c_void) -> bool;
    pub fn wxCollapsiblePaneEvent_SetCollapsed(self_: *mut c_void, collapsed: bool);

    // wxColour
    pub fn wxColour_CLASSINFO() -> *mut c_void;
    pub fn wxColour_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColour_new1(red: unsigned char, green: unsigned char, blue: unsigned char, alpha: unsigned char) -> *mut c_void;
    pub fn wxColour_new2(colour_name: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColour_new3(col_rgb: unsigned long) -> *mut c_void;
    pub fn wxColour_new4(colour: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColour_Alpha(self_: *const c_void) -> unsigned char;
    // NOT_SUPPORTED: pub fn wxColour_Blue(self_: *const c_void) -> unsigned char;
    pub fn wxColour_GetAlpha(self_: *const c_void) -> c_uint;
    pub fn wxColour_GetBlue(self_: *const c_void) -> c_uint;
    pub fn wxColour_GetGreen(self_: *const c_void) -> c_uint;
    pub fn wxColour_GetRed(self_: *const c_void) -> c_uint;
    pub fn wxColour_GetAsString(self_: *const c_void, flags: c_long) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColour_SetRGB(self_: *mut c_void, col_rgb: wxUint32);
    // NOT_SUPPORTED: pub fn wxColour_SetRGBA(self_: *mut c_void, col_rgba: wxUint32);
    // NOT_SUPPORTED: pub fn wxColour_GetRGB(self_: *const c_void) -> wxUint32;
    // NOT_SUPPORTED: pub fn wxColour_GetRGBA(self_: *const c_void) -> wxUint32;
    pub fn wxColour_GetLuminance(self_: *const c_void) -> c_double;
    // NOT_SUPPORTED: pub fn wxColour_GetPixel(self_: *const c_void) -> wxIntPtr;
    // NOT_SUPPORTED: pub fn wxColour_Green(self_: *const c_void) -> unsigned char;
    pub fn wxColour_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxColour_Red(self_: *const c_void) -> unsigned char;
    pub fn wxColour_IsSolid(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxColour_Set(self_: *mut c_void, red: unsigned char, green: unsigned char, blue: unsigned char, alpha: unsigned char);
    // NOT_SUPPORTED: pub fn wxColour_Set1(self_: *mut c_void, rgb: unsigned long);
    pub fn wxColour_Set2(self_: *mut c_void, str: *const c_void) -> bool;
    // BLOCKED: pub fn wxColour_operator!=(self_: *const c_void, colour: *const c_void) -> bool;
    // BLOCKED: pub fn wxColour_operator=(self_: *mut c_void, colour: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxColour_operator==(self_: *const c_void, colour: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxColour_MakeDisabled(self_: *mut c_void, brightness: unsigned char) -> *mut c_void;
    // BLOCKED: pub fn wxColour_ChangeLightness(self_: *const c_void, ialpha: c_int) -> wxColour;
    pub fn wxColour_MakeMono(r: *mut c_void, g: *mut c_void, b: *mut c_void, on: bool);
    // NOT_SUPPORTED: pub fn wxColour_MakeDisabled1(r: *mut c_void, g: *mut c_void, b: *mut c_void, brightness: unsigned char);
    pub fn wxColour_MakeGrey(r: *mut c_void, g: *mut c_void, b: *mut c_void);
    pub fn wxColour_MakeGrey1(
        r: *mut c_void,
        g: *mut c_void,
        b: *mut c_void,
        weight_r: c_double,
        weight_g: c_double,
        weight_b: c_double,
    );
    // NOT_SUPPORTED: pub fn wxColour_AlphaBlend(fg: unsigned char, bg: unsigned char, alpha: c_double) -> unsigned char;
    pub fn wxColour_ChangeLightness1(r: *mut c_void, g: *mut c_void, b: *mut c_void, ialpha: c_int);

    // wxColourData
    pub fn wxColourData_CLASSINFO() -> *mut c_void;
    pub fn wxColourData_new() -> *mut c_void;
    // DTOR: pub fn wxColourData_~wxColourData(self_: *mut c_void);
    pub fn wxColourData_GetChooseFull(self_: *const c_void) -> bool;
    pub fn wxColourData_GetChooseAlpha(self_: *const c_void) -> bool;
    pub fn wxColourData_GetColour(self_: *mut c_void) -> *mut c_void;
    pub fn wxColourData_GetCustomColour(self_: *const c_void, i: c_int) -> *mut c_void;
    pub fn wxColourData_SetChooseFull(self_: *mut c_void, flag: bool);
    pub fn wxColourData_SetChooseAlpha(self_: *mut c_void, flag: bool);
    pub fn wxColourData_SetColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxColourData_SetCustomColour(self_: *mut c_void, i: c_int, colour: *const c_void);
    pub fn wxColourData_ToString(self_: *const c_void) -> *mut c_void;
    pub fn wxColourData_FromString(self_: *mut c_void, str: *const c_void) -> bool;
    // BLOCKED: pub fn wxColourData_operator=(self_: *mut c_void, data: *const c_void) -> *mut c_void;

    // wxColourDatabase
    pub fn wxColourDatabase_delete(self_: *mut c_void);
    pub fn wxColourDatabase_new() -> *mut c_void;
    pub fn wxColourDatabase_AddColour(
        self_: *mut c_void,
        colour_name: *const c_void,
        colour: *const c_void,
    );
    pub fn wxColourDatabase_Find(self_: *const c_void, colour_name: *const c_void) -> *mut c_void;
    pub fn wxColourDatabase_FindName(self_: *const c_void, colour: *const c_void) -> *mut c_void;

    // wxColourDialogEvent
    pub fn wxColourDialogEvent_CLASSINFO() -> *mut c_void;
    pub fn wxColourDialogEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColourDialogEvent_new1(evt_type: wxEventType, dialog: *mut c_void, colour: *const c_void) -> *mut c_void;
    pub fn wxColourDialogEvent_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxColourDialogEvent_SetColour(self_: *mut c_void, colour: *const c_void);

    // wxColourPickerCtrl
    pub fn wxColourPickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxColourPickerCtrl_new() -> *mut c_void;
    pub fn wxColourPickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        colour: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxColourPickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        colour: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxColourPickerCtrl_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxColourPickerCtrl_SetColour(self_: *mut c_void, col: *const c_void);
    // BLOCKED: pub fn wxColourPickerCtrl_SetColour1(self_: *mut c_void, colname: *const c_void);

    // wxColourPickerEvent
    pub fn wxColourPickerEvent_CLASSINFO() -> *mut c_void;
    pub fn wxColourPickerEvent_new() -> *mut c_void;
    pub fn wxColourPickerEvent_new1(
        generator: *mut c_void,
        id: c_int,
        colour: *const c_void,
    ) -> *mut c_void;
    pub fn wxColourPickerEvent_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxColourPickerEvent_SetColour(self_: *mut c_void, pos: *const c_void);

    // wxComboBox
    pub fn wxComboBox_CLASSINFO() -> *mut c_void;
    pub fn wxComboBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxComboBox_new1(parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxComboBox_new2(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxComboBox_~wxComboBox(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxComboBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxComboBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxComboBox_GetCurrentSelection(self_: *const c_void) -> c_int;
    pub fn wxComboBox_IsListEmpty(self_: *const c_void) -> bool;
    pub fn wxComboBox_IsTextEmpty(self_: *const c_void) -> bool;
    pub fn wxComboBox_Popup(self_: *mut c_void);
    pub fn wxComboBox_Dismiss(self_: *mut c_void);
    // Mix-in(s) to wxComboBox
    pub fn wxComboBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;
    pub fn wxComboBox_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxCommandEvent
    pub fn wxCommandEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCommandEvent_new(command_event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxCommandEvent_GetClientData(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_GetClientObject(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_GetExtraLong(self_: *const c_void) -> c_long;
    pub fn wxCommandEvent_GetInt(self_: *const c_void) -> c_int;
    pub fn wxCommandEvent_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxCommandEvent_GetString(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_IsChecked(self_: *const c_void) -> bool;
    pub fn wxCommandEvent_IsSelection(self_: *const c_void) -> bool;
    pub fn wxCommandEvent_SetClientData(self_: *mut c_void, client_data: *mut c_void);
    pub fn wxCommandEvent_SetClientObject(self_: *mut c_void, client_object: *mut c_void);
    pub fn wxCommandEvent_SetExtraLong(self_: *mut c_void, extra_long: c_long);
    pub fn wxCommandEvent_SetInt(self_: *mut c_void, int_command: c_int);
    pub fn wxCommandEvent_SetString(self_: *mut c_void, string: *const c_void);

    // wxCommandLinkButton
    pub fn wxCommandLinkButton_CLASSINFO() -> *mut c_void;
    pub fn wxCommandLinkButton_new() -> *mut c_void;
    pub fn wxCommandLinkButton_new1(
        parent: *mut c_void,
        id: c_int,
        main_label: *const c_void,
        note: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxCommandLinkButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        main_label: *const c_void,
        note: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxCommandLinkButton_SetMainLabelAndNote(
        self_: *mut c_void,
        main_label: *const c_void,
        note: *const c_void,
    );
    pub fn wxCommandLinkButton_SetMainLabel(self_: *mut c_void, main_label: *const c_void);
    pub fn wxCommandLinkButton_SetNote(self_: *mut c_void, note: *const c_void);
    pub fn wxCommandLinkButton_GetMainLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandLinkButton_GetNote(self_: *const c_void) -> *mut c_void;

    // wxContextHelp
    pub fn wxContextHelp_CLASSINFO() -> *mut c_void;
    pub fn wxContextHelp_new(window: *mut c_void, do_now: bool) -> *mut c_void;
    // DTOR: pub fn wxContextHelp_~wxContextHelp(self_: *mut c_void);
    pub fn wxContextHelp_BeginContextHelp(self_: *mut c_void, window: *mut c_void) -> bool;
    pub fn wxContextHelp_EndContextHelp(self_: *mut c_void) -> bool;

    // wxContextHelpButton
    pub fn wxContextHelpButton_CLASSINFO() -> *mut c_void;
    pub fn wxContextHelpButton_new(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
    ) -> *mut c_void;

    // wxContextMenuEvent
    pub fn wxContextMenuEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxContextMenuEvent_new(type_: wxEventType, id: c_int, pos: *const c_void) -> *mut c_void;
    pub fn wxContextMenuEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxContextMenuEvent_SetPosition(self_: *mut c_void, point: *const c_void);

    // wxControl
    pub fn wxControl_CLASSINFO() -> *mut c_void;
    pub fn wxControl_new(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxControl_new1() -> *mut c_void;
    pub fn wxControl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxControl_Command(self_: *mut c_void, event: *mut c_void);
    pub fn wxControl_GetLabelText(self_: *const c_void) -> *mut c_void;
    pub fn wxControl_GetSizeFromTextSize(
        self_: *const c_void,
        xlen: c_int,
        ylen: c_int,
    ) -> *mut c_void;
    pub fn wxControl_GetSizeFromTextSize1(
        self_: *const c_void,
        tsize: *const c_void,
    ) -> *mut c_void;
    pub fn wxControl_GetSizeFromText(self_: *const c_void, text: *const c_void) -> *mut c_void;
    pub fn wxControl_SetLabelText(self_: *mut c_void, text: *const c_void);
    pub fn wxControl_SetLabelMarkup(self_: *mut c_void, markup: *const c_void) -> bool;
    pub fn wxControl_GetLabelText1(label: *const c_void) -> *mut c_void;
    pub fn wxControl_RemoveMnemonics(str: *const c_void) -> *mut c_void;
    pub fn wxControl_EscapeMnemonics(text: *const c_void) -> *mut c_void;
    pub fn wxControl_Ellipsize(
        label: *const c_void,
        dc: *const c_void,
        mode: c_int,
        max_width: c_int,
        flags: c_int,
    ) -> *mut c_void;

    // wxControlWithItems
    pub fn wxControlWithItems_CLASSINFO() -> *mut c_void;
    // Mix-in(s) to wxControlWithItems
    pub fn wxControlWithItems_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxCursor
    pub fn wxCursor_CLASSINFO() -> *mut c_void;
    pub fn wxCursor_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCursor_new1(bits: char, width: c_int, height: c_int, hot_spot_x: c_int, hot_spot_y: c_int, mask_bits: char) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCursor_new2(cursor_name: *const c_void, type_: wxBitmapType, hot_spot_x: c_int, hot_spot_y: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCursor_new3(cursor_id: wxStockCursor) -> *mut c_void;
    pub fn wxCursor_new4(image: *const c_void) -> *mut c_void;
    pub fn wxCursor_new5(xpm_data: *const c_void) -> *mut c_void;
    pub fn wxCursor_new6(cursor: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxCursor_~wxCursor(self_: *mut c_void);
    pub fn wxCursor_IsOk(self_: *const c_void) -> bool;
    pub fn wxCursor_GetHotSpot(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxCursor_operator=(self_: *mut c_void, cursor: *const c_void) -> *mut c_void;

    // wxDC
    pub fn wxDC_CLASSINFO() -> *mut c_void;
    pub fn wxDC_DeviceToLogicalX(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDC_DeviceToLogicalXRel(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDC_DeviceToLogicalY(self_: *const c_void, y: c_int) -> c_int;
    pub fn wxDC_DeviceToLogicalYRel(self_: *const c_void, y: c_int) -> c_int;
    pub fn wxDC_LogicalToDeviceX(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDC_LogicalToDeviceXRel(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDC_LogicalToDeviceY(self_: *const c_void, y: c_int) -> c_int;
    pub fn wxDC_LogicalToDeviceYRel(self_: *const c_void, y: c_int) -> c_int;
    pub fn wxDC_DeviceToLogical(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxDC_DeviceToLogical1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxDC_DeviceToLogicalRel(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxDC_DeviceToLogicalRel1(self_: *const c_void, dim: *const c_void) -> *mut c_void;
    pub fn wxDC_LogicalToDevice(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxDC_LogicalToDevice1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxDC_LogicalToDeviceRel(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxDC_LogicalToDeviceRel1(self_: *const c_void, dim: *const c_void) -> *mut c_void;
    pub fn wxDC_Clear(self_: *mut c_void);
    pub fn wxDC_DrawArc(
        self_: *mut c_void,
        x_start: c_int,
        y_start: c_int,
        x_end: c_int,
        y_end: c_int,
        xc: c_int,
        yc: c_int,
    );
    pub fn wxDC_DrawArc1(
        self_: *mut c_void,
        pt_start: *const c_void,
        pt_end: *const c_void,
        centre: *const c_void,
    );
    pub fn wxDC_DrawBitmap(
        self_: *mut c_void,
        bitmap: *const c_void,
        x: c_int,
        y: c_int,
        use_mask: bool,
    );
    pub fn wxDC_DrawBitmap1(
        self_: *mut c_void,
        bmp: *const c_void,
        pt: *const c_void,
        use_mask: bool,
    );
    pub fn wxDC_DrawCheckMark(self_: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn wxDC_DrawCheckMark1(self_: *mut c_void, rect: *const c_void);
    pub fn wxDC_DrawCircle(self_: *mut c_void, x: c_int, y: c_int, radius: c_int);
    pub fn wxDC_DrawCircle1(self_: *mut c_void, pt: *const c_void, radius: c_int);
    pub fn wxDC_DrawEllipse(self_: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn wxDC_DrawEllipse1(self_: *mut c_void, pt: *const c_void, size: *const c_void);
    pub fn wxDC_DrawEllipse2(self_: *mut c_void, rect: *const c_void);
    pub fn wxDC_DrawEllipticArc(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        start: c_double,
        end: c_double,
    );
    pub fn wxDC_DrawEllipticArc1(
        self_: *mut c_void,
        pt: *const c_void,
        sz: *const c_void,
        sa: c_double,
        ea: c_double,
    );
    pub fn wxDC_DrawIcon(self_: *mut c_void, icon: *const c_void, x: c_int, y: c_int);
    pub fn wxDC_DrawIcon1(self_: *mut c_void, icon: *const c_void, pt: *const c_void);
    pub fn wxDC_DrawLabel(
        self_: *mut c_void,
        text: *const c_void,
        bitmap: *const c_void,
        rect: *const c_void,
        alignment: c_int,
        index_accel: c_int,
        rect_bounding: *mut c_void,
    );
    pub fn wxDC_DrawLabel1(
        self_: *mut c_void,
        text: *const c_void,
        rect: *const c_void,
        alignment: c_int,
        index_accel: c_int,
    );
    pub fn wxDC_DrawLine(self_: *mut c_void, x1: c_int, y1: c_int, x2: c_int, y2: c_int);
    pub fn wxDC_DrawLine1(self_: *mut c_void, pt1: *const c_void, pt2: *const c_void);
    // NOT_SUPPORTED: pub fn wxDC_DrawLines(self_: *mut c_void, n: c_int, points: wxPoint, xoffset: c_int, yoffset: c_int);
    pub fn wxDC_DrawLines1(
        self_: *mut c_void,
        points: *const c_void,
        xoffset: c_int,
        yoffset: c_int,
    );
    pub fn wxDC_DrawPoint(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDC_DrawPoint1(self_: *mut c_void, pt: *const c_void);
    // NOT_SUPPORTED: pub fn wxDC_DrawPolygon(self_: *mut c_void, n: c_int, points: wxPoint, xoffset: c_int, yoffset: c_int, fill_style: wxPolygonFillMode);
    // NOT_SUPPORTED: pub fn wxDC_DrawPolygon1(self_: *mut c_void, points: *const c_void, xoffset: c_int, yoffset: c_int, fill_style: wxPolygonFillMode);
    // NOT_SUPPORTED: pub fn wxDC_DrawPolyPolygon(self_: *mut c_void, n: c_int, count: c_int, points: wxPoint, xoffset: c_int, yoffset: c_int, fill_style: wxPolygonFillMode);
    pub fn wxDC_DrawRectangle(self_: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn wxDC_DrawRectangle1(self_: *mut c_void, pt: *const c_void, sz: *const c_void);
    pub fn wxDC_DrawRectangle2(self_: *mut c_void, rect: *const c_void);
    pub fn wxDC_DrawRotatedText(
        self_: *mut c_void,
        text: *const c_void,
        x: c_int,
        y: c_int,
        angle: c_double,
    );
    pub fn wxDC_DrawRotatedText1(
        self_: *mut c_void,
        text: *const c_void,
        point: *const c_void,
        angle: c_double,
    );
    pub fn wxDC_DrawRoundedRectangle(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        radius: c_double,
    );
    pub fn wxDC_DrawRoundedRectangle1(
        self_: *mut c_void,
        pt: *const c_void,
        sz: *const c_void,
        radius: c_double,
    );
    pub fn wxDC_DrawRoundedRectangle2(self_: *mut c_void, rect: *const c_void, radius: c_double);
    // NOT_SUPPORTED: pub fn wxDC_DrawSpline(self_: *mut c_void, n: c_int, points: wxPoint);
    pub fn wxDC_DrawSpline1(self_: *mut c_void, points: *const c_void);
    pub fn wxDC_DrawSpline2(
        self_: *mut c_void,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        x3: c_int,
        y3: c_int,
    );
    pub fn wxDC_DrawText(self_: *mut c_void, text: *const c_void, x: c_int, y: c_int);
    pub fn wxDC_DrawText1(self_: *mut c_void, text: *const c_void, pt: *const c_void);
    pub fn wxDC_GradientFillConcentric(
        self_: *mut c_void,
        rect: *const c_void,
        initial_colour: *const c_void,
        dest_colour: *const c_void,
    );
    pub fn wxDC_GradientFillConcentric1(
        self_: *mut c_void,
        rect: *const c_void,
        initial_colour: *const c_void,
        dest_colour: *const c_void,
        circle_center: *const c_void,
    );
    pub fn wxDC_GradientFillLinear(
        self_: *mut c_void,
        rect: *const c_void,
        initial_colour: *const c_void,
        dest_colour: *const c_void,
        n_direction: c_int,
    );
    // NOT_SUPPORTED: pub fn wxDC_FloodFill(self_: *mut c_void, x: c_int, y: c_int, colour: *const c_void, style: wxFloodFillStyle) -> bool;
    // NOT_SUPPORTED: pub fn wxDC_FloodFill1(self_: *mut c_void, pt: *const c_void, col: *const c_void, style: wxFloodFillStyle) -> bool;
    pub fn wxDC_CrossHair(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDC_CrossHair1(self_: *mut c_void, pt: *const c_void);
    pub fn wxDC_DestroyClippingRegion(self_: *mut c_void);
    pub fn wxDC_GetClippingBox(
        self_: *const c_void,
        x: *mut c_void,
        y: *mut c_void,
        width: *mut c_void,
        height: *mut c_void,
    ) -> bool;
    pub fn wxDC_GetClippingBox1(self_: *const c_void, rect: *mut c_void) -> bool;
    pub fn wxDC_SetClippingRegion(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn wxDC_SetClippingRegion1(self_: *mut c_void, pt: *const c_void, sz: *const c_void);
    pub fn wxDC_SetClippingRegion2(self_: *mut c_void, rect: *const c_void);
    pub fn wxDC_SetDeviceClippingRegion(self_: *mut c_void, region: *const c_void);
    pub fn wxDC_GetCharHeight(self_: *const c_void) -> c_int;
    pub fn wxDC_GetCharWidth(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxDC_GetFontMetrics(self_: *const c_void) -> wxFontMetrics;
    pub fn wxDC_GetMultiLineTextExtent(
        self_: *const c_void,
        string: *const c_void,
        w: *mut c_void,
        h: *mut c_void,
        height_line: *mut c_void,
        font: *const c_void,
    );
    pub fn wxDC_GetMultiLineTextExtent1(self_: *const c_void, string: *const c_void)
        -> *mut c_void;
    pub fn wxDC_GetPartialTextExtents(
        self_: *const c_void,
        text: *const c_void,
        widths: *mut c_void,
    ) -> bool;
    pub fn wxDC_GetTextExtent(
        self_: *const c_void,
        string: *const c_void,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: *const c_void,
    );
    pub fn wxDC_GetTextExtent1(self_: *const c_void, string: *const c_void) -> *mut c_void;
    pub fn wxDC_GetBackgroundMode(self_: *const c_void) -> c_int;
    pub fn wxDC_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetLayoutDirection(self_: *const c_void) -> c_int;
    pub fn wxDC_GetTextBackground(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetTextForeground(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_SetBackgroundMode(self_: *mut c_void, mode: c_int);
    pub fn wxDC_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxDC_SetTextBackground(self_: *mut c_void, colour: *const c_void);
    pub fn wxDC_SetTextForeground(self_: *mut c_void, colour: *const c_void);
    pub fn wxDC_SetLayoutDirection(self_: *mut c_void, dir: c_int);
    pub fn wxDC_CalcBoundingBox(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDC_MaxX(self_: *const c_void) -> c_int;
    pub fn wxDC_MaxY(self_: *const c_void) -> c_int;
    pub fn wxDC_MinX(self_: *const c_void) -> c_int;
    pub fn wxDC_MinY(self_: *const c_void) -> c_int;
    pub fn wxDC_ResetBoundingBox(self_: *mut c_void);
    pub fn wxDC_StartDoc(self_: *mut c_void, message: *const c_void) -> bool;
    pub fn wxDC_StartPage(self_: *mut c_void);
    pub fn wxDC_EndDoc(self_: *mut c_void);
    pub fn wxDC_EndPage(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDC_Blit(self_: *mut c_void, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: *mut c_void, xsrc: c_int, ysrc: c_int, logical_func: wxRasterOperationMode, use_mask: bool, xsrc_mask: c_int, ysrc_mask: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxDC_StretchBlit(self_: *mut c_void, xdest: c_int, ydest: c_int, dst_width: c_int, dst_height: c_int, source: *mut c_void, xsrc: c_int, ysrc: c_int, src_width: c_int, src_height: c_int, logical_func: wxRasterOperationMode, use_mask: bool, xsrc_mask: c_int, ysrc_mask: c_int) -> bool;
    pub fn wxDC_GetBackground(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetBrush(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetPen(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_SetBackground(self_: *mut c_void, brush: *const c_void);
    pub fn wxDC_SetBrush(self_: *mut c_void, brush: *const c_void);
    pub fn wxDC_SetPen(self_: *mut c_void, pen: *const c_void);
    pub fn wxDC_CopyAttributes(self_: *mut c_void, dc: *const c_void);
    pub fn wxDC_GetContentScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxDC_GetDepth(self_: *const c_void) -> c_int;
    pub fn wxDC_GetDeviceOrigin(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDC_GetLogicalFunction(self_: *const c_void) -> wxRasterOperationMode;
    // NOT_SUPPORTED: pub fn wxDC_GetMapMode(self_: *const c_void) -> wxMappingMode;
    pub fn wxDC_GetPixel(self_: *const c_void, x: c_int, y: c_int, colour: *mut c_void) -> bool;
    pub fn wxDC_GetPPI(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_FromDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxDC_FromDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxDC_FromDIP2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxDC_ToDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxDC_ToDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxDC_ToDIP2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxDC_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxDC_GetSize1(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetSizeMM(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxDC_GetSizeMM1(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetUserScale(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxDC_IsOk(self_: *const c_void) -> bool;
    pub fn wxDC_SetAxisOrientation(self_: *mut c_void, x_left_right: bool, y_bottom_up: bool);
    pub fn wxDC_SetDeviceOrigin(self_: *mut c_void, x: c_int, y: c_int);
    // NOT_SUPPORTED: pub fn wxDC_SetLogicalFunction(self_: *mut c_void, function: wxRasterOperationMode);
    // NOT_SUPPORTED: pub fn wxDC_SetMapMode(self_: *mut c_void, mode: wxMappingMode);
    pub fn wxDC_SetPalette(self_: *mut c_void, palette: *const c_void);
    pub fn wxDC_SetUserScale(self_: *mut c_void, x_scale: c_double, y_scale: c_double);
    pub fn wxDC_CanUseTransformMatrix(self_: *const c_void) -> bool;
    pub fn wxDC_SetTransformMatrix(self_: *mut c_void, matrix: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDC_GetTransformMatrix(self_: *const c_void) -> wxAffineMatrix2D;
    pub fn wxDC_ResetTransformMatrix(self_: *mut c_void);
    pub fn wxDC_CanDrawBitmap(self_: *const c_void) -> bool;
    pub fn wxDC_CanGetTextExtent(self_: *const c_void) -> bool;
    pub fn wxDC_GetHandle(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetAsBitmap(self_: *const c_void, subrect: *const c_void) -> *mut c_void;
    pub fn wxDC_SetLogicalScale(self_: *mut c_void, x: c_double, y: c_double);
    pub fn wxDC_GetLogicalScale(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxDC_SetLogicalOrigin(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDC_GetLogicalOrigin(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxDC_GetLogicalOrigin1(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetGraphicsContext(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_SetGraphicsContext(self_: *mut c_void, ctx: *mut c_void);

    // wxDCBrushChanger
    pub fn wxDCBrushChanger_delete(self_: *mut c_void);
    pub fn wxDCBrushChanger_new(dc: *mut c_void, brush: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDCBrushChanger_~wxDCBrushChanger(self_: *mut c_void);

    // wxDCClipper
    pub fn wxDCClipper_delete(self_: *mut c_void);
    pub fn wxDCClipper_new(dc: *mut c_void, region: *const c_void) -> *mut c_void;
    pub fn wxDCClipper_new1(dc: *mut c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxDCClipper_new2(dc: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int)
        -> *mut c_void;
    // DTOR: pub fn wxDCClipper_~wxDCClipper(self_: *mut c_void);

    // wxDCFontChanger
    pub fn wxDCFontChanger_delete(self_: *mut c_void);
    pub fn wxDCFontChanger_new(dc: *mut c_void) -> *mut c_void;
    pub fn wxDCFontChanger_new1(dc: *mut c_void, font: *const c_void) -> *mut c_void;
    pub fn wxDCFontChanger_Set(self_: *mut c_void, font: *const c_void);
    // DTOR: pub fn wxDCFontChanger_~wxDCFontChanger(self_: *mut c_void);

    // wxDCOverlay
    pub fn wxDCOverlay_delete(self_: *mut c_void);
    pub fn wxDCOverlay_new(
        overlay: *mut c_void,
        dc: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> *mut c_void;
    pub fn wxDCOverlay_new1(overlay: *mut c_void, dc: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxDCOverlay_~wxDCOverlay(self_: *mut c_void);
    pub fn wxDCOverlay_Clear(self_: *mut c_void);

    // wxDCPenChanger
    pub fn wxDCPenChanger_delete(self_: *mut c_void);
    pub fn wxDCPenChanger_new(dc: *mut c_void, pen: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDCPenChanger_~wxDCPenChanger(self_: *mut c_void);

    // wxDCTextBgColourChanger
    pub fn wxDCTextBgColourChanger_delete(self_: *mut c_void);
    pub fn wxDCTextBgColourChanger_new(dc: *mut c_void) -> *mut c_void;
    pub fn wxDCTextBgColourChanger_new1(dc: *mut c_void, col: *const c_void) -> *mut c_void;
    pub fn wxDCTextBgColourChanger_Set(self_: *mut c_void, col: *const c_void);
    // DTOR: pub fn wxDCTextBgColourChanger_~wxDCTextBgColourChanger(self_: *mut c_void);

    // wxDCTextBgModeChanger
    pub fn wxDCTextBgModeChanger_delete(self_: *mut c_void);

    // wxDCTextColourChanger
    pub fn wxDCTextColourChanger_delete(self_: *mut c_void);
    pub fn wxDCTextColourChanger_new(dc: *mut c_void) -> *mut c_void;
    pub fn wxDCTextColourChanger_new1(dc: *mut c_void, col: *const c_void) -> *mut c_void;
    pub fn wxDCTextColourChanger_Set(self_: *mut c_void, col: *const c_void);
    // DTOR: pub fn wxDCTextColourChanger_~wxDCTextColourChanger(self_: *mut c_void);

    // wxDPIChangedEvent
    pub fn wxDPIChangedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxDPIChangedEvent_GetOldDPI(self_: *const c_void) -> *mut c_void;
    pub fn wxDPIChangedEvent_GetNewDPI(self_: *const c_void) -> *mut c_void;
    pub fn wxDPIChangedEvent_Scale(self_: *const c_void, sz: wxSize) -> *mut c_void;
    pub fn wxDPIChangedEvent_ScaleX(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDPIChangedEvent_ScaleY(self_: *const c_void, y: c_int) -> c_int;

    // wxDataViewColumn
    pub fn wxDataViewColumn_delete(self_: *mut c_void);
    pub fn wxDataViewColumn_new(
        title: *const c_void,
        renderer: *mut c_void,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxDataViewColumn_new1(
        bitmap: *const c_void,
        renderer: *mut c_void,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxDataViewColumn_GetModelColumn(self_: *const c_void) -> c_uint;
    pub fn wxDataViewColumn_GetOwner(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewColumn_GetRenderer(self_: *const c_void) -> *mut c_void;

    // wxDataViewCtrl
    pub fn wxDataViewCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewCtrl_new() -> *mut c_void;
    pub fn wxDataViewCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDataViewCtrl_~wxDataViewCtrl(self_: *mut c_void);
    pub fn wxDataViewCtrl_AllowMultiColumnSort(self_: *mut c_void, allow: bool) -> bool;
    pub fn wxDataViewCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxDataViewCtrl_AppendColumn(self_: *mut c_void, col: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_PrependColumn(self_: *mut c_void, col: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_InsertColumn(self_: *mut c_void, pos: c_uint, col: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendBitmapColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendBitmapColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependBitmapColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependBitmapColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendDateColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendDateColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependDateColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependDateColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendIconTextColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendIconTextColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependIconTextColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependIconTextColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendProgressColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendProgressColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependProgressColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependProgressColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendTextColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendTextColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependTextColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependTextColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendToggleColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendToggleColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependToggleColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependToggleColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    pub fn wxDataViewCtrl_AssociateModel(self_: *mut c_void, model: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_ClearColumns(self_: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_Collapse(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_DeleteColumn(self_: *mut c_void, column: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_EditItem(self_: *mut c_void, item: *const c_void, column: *const c_void);
    pub fn wxDataViewCtrl_EnableDragSource(self_: *mut c_void, format: *const c_void) -> bool;
    pub fn wxDataViewCtrl_EnableDropTargets(self_: *mut c_void, formats: *const c_void) -> bool;
    pub fn wxDataViewCtrl_EnableDropTarget(self_: *mut c_void, format: *const c_void) -> bool;
    pub fn wxDataViewCtrl_EnsureVisible(
        self_: *mut c_void,
        item: *const c_void,
        column: *const c_void,
    );
    pub fn wxDataViewCtrl_Expand(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_ExpandAncestors(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_ExpandChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_GetColumn(self_: *const c_void, pos: c_uint) -> *mut c_void;
    pub fn wxDataViewCtrl_GetColumnCount(self_: *const c_void) -> c_uint;
    pub fn wxDataViewCtrl_GetColumnPosition(self_: *const c_void, column: *const c_void) -> c_int;
    pub fn wxDataViewCtrl_GetExpanderColumn(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetCurrentItem(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetCurrentColumn(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetIndent(self_: *const c_void) -> c_int;
    pub fn wxDataViewCtrl_GetItemRect(
        self_: *const c_void,
        item: *const c_void,
        col: *const c_void,
    ) -> *mut c_void;
    pub fn wxDataViewCtrl_GetMainWindow(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetModel(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetSelectedItemsCount(self_: *const c_void) -> c_int;
    pub fn wxDataViewCtrl_GetSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetSelections(self_: *const c_void, sel: *mut c_void) -> c_int;
    pub fn wxDataViewCtrl_GetSortingColumn(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetSortingColumns(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_HasSelection(self_: *const c_void) -> bool;
    pub fn wxDataViewCtrl_HitTest(
        self_: *const c_void,
        point: *const c_void,
        item: *mut c_void,
        col: *mut c_void,
    );
    pub fn wxDataViewCtrl_IsExpanded(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxDataViewCtrl_IsMultiColumnSortAllowed(self_: *const c_void) -> bool;
    pub fn wxDataViewCtrl_IsSelected(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxDataViewCtrl_Select(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_SelectAll(self_: *mut c_void);
    pub fn wxDataViewCtrl_SetAlternateRowColour(self_: *mut c_void, colour: *const c_void) -> bool;
    pub fn wxDataViewCtrl_SetExpanderColumn(self_: *mut c_void, col: *mut c_void);
    pub fn wxDataViewCtrl_SetCurrentItem(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_SetHeaderAttr(self_: *mut c_void, attr: *const c_void) -> bool;
    pub fn wxDataViewCtrl_SetIndent(self_: *mut c_void, indent: c_int);
    pub fn wxDataViewCtrl_SetSelections(self_: *mut c_void, sel: *const c_void);
    pub fn wxDataViewCtrl_Unselect(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_UnselectAll(self_: *mut c_void);
    pub fn wxDataViewCtrl_SetRowHeight(self_: *mut c_void, row_height: c_int) -> bool;
    pub fn wxDataViewCtrl_ToggleSortByColumn(self_: *mut c_void, column: c_int);
    pub fn wxDataViewCtrl_GetCountPerPage(self_: *const c_void) -> c_int;
    pub fn wxDataViewCtrl_GetTopItem(self_: *const c_void) -> *mut c_void;

    // wxDataViewIconText
    pub fn wxDataViewIconText_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewIconText_new(text: *const c_void, bitmap: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_new1(other: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_GetBitmapBundle(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_GetIcon(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_SetBitmapBundle(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxDataViewIconText_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxDataViewIconText_SetText(self_: *mut c_void, text: *const c_void);

    // wxDataViewItem
    pub fn wxDataViewItem_delete(self_: *mut c_void);
    pub fn wxDataViewItem_new() -> *mut c_void;
    pub fn wxDataViewItem_new1(item: *const c_void) -> *mut c_void;
    pub fn wxDataViewItem_new2(id: *mut c_void) -> *mut c_void;
    pub fn wxDataViewItem_GetID(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewItem_IsOk(self_: *const c_void) -> bool;

    // wxDataViewItemAttr
    pub fn wxDataViewItemAttr_delete(self_: *mut c_void);
    pub fn wxDataViewItemAttr_new() -> *mut c_void;
    pub fn wxDataViewItemAttr_SetBold(self_: *mut c_void, set: bool);
    pub fn wxDataViewItemAttr_SetColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxDataViewItemAttr_SetBackgroundColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxDataViewItemAttr_SetItalic(self_: *mut c_void, set: bool);
    pub fn wxDataViewItemAttr_SetStrikethrough(self_: *mut c_void, set: bool);
    pub fn wxDataViewItemAttr_HasColour(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewItemAttr_HasFont(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetBold(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetItalic(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_HasBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewItemAttr_IsDefault(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetEffectiveFont(
        self_: *const c_void,
        font: *const c_void,
    ) -> *mut c_void;

    // wxDataViewTreeCtrl
    pub fn wxDataViewTreeCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewTreeCtrl_new() -> *mut c_void;
    pub fn wxDataViewTreeCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDataViewTreeCtrl_~wxDataViewTreeCtrl(self_: *mut c_void);
    pub fn wxDataViewTreeCtrl_AppendContainer(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_AppendItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
    ) -> bool;
    pub fn wxDataViewTreeCtrl_DeleteAllItems(self_: *mut c_void);
    pub fn wxDataViewTreeCtrl_DeleteChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewTreeCtrl_DeleteItem(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewTreeCtrl_GetChildCount(self_: *const c_void, parent: *const c_void) -> c_int;
    pub fn wxDataViewTreeCtrl_GetImageList(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetItemData(self_: *const c_void, item: *const c_void)
        -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetItemExpandedIcon(
        self_: *const c_void,
        item: *const c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetItemIcon(self_: *const c_void, item: *const c_void)
        -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetItemParent(
        self_: *const c_void,
        item: wxDataViewItem,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetItemText(self_: *const c_void, item: *const c_void)
        -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetNthChild(
        self_: *const c_void,
        parent: *const c_void,
        pos: c_uint,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetStore(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetStore1(self_: *const c_void) -> *const c_void;
    pub fn wxDataViewTreeCtrl_InsertContainer(
        self_: *mut c_void,
        parent: *const c_void,
        previous: *const c_void,
        text: *const c_void,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_InsertItem(
        self_: *mut c_void,
        parent: *const c_void,
        previous: *const c_void,
        text: *const c_void,
        icon: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_IsContainer(self_: *mut c_void, item: *const c_void) -> bool;
    pub fn wxDataViewTreeCtrl_PrependContainer(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_PrependItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_SetImageList(self_: *mut c_void, imagelist: *mut c_void);
    pub fn wxDataViewTreeCtrl_SetItemData(
        self_: *mut c_void,
        item: *const c_void,
        data: *mut c_void,
    );
    pub fn wxDataViewTreeCtrl_SetItemExpandedIcon(
        self_: *mut c_void,
        item: *const c_void,
        icon: *const c_void,
    );
    pub fn wxDataViewTreeCtrl_SetItemIcon(
        self_: *mut c_void,
        item: *const c_void,
        icon: *const c_void,
    );
    pub fn wxDataViewTreeCtrl_SetItemText(
        self_: *mut c_void,
        item: *const c_void,
        text: *const c_void,
    );

    // wxDataViewValueAdjuster
    pub fn wxDataViewValueAdjuster_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDataViewValueAdjuster_MakeHighlighted(self_: *const c_void, value: *const c_void) -> wxVariant;

    // wxDateEvent
    pub fn wxDateEvent_CLASSINFO() -> *mut c_void;
    pub fn wxDateEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateEvent_new1(win: *mut c_void, dt: *const c_void, type_: wxEventType) -> *mut c_void;
    pub fn wxDateEvent_GetDate(self_: *const c_void) -> *mut c_void;
    pub fn wxDateEvent_SetDate(self_: *mut c_void, date: *const c_void);

    // wxDatePickerCtrl
    pub fn wxDatePickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDatePickerCtrl_new() -> *mut c_void;
    pub fn wxDatePickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxDatePickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxDatePickerCtrl_GetRange(
        self_: *const c_void,
        dt1: *mut c_void,
        dt2: *mut c_void,
    ) -> bool;
    pub fn wxDatePickerCtrl_GetValue(self_: *const c_void) -> *mut c_void;
    pub fn wxDatePickerCtrl_SetNullText(self_: *mut c_void, text: *const c_void);
    pub fn wxDatePickerCtrl_SetRange(self_: *mut c_void, dt1: *const c_void, dt2: *const c_void);
    pub fn wxDatePickerCtrl_SetValue(self_: *mut c_void, dt: *const c_void);

    // wxDelegateRendererNative
    pub fn wxDelegateRendererNative_delete(self_: *mut c_void);
    pub fn wxDelegateRendererNative_new() -> *mut c_void;
    pub fn wxDelegateRendererNative_new1(renderer_native: *mut c_void) -> *mut c_void;

    // wxDirPickerCtrl
    pub fn wxDirPickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDirPickerCtrl_new() -> *mut c_void;
    pub fn wxDirPickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxDirPickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxDirPickerCtrl_GetDirName(self_: *const c_void) -> *mut c_void;
    pub fn wxDirPickerCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxDirPickerCtrl_SetDirName(self_: *mut c_void, dirname: *const c_void);
    pub fn wxDirPickerCtrl_SetInitialDirectory(self_: *mut c_void, dir: *const c_void);
    pub fn wxDirPickerCtrl_SetPath(self_: *mut c_void, dirname: *const c_void);

    // wxDisplay
    pub fn wxDisplay_delete(self_: *mut c_void);
    pub fn wxDisplay_new() -> *mut c_void;
    pub fn wxDisplay_new1(index: c_uint) -> *mut c_void;
    pub fn wxDisplay_new2(window: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDisplay_~wxDisplay(self_: *mut c_void);
    pub fn wxDisplay_ChangeMode(self_: *mut c_void, mode: *const c_void) -> bool;
    pub fn wxDisplay_GetClientArea(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDisplay_GetCurrentMode(self_: *const c_void) -> wxVideoMode;
    pub fn wxDisplay_GetGeometry(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDisplay_GetModes(self_: *const c_void, mode: *const c_void) -> wxArrayVideoModes;
    pub fn wxDisplay_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxDisplay_GetPPI(self_: *const c_void) -> *mut c_void;
    pub fn wxDisplay_GetScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxDisplay_IsPrimary(self_: *const c_void) -> bool;
    pub fn wxDisplay_GetCount() -> c_uint;
    pub fn wxDisplay_GetFromPoint(pt: *const c_void) -> c_int;
    pub fn wxDisplay_GetFromWindow(win: *const c_void) -> c_int;
    pub fn wxDisplay_GetStdPPIValue() -> c_int;
    pub fn wxDisplay_GetStdPPI() -> *mut c_void;

    // wxDisplayChangedEvent
    pub fn wxDisplayChangedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxDisplayChangedEvent_new() -> *mut c_void;

    // wxDocChildFrame
    pub fn wxDocChildFrame_CLASSINFO() -> *mut c_void;
    pub fn wxDocChildFrame_new(
        doc: *mut c_void,
        view: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDocChildFrame_~wxDocChildFrame(self_: *mut c_void);
    pub fn wxDocChildFrame_GetDocument(self_: *const c_void) -> *mut c_void;
    pub fn wxDocChildFrame_GetView(self_: *const c_void) -> *mut c_void;
    pub fn wxDocChildFrame_SetDocument(self_: *mut c_void, doc: *mut c_void);
    pub fn wxDocChildFrame_SetView(self_: *mut c_void, view: *mut c_void);

    // wxDocMDIChildFrame
    pub fn wxDocMDIChildFrame_CLASSINFO() -> *mut c_void;
    pub fn wxDocMDIChildFrame_new(
        doc: *mut c_void,
        view: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDocMDIChildFrame_~wxDocMDIChildFrame(self_: *mut c_void);
    pub fn wxDocMDIChildFrame_GetDocument(self_: *const c_void) -> *mut c_void;
    pub fn wxDocMDIChildFrame_GetView(self_: *const c_void) -> *mut c_void;
    pub fn wxDocMDIChildFrame_SetDocument(self_: *mut c_void, doc: *mut c_void);
    pub fn wxDocMDIChildFrame_SetView(self_: *mut c_void, view: *mut c_void);

    // wxDocMDIParentFrame
    pub fn wxDocMDIParentFrame_CLASSINFO() -> *mut c_void;
    pub fn wxDocMDIParentFrame_new() -> *mut c_void;
    pub fn wxDocMDIParentFrame_new1(
        manager: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDocMDIParentFrame_~wxDocMDIParentFrame(self_: *mut c_void);
    pub fn wxDocMDIParentFrame_Create(
        self_: *mut c_void,
        manager: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxDocParentFrame
    pub fn wxDocParentFrame_CLASSINFO() -> *mut c_void;
    pub fn wxDocParentFrame_new() -> *mut c_void;
    pub fn wxDocParentFrame_new1(
        manager: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDocParentFrame_~wxDocParentFrame(self_: *mut c_void);
    pub fn wxDocParentFrame_Create(
        self_: *mut c_void,
        manager: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxDocParentFrame_GetDocumentManager(self_: *const c_void) -> *mut c_void;

    // wxDragImage
    pub fn wxDragImage_CLASSINFO() -> *mut c_void;
    pub fn wxDragImage_new() -> *mut c_void;
    pub fn wxDragImage_new1(image: *const c_void, cursor: *const c_void) -> *mut c_void;
    pub fn wxDragImage_new2(image: *const c_void, cursor: *const c_void) -> *mut c_void;
    pub fn wxDragImage_new3(text: *const c_void, cursor: *const c_void) -> *mut c_void;
    pub fn wxDragImage_new4(tree_ctrl: *const c_void, id: *mut c_void) -> *mut c_void;
    pub fn wxDragImage_new5(list_ctrl: *const c_void, id: c_long) -> *mut c_void;
    pub fn wxDragImage_BeginDrag(
        self_: *mut c_void,
        hotspot: *const c_void,
        window: *mut c_void,
        full_screen: bool,
        rect: *mut c_void,
    ) -> bool;
    pub fn wxDragImage_BeginDrag1(
        self_: *mut c_void,
        hotspot: *const c_void,
        window: *mut c_void,
        bounding_window: *mut c_void,
    ) -> bool;
    pub fn wxDragImage_DoDrawImage(
        self_: *const c_void,
        dc: *mut c_void,
        pos: *const c_void,
    ) -> bool;
    pub fn wxDragImage_EndDrag(self_: *mut c_void) -> bool;
    pub fn wxDragImage_GetImageRect(self_: *const c_void, pos: *const c_void) -> *mut c_void;
    pub fn wxDragImage_Hide(self_: *mut c_void) -> bool;
    pub fn wxDragImage_Move(self_: *mut c_void, pt: *const c_void) -> bool;
    pub fn wxDragImage_Show(self_: *mut c_void) -> bool;
    pub fn wxDragImage_UpdateBackingFromWindow(
        self_: *const c_void,
        window_dc: *mut c_void,
        dest_dc: *mut c_void,
        source_rect: *const c_void,
        dest_rect: *const c_void,
    ) -> bool;

    // wxDropTarget
    pub fn wxDropTarget_delete(self_: *mut c_void);
    pub fn wxDropTarget_new(data: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxDropTarget_~wxDropTarget(self_: *mut c_void);
    pub fn wxDropTarget_GetData(self_: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDropTarget_OnData(self_: *mut c_void, x: c_int, y: c_int, def_result: wxDragResult) -> wxDragResult;
    // NOT_SUPPORTED: pub fn wxDropTarget_OnDragOver(self_: *mut c_void, x: c_int, y: c_int, def_result: wxDragResult) -> wxDragResult;
    pub fn wxDropTarget_OnDrop(self_: *mut c_void, x: c_int, y: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxDropTarget_OnEnter(self_: *mut c_void, x: c_int, y: c_int, def_result: wxDragResult) -> wxDragResult;
    pub fn wxDropTarget_OnLeave(self_: *mut c_void);
    pub fn wxDropTarget_GetDataObject(self_: *const c_void) -> *mut c_void;
    pub fn wxDropTarget_SetDataObject(self_: *mut c_void, data: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDropTarget_SetDefaultAction(self_: *mut c_void, action: wxDragResult);
    // NOT_SUPPORTED: pub fn wxDropTarget_GetDefaultAction(self_: *mut c_void) -> wxDragResult;

    // wxEditableListBox
    pub fn wxEditableListBox_CLASSINFO() -> *mut c_void;
    pub fn wxEditableListBox_new() -> *mut c_void;
    pub fn wxEditableListBox_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxEditableListBox_~wxEditableListBox(self_: *mut c_void);
    pub fn wxEditableListBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxEditableListBox_SetStrings(self_: *mut c_void, strings: *const c_void);
    pub fn wxEditableListBox_GetStrings(self_: *const c_void, strings: *mut c_void);

    // wxEraseEvent
    pub fn wxEraseEvent_CLASSINFO() -> *mut c_void;
    pub fn wxEraseEvent_new(id: c_int, dc: *mut c_void) -> *mut c_void;
    pub fn wxEraseEvent_GetDC(self_: *const c_void) -> *mut c_void;

    // wxEventBlocker
    pub fn wxEventBlocker_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxEventBlocker_new(win: *mut c_void, type_: wxEventType) -> *mut c_void;
    // DTOR: pub fn wxEventBlocker_~wxEventBlocker(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEventBlocker_Block(self_: *mut c_void, event_type: wxEventType);

    // wxFileCtrl
    pub fn wxFileCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxFileCtrl_new() -> *mut c_void;
    pub fn wxFileCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        default_directory: *const c_void,
        default_filename: *const c_void,
        wild_card: *const c_void,
        style: c_long,
        pos: *const c_void,
        size: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFileCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        default_directory: *const c_void,
        default_filename: *const c_void,
        wild_card: *const c_void,
        style: c_long,
        pos: *const c_void,
        size: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxFileCtrl_GetDirectory(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrl_GetFilename(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrl_GetFilenames(self_: *const c_void, filenames: *mut c_void);
    pub fn wxFileCtrl_GetFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxFileCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrl_GetPaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxFileCtrl_GetWildcard(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrl_SetDirectory(self_: *mut c_void, directory: *const c_void) -> bool;
    pub fn wxFileCtrl_SetFilename(self_: *mut c_void, filename: *const c_void) -> bool;
    pub fn wxFileCtrl_SetPath(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxFileCtrl_SetFilterIndex(self_: *mut c_void, filter_index: c_int);
    pub fn wxFileCtrl_SetWildcard(self_: *mut c_void, wild_card: *const c_void);
    pub fn wxFileCtrl_ShowHidden(self_: *mut c_void, show: bool);

    // wxFileCtrlEvent
    pub fn wxFileCtrlEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileCtrlEvent_new(type_: wxEventType, evt_object: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxFileCtrlEvent_GetDirectory(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrlEvent_GetFile(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrlEvent_GetFiles(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrlEvent_GetFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxFileCtrlEvent_SetFiles(self_: *mut c_void, files: *const c_void);
    pub fn wxFileCtrlEvent_SetDirectory(self_: *mut c_void, directory: *const c_void);
    pub fn wxFileCtrlEvent_SetFilterIndex(self_: *mut c_void, index: c_int);

    // wxFileDialogCustomize
    pub fn wxFileDialogCustomize_delete(self_: *mut c_void);
    pub fn wxFileDialogCustomize_AddButton(self_: *mut c_void, label: *const c_void)
        -> *mut c_void;
    pub fn wxFileDialogCustomize_AddCheckBox(
        self_: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxFileDialogCustomize_AddRadioButton(
        self_: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxFileDialogCustomize_AddChoice(
        self_: *mut c_void,
        n: usize,
        strings: *const c_void,
    ) -> *mut c_void;
    pub fn wxFileDialogCustomize_AddTextCtrl(
        self_: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxFileDialogCustomize_AddStaticText(
        self_: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;

    // wxFileDialogCustomizeHook
    pub fn wxFileDialogCustomizeHook_delete(self_: *mut c_void);
    pub fn wxFileDialogCustomizeHook_AddCustomControls(self_: *mut c_void, customizer: *mut c_void);
    pub fn wxFileDialogCustomizeHook_UpdateCustomControls(self_: *mut c_void);
    pub fn wxFileDialogCustomizeHook_TransferDataFromCustomControls(self_: *mut c_void);

    // wxFileDirPickerEvent
    pub fn wxFileDirPickerEvent_CLASSINFO() -> *mut c_void;
    pub fn wxFileDirPickerEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileDirPickerEvent_new1(type_: wxEventType, generator: *mut c_void, id: c_int, path: *const c_void) -> *mut c_void;
    pub fn wxFileDirPickerEvent_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDirPickerEvent_SetPath(self_: *mut c_void, path: *const c_void);

    // wxFilePickerCtrl
    pub fn wxFilePickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxFilePickerCtrl_new() -> *mut c_void;
    pub fn wxFilePickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        wildcard: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFilePickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        wildcard: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxFilePickerCtrl_GetFileName(self_: *const c_void) -> *mut c_void;
    pub fn wxFilePickerCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFilePickerCtrl_SetFileName(self_: *mut c_void, filename: *const c_void);
    pub fn wxFilePickerCtrl_SetInitialDirectory(self_: *mut c_void, dir: *const c_void);
    pub fn wxFilePickerCtrl_SetPath(self_: *mut c_void, filename: *const c_void);

    // wxFindDialogEvent
    pub fn wxFindDialogEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFindDialogEvent_new(command_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxFindDialogEvent_GetDialog(self_: *const c_void) -> *mut c_void;
    pub fn wxFindDialogEvent_GetFindString(self_: *const c_void) -> *mut c_void;
    pub fn wxFindDialogEvent_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxFindDialogEvent_GetReplaceString(self_: *const c_void) -> *mut c_void;

    // wxFindReplaceData
    pub fn wxFindReplaceData_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFindReplaceData_new(flags: wxUint32) -> *mut c_void;
    pub fn wxFindReplaceData_GetFindString(self_: *const c_void) -> *mut c_void;
    pub fn wxFindReplaceData_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxFindReplaceData_GetReplaceString(self_: *const c_void) -> *mut c_void;
    pub fn wxFindReplaceData_SetFindString(self_: *mut c_void, str: *const c_void);
    // NOT_SUPPORTED: pub fn wxFindReplaceData_SetFlags(self_: *mut c_void, flags: wxUint32);
    pub fn wxFindReplaceData_SetReplaceString(self_: *mut c_void, str: *const c_void);

    // wxFlexGridSizer
    pub fn wxFlexGridSizer_CLASSINFO() -> *mut c_void;
    pub fn wxFlexGridSizer_new(cols: c_int, vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxFlexGridSizer_new1(cols: c_int, gap: *const c_void) -> *mut c_void;
    pub fn wxFlexGridSizer_new2(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxFlexGridSizer_new3(rows: c_int, cols: c_int, gap: *const c_void) -> *mut c_void;
    pub fn wxFlexGridSizer_AddGrowableCol(self_: *mut c_void, idx: usize, proportion: c_int);
    pub fn wxFlexGridSizer_AddGrowableRow(self_: *mut c_void, idx: usize, proportion: c_int);
    pub fn wxFlexGridSizer_GetFlexibleDirection(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxFlexGridSizer_GetNonFlexibleGrowMode(self_: *const c_void) -> wxFlexSizerGrowMode;
    pub fn wxFlexGridSizer_IsColGrowable(self_: *mut c_void, idx: usize) -> bool;
    pub fn wxFlexGridSizer_IsRowGrowable(self_: *mut c_void, idx: usize) -> bool;
    pub fn wxFlexGridSizer_RemoveGrowableCol(self_: *mut c_void, idx: usize);
    pub fn wxFlexGridSizer_RemoveGrowableRow(self_: *mut c_void, idx: usize);
    pub fn wxFlexGridSizer_SetFlexibleDirection(self_: *mut c_void, direction: c_int);
    // NOT_SUPPORTED: pub fn wxFlexGridSizer_SetNonFlexibleGrowMode(self_: *mut c_void, mode: wxFlexSizerGrowMode);
    pub fn wxFlexGridSizer_GetRowHeights(self_: *const c_void) -> *mut c_void;
    pub fn wxFlexGridSizer_GetColWidths(self_: *const c_void) -> *mut c_void;

    // wxFocusEvent
    pub fn wxFocusEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFocusEvent_new(event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxFocusEvent_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxFocusEvent_SetWindow(self_: *mut c_void, win: *mut c_void);

    // wxFont
    pub fn wxFont_CLASSINFO() -> *mut c_void;
    pub fn wxFont_GetBaseFont(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_GetEncoding(self_: *const c_void) -> wxFontEncoding;
    pub fn wxFont_GetFaceName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_GetFamily(self_: *const c_void) -> wxFontFamily;
    pub fn wxFont_GetNativeFontInfoDesc(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_GetNativeFontInfoUserDesc(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_GetNativeFontInfo(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_GetPointSize(self_: *const c_void) -> c_int;
    pub fn wxFont_GetFractionalPointSize(self_: *const c_void) -> c_double;
    pub fn wxFont_GetPixelSize(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_GetStyle(self_: *const c_void) -> wxFontStyle;
    pub fn wxFont_GetUnderlined(self_: *const c_void) -> bool;
    pub fn wxFont_GetStrikethrough(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFont_GetWeight(self_: *const c_void) -> wxFontWeight;
    pub fn wxFont_GetNumericWeight(self_: *const c_void) -> c_int;
    pub fn wxFont_IsFixedWidth(self_: *const c_void) -> bool;
    pub fn wxFont_IsOk(self_: *const c_void) -> bool;
    pub fn wxFont_AddPrivateFont(filename: *const c_void) -> bool;
    pub fn wxFont_Bold(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Italic(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Larger(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Smaller(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Underlined(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Strikethrough(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_MakeBold(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeItalic(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeLarger(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeSmaller(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeUnderlined(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeStrikethrough(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_Scale(self_: *mut c_void, x: float) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_Scaled(self_: *const c_void, x: float) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_SetEncoding(self_: *mut c_void, encoding: wxFontEncoding);
    pub fn wxFont_SetFaceName(self_: *mut c_void, face_name: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFont_SetFamily(self_: *mut c_void, family: wxFontFamily);
    pub fn wxFont_SetNativeFontInfo(self_: *mut c_void, info: *const c_void) -> bool;
    pub fn wxFont_SetNativeFontInfoUserDesc(self_: *mut c_void, info: *const c_void) -> bool;
    pub fn wxFont_SetNativeFontInfo1(self_: *mut c_void, info: *const c_void);
    pub fn wxFont_SetPointSize(self_: *mut c_void, point_size: c_int);
    pub fn wxFont_SetFractionalPointSize(self_: *mut c_void, point_size: c_double);
    pub fn wxFont_SetPixelSize(self_: *mut c_void, pixel_size: *const c_void);
    // NOT_SUPPORTED: pub fn wxFont_SetStyle(self_: *mut c_void, style: wxFontStyle);
    // NOT_SUPPORTED: pub fn wxFont_SetSymbolicSize(self_: *mut c_void, size: wxFontSymbolicSize);
    // NOT_SUPPORTED: pub fn wxFont_SetSymbolicSizeRelativeTo(self_: *mut c_void, size: wxFontSymbolicSize, base: c_int);
    pub fn wxFont_SetUnderlined(self_: *mut c_void, underlined: bool);
    pub fn wxFont_SetStrikethrough(self_: *mut c_void, strikethrough: bool);
    // NOT_SUPPORTED: pub fn wxFont_SetWeight(self_: *mut c_void, weight: wxFontWeight);
    pub fn wxFont_SetNumericWeight(self_: *mut c_void, weight: c_int);
    // BLOCKED: pub fn wxFont_operator!=(self_: *const c_void, font: *const c_void) -> bool;
    // BLOCKED: pub fn wxFont_operator==(self_: *const c_void, font: *const c_void) -> bool;
    // BLOCKED: pub fn wxFont_operator=(self_: *mut c_void, font: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_GetDefaultEncoding() -> wxFontEncoding;
    // NOT_SUPPORTED: pub fn wxFont_SetDefaultEncoding(encoding: wxFontEncoding);
    // NOT_SUPPORTED: pub fn wxFont_GetNumericWeightOf(weight: wxFontWeight) -> c_int;
    // NOT_SUPPORTED: pub fn wxFont_New(point_size: c_int, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_New1(point_size: c_int, family: wxFontFamily, flags: c_int, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_New2(pixel_size: *const c_void, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_New3(pixel_size: *const c_void, family: wxFontFamily, flags: c_int, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    pub fn wxFont_New4(native_info: *const c_void) -> *mut c_void;
    pub fn wxFont_New5(native_info_string: *const c_void) -> *mut c_void;
    pub fn wxFont_new() -> *mut c_void;
    pub fn wxFont_new1(font: *const c_void) -> *mut c_void;
    pub fn wxFont_new2(font_info: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_new3(point_size: c_int, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_new4(pixel_size: *const c_void, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    pub fn wxFont_new5(native_info_string: *const c_void) -> *mut c_void;
    pub fn wxFont_new6(native_info: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxFont_~wxFont(self_: *mut c_void);

    // wxFontData
    pub fn wxFontData_CLASSINFO() -> *mut c_void;
    pub fn wxFontData_new() -> *mut c_void;
    pub fn wxFontData_EnableEffects(self_: *mut c_void, enable: bool);
    pub fn wxFontData_GetAllowSymbols(self_: *const c_void) -> bool;
    pub fn wxFontData_GetChosenFont(self_: *const c_void) -> *mut c_void;
    pub fn wxFontData_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxFontData_GetEnableEffects(self_: *const c_void) -> bool;
    pub fn wxFontData_GetRestrictSelection(self_: *const c_void) -> c_int;
    pub fn wxFontData_GetInitialFont(self_: *const c_void) -> *mut c_void;
    pub fn wxFontData_GetShowHelp(self_: *const c_void) -> bool;
    pub fn wxFontData_RestrictSelection(self_: *mut c_void, flags: c_int);
    pub fn wxFontData_SetAllowSymbols(self_: *mut c_void, allow_symbols: bool);
    pub fn wxFontData_SetChosenFont(self_: *mut c_void, font: *const c_void);
    pub fn wxFontData_SetColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxFontData_SetInitialFont(self_: *mut c_void, font: *const c_void);
    pub fn wxFontData_SetRange(self_: *mut c_void, min: c_int, max: c_int);
    pub fn wxFontData_SetShowHelp(self_: *mut c_void, show_help: bool);
    // BLOCKED: pub fn wxFontData_operator=(self_: *mut c_void, data: *const c_void) -> *mut c_void;

    // wxFontEnumerator
    pub fn wxFontEnumerator_delete(self_: *mut c_void);
    pub fn wxFontEnumerator_new() -> *mut c_void;
    // DTOR: pub fn wxFontEnumerator_~wxFontEnumerator(self_: *mut c_void);
    pub fn wxFontEnumerator_EnumerateEncodings(self_: *mut c_void, font: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFontEnumerator_EnumerateFacenames(self_: *mut c_void, encoding: wxFontEncoding, fixed_width_only: bool) -> bool;
    pub fn wxFontEnumerator_OnFacename(self_: *mut c_void, font: *const c_void) -> bool;
    pub fn wxFontEnumerator_OnFontEncoding(
        self_: *mut c_void,
        font: *const c_void,
        encoding: *const c_void,
    ) -> bool;
    pub fn wxFontEnumerator_GetEncodings(facename: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFontEnumerator_GetFacenames(encoding: wxFontEncoding, fixed_width_only: bool) -> *mut c_void;
    pub fn wxFontEnumerator_IsValidFacename(facename: *const c_void) -> bool;
    pub fn wxFontEnumerator_InvalidateCache();

    // wxFontList
    pub fn wxFontList_delete(self_: *mut c_void);
    pub fn wxFontList_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFontList_FindOrCreateFont(self_: *mut c_void, point_size: c_int, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, facename: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    pub fn wxFontList_FindOrCreateFont1(
        self_: *mut c_void,
        font_info: *const c_void,
    ) -> *mut c_void;

    // wxFontMapper
    pub fn wxFontMapper_delete(self_: *mut c_void);
    pub fn wxFontMapper_new() -> *mut c_void;
    // DTOR: pub fn wxFontMapper_~wxFontMapper(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxFontMapper_CharsetToEncoding(self_: *mut c_void, charset: *const c_void, interactive: bool) -> wxFontEncoding;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetAltForEncoding(self_: *mut c_void, encoding: wxFontEncoding, info: *mut c_void, facename: *const c_void, interactive: bool) -> bool;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetAltForEncoding1(self_: *mut c_void, encoding: wxFontEncoding, alt_encoding: *mut c_void, facename: *const c_void, interactive: bool) -> bool;
    // NOT_SUPPORTED: pub fn wxFontMapper_IsEncodingAvailable(self_: *mut c_void, encoding: wxFontEncoding, facename: *const c_void) -> bool;
    pub fn wxFontMapper_SetConfigPath(self_: *mut c_void, prefix: *const c_void);
    pub fn wxFontMapper_SetDialogParent(self_: *mut c_void, parent: *mut c_void);
    pub fn wxFontMapper_SetDialogTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxFontMapper_Get() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetAllEncodingNames(encoding: wxFontEncoding) -> *const c_void;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetEncoding(n: usize) -> wxFontEncoding;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetEncodingDescription(encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetEncodingFromName(encoding: *const c_void) -> wxFontEncoding;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetEncodingName(encoding: wxFontEncoding) -> *mut c_void;
    pub fn wxFontMapper_GetSupportedEncodingsCount() -> usize;
    pub fn wxFontMapper_Set(mapper: *mut c_void) -> *mut c_void;

    // wxFontPickerCtrl
    pub fn wxFontPickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxFontPickerCtrl_new() -> *mut c_void;
    pub fn wxFontPickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        font: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFontPickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        font: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxFontPickerCtrl_GetMaxPointSize(self_: *const c_void) -> c_uint;
    pub fn wxFontPickerCtrl_GetMinPointSize(self_: *const c_void) -> c_uint;
    pub fn wxFontPickerCtrl_GetSelectedColour(self_: *const c_void) -> *mut c_void;
    pub fn wxFontPickerCtrl_GetSelectedFont(self_: *const c_void) -> *mut c_void;
    pub fn wxFontPickerCtrl_SetMaxPointSize(self_: *mut c_void, max: c_uint);
    pub fn wxFontPickerCtrl_SetMinPointSize(self_: *mut c_void, min: c_uint);
    pub fn wxFontPickerCtrl_SetSelectedColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxFontPickerCtrl_SetSelectedFont(self_: *mut c_void, font: *const c_void);

    // wxFontPickerEvent
    pub fn wxFontPickerEvent_CLASSINFO() -> *mut c_void;
    pub fn wxFontPickerEvent_new(
        generator: *mut c_void,
        id: c_int,
        font: *const c_void,
    ) -> *mut c_void;
    pub fn wxFontPickerEvent_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxFontPickerEvent_SetFont(self_: *mut c_void, f: *const c_void);

    // wxFrame
    pub fn wxFrame_CLASSINFO() -> *mut c_void;
    pub fn wxFrame_new() -> *mut c_void;
    pub fn wxFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxFrame_~wxFrame(self_: *mut c_void);
    pub fn wxFrame_Centre(self_: *mut c_void, direction: c_int);
    pub fn wxFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxFrame_CreateStatusBar(
        self_: *mut c_void,
        number: c_int,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_CreateToolBar(
        self_: *mut c_void,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_DoGiveHelp(self_: *mut c_void, text: *const c_void, show: bool);
    pub fn wxFrame_GetMenuBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_GetStatusBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_GetStatusBarPane(self_: *const c_void) -> c_int;
    pub fn wxFrame_GetToolBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_OnCreateStatusBar(
        self_: *mut c_void,
        number: c_int,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_OnCreateToolBar(
        self_: *mut c_void,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_ProcessCommand(self_: *mut c_void, id: c_int) -> bool;
    pub fn wxFrame_SetMenuBar(self_: *mut c_void, menu_bar: *mut c_void);
    pub fn wxFrame_SetStatusBar(self_: *mut c_void, status_bar: *mut c_void);
    pub fn wxFrame_SetStatusBarPane(self_: *mut c_void, n: c_int);
    pub fn wxFrame_SetStatusText(self_: *mut c_void, text: *const c_void, number: c_int);
    pub fn wxFrame_SetStatusWidths(self_: *mut c_void, n: c_int, widths_field: *const c_void);
    pub fn wxFrame_SetToolBar(self_: *mut c_void, tool_bar: *mut c_void);
    pub fn wxFrame_MSWGetTaskBarButton(self_: *mut c_void) -> *mut c_void;
    pub fn wxFrame_PushStatusText(self_: *mut c_void, text: *const c_void, number: c_int);
    pub fn wxFrame_PopStatusText(self_: *mut c_void, number: c_int);

    // wxFullScreenEvent
    pub fn wxFullScreenEvent_CLASSINFO() -> *mut c_void;
    pub fn wxFullScreenEvent_new(id: c_int, fullscreen: bool) -> *mut c_void;
    pub fn wxFullScreenEvent_IsFullScreen(self_: *const c_void) -> bool;

    // wxGBPosition
    pub fn wxGBPosition_delete(self_: *mut c_void);
    pub fn wxGBPosition_new() -> *mut c_void;
    pub fn wxGBPosition_new1(row: c_int, col: c_int) -> *mut c_void;
    pub fn wxGBPosition_GetCol(self_: *const c_void) -> c_int;
    pub fn wxGBPosition_GetRow(self_: *const c_void) -> c_int;
    pub fn wxGBPosition_SetCol(self_: *mut c_void, col: c_int);
    pub fn wxGBPosition_SetRow(self_: *mut c_void, row: c_int);
    // BLOCKED: pub fn wxGBPosition_operator!=(self_: *const c_void, p: *const c_void) -> bool;
    // BLOCKED: pub fn wxGBPosition_operator==(self_: *const c_void, p: *const c_void) -> bool;

    // wxGBSpan
    pub fn wxGBSpan_delete(self_: *mut c_void);
    pub fn wxGBSpan_new() -> *mut c_void;
    pub fn wxGBSpan_new1(rowspan: c_int, colspan: c_int) -> *mut c_void;
    pub fn wxGBSpan_GetColspan(self_: *const c_void) -> c_int;
    pub fn wxGBSpan_GetRowspan(self_: *const c_void) -> c_int;
    pub fn wxGBSpan_SetColspan(self_: *mut c_void, colspan: c_int);
    pub fn wxGBSpan_SetRowspan(self_: *mut c_void, rowspan: c_int);
    // BLOCKED: pub fn wxGBSpan_operator!=(self_: *const c_void, o: *const c_void) -> bool;
    // BLOCKED: pub fn wxGBSpan_operator==(self_: *const c_void, o: *const c_void) -> bool;

    // wxGDIObject
    pub fn wxGDIObject_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxGDIObject_new() -> *mut c_void;

    // wxGauge
    pub fn wxGauge_CLASSINFO() -> *mut c_void;
    pub fn wxGauge_new() -> *mut c_void;
    pub fn wxGauge_new1(
        parent: *mut c_void,
        id: c_int,
        range: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxGauge_~wxGauge(self_: *mut c_void);
    pub fn wxGauge_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        range: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxGauge_GetRange(self_: *const c_void) -> c_int;
    pub fn wxGauge_GetValue(self_: *const c_void) -> c_int;
    pub fn wxGauge_IsVertical(self_: *const c_void) -> bool;
    pub fn wxGauge_Pulse(self_: *mut c_void);
    pub fn wxGauge_SetRange(self_: *mut c_void, range: c_int);
    pub fn wxGauge_SetValue(self_: *mut c_void, pos: c_int);

    // wxGenericAboutDialog
    pub fn wxGenericAboutDialog_delete(self_: *mut c_void);
    pub fn wxGenericAboutDialog_new() -> *mut c_void;
    pub fn wxGenericAboutDialog_new1(info: *const c_void, parent: *mut c_void) -> *mut c_void;
    pub fn wxGenericAboutDialog_Create(
        self_: *mut c_void,
        info: *const c_void,
        parent: *mut c_void,
    ) -> bool;

    // wxGenericAnimationCtrl
    pub fn wxGenericAnimationCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxGenericAnimationCtrl_new(
        parent: *mut c_void,
        id: c_int,
        anim: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxGenericAnimationCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        anim: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxGenericAnimationCtrl_DrawCurrentFrame(self_: *mut c_void, dc: *mut c_void);
    pub fn wxGenericAnimationCtrl_GetBackingStore(self_: *mut c_void) -> *mut c_void;
    pub fn wxGenericAnimationCtrl_Play(self_: *mut c_void, looped: bool) -> bool;
    pub fn wxGenericAnimationCtrl_SetUseWindowBackgroundColour(
        self_: *mut c_void,
        use_win_background: bool,
    );
    pub fn wxGenericAnimationCtrl_IsUsingWindowBackgroundColour(self_: *const c_void) -> bool;

    // wxGenericDirCtrl
    pub fn wxGenericDirCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxGenericDirCtrl_new() -> *mut c_void;
    pub fn wxGenericDirCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        dir: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        filter: *const c_void,
        default_filter: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxGenericDirCtrl_~wxGenericDirCtrl(self_: *mut c_void);
    pub fn wxGenericDirCtrl_CollapsePath(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxGenericDirCtrl_CollapseTree(self_: *mut c_void);
    pub fn wxGenericDirCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        dir: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        filter: *const c_void,
        default_filter: c_int,
        name: *const c_void,
    ) -> bool;
    pub fn wxGenericDirCtrl_ExpandPath(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxGenericDirCtrl_GetDefaultPath(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilePath(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilePaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxGenericDirCtrl_GetFilter(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxGenericDirCtrl_GetFilterListCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetPath1(self_: *const c_void, item_id: wxTreeItemId) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetPaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxGenericDirCtrl_GetRootId(self_: *mut c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetTreeCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_Init(self_: *mut c_void);
    pub fn wxGenericDirCtrl_ReCreateTree(self_: *mut c_void);
    pub fn wxGenericDirCtrl_SetDefaultPath(self_: *mut c_void, path: *const c_void);
    pub fn wxGenericDirCtrl_SetFilter(self_: *mut c_void, filter: *const c_void);
    pub fn wxGenericDirCtrl_SetFilterIndex(self_: *mut c_void, n: c_int);
    pub fn wxGenericDirCtrl_SetPath(self_: *mut c_void, path: *const c_void);
    pub fn wxGenericDirCtrl_ShowHidden(self_: *mut c_void, show: bool);
    pub fn wxGenericDirCtrl_SelectPath(self_: *mut c_void, path: *const c_void, select: bool);
    pub fn wxGenericDirCtrl_SelectPaths(self_: *mut c_void, paths: *const c_void);
    pub fn wxGenericDirCtrl_UnselectAll(self_: *mut c_void);

    // wxGenericValidator
    pub fn wxGenericValidator_CLASSINFO() -> *mut c_void;
    pub fn wxGenericValidator_new(validator: *const c_void) -> *mut c_void;
    pub fn wxGenericValidator_new1(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new2(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new3(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new4(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new5(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new6(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new7(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new8(val_ptr: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxGenericValidator_~wxGenericValidator(self_: *mut c_void);

    // wxGestureEvent
    pub fn wxGestureEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGestureEvent_new(winid: c_int, type_: wxEventType) -> *mut c_void;
    pub fn wxGestureEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxGestureEvent_IsGestureStart(self_: *const c_void) -> bool;
    pub fn wxGestureEvent_IsGestureEnd(self_: *const c_void) -> bool;
    pub fn wxGestureEvent_SetPosition(self_: *mut c_void, pos: *const c_void);
    pub fn wxGestureEvent_SetGestureStart(self_: *mut c_void, is_start: bool);
    pub fn wxGestureEvent_SetGestureEnd(self_: *mut c_void, is_end: bool);

    // wxGraphicsBrush
    pub fn wxGraphicsBrush_CLASSINFO() -> *mut c_void;

    // wxGraphicsFont
    pub fn wxGraphicsFont_CLASSINFO() -> *mut c_void;

    // wxGraphicsGradientStop
    pub fn wxGraphicsGradientStop_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStop_new(col: wxColour, pos: float) -> *mut c_void;
    pub fn wxGraphicsGradientStop_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsGradientStop_SetColour(self_: *mut c_void, col: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStop_GetPosition(self_: *const c_void) -> float;
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStop_SetPosition(self_: *mut c_void, pos: float);

    // wxGraphicsGradientStops
    pub fn wxGraphicsGradientStops_delete(self_: *mut c_void);
    pub fn wxGraphicsGradientStops_new(start_col: wxColour, end_col: wxColour) -> *mut c_void;
    pub fn wxGraphicsGradientStops_Add(self_: *mut c_void, stop: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStops_Add1(self_: *mut c_void, col: wxColour, pos: float);
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStops_Item(self_: *const c_void, n: unsigned) -> *mut c_void;
    pub fn wxGraphicsGradientStops_GetCount(self_: *const c_void) -> usize;
    pub fn wxGraphicsGradientStops_SetStartColour(self_: *mut c_void, col: wxColour);
    pub fn wxGraphicsGradientStops_GetStartColour(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsGradientStops_SetEndColour(self_: *mut c_void, col: wxColour);
    pub fn wxGraphicsGradientStops_GetEndColour(self_: *const c_void) -> *mut c_void;

    // wxGraphicsMatrix
    pub fn wxGraphicsMatrix_CLASSINFO() -> *mut c_void;
    pub fn wxGraphicsMatrix_Concat(self_: *mut c_void, t: *const c_void);
    pub fn wxGraphicsMatrix_Concat1(self_: *mut c_void, t: *const c_void);
    pub fn wxGraphicsMatrix_Get(
        self_: *const c_void,
        a: *mut c_void,
        b: *mut c_void,
        c: *mut c_void,
        d: *mut c_void,
        tx: *mut c_void,
        ty: *mut c_void,
    );
    pub fn wxGraphicsMatrix_GetNativeMatrix(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsMatrix_Invert(self_: *mut c_void);
    pub fn wxGraphicsMatrix_IsEqual(self_: *const c_void, t: *const c_void) -> bool;
    pub fn wxGraphicsMatrix_IsEqual1(self_: *const c_void, t: *const c_void) -> bool;
    pub fn wxGraphicsMatrix_IsIdentity(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxGraphicsMatrix_Rotate(self_: *mut c_void, angle: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsMatrix_Scale(self_: *mut c_void, x_scale: wxDouble, y_scale: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsMatrix_Set(self_: *mut c_void, a: wxDouble, b: wxDouble, c: wxDouble, d: wxDouble, tx: wxDouble, ty: wxDouble);
    pub fn wxGraphicsMatrix_TransformDistance(
        self_: *const c_void,
        dx: *mut c_void,
        dy: *mut c_void,
    );
    pub fn wxGraphicsMatrix_TransformPoint(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsMatrix_Translate(self_: *mut c_void, dx: wxDouble, dy: wxDouble);

    // wxGraphicsObject
    pub fn wxGraphicsObject_CLASSINFO() -> *mut c_void;
    pub fn wxGraphicsObject_GetRenderer(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsObject_IsNull(self_: *const c_void) -> bool;

    // wxGraphicsPath
    pub fn wxGraphicsPath_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddArc(self_: *mut c_void, x: wxDouble, y: wxDouble, r: wxDouble, start_angle: wxDouble, end_angle: wxDouble, clockwise: bool);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddArc1(self_: *mut c_void, c: *const c_void, r: wxDouble, start_angle: wxDouble, end_angle: wxDouble, clockwise: bool);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddArcToPoint(self_: *mut c_void, x1: wxDouble, y1: wxDouble, x2: wxDouble, y2: wxDouble, r: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddCircle(self_: *mut c_void, x: wxDouble, y: wxDouble, r: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddCurveToPoint(self_: *mut c_void, cx1: wxDouble, cy1: wxDouble, cx2: wxDouble, cy2: wxDouble, x: wxDouble, y: wxDouble);
    pub fn wxGraphicsPath_AddCurveToPoint1(
        self_: *mut c_void,
        c1: *const c_void,
        c2: *const c_void,
        e: *const c_void,
    );
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddEllipse(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddLineToPoint(self_: *mut c_void, x: wxDouble, y: wxDouble);
    pub fn wxGraphicsPath_AddLineToPoint1(self_: *mut c_void, p: *const c_void);
    pub fn wxGraphicsPath_AddPath(self_: *mut c_void, path: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddQuadCurveToPoint(self_: *mut c_void, cx: wxDouble, cy: wxDouble, x: wxDouble, y: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddRectangle(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddRoundedRectangle(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble, radius: wxDouble);
    pub fn wxGraphicsPath_CloseSubpath(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_Contains(self_: *const c_void, c: *const c_void, fill_style: wxPolygonFillMode) -> bool;
    // NOT_SUPPORTED: pub fn wxGraphicsPath_Contains1(self_: *const c_void, x: wxDouble, y: wxDouble, fill_style: wxPolygonFillMode) -> bool;
    pub fn wxGraphicsPath_GetBox(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsPath_GetBox1(
        self_: *const c_void,
        x: *mut c_void,
        y: *mut c_void,
        w: *mut c_void,
        h: *mut c_void,
    );
    pub fn wxGraphicsPath_GetCurrentPoint(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxGraphicsPath_GetCurrentPoint1(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsPath_GetNativePath(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsPath_MoveToPoint(self_: *mut c_void, x: wxDouble, y: wxDouble);
    pub fn wxGraphicsPath_MoveToPoint1(self_: *mut c_void, p: *const c_void);
    pub fn wxGraphicsPath_Transform(self_: *mut c_void, matrix: *const c_void);
    pub fn wxGraphicsPath_UnGetNativePath(self_: *const c_void, p: *mut c_void);

    // wxGraphicsPen
    pub fn wxGraphicsPen_CLASSINFO() -> *mut c_void;

    // wxGridBagSizer
    pub fn wxGridBagSizer_CLASSINFO() -> *mut c_void;
    pub fn wxGridBagSizer_new(vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxGridBagSizer_Add(
        self_: *mut c_void,
        window: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGridBagSizer_Add1(
        self_: *mut c_void,
        sizer: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGridBagSizer_Add2(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_Add3(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGridBagSizer_CheckForIntersection(
        self_: *mut c_void,
        item: *mut c_void,
        exclude_item: *mut c_void,
    ) -> bool;
    pub fn wxGridBagSizer_CheckForIntersection1(
        self_: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
        exclude_item: *mut c_void,
    ) -> bool;
    pub fn wxGridBagSizer_FindItem(self_: *mut c_void, window: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_FindItem1(self_: *mut c_void, sizer: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_FindItemAtPoint(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxGridBagSizer_FindItemAtPosition(self_: *mut c_void, pos: *const c_void)
        -> *mut c_void;
    pub fn wxGridBagSizer_FindItemWithData(
        self_: *mut c_void,
        user_data: *const c_void,
    ) -> *mut c_void;
    pub fn wxGridBagSizer_GetCellSize(self_: *const c_void, row: c_int, col: c_int) -> *mut c_void;
    pub fn wxGridBagSizer_GetEmptyCellSize(self_: *const c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemPosition(self_: *mut c_void, window: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemPosition1(self_: *mut c_void, sizer: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemPosition2(self_: *mut c_void, index: usize) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemSpan(self_: *mut c_void, window: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemSpan1(self_: *mut c_void, sizer: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemSpan2(self_: *mut c_void, index: usize) -> *mut c_void;
    pub fn wxGridBagSizer_SetEmptyCellSize(self_: *mut c_void, sz: *const c_void);
    pub fn wxGridBagSizer_SetItemPosition(
        self_: *mut c_void,
        window: *mut c_void,
        pos: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemPosition1(
        self_: *mut c_void,
        sizer: *mut c_void,
        pos: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemPosition2(
        self_: *mut c_void,
        index: usize,
        pos: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemSpan(
        self_: *mut c_void,
        window: *mut c_void,
        span: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemSpan1(
        self_: *mut c_void,
        sizer: *mut c_void,
        span: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemSpan2(
        self_: *mut c_void,
        index: usize,
        span: *const c_void,
    ) -> bool;

    // wxGridEditorCreatedEvent
    pub fn wxGridEditorCreatedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxGridEditorCreatedEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridEditorCreatedEvent_new1(id: c_int, type_: wxEventType, obj: *mut c_void, row: c_int, col: c_int, ctrl: *mut c_void) -> *mut c_void;
    pub fn wxGridEditorCreatedEvent_GetCol(self_: *const c_void) -> c_int;
    pub fn wxGridEditorCreatedEvent_GetControl(self_: *mut c_void) -> *mut c_void;
    pub fn wxGridEditorCreatedEvent_GetRow(self_: *const c_void) -> c_int;
    pub fn wxGridEditorCreatedEvent_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxGridEditorCreatedEvent_SetCol(self_: *mut c_void, col: c_int);
    pub fn wxGridEditorCreatedEvent_SetControl(self_: *mut c_void, ctrl: *mut c_void);
    pub fn wxGridEditorCreatedEvent_SetRow(self_: *mut c_void, row: c_int);
    pub fn wxGridEditorCreatedEvent_SetWindow(self_: *mut c_void, window: *mut c_void);

    // wxGridEvent
    pub fn wxGridEvent_CLASSINFO() -> *mut c_void;
    pub fn wxGridEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridEvent_new1(id: c_int, type_: wxEventType, obj: *mut c_void, row: c_int, col: c_int, x: c_int, y: c_int, sel: bool, kbd: *const c_void) -> *mut c_void;
    pub fn wxGridEvent_AltDown(self_: *const c_void) -> bool;
    pub fn wxGridEvent_ControlDown(self_: *const c_void) -> bool;
    pub fn wxGridEvent_GetCol(self_: *const c_void) -> c_int;
    pub fn wxGridEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxGridEvent_GetRow(self_: *const c_void) -> c_int;
    pub fn wxGridEvent_MetaDown(self_: *const c_void) -> bool;
    pub fn wxGridEvent_Selecting(self_: *const c_void) -> bool;
    pub fn wxGridEvent_ShiftDown(self_: *const c_void) -> bool;

    // wxGridFitMode
    pub fn wxGridFitMode_delete(self_: *mut c_void);
    pub fn wxGridFitMode_new() -> *mut c_void;
    pub fn wxGridFitMode_IsSpecified(self_: *const c_void) -> bool;
    pub fn wxGridFitMode_IsClip(self_: *const c_void) -> bool;
    pub fn wxGridFitMode_IsOverflow(self_: *const c_void) -> bool;
    pub fn wxGridFitMode_GetEllipsizeMode(self_: *const c_void) -> c_int;
    pub fn wxGridFitMode_Clip() -> *mut c_void;
    pub fn wxGridFitMode_Overflow() -> *mut c_void;
    pub fn wxGridFitMode_Ellipsize(ellipsize: c_int) -> *mut c_void;

    // wxGridRangeSelectEvent
    pub fn wxGridRangeSelectEvent_CLASSINFO() -> *mut c_void;
    pub fn wxGridRangeSelectEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridRangeSelectEvent_new1(id: c_int, type_: wxEventType, obj: *mut c_void, top_left: *const c_void, bottom_right: *const c_void, sel: bool, kbd: *const c_void) -> *mut c_void;
    pub fn wxGridRangeSelectEvent_AltDown(self_: *const c_void) -> bool;
    pub fn wxGridRangeSelectEvent_ControlDown(self_: *const c_void) -> bool;
    pub fn wxGridRangeSelectEvent_GetBottomRightCoords(self_: *const c_void) -> *mut c_void;
    pub fn wxGridRangeSelectEvent_GetBottomRow(self_: *const c_void) -> c_int;
    pub fn wxGridRangeSelectEvent_GetLeftCol(self_: *const c_void) -> c_int;
    pub fn wxGridRangeSelectEvent_GetRightCol(self_: *const c_void) -> c_int;
    pub fn wxGridRangeSelectEvent_GetTopLeftCoords(self_: *const c_void) -> *mut c_void;
    pub fn wxGridRangeSelectEvent_GetTopRow(self_: *const c_void) -> c_int;
    pub fn wxGridRangeSelectEvent_MetaDown(self_: *const c_void) -> bool;
    pub fn wxGridRangeSelectEvent_Selecting(self_: *const c_void) -> bool;
    pub fn wxGridRangeSelectEvent_ShiftDown(self_: *const c_void) -> bool;

    // wxGridSizeEvent
    pub fn wxGridSizeEvent_CLASSINFO() -> *mut c_void;
    pub fn wxGridSizeEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridSizeEvent_new1(id: c_int, type_: wxEventType, obj: *mut c_void, row_or_col: c_int, x: c_int, y: c_int, kbd: *const c_void) -> *mut c_void;
    pub fn wxGridSizeEvent_AltDown(self_: *const c_void) -> bool;
    pub fn wxGridSizeEvent_ControlDown(self_: *const c_void) -> bool;
    pub fn wxGridSizeEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxGridSizeEvent_GetRowOrCol(self_: *const c_void) -> c_int;
    pub fn wxGridSizeEvent_MetaDown(self_: *const c_void) -> bool;
    pub fn wxGridSizeEvent_ShiftDown(self_: *const c_void) -> bool;

    // wxGridSizer
    pub fn wxGridSizer_CLASSINFO() -> *mut c_void;
    pub fn wxGridSizer_new(cols: c_int, vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxGridSizer_new1(cols: c_int, gap: *const c_void) -> *mut c_void;
    pub fn wxGridSizer_new2(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxGridSizer_new3(rows: c_int, cols: c_int, gap: *const c_void) -> *mut c_void;
    pub fn wxGridSizer_GetCols(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetRows(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetEffectiveColsCount(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetEffectiveRowsCount(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetHGap(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetVGap(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_SetCols(self_: *mut c_void, cols: c_int);
    pub fn wxGridSizer_SetHGap(self_: *mut c_void, gap: c_int);
    pub fn wxGridSizer_SetRows(self_: *mut c_void, rows: c_int);
    pub fn wxGridSizer_SetVGap(self_: *mut c_void, gap: c_int);

    // wxGridUpdateLocker
    pub fn wxGridUpdateLocker_delete(self_: *mut c_void);
    pub fn wxGridUpdateLocker_new(grid: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxGridUpdateLocker_~wxGridUpdateLocker(self_: *mut c_void);
    pub fn wxGridUpdateLocker_Create(self_: *mut c_void, grid: *mut c_void);

    // wxHeaderColumn
    pub fn wxHeaderColumn_delete(self_: *mut c_void);
    pub fn wxHeaderColumn_GetTitle(self_: *const c_void) -> *mut c_void;
    pub fn wxHeaderColumn_GetBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxHeaderColumn_GetBitmapBundle(self_: *const c_void) -> *mut c_void;
    pub fn wxHeaderColumn_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxHeaderColumn_GetMinWidth(self_: *const c_void) -> c_int;
    pub fn wxHeaderColumn_GetAlignment(self_: *const c_void) -> c_int;
    pub fn wxHeaderColumn_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxHeaderColumn_HasFlag(self_: *const c_void, flag: c_int) -> bool;
    pub fn wxHeaderColumn_IsResizeable(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsSortable(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsReorderable(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsHidden(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsShown(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsSortKey(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsSortOrderAscending(self_: *const c_void) -> bool;

    // wxHeaderColumnSimple
    pub fn wxHeaderColumnSimple_delete(self_: *mut c_void);
    pub fn wxHeaderColumnSimple_new(
        title: *const c_void,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxHeaderColumnSimple_new1(
        bitmap: *const c_void,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> *mut c_void;

    // wxHeaderCtrl
    pub fn wxHeaderCtrl_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxHeaderCtrl_new() -> *mut c_void;
    // BLOCKED: pub fn wxHeaderCtrl_new1(parent: *mut c_void, winid: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    pub fn wxHeaderCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxHeaderCtrl_SetColumnCount(self_: *mut c_void, count: c_uint);
    pub fn wxHeaderCtrl_GetColumnCount(self_: *const c_void) -> c_uint;
    pub fn wxHeaderCtrl_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxHeaderCtrl_UpdateColumn(self_: *mut c_void, idx: c_uint);
    pub fn wxHeaderCtrl_SetColumnsOrder(self_: *mut c_void, order: *const c_void);
    pub fn wxHeaderCtrl_GetColumnsOrder(self_: *const c_void) -> *mut c_void;
    pub fn wxHeaderCtrl_GetColumnAt(self_: *const c_void, pos: c_uint) -> c_uint;
    pub fn wxHeaderCtrl_GetColumnPos(self_: *const c_void, idx: c_uint) -> c_uint;
    pub fn wxHeaderCtrl_ResetColumnsOrder(self_: *mut c_void);
    pub fn wxHeaderCtrl_ShowColumnsMenu(
        self_: *mut c_void,
        pt: *const c_void,
        title: *const c_void,
    ) -> bool;
    pub fn wxHeaderCtrl_AddColumnsItems(
        self_: *mut c_void,
        menu: *mut c_void,
        id_columns_base: c_int,
    );
    pub fn wxHeaderCtrl_ShowCustomizeDialog(self_: *mut c_void) -> bool;
    pub fn wxHeaderCtrl_GetColumnTitleWidth(self_: *mut c_void, col: *const c_void) -> c_int;
    pub fn wxHeaderCtrl_GetColumnTitleWidth1(self_: *mut c_void, idx: c_uint) -> c_int;
    pub fn wxHeaderCtrl_MoveColumnInOrderArray(order: *mut c_void, idx: c_uint, pos: c_uint);

    // wxHeaderCtrlEvent
    pub fn wxHeaderCtrlEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHeaderCtrlEvent_new(command_type: wxEventType, winid: c_int) -> *mut c_void;
    pub fn wxHeaderCtrlEvent_new1(event: *const c_void) -> *mut c_void;
    pub fn wxHeaderCtrlEvent_GetColumn(self_: *const c_void) -> c_int;
    pub fn wxHeaderCtrlEvent_SetColumn(self_: *mut c_void, col: c_int);
    pub fn wxHeaderCtrlEvent_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxHeaderCtrlEvent_SetWidth(self_: *mut c_void, width: c_int);
    pub fn wxHeaderCtrlEvent_GetNewOrder(self_: *const c_void) -> c_uint;
    pub fn wxHeaderCtrlEvent_SetNewOrder(self_: *mut c_void, order: c_uint);

    // wxHeaderCtrlSimple
    pub fn wxHeaderCtrlSimple_CLASSINFO() -> *mut c_void;
    pub fn wxHeaderCtrlSimple_new() -> *mut c_void;
    pub fn wxHeaderCtrlSimple_new1(
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxHeaderCtrlSimple_InsertColumn(self_: *mut c_void, col: *const c_void, idx: c_uint);
    pub fn wxHeaderCtrlSimple_AppendColumn(self_: *mut c_void, col: *const c_void);
    pub fn wxHeaderCtrlSimple_DeleteColumn(self_: *mut c_void, idx: c_uint);
    pub fn wxHeaderCtrlSimple_ShowColumn(self_: *mut c_void, idx: c_uint, show: bool);
    pub fn wxHeaderCtrlSimple_HideColumn(self_: *mut c_void, idx: c_uint);
    pub fn wxHeaderCtrlSimple_ShowSortIndicator(self_: *mut c_void, idx: c_uint, sort_order: bool);
    pub fn wxHeaderCtrlSimple_RemoveSortIndicator(self_: *mut c_void);

    // wxHelpControllerHelpProvider
    pub fn wxHelpControllerHelpProvider_delete(self_: *mut c_void);
    pub fn wxHelpControllerHelpProvider_new(hc: *mut c_void) -> *mut c_void;
    pub fn wxHelpControllerHelpProvider_GetHelpController(self_: *const c_void) -> *mut c_void;
    pub fn wxHelpControllerHelpProvider_SetHelpController(self_: *mut c_void, hc: *mut c_void);

    // wxHelpEvent
    pub fn wxHelpEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHelpEvent_new(type_: wxEventType, winid: c_int, pt: *const c_void, origin: wxHelpEvent::Origin) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHelpEvent_GetOrigin(self_: *const c_void) -> wxHelpEvent::Origin;
    pub fn wxHelpEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHelpEvent_SetOrigin(self_: *mut c_void, origin: wxHelpEvent::Origin);
    pub fn wxHelpEvent_SetPosition(self_: *mut c_void, pt: *const c_void);

    // wxHelpProvider
    pub fn wxHelpProvider_delete(self_: *mut c_void);
    // DTOR: pub fn wxHelpProvider_~wxHelpProvider(self_: *mut c_void);
    pub fn wxHelpProvider_AddHelp(self_: *mut c_void, window: *mut c_void, text: *const c_void);
    pub fn wxHelpProvider_AddHelp1(self_: *mut c_void, id: c_int, text: *const c_void);
    pub fn wxHelpProvider_GetHelp(self_: *mut c_void, window: *const c_void) -> *mut c_void;
    pub fn wxHelpProvider_RemoveHelp(self_: *mut c_void, window: *mut c_void);
    pub fn wxHelpProvider_ShowHelp(self_: *mut c_void, window: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxHelpProvider_ShowHelpAtPoint(self_: *mut c_void, window: *mut c_void, point: *const c_void, origin: wxHelpEvent::Origin) -> bool;
    pub fn wxHelpProvider_Get() -> *mut c_void;
    pub fn wxHelpProvider_Set(help_provider: *mut c_void) -> *mut c_void;

    // wxHyperlinkCtrl
    pub fn wxHyperlinkCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxHyperlinkCtrl_new() -> *mut c_void;
    pub fn wxHyperlinkCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        url: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxHyperlinkCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        url: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxHyperlinkCtrl_GetHoverColour(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkCtrl_GetNormalColour(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkCtrl_GetURL(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkCtrl_GetVisited(self_: *const c_void) -> bool;
    pub fn wxHyperlinkCtrl_GetVisitedColour(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkCtrl_SetHoverColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxHyperlinkCtrl_SetNormalColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxHyperlinkCtrl_SetURL(self_: *mut c_void, url: *const c_void);
    pub fn wxHyperlinkCtrl_SetVisited(self_: *mut c_void, visited: bool);
    pub fn wxHyperlinkCtrl_SetVisitedColour(self_: *mut c_void, colour: *const c_void);

    // wxHyperlinkEvent
    pub fn wxHyperlinkEvent_CLASSINFO() -> *mut c_void;
    pub fn wxHyperlinkEvent_new(
        generator: *mut c_void,
        id: c_int,
        url: *const c_void,
    ) -> *mut c_void;
    pub fn wxHyperlinkEvent_GetURL(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkEvent_SetURL(self_: *mut c_void, url: *const c_void);

    // wxIcon
    pub fn wxIcon_CLASSINFO() -> *mut c_void;
    pub fn wxIcon_new() -> *mut c_void;
    pub fn wxIcon_new1(icon: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIcon_new2(bits: char, width: c_int, height: c_int) -> *mut c_void;
    pub fn wxIcon_new3(bits: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIcon_new4(name: *const c_void, type_: wxBitmapType, desired_width: c_int, desired_height: c_int) -> *mut c_void;
    pub fn wxIcon_new5(loc: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxIcon_~wxIcon(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxIcon_CreateFromHICON(self_: *mut c_void, icon: WXHICON) -> bool;
    // NOT_SUPPORTED: pub fn wxIcon_ConvertToDisabled(self_: *const c_void, brightness: unsigned char) -> *mut c_void;
    pub fn wxIcon_CopyFromBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxIcon_GetDepth(self_: *const c_void) -> c_int;
    pub fn wxIcon_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxIcon_GetLogicalHeight(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetLogicalSize(self_: *const c_void) -> *mut c_void;
    pub fn wxIcon_GetLogicalWidth(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxIcon_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxIcon_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxIcon_LoadFile(self_: *mut c_void, name: *const c_void, type_: wxBitmapType, desired_width: c_int, desired_height: c_int) -> bool;
    pub fn wxIcon_SetDepth(self_: *mut c_void, depth: c_int);
    pub fn wxIcon_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxIcon_SetWidth(self_: *mut c_void, width: c_int);
    // BLOCKED: pub fn wxIcon_operator=(self_: *mut c_void, icon: *const c_void) -> *mut c_void;

    // wxIconBundle
    pub fn wxIconBundle_CLASSINFO() -> *mut c_void;
    pub fn wxIconBundle_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIconBundle_new1(file: *const c_void, type_: wxBitmapType) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIconBundle_new2(stream: *mut c_void, type_: wxBitmapType) -> *mut c_void;
    pub fn wxIconBundle_new3(icon: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIconBundle_new4(resource_name: *const c_void, module: WXHINSTANCE) -> *mut c_void;
    pub fn wxIconBundle_new5(ic: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxIconBundle_~wxIconBundle(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxIconBundle_AddIcon(self_: *mut c_void, file: *const c_void, type_: wxBitmapType);
    // NOT_SUPPORTED: pub fn wxIconBundle_AddIcon1(self_: *mut c_void, stream: *mut c_void, type_: wxBitmapType);
    // NOT_SUPPORTED: pub fn wxIconBundle_AddIcon2(self_: *mut c_void, resource_name: *const c_void, module: WXHINSTANCE);
    pub fn wxIconBundle_AddIcon3(self_: *mut c_void, icon: *const c_void);
    pub fn wxIconBundle_GetIcon(
        self_: *const c_void,
        size: *const c_void,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxIconBundle_GetIcon1(self_: *const c_void, size: c_int, flags: c_int) -> *mut c_void;
    pub fn wxIconBundle_GetIconOfExactSize(
        self_: *const c_void,
        size: *const c_void,
    ) -> *mut c_void;
    pub fn wxIconBundle_GetIconCount(self_: *const c_void) -> usize;
    pub fn wxIconBundle_GetIconByIndex(self_: *const c_void, n: usize) -> *mut c_void;
    pub fn wxIconBundle_IsEmpty(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxIconBundle_operator=(self_: *mut c_void, ic: *const c_void) -> *mut c_void;

    // wxIdManager
    pub fn wxIdManager_delete(self_: *mut c_void);
    pub fn wxIdManager_ReserveId(count: c_int) -> c_int;
    pub fn wxIdManager_UnreserveId(id: c_int, count: c_int);

    // wxImageList
    pub fn wxImageList_CLASSINFO() -> *mut c_void;
    pub fn wxImageList_new() -> *mut c_void;
    pub fn wxImageList_new1(
        width: c_int,
        height: c_int,
        mask: bool,
        initial_count: c_int,
    ) -> *mut c_void;
    pub fn wxImageList_Add(self_: *mut c_void, bitmap: *const c_void, mask: *const c_void)
        -> c_int;
    pub fn wxImageList_Add1(
        self_: *mut c_void,
        bitmap: *const c_void,
        mask_colour: *const c_void,
    ) -> c_int;
    pub fn wxImageList_Add2(self_: *mut c_void, icon: *const c_void) -> c_int;
    pub fn wxImageList_Create(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        mask: bool,
        initial_count: c_int,
    ) -> bool;
    pub fn wxImageList_Destroy(self_: *mut c_void);
    pub fn wxImageList_Draw(
        self_: *mut c_void,
        index: c_int,
        dc: *mut c_void,
        x: c_int,
        y: c_int,
        flags: c_int,
        solid_background: bool,
    ) -> bool;
    pub fn wxImageList_GetBitmap(self_: *const c_void, index: c_int) -> *mut c_void;
    pub fn wxImageList_GetIcon(self_: *const c_void, index: c_int) -> *mut c_void;
    pub fn wxImageList_GetImageCount(self_: *const c_void) -> c_int;
    pub fn wxImageList_GetSize(
        self_: *const c_void,
        index: c_int,
        width: *mut c_void,
        height: *mut c_void,
    ) -> bool;
    pub fn wxImageList_GetSize1(self_: *const c_void) -> *mut c_void;
    pub fn wxImageList_Remove(self_: *mut c_void, index: c_int) -> bool;
    pub fn wxImageList_RemoveAll(self_: *mut c_void) -> bool;
    pub fn wxImageList_Replace(
        self_: *mut c_void,
        index: c_int,
        bitmap: *const c_void,
        mask: *const c_void,
    ) -> bool;
    pub fn wxImageList_Replace1(self_: *mut c_void, index: c_int, icon: *const c_void) -> bool;

    // wxInfoBar
    pub fn wxInfoBar_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxInfoBar_SetShowHideEffects(self_: *mut c_void, show_effect: wxShowEffect, hide_effect: wxShowEffect);
    // NOT_SUPPORTED: pub fn wxInfoBar_GetShowEffect(self_: *const c_void) -> wxShowEffect;
    // NOT_SUPPORTED: pub fn wxInfoBar_GetHideEffect(self_: *const c_void) -> wxShowEffect;
    pub fn wxInfoBar_SetEffectDuration(self_: *mut c_void, duration: c_int);
    pub fn wxInfoBar_GetEffectDuration(self_: *const c_void) -> c_int;
    pub fn wxInfoBar_new() -> *mut c_void;
    pub fn wxInfoBar_new1(parent: *mut c_void, winid: c_int) -> *mut c_void;
    pub fn wxInfoBar_Create(self_: *mut c_void, parent: *mut c_void, winid: c_int) -> bool;
    pub fn wxInfoBar_AddButton(self_: *mut c_void, btnid: c_int, label: *const c_void);
    pub fn wxInfoBar_Dismiss(self_: *mut c_void);
    pub fn wxInfoBar_RemoveButton(self_: *mut c_void, btnid: c_int);
    pub fn wxInfoBar_ShowMessage(self_: *mut c_void, msg: *const c_void, flags: c_int);
    pub fn wxInfoBar_GetButtonCount(self_: *const c_void) -> usize;
    pub fn wxInfoBar_GetButtonId(self_: *const c_void, idx: usize) -> c_int;
    pub fn wxInfoBar_HasButtonId(self_: *const c_void, btnid: c_int) -> bool;

    // wxInitDialogEvent
    pub fn wxInitDialogEvent_CLASSINFO() -> *mut c_void;
    pub fn wxInitDialogEvent_new(id: c_int) -> *mut c_void;

    // wxItemAttr
    pub fn wxItemAttr_delete(self_: *mut c_void);
    pub fn wxItemAttr_new() -> *mut c_void;
    pub fn wxItemAttr_new1(
        col_text: *const c_void,
        col_back: *const c_void,
        font: *const c_void,
    ) -> *mut c_void;
    // BLOCKED: pub fn wxItemAttr_operator==(self_: *const c_void, other: *const c_void) -> bool;
    // BLOCKED: pub fn wxItemAttr_operator!=(self_: *const c_void, other: *const c_void) -> bool;
    pub fn wxItemAttr_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxItemAttr_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxItemAttr_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxItemAttr_HasBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxItemAttr_HasColours(self_: *const c_void) -> bool;
    pub fn wxItemAttr_HasFont(self_: *const c_void) -> bool;
    pub fn wxItemAttr_HasTextColour(self_: *const c_void) -> bool;
    pub fn wxItemAttr_IsDefault(self_: *const c_void) -> bool;
    pub fn wxItemAttr_SetBackgroundColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxItemAttr_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxItemAttr_SetTextColour(self_: *mut c_void, colour: *const c_void);

    // wxItemContainer
    pub fn wxItemContainer_delete(self_: *mut c_void);
    pub fn wxItemContainer_Append(self_: *mut c_void, item: *const c_void) -> c_int;
    pub fn wxItemContainer_Append1(
        self_: *mut c_void,
        item: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append2(
        self_: *mut c_void,
        item: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append3(self_: *mut c_void, items: *const c_void) -> c_int;
    // BLOCKED: pub fn wxItemContainer_Append4(self_: *mut c_void, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Append5(
        self_: *mut c_void,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append6(
        self_: *mut c_void,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append7(self_: *mut c_void, n: c_uint, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Append8(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append9(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Clear(self_: *mut c_void);
    pub fn wxItemContainer_Delete(self_: *mut c_void, n: c_uint);
    pub fn wxItemContainer_DetachClientObject(self_: *mut c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_HasClientData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_HasClientObjectData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_HasClientUntypedData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_GetClientData(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_GetClientObject(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_SetClientData(self_: *mut c_void, n: c_uint, data: *mut c_void);
    pub fn wxItemContainer_SetClientObject(self_: *mut c_void, n: c_uint, data: *mut c_void);
    pub fn wxItemContainer_Insert(self_: *mut c_void, item: *const c_void, pos: c_uint) -> c_int;
    pub fn wxItemContainer_Insert1(
        self_: *mut c_void,
        item: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert2(
        self_: *mut c_void,
        item: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert3(self_: *mut c_void, items: *const c_void, pos: c_uint) -> c_int;
    // BLOCKED: pub fn wxItemContainer_Insert4(self_: *mut c_void, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Insert5(
        self_: *mut c_void,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert6(
        self_: *mut c_void,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert7(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
    ) -> c_int;
    pub fn wxItemContainer_Insert8(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert9(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Set(self_: *mut c_void, items: *const c_void);
    // BLOCKED: pub fn wxItemContainer_Set1(self_: *mut c_void, items: *const c_void);
    pub fn wxItemContainer_Set2(self_: *mut c_void, items: *const c_void, client_data: *mut c_void);
    pub fn wxItemContainer_Set3(self_: *mut c_void, items: *const c_void, client_data: *mut c_void);
    pub fn wxItemContainer_Set4(self_: *mut c_void, n: c_uint, items: *const c_void);
    pub fn wxItemContainer_Set5(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    );
    pub fn wxItemContainer_Set6(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    );

    // wxItemContainerImmutable
    pub fn wxItemContainerImmutable_delete(self_: *mut c_void);
    pub fn wxItemContainerImmutable_SetSelection(self_: *mut c_void, n: c_int);
    pub fn wxItemContainerImmutable_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxItemContainerImmutable_SetStringSelection(
        self_: *mut c_void,
        string: *const c_void,
    ) -> bool;
    pub fn wxItemContainerImmutable_GetStringSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxItemContainerImmutable_Select(self_: *mut c_void, n: c_int);
    // BLOCKED: pub fn wxItemContainerImmutable_new() -> *mut c_void;
    pub fn wxItemContainerImmutable_GetCount(self_: *const c_void) -> c_uint;
    pub fn wxItemContainerImmutable_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxItemContainerImmutable_GetString(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainerImmutable_GetStrings(self_: *const c_void) -> *mut c_void;
    pub fn wxItemContainerImmutable_SetString(self_: *mut c_void, n: c_uint, string: *const c_void);
    pub fn wxItemContainerImmutable_FindString(
        self_: *const c_void,
        string: *const c_void,
        case_sensitive: bool,
    ) -> c_int;

    // wxJoystick
    pub fn wxJoystick_CLASSINFO() -> *mut c_void;
    pub fn wxJoystick_new(joystick: c_int) -> *mut c_void;
    // DTOR: pub fn wxJoystick_~wxJoystick(self_: *mut c_void);
    pub fn wxJoystick_GetButtonState(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetButtonState1(self_: *const c_void, id: c_uint) -> bool;
    pub fn wxJoystick_GetManufacturerId(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetMovementThreshold(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetNumberAxes(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetNumberButtons(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPOVCTSPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPOVPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPollingMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPollingMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxJoystick_GetPosition1(self_: *const c_void, axis: c_uint) -> c_int;
    pub fn wxJoystick_GetProductId(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetProductName(self_: *const c_void) -> *mut c_void;
    pub fn wxJoystick_GetRudderMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetRudderMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetRudderPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetUMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetUMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetUPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetVMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetVMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetVPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetXMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetXMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetYMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetYMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetZMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetZMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetZPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_HasPOV(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasPOV4Dir(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasPOVCTS(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasRudder(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasU(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasV(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasZ(self_: *const c_void) -> bool;
    pub fn wxJoystick_IsOk(self_: *const c_void) -> bool;
    pub fn wxJoystick_ReleaseCapture(self_: *mut c_void) -> bool;
    pub fn wxJoystick_SetCapture(self_: *mut c_void, win: *mut c_void, polling_freq: c_int)
        -> bool;
    pub fn wxJoystick_SetMovementThreshold(self_: *mut c_void, threshold: c_int);
    pub fn wxJoystick_GetNumberJoysticks() -> c_int;

    // wxJoystickEvent
    pub fn wxJoystickEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxJoystickEvent_new(event_type: wxEventType, state: c_int, joystick: c_int, change: c_int) -> *mut c_void;
    pub fn wxJoystickEvent_ButtonDown(self_: *const c_void, button: c_int) -> bool;
    pub fn wxJoystickEvent_ButtonIsDown(self_: *const c_void, button: c_int) -> bool;
    pub fn wxJoystickEvent_ButtonUp(self_: *const c_void, button: c_int) -> bool;
    pub fn wxJoystickEvent_GetButtonChange(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_GetButtonOrdinal(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_GetButtonState(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_GetJoystick(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxJoystickEvent_GetZPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_IsButton(self_: *const c_void) -> bool;
    pub fn wxJoystickEvent_IsMove(self_: *const c_void) -> bool;
    pub fn wxJoystickEvent_IsZMove(self_: *const c_void) -> bool;

    // wxKeyEvent
    pub fn wxKeyEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxKeyEvent_new(key_event_type: wxEventType) -> *mut c_void;
    pub fn wxKeyEvent_GetKeyCode(self_: *const c_void) -> c_int;
    pub fn wxKeyEvent_IsKeyInCategory(self_: *const c_void, category: c_int) -> bool;
    pub fn wxKeyEvent_IsAutoRepeat(self_: *const c_void) -> bool;
    pub fn wxKeyEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxKeyEvent_GetPosition1(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    // NOT_SUPPORTED: pub fn wxKeyEvent_GetRawKeyCode(self_: *const c_void) -> wxUint32;
    // NOT_SUPPORTED: pub fn wxKeyEvent_GetRawKeyFlags(self_: *const c_void) -> wxUint32;
    // NOT_SUPPORTED: pub fn wxKeyEvent_GetUnicodeKey(self_: *const c_void) -> wxChar;
    pub fn wxKeyEvent_GetX(self_: *const c_void) -> c_int;
    pub fn wxKeyEvent_GetY(self_: *const c_void) -> c_int;
    pub fn wxKeyEvent_DoAllowNextEvent(self_: *mut c_void);
    pub fn wxKeyEvent_IsNextEventAllowed(self_: *const c_void) -> bool;
    // Mix-in(s) to wxKeyEvent
    pub fn wxKeyEvent_AsKeyboardState(obj: *mut c_void) -> *mut c_void;

    // wxLayoutAlgorithm
    pub fn wxLayoutAlgorithm_CLASSINFO() -> *mut c_void;
    pub fn wxLayoutAlgorithm_new() -> *mut c_void;
    // DTOR: pub fn wxLayoutAlgorithm_~wxLayoutAlgorithm(self_: *mut c_void);
    pub fn wxLayoutAlgorithm_LayoutFrame(
        self_: *mut c_void,
        frame: *mut c_void,
        main_window: *mut c_void,
    ) -> bool;
    pub fn wxLayoutAlgorithm_LayoutMDIFrame(
        self_: *mut c_void,
        frame: *mut c_void,
        rect: *mut c_void,
    ) -> bool;
    pub fn wxLayoutAlgorithm_LayoutWindow(
        self_: *mut c_void,
        parent: *mut c_void,
        main_window: *mut c_void,
    ) -> bool;

    // wxListBox
    pub fn wxListBox_CLASSINFO() -> *mut c_void;
    pub fn wxListBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxListBox_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxListBox_new2(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxListBox_~wxListBox(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxListBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxListBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxListBox_Deselect(self_: *mut c_void, n: c_int);
    pub fn wxListBox_SetStringSelection(self_: *mut c_void, s: *const c_void, select: bool)
        -> bool;
    pub fn wxListBox_SetStringSelection1(self_: *mut c_void, s: *const c_void) -> bool;
    pub fn wxListBox_GetSelections(self_: *const c_void, selections: *mut c_void) -> c_int;
    pub fn wxListBox_HitTest(self_: *const c_void, point: *const c_void) -> c_int;
    pub fn wxListBox_HitTest1(self_: *const c_void, x: c_int, y: c_int) -> c_int;
    // BLOCKED: pub fn wxListBox_InsertItems(self_: *mut c_void, n_items: c_uint, items: *const c_void, pos: c_uint);
    pub fn wxListBox_InsertItems1(self_: *mut c_void, items: *const c_void, pos: c_uint);
    pub fn wxListBox_IsSelected(self_: *const c_void, n: c_int) -> bool;
    pub fn wxListBox_SetFirstItem(self_: *mut c_void, n: c_int);
    pub fn wxListBox_SetFirstItem1(self_: *mut c_void, string: *const c_void);
    pub fn wxListBox_EnsureVisible(self_: *mut c_void, n: c_int);
    pub fn wxListBox_IsSorted(self_: *const c_void) -> bool;
    pub fn wxListBox_GetCountPerPage(self_: *const c_void) -> c_int;
    pub fn wxListBox_GetTopItem(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxListBox_MSWSetTabStops(self_: *mut c_void, tab_stops: *const c_void) -> bool;
    // Mix-in(s) to wxListBox
    pub fn wxListBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxListCtrl
    pub fn wxListCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxListCtrl_new() -> *mut c_void;
    pub fn wxListCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxListCtrl_~wxListCtrl(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxListCtrl_AppendColumn(self_: *mut c_void, heading: *const c_void, format: wxListColumnFormat, width: c_int) -> c_long;
    pub fn wxListCtrl_Arrange(self_: *mut c_void, flag: c_int) -> bool;
    pub fn wxListCtrl_AssignImageList(self_: *mut c_void, image_list: *mut c_void, which: c_int);
    pub fn wxListCtrl_ClearAll(self_: *mut c_void);
    pub fn wxListCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxListCtrl_DeleteAllColumns(self_: *mut c_void) -> bool;
    pub fn wxListCtrl_DeleteAllItems(self_: *mut c_void) -> bool;
    pub fn wxListCtrl_DeleteColumn(self_: *mut c_void, col: c_int) -> bool;
    pub fn wxListCtrl_DeleteItem(self_: *mut c_void, item: c_long) -> bool;
    pub fn wxListCtrl_EditLabel(
        self_: *mut c_void,
        item: c_long,
        text_control_class: *mut c_void,
    ) -> *mut c_void;
    pub fn wxListCtrl_EnableAlternateRowColours(self_: *mut c_void, enable: bool);
    pub fn wxListCtrl_EnableBellOnNoMatch(self_: *mut c_void, on: bool);
    pub fn wxListCtrl_EndEditLabel(self_: *mut c_void, cancel: bool) -> bool;
    pub fn wxListCtrl_EnsureVisible(self_: *mut c_void, item: c_long) -> bool;
    pub fn wxListCtrl_FindItem(
        self_: *mut c_void,
        start: c_long,
        str: *const c_void,
        partial: bool,
    ) -> c_long;
    // NOT_SUPPORTED: pub fn wxListCtrl_FindItem1(self_: *mut c_void, start: c_long, data: wxUIntPtr) -> c_long;
    pub fn wxListCtrl_FindItem2(
        self_: *mut c_void,
        start: c_long,
        pt: *const c_void,
        direction: c_int,
    ) -> c_long;
    pub fn wxListCtrl_GetColumn(self_: *const c_void, col: c_int, item: *mut c_void) -> bool;
    pub fn wxListCtrl_GetColumnCount(self_: *const c_void) -> c_int;
    pub fn wxListCtrl_GetColumnIndexFromOrder(self_: *const c_void, pos: c_int) -> c_int;
    pub fn wxListCtrl_GetColumnOrder(self_: *const c_void, col: c_int) -> c_int;
    pub fn wxListCtrl_GetColumnWidth(self_: *const c_void, col: c_int) -> c_int;
    pub fn wxListCtrl_GetColumnsOrder(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_GetCountPerPage(self_: *const c_void) -> c_int;
    pub fn wxListCtrl_GetEditControl(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_GetImageList(self_: *const c_void, which: c_int) -> *mut c_void;
    pub fn wxListCtrl_GetItem(self_: *const c_void, info: *mut c_void) -> bool;
    pub fn wxListCtrl_GetItemBackgroundColour(self_: *const c_void, item: c_long) -> *mut c_void;
    pub fn wxListCtrl_GetItemCount(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxListCtrl_GetItemData(self_: *const c_void, item: c_long) -> wxUIntPtr;
    pub fn wxListCtrl_GetItemFont(self_: *const c_void, item: c_long) -> *mut c_void;
    pub fn wxListCtrl_GetItemPosition(self_: *const c_void, item: c_long, pos: *mut c_void)
        -> bool;
    pub fn wxListCtrl_GetItemRect(
        self_: *const c_void,
        item: c_long,
        rect: *mut c_void,
        code: c_int,
    ) -> bool;
    pub fn wxListCtrl_GetItemSpacing(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_GetItemState(self_: *const c_void, item: c_long, state_mask: c_long)
        -> c_int;
    pub fn wxListCtrl_GetItemText(self_: *const c_void, item: c_long, col: c_int) -> *mut c_void;
    pub fn wxListCtrl_GetItemTextColour(self_: *const c_void, item: c_long) -> *mut c_void;
    pub fn wxListCtrl_GetNextItem(
        self_: *const c_void,
        item: c_long,
        geometry: c_int,
        state: c_int,
    ) -> c_long;
    pub fn wxListCtrl_GetSelectedItemCount(self_: *const c_void) -> c_int;
    pub fn wxListCtrl_GetSubItemRect(
        self_: *const c_void,
        item: c_long,
        sub_item: c_long,
        rect: *mut c_void,
        code: c_int,
    ) -> bool;
    pub fn wxListCtrl_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_GetTopItem(self_: *const c_void) -> c_long;
    pub fn wxListCtrl_GetViewRect(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_SetAlternateRowColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxListCtrl_GetAlternateRowColour(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_HitTest(
        self_: *const c_void,
        point: *const c_void,
        flags: *mut c_void,
        ptr_sub_item: *mut c_void,
    ) -> c_long;
    pub fn wxListCtrl_InReportView(self_: *const c_void) -> bool;
    pub fn wxListCtrl_InsertColumn(self_: *mut c_void, col: c_long, info: *const c_void) -> c_long;
    pub fn wxListCtrl_InsertColumn1(
        self_: *mut c_void,
        col: c_long,
        heading: *const c_void,
        format: c_int,
        width: c_int,
    ) -> c_long;
    pub fn wxListCtrl_InsertItem(self_: *mut c_void, info: *mut c_void) -> c_long;
    pub fn wxListCtrl_InsertItem1(
        self_: *mut c_void,
        index: c_long,
        label: *const c_void,
    ) -> c_long;
    pub fn wxListCtrl_InsertItem2(self_: *mut c_void, index: c_long, image_index: c_int) -> c_long;
    pub fn wxListCtrl_InsertItem3(
        self_: *mut c_void,
        index: c_long,
        label: *const c_void,
        image_index: c_int,
    ) -> c_long;
    pub fn wxListCtrl_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxListCtrl_IsVirtual(self_: *const c_void) -> bool;
    pub fn wxListCtrl_RefreshItem(self_: *mut c_void, item: c_long);
    pub fn wxListCtrl_RefreshItems(self_: *mut c_void, item_from: c_long, item_to: c_long);
    pub fn wxListCtrl_ScrollList(self_: *mut c_void, dx: c_int, dy: c_int) -> bool;
    pub fn wxListCtrl_SetColumn(self_: *mut c_void, col: c_int, item: *mut c_void) -> bool;
    pub fn wxListCtrl_SetColumnWidth(self_: *mut c_void, col: c_int, width: c_int) -> bool;
    pub fn wxListCtrl_SetColumnsOrder(self_: *mut c_void, orders: *const c_void) -> bool;
    pub fn wxListCtrl_SetHeaderAttr(self_: *mut c_void, attr: *const c_void) -> bool;
    pub fn wxListCtrl_SetImageList(self_: *mut c_void, image_list: *mut c_void, which: c_int);
    pub fn wxListCtrl_SetNormalImages(self_: *mut c_void, images: *const c_void);
    pub fn wxListCtrl_SetSmallImages(self_: *mut c_void, images: *const c_void);
    pub fn wxListCtrl_IsVisible(self_: *const c_void, item: c_long) -> bool;
    pub fn wxListCtrl_SetItem(self_: *mut c_void, info: *mut c_void) -> bool;
    pub fn wxListCtrl_SetItem1(
        self_: *mut c_void,
        index: c_long,
        column: c_int,
        label: *const c_void,
        image_id: c_int,
    ) -> bool;
    pub fn wxListCtrl_SetItemBackgroundColour(self_: *mut c_void, item: c_long, col: *const c_void);
    pub fn wxListCtrl_SetItemColumnImage(
        self_: *mut c_void,
        item: c_long,
        column: c_long,
        image: c_int,
    ) -> bool;
    pub fn wxListCtrl_SetItemCount(self_: *mut c_void, count: c_long);
    pub fn wxListCtrl_SetItemData(self_: *mut c_void, item: c_long, data: c_long) -> bool;
    pub fn wxListCtrl_SetItemFont(self_: *mut c_void, item: c_long, font: *const c_void);
    pub fn wxListCtrl_SetItemImage(
        self_: *mut c_void,
        item: c_long,
        image: c_int,
        sel_image: c_int,
    ) -> bool;
    pub fn wxListCtrl_SetItemPosition(self_: *mut c_void, item: c_long, pos: *const c_void)
        -> bool;
    // NOT_SUPPORTED: pub fn wxListCtrl_SetItemPtrData(self_: *mut c_void, item: c_long, data: wxUIntPtr) -> bool;
    pub fn wxListCtrl_SetItemState(
        self_: *mut c_void,
        item: c_long,
        state: c_long,
        state_mask: c_long,
    ) -> bool;
    pub fn wxListCtrl_SetItemText(self_: *mut c_void, item: c_long, text: *const c_void);
    pub fn wxListCtrl_SetItemTextColour(self_: *mut c_void, item: c_long, col: *const c_void);
    pub fn wxListCtrl_SetSingleStyle(self_: *mut c_void, style: c_long, add: bool);
    pub fn wxListCtrl_SetTextColour(self_: *mut c_void, col: *const c_void);
    // NOT_SUPPORTED: pub fn wxListCtrl_SortItems(self_: *mut c_void, fn_sort_call_back: wxListCtrlCompare, data: wxIntPtr) -> bool;
    pub fn wxListCtrl_HasCheckBoxes(self_: *const c_void) -> bool;
    pub fn wxListCtrl_EnableCheckBoxes(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxListCtrl_IsItemChecked(self_: *const c_void, item: c_long) -> bool;
    pub fn wxListCtrl_CheckItem(self_: *mut c_void, item: c_long, check: bool);
    pub fn wxListCtrl_ExtendRulesAndAlternateColour(self_: *mut c_void, extend: bool);
    pub fn wxListCtrl_ShowSortIndicator(self_: *mut c_void, col: c_int, ascending: bool);
    pub fn wxListCtrl_RemoveSortIndicator(self_: *mut c_void);
    pub fn wxListCtrl_GetSortIndicator(self_: *const c_void) -> c_int;
    pub fn wxListCtrl_GetUpdatedAscendingSortIndicator(self_: *const c_void, col: c_int) -> bool;
    pub fn wxListCtrl_IsAscendingSortIndicator(self_: *const c_void) -> bool;

    // wxListEvent
    pub fn wxListEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxListEvent_new(command_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxListEvent_GetCacheFrom(self_: *const c_void) -> c_long;
    pub fn wxListEvent_GetCacheTo(self_: *const c_void) -> c_long;
    pub fn wxListEvent_GetColumn(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxListEvent_GetData(self_: *const c_void) -> wxUIntPtr;
    pub fn wxListEvent_GetImage(self_: *const c_void) -> c_int;
    pub fn wxListEvent_GetIndex(self_: *const c_void) -> c_long;
    pub fn wxListEvent_GetItem(self_: *const c_void) -> *mut c_void;
    pub fn wxListEvent_GetKeyCode(self_: *const c_void) -> c_int;
    pub fn wxListEvent_GetLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxListEvent_GetMask(self_: *const c_void) -> c_long;
    pub fn wxListEvent_GetPoint(self_: *const c_void) -> *mut c_void;
    pub fn wxListEvent_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxListEvent_IsEditCancelled(self_: *const c_void) -> bool;
    pub fn wxListEvent_SetKeyCode(self_: *mut c_void, code: c_int);
    pub fn wxListEvent_SetIndex(self_: *mut c_void, index: c_long);
    pub fn wxListEvent_SetColumn(self_: *mut c_void, col: c_int);
    pub fn wxListEvent_SetPoint(self_: *mut c_void, point: *const c_void);
    pub fn wxListEvent_SetItem(self_: *mut c_void, item: *const c_void);
    pub fn wxListEvent_SetCacheFrom(self_: *mut c_void, cache_from: c_long);
    pub fn wxListEvent_SetCacheTo(self_: *mut c_void, cache_to: c_long);

    // wxListItem
    pub fn wxListItem_CLASSINFO() -> *mut c_void;
    pub fn wxListItem_new() -> *mut c_void;
    pub fn wxListItem_Clear(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxListItem_GetAlign(self_: *const c_void) -> wxListColumnFormat;
    pub fn wxListItem_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxListItem_GetColumn(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxListItem_GetData(self_: *const c_void) -> wxUIntPtr;
    pub fn wxListItem_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxListItem_GetId(self_: *const c_void) -> c_long;
    pub fn wxListItem_GetImage(self_: *const c_void) -> c_int;
    pub fn wxListItem_GetMask(self_: *const c_void) -> c_long;
    pub fn wxListItem_GetState(self_: *const c_void) -> c_long;
    pub fn wxListItem_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxListItem_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxListItem_GetWidth(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxListItem_SetAlign(self_: *mut c_void, align: wxListColumnFormat);
    pub fn wxListItem_SetBackgroundColour(self_: *mut c_void, col_back: *const c_void);
    pub fn wxListItem_SetColumn(self_: *mut c_void, col: c_int);
    pub fn wxListItem_SetData(self_: *mut c_void, data: c_long);
    pub fn wxListItem_SetData1(self_: *mut c_void, data: *mut c_void);
    pub fn wxListItem_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxListItem_SetId(self_: *mut c_void, id: c_long);
    pub fn wxListItem_SetImage(self_: *mut c_void, image: c_int);
    pub fn wxListItem_SetMask(self_: *mut c_void, mask: c_long);
    pub fn wxListItem_SetState(self_: *mut c_void, state: c_long);
    pub fn wxListItem_SetStateMask(self_: *mut c_void, state_mask: c_long);
    pub fn wxListItem_SetText(self_: *mut c_void, text: *const c_void);
    pub fn wxListItem_SetTextColour(self_: *mut c_void, col_text: *const c_void);
    pub fn wxListItem_SetWidth(self_: *mut c_void, width: c_int);

    // wxListView
    pub fn wxListView_CLASSINFO() -> *mut c_void;
    pub fn wxListView_new() -> *mut c_void;
    pub fn wxListView_new1(
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxListView_~wxListView(self_: *mut c_void);
    pub fn wxListView_ClearColumnImage(self_: *mut c_void, col: c_int);
    pub fn wxListView_Focus(self_: *mut c_void, index: c_long);
    pub fn wxListView_GetFirstSelected(self_: *const c_void) -> c_long;
    pub fn wxListView_GetFocusedItem(self_: *const c_void) -> c_long;
    pub fn wxListView_GetNextSelected(self_: *const c_void, item: c_long) -> c_long;
    pub fn wxListView_IsSelected(self_: *const c_void, index: c_long) -> bool;
    pub fn wxListView_Select(self_: *mut c_void, n: c_long, on: bool);
    pub fn wxListView_SetColumnImage(self_: *mut c_void, col: c_int, image: c_int);

    // wxListbook
    pub fn wxListbook_CLASSINFO() -> *mut c_void;
    pub fn wxListbook_new() -> *mut c_void;
    pub fn wxListbook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxListbook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxListbook_GetListView(self_: *const c_void) -> *mut c_void;
    // Mix-in(s) to wxListbook
    pub fn wxListbook_AsWithImages(obj: *mut c_void) -> *mut c_void;

    // wxLogGui
    pub fn wxLogGui_delete(self_: *mut c_void);
    pub fn wxLogGui_new() -> *mut c_void;

    // wxLogTextCtrl
    pub fn wxLogTextCtrl_delete(self_: *mut c_void);
    pub fn wxLogTextCtrl_new(p_text_ctrl: *mut c_void) -> *mut c_void;

    // wxLogWindow
    pub fn wxLogWindow_delete(self_: *mut c_void);
    pub fn wxLogWindow_new(
        p_parent: *mut c_void,
        sz_title: *const c_void,
        show: bool,
        pass_to_old: bool,
    ) -> *mut c_void;
    pub fn wxLogWindow_GetFrame(self_: *const c_void) -> *mut c_void;
    pub fn wxLogWindow_OnFrameClose(self_: *mut c_void, frame: *mut c_void) -> bool;
    pub fn wxLogWindow_OnFrameDelete(self_: *mut c_void, frame: *mut c_void);
    pub fn wxLogWindow_Show(self_: *mut c_void, show: bool);

    // wxLongPressEvent
    pub fn wxLongPressEvent_CLASSINFO() -> *mut c_void;
    pub fn wxLongPressEvent_new(windid: c_int) -> *mut c_void;

    // wxMDIChildFrame
    pub fn wxMDIChildFrame_CLASSINFO() -> *mut c_void;
    pub fn wxMDIChildFrame_new() -> *mut c_void;
    pub fn wxMDIChildFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxMDIChildFrame_~wxMDIChildFrame(self_: *mut c_void);
    pub fn wxMDIChildFrame_Activate(self_: *mut c_void);
    pub fn wxMDIChildFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxMDIChildFrame_GetMDIParent(self_: *const c_void) -> *mut c_void;
    pub fn wxMDIChildFrame_Restore(self_: *mut c_void);

    // wxMDIClientWindow
    pub fn wxMDIClientWindow_CLASSINFO() -> *mut c_void;
    pub fn wxMDIClientWindow_new() -> *mut c_void;
    pub fn wxMDIClientWindow_CreateClient(
        self_: *mut c_void,
        parent: *mut c_void,
        style: c_long,
    ) -> bool;

    // wxMDIParentFrame
    pub fn wxMDIParentFrame_CLASSINFO() -> *mut c_void;
    pub fn wxMDIParentFrame_new() -> *mut c_void;
    pub fn wxMDIParentFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxMDIParentFrame_~wxMDIParentFrame(self_: *mut c_void);
    pub fn wxMDIParentFrame_ActivateNext(self_: *mut c_void);
    pub fn wxMDIParentFrame_ActivatePrevious(self_: *mut c_void);
    pub fn wxMDIParentFrame_ArrangeIcons(self_: *mut c_void);
    pub fn wxMDIParentFrame_Cascade(self_: *mut c_void);
    pub fn wxMDIParentFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxMDIParentFrame_GetActiveChild(self_: *const c_void) -> *mut c_void;
    pub fn wxMDIParentFrame_GetClientWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxMDIParentFrame_GetWindowMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxMDIParentFrame_OnCreateClient(self_: *mut c_void) -> *mut c_void;
    pub fn wxMDIParentFrame_SetWindowMenu(self_: *mut c_void, menu: *mut c_void);
    // NOT_SUPPORTED: pub fn wxMDIParentFrame_Tile(self_: *mut c_void, orient: wxOrientation);
    pub fn wxMDIParentFrame_IsTDI() -> bool;

    // wxMask
    pub fn wxMask_CLASSINFO() -> *mut c_void;
    pub fn wxMask_new() -> *mut c_void;
    pub fn wxMask_new1(bitmap: *const c_void, index: c_int) -> *mut c_void;
    pub fn wxMask_new2(bitmap: *const c_void) -> *mut c_void;
    pub fn wxMask_new3(bitmap: *const c_void, colour: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxMask_~wxMask(self_: *mut c_void);
    pub fn wxMask_Create(self_: *mut c_void, bitmap: *const c_void, index: c_int) -> bool;
    pub fn wxMask_Create1(self_: *mut c_void, bitmap: *const c_void) -> bool;
    pub fn wxMask_Create2(self_: *mut c_void, bitmap: *const c_void, colour: *const c_void)
        -> bool;
    pub fn wxMask_GetBitmap(self_: *const c_void) -> *mut c_void;

    // wxMaximizeEvent
    pub fn wxMaximizeEvent_CLASSINFO() -> *mut c_void;
    pub fn wxMaximizeEvent_new(id: c_int) -> *mut c_void;

    // wxMemoryDC
    pub fn wxMemoryDC_CLASSINFO() -> *mut c_void;
    pub fn wxMemoryDC_new() -> *mut c_void;
    pub fn wxMemoryDC_new1(dc: *mut c_void) -> *mut c_void;
    pub fn wxMemoryDC_new2(bitmap: *mut c_void) -> *mut c_void;
    pub fn wxMemoryDC_SelectObject(self_: *mut c_void, bitmap: *mut c_void);
    pub fn wxMemoryDC_SelectObjectAsSource(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxMemoryDC_GetSelectedBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxMemoryDC_GetSelectedBitmap1(self_: *mut c_void) -> *mut c_void;

    // wxMenu
    pub fn wxMenu_CLASSINFO() -> *mut c_void;
    pub fn wxMenu_new() -> *mut c_void;
    pub fn wxMenu_new1(style: c_long) -> *mut c_void;
    pub fn wxMenu_new2(title: *const c_void, style: c_long) -> *mut c_void;
    // DTOR: pub fn wxMenu_~wxMenu(self_: *mut c_void);
    pub fn wxMenu_Append(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxMenu_Append1(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        sub_menu: *mut c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_Append2(self_: *mut c_void, menu_item: *mut c_void) -> *mut c_void;
    pub fn wxMenu_AppendCheckItem(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_AppendRadioItem(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_AppendSeparator(self_: *mut c_void) -> *mut c_void;
    pub fn wxMenu_AppendSubMenu(
        self_: *mut c_void,
        submenu: *mut c_void,
        text: *const c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_Break(self_: *mut c_void);
    pub fn wxMenu_Check(self_: *mut c_void, id: c_int, check: bool);
    pub fn wxMenu_Delete(self_: *mut c_void, id: c_int) -> bool;
    pub fn wxMenu_Delete1(self_: *mut c_void, item: *mut c_void) -> bool;
    pub fn wxMenu_Destroy(self_: *mut c_void, id: c_int) -> bool;
    pub fn wxMenu_Destroy1(self_: *mut c_void, item: *mut c_void) -> bool;
    pub fn wxMenu_Enable(self_: *mut c_void, id: c_int, enable: bool);
    pub fn wxMenu_FindChildItem(self_: *const c_void, id: c_int, pos: *mut c_void) -> *mut c_void;
    pub fn wxMenu_FindItem(self_: *const c_void, item_string: *const c_void) -> c_int;
    pub fn wxMenu_FindItem1(self_: *const c_void, id: c_int, menu: *mut c_void) -> *mut c_void;
    pub fn wxMenu_FindItemByPosition(self_: *const c_void, position: usize) -> *mut c_void;
    pub fn wxMenu_GetHelpString(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMenu_GetLabel(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMenu_GetLabelText(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMenu_GetMenuItemCount(self_: *const c_void) -> usize;
    // BLOCKED: pub fn wxMenu_GetMenuItems(self_: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenu_GetMenuItems1(self_: *const c_void) -> *const c_void;
    pub fn wxMenu_GetTitle(self_: *const c_void) -> *mut c_void;
    pub fn wxMenu_Insert(self_: *mut c_void, pos: usize, menu_item: *mut c_void) -> *mut c_void;
    pub fn wxMenu_Insert1(
        self_: *mut c_void,
        pos: usize,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxMenu_Insert2(
        self_: *mut c_void,
        pos: usize,
        id: c_int,
        text: *const c_void,
        submenu: *mut c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_InsertCheckItem(
        self_: *mut c_void,
        pos: usize,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_InsertRadioItem(
        self_: *mut c_void,
        pos: usize,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_InsertSeparator(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxMenu_IsChecked(self_: *const c_void, id: c_int) -> bool;
    pub fn wxMenu_IsEnabled(self_: *const c_void, id: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxMenu_MSWCommand(self_: *mut c_void, param: WXUINT, id: WXWORD) -> bool;
    pub fn wxMenu_Prepend(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxMenu_Prepend1(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxMenu_Prepend2(
        self_: *mut c_void,
        id: c_int,
        text: *const c_void,
        submenu: *mut c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_PrependCheckItem(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_PrependRadioItem(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_PrependSeparator(self_: *mut c_void) -> *mut c_void;
    pub fn wxMenu_Remove(self_: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxMenu_Remove1(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxMenu_SetHelpString(self_: *mut c_void, id: c_int, help_string: *const c_void);
    pub fn wxMenu_SetLabel(self_: *mut c_void, id: c_int, label: *const c_void);
    pub fn wxMenu_SetTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxMenu_UpdateUI(self_: *mut c_void, source: *mut c_void);
    pub fn wxMenu_SetInvokingWindow(self_: *mut c_void, win: *mut c_void);
    pub fn wxMenu_GetInvokingWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxMenu_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxMenu_GetStyle(self_: *const c_void) -> c_long;
    pub fn wxMenu_SetParent(self_: *mut c_void, parent: *mut c_void);
    pub fn wxMenu_GetParent(self_: *const c_void) -> *mut c_void;
    pub fn wxMenu_Attach(self_: *mut c_void, menubar: *mut c_void);
    pub fn wxMenu_Detach(self_: *mut c_void);
    pub fn wxMenu_IsAttached(self_: *const c_void) -> bool;

    // wxMenuBar
    pub fn wxMenuBar_CLASSINFO() -> *mut c_void;
    pub fn wxMenuBar_new(style: c_long) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMenuBar_new1(n: usize, menus: *mut c_void, titles: wxString, style: c_long) -> *mut c_void;
    // DTOR: pub fn wxMenuBar_~wxMenuBar(self_: *mut c_void);
    pub fn wxMenuBar_Append(self_: *mut c_void, menu: *mut c_void, title: *const c_void) -> bool;
    pub fn wxMenuBar_Check(self_: *mut c_void, id: c_int, check: bool);
    pub fn wxMenuBar_Enable(self_: *mut c_void, id: c_int, enable: bool);
    pub fn wxMenuBar_IsEnabledTop(self_: *const c_void, pos: usize) -> bool;
    pub fn wxMenuBar_EnableTop(self_: *mut c_void, pos: usize, enable: bool);
    pub fn wxMenuBar_FindItem(self_: *const c_void, id: c_int, menu: *mut c_void) -> *mut c_void;
    pub fn wxMenuBar_FindMenu(self_: *const c_void, title: *const c_void) -> c_int;
    pub fn wxMenuBar_FindMenuItem(
        self_: *const c_void,
        menu_string: *const c_void,
        item_string: *const c_void,
    ) -> c_int;
    pub fn wxMenuBar_GetHelpString(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMenuBar_GetLabel(self_: *const c_void, id: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxMenuBar_GetLabelTop(self_: *const c_void, pos: usize) -> wxString;
    pub fn wxMenuBar_GetMenu(self_: *const c_void, menu_index: usize) -> *mut c_void;
    pub fn wxMenuBar_GetMenuCount(self_: *const c_void) -> usize;
    pub fn wxMenuBar_GetMenuLabel(self_: *const c_void, pos: usize) -> *mut c_void;
    pub fn wxMenuBar_GetMenuLabelText(self_: *const c_void, pos: usize) -> *mut c_void;
    pub fn wxMenuBar_Insert(
        self_: *mut c_void,
        pos: usize,
        menu: *mut c_void,
        title: *const c_void,
    ) -> bool;
    pub fn wxMenuBar_IsChecked(self_: *const c_void, id: c_int) -> bool;
    pub fn wxMenuBar_IsEnabled(self_: *const c_void, id: c_int) -> bool;
    pub fn wxMenuBar_Remove(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxMenuBar_Replace(
        self_: *mut c_void,
        pos: usize,
        menu: *mut c_void,
        title: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenuBar_SetHelpString(self_: *mut c_void, id: c_int, help_string: *const c_void);
    pub fn wxMenuBar_SetLabel(self_: *mut c_void, id: c_int, label: *const c_void);
    // BLOCKED: pub fn wxMenuBar_SetLabelTop(self_: *mut c_void, pos: usize, label: *const c_void);
    pub fn wxMenuBar_SetMenuLabel(self_: *mut c_void, pos: usize, label: *const c_void);
    pub fn wxMenuBar_OSXGetAppleMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuBar_GetFrame(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuBar_IsAttached(self_: *const c_void) -> bool;
    pub fn wxMenuBar_Attach(self_: *mut c_void, frame: *mut c_void);
    pub fn wxMenuBar_Detach(self_: *mut c_void);
    pub fn wxMenuBar_MacSetCommonMenuBar(menubar: *mut c_void);
    pub fn wxMenuBar_MacGetCommonMenuBar() -> *mut c_void;

    // wxMenuEvent
    pub fn wxMenuEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMenuEvent_new(type_: wxEventType, id: c_int, menu: *mut c_void) -> *mut c_void;
    pub fn wxMenuEvent_GetMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuEvent_GetMenuId(self_: *const c_void) -> c_int;
    pub fn wxMenuEvent_IsPopup(self_: *const c_void) -> bool;

    // wxMenuItem
    pub fn wxMenuItem_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetBitmap1(self_: *const c_void, checked: bool) -> *mut c_void;
    pub fn wxMenuItem_GetBitmapBundle(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetDisabledBitmap(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetHelp(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetId(self_: *const c_void) -> c_int;
    pub fn wxMenuItem_GetItemLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetItemLabelText(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetKind(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxMenuItem_GetLabel(self_: *const c_void) -> wxString;
    pub fn wxMenuItem_GetMarginWidth(self_: *const c_void) -> c_int;
    pub fn wxMenuItem_GetMenu(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetName(self_: *const c_void) -> wxString;
    pub fn wxMenuItem_GetSubMenu(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetText(self_: *const c_void) -> *const c_void;
    // BLOCKED: pub fn wxMenuItem_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetAccel(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetAccelFromString(label: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_IsCheck(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsCheckable(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsChecked(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsEnabled(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsRadio(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsSeparator(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsSubMenu(self_: *const c_void) -> bool;
    pub fn wxMenuItem_SetBackgroundColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxMenuItem_SetBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxMenuItem_SetBitmap1(self_: *mut c_void, bmp: *const c_void, checked: bool);
    pub fn wxMenuItem_SetBitmaps(
        self_: *mut c_void,
        checked: *const c_void,
        unchecked: *const c_void,
    );
    pub fn wxMenuItem_SetDisabledBitmap(self_: *mut c_void, disabled: *const c_void);
    pub fn wxMenuItem_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxMenuItem_SetHelp(self_: *mut c_void, help_string: *const c_void);
    pub fn wxMenuItem_SetItemLabel(self_: *mut c_void, label: *const c_void);
    pub fn wxMenuItem_SetMarginWidth(self_: *mut c_void, width: c_int);
    pub fn wxMenuItem_SetMenu(self_: *mut c_void, menu: *mut c_void);
    pub fn wxMenuItem_SetSubMenu(self_: *mut c_void, menu: *mut c_void);
    // BLOCKED: pub fn wxMenuItem_SetText(self_: *mut c_void, text: *const c_void);
    pub fn wxMenuItem_SetTextColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxMenuItem_SetAccel(self_: *mut c_void, accel: *mut c_void);
    pub fn wxMenuItem_AddExtraAccel(self_: *mut c_void, accel: *const c_void);
    pub fn wxMenuItem_ClearExtraAccels(self_: *mut c_void);
    pub fn wxMenuItem_new(
        parent_menu: *mut c_void,
        id: c_int,
        text: *const c_void,
        help_string: *const c_void,
        kind: c_int,
        sub_menu: *mut c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxMenuItem_~wxMenuItem(self_: *mut c_void);
    pub fn wxMenuItem_Check(self_: *mut c_void, check: bool);
    pub fn wxMenuItem_Enable(self_: *mut c_void, enable: bool);
    // BLOCKED: pub fn wxMenuItem_GetLabelFromText(text: *const c_void) -> wxString;
    pub fn wxMenuItem_GetLabelText(text: *const c_void) -> *mut c_void;

    // wxMessageOutputMessageBox
    pub fn wxMessageOutputMessageBox_delete(self_: *mut c_void);
    pub fn wxMessageOutputMessageBox_new() -> *mut c_void;

    // wxMetafile
    pub fn wxMetafile_CLASSINFO() -> *mut c_void;
    pub fn wxMetafile_new(filename: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxMetafile_~wxMetafile(self_: *mut c_void);
    pub fn wxMetafile_IsOk(self_: *mut c_void) -> bool;
    pub fn wxMetafile_Play(self_: *mut c_void, dc: *mut c_void) -> bool;
    pub fn wxMetafile_SetClipboard(self_: *mut c_void, width: c_int, height: c_int) -> bool;

    // wxMiniFrame
    pub fn wxMiniFrame_CLASSINFO() -> *mut c_void;
    pub fn wxMiniFrame_new() -> *mut c_void;
    pub fn wxMiniFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxMiniFrame_~wxMiniFrame(self_: *mut c_void);
    pub fn wxMiniFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxMirrorDC
    pub fn wxMirrorDC_CLASSINFO() -> *mut c_void;
    pub fn wxMirrorDC_new(dc: *mut c_void, mirror: bool) -> *mut c_void;

    // wxMouseCaptureChangedEvent
    pub fn wxMouseCaptureChangedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxMouseCaptureChangedEvent_new(
        window_id: c_int,
        gained_capture: *mut c_void,
    ) -> *mut c_void;
    pub fn wxMouseCaptureChangedEvent_GetCapturedWindow(self_: *const c_void) -> *mut c_void;

    // wxMouseCaptureLostEvent
    pub fn wxMouseCaptureLostEvent_CLASSINFO() -> *mut c_void;
    pub fn wxMouseCaptureLostEvent_new(window_id: c_int) -> *mut c_void;

    // wxMouseEvent
    pub fn wxMouseEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMouseEvent_new(mouse_event_type: wxEventType) -> *mut c_void;
    pub fn wxMouseEvent_Aux1DClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux1Down(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux1Up(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux2DClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux2Down(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux2Up(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxMouseEvent_Button(self_: *const c_void, but: wxMouseButton) -> bool;
    // NOT_SUPPORTED: pub fn wxMouseEvent_ButtonDClick(self_: *const c_void, but: wxMouseButton) -> bool;
    // NOT_SUPPORTED: pub fn wxMouseEvent_ButtonDown(self_: *const c_void, but: wxMouseButton) -> bool;
    // NOT_SUPPORTED: pub fn wxMouseEvent_ButtonUp(self_: *const c_void, but: wxMouseButton) -> bool;
    pub fn wxMouseEvent_Dragging(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Entering(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_GetButton(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_GetClickCount(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_GetLinesPerAction(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_GetColumnsPerAction(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_GetLogicalPosition(self_: *const c_void, dc: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMouseEvent_GetMagnification(self_: *const c_void) -> float;
    pub fn wxMouseEvent_GetWheelDelta(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_IsWheelInverted(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_GetWheelRotation(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxMouseEvent_GetWheelAxis(self_: *const c_void) -> wxMouseWheelAxis;
    pub fn wxMouseEvent_IsButton(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_IsPageScroll(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Leaving(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_LeftDClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_LeftDown(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_LeftUp(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Magnify(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_MetaDown(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_MiddleDClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_MiddleDown(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_MiddleUp(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Moving(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_RightDClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_RightDown(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_RightUp(self_: *const c_void) -> bool;
    // Mix-in(s) to wxMouseEvent
    pub fn wxMouseEvent_AsMouseState(obj: *mut c_void) -> *mut c_void;

    // wxMoveEvent
    pub fn wxMoveEvent_CLASSINFO() -> *mut c_void;
    pub fn wxMoveEvent_new(pt: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMoveEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxMoveEvent_GetRect(self_: *const c_void) -> *mut c_void;
    pub fn wxMoveEvent_SetRect(self_: *mut c_void, rect: *const c_void);
    pub fn wxMoveEvent_SetPosition(self_: *mut c_void, pos: *const c_void);

    // wxNativeFontInfo
    pub fn wxNativeFontInfo_delete(self_: *mut c_void);
    pub fn wxNativeFontInfo_new() -> *mut c_void;
    pub fn wxNativeFontInfo_new1(info: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxNativeFontInfo_~wxNativeFontInfo(self_: *mut c_void);
    // BLOCKED: pub fn wxNativeFontInfo_operator=(self_: *mut c_void, info: *const c_void) -> *mut c_void;
    pub fn wxNativeFontInfo_Init(self_: *mut c_void);
    pub fn wxNativeFontInfo_InitFromFont(self_: *mut c_void, font: *const c_void);
    pub fn wxNativeFontInfo_GetPointSize(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_GetFractionalPointSize(self_: *const c_void) -> float;
    pub fn wxNativeFontInfo_GetPixelSize(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_GetStyle(self_: *const c_void) -> wxFontStyle;
    pub fn wxNativeFontInfo_GetNumericWeight(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_GetWeight(self_: *const c_void) -> wxFontWeight;
    pub fn wxNativeFontInfo_GetUnderlined(self_: *const c_void) -> bool;
    pub fn wxNativeFontInfo_GetFaceName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_GetFamily(self_: *const c_void) -> wxFontFamily;
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_GetEncoding(self_: *const c_void) -> wxFontEncoding;
    pub fn wxNativeFontInfo_SetPointSize(self_: *mut c_void, pointsize: c_int);
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_SetFractionalPointSize(self_: *mut c_void, pointsize: float);
    pub fn wxNativeFontInfo_SetPixelSize(self_: *mut c_void, pixel_size: *const c_void);
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_SetStyle(self_: *mut c_void, style: wxFontStyle);
    pub fn wxNativeFontInfo_SetNumericWeight(self_: *mut c_void, weight: c_int);
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_SetWeight(self_: *mut c_void, weight: wxFontWeight);
    pub fn wxNativeFontInfo_SetUnderlined(self_: *mut c_void, underlined: bool);
    pub fn wxNativeFontInfo_SetFaceName(self_: *mut c_void, facename: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_SetFamily(self_: *mut c_void, family: wxFontFamily);
    // NOT_SUPPORTED: pub fn wxNativeFontInfo_SetEncoding(self_: *mut c_void, encoding: wxFontEncoding);
    pub fn wxNativeFontInfo_SetFaceName1(self_: *mut c_void, facenames: *const c_void);
    pub fn wxNativeFontInfo_FromString(self_: *mut c_void, s: *const c_void) -> bool;
    pub fn wxNativeFontInfo_ToString(self_: *const c_void) -> *mut c_void;
    pub fn wxNativeFontInfo_FromUserString(self_: *mut c_void, s: *const c_void) -> bool;
    pub fn wxNativeFontInfo_ToUserString(self_: *const c_void) -> *mut c_void;

    // wxNativeWindow
    pub fn wxNativeWindow_CLASSINFO() -> *mut c_void;
    pub fn wxNativeWindow_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxNativeWindow_new1(parent: *mut c_void, winid: c_int, handle: wxNativeWindowHandle) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxNativeWindow_Create(self_: *mut c_void, parent: *mut c_void, winid: c_int, handle: wxNativeWindowHandle) -> bool;
    pub fn wxNativeWindow_Disown(self_: *mut c_void);

    // wxNavigationKeyEvent
    pub fn wxNavigationKeyEvent_CLASSINFO() -> *mut c_void;
    pub fn wxNavigationKeyEvent_new() -> *mut c_void;
    pub fn wxNavigationKeyEvent_new1(event: *const c_void) -> *mut c_void;
    pub fn wxNavigationKeyEvent_GetCurrentFocus(self_: *const c_void) -> *mut c_void;
    pub fn wxNavigationKeyEvent_GetDirection(self_: *const c_void) -> bool;
    pub fn wxNavigationKeyEvent_IsFromTab(self_: *const c_void) -> bool;
    pub fn wxNavigationKeyEvent_IsWindowChange(self_: *const c_void) -> bool;
    pub fn wxNavigationKeyEvent_SetCurrentFocus(self_: *mut c_void, current_focus: *mut c_void);
    pub fn wxNavigationKeyEvent_SetDirection(self_: *mut c_void, direction: bool);
    pub fn wxNavigationKeyEvent_SetFlags(self_: *mut c_void, flags: c_long);
    pub fn wxNavigationKeyEvent_SetFromTab(self_: *mut c_void, from_tab: bool);
    pub fn wxNavigationKeyEvent_SetWindowChange(self_: *mut c_void, window_change: bool);

    // wxNonOwnedWindow
    pub fn wxNonOwnedWindow_CLASSINFO() -> *mut c_void;
    pub fn wxNonOwnedWindow_SetShape(self_: *mut c_void, region: *const c_void) -> bool;
    pub fn wxNonOwnedWindow_SetShape1(self_: *mut c_void, path: *const c_void) -> bool;

    // wxNotebook
    pub fn wxNotebook_CLASSINFO() -> *mut c_void;
    pub fn wxNotebook_new() -> *mut c_void;
    pub fn wxNotebook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxNotebook_~wxNotebook(self_: *mut c_void);
    // BLOCKED: pub fn wxNotebook_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> bool;
    pub fn wxNotebook_GetRowCount(self_: *const c_void) -> c_int;
    pub fn wxNotebook_GetThemeBackgroundColour(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxNotebook_OnSelChange(self_: *mut c_void, event: *mut c_void);
    pub fn wxNotebook_SetPadding(self_: *mut c_void, padding: *const c_void);
    // Mix-in(s) to wxNotebook
    pub fn wxNotebook_AsWithImages(obj: *mut c_void) -> *mut c_void;

    // wxNotifyEvent
    pub fn wxNotifyEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxNotifyEvent_new(event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxNotifyEvent_Allow(self_: *mut c_void);
    pub fn wxNotifyEvent_IsAllowed(self_: *const c_void) -> bool;
    pub fn wxNotifyEvent_Veto(self_: *mut c_void);

    // wxOverlay
    pub fn wxOverlay_delete(self_: *mut c_void);
    pub fn wxOverlay_new() -> *mut c_void;
    // DTOR: pub fn wxOverlay_~wxOverlay(self_: *mut c_void);
    pub fn wxOverlay_Reset(self_: *mut c_void);

    // wxPageSetupDialog
    pub fn wxPageSetupDialog_CLASSINFO() -> *mut c_void;
    pub fn wxPageSetupDialog_new(parent: *mut c_void, data: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxPageSetupDialog_~wxPageSetupDialog(self_: *mut c_void);
    pub fn wxPageSetupDialog_GetPageSetupData(self_: *mut c_void) -> *mut c_void;
    pub fn wxPageSetupDialog_ShowModal(self_: *mut c_void) -> c_int;

    // wxPageSetupDialogData
    pub fn wxPageSetupDialogData_CLASSINFO() -> *mut c_void;
    pub fn wxPageSetupDialogData_new() -> *mut c_void;
    pub fn wxPageSetupDialogData_new1(data: *const c_void) -> *mut c_void;
    pub fn wxPageSetupDialogData_new2(print_data: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPageSetupDialogData_~wxPageSetupDialogData(self_: *mut c_void);
    pub fn wxPageSetupDialogData_EnableHelp(self_: *mut c_void, flag: bool);
    pub fn wxPageSetupDialogData_EnableMargins(self_: *mut c_void, flag: bool);
    pub fn wxPageSetupDialogData_EnableOrientation(self_: *mut c_void, flag: bool);
    pub fn wxPageSetupDialogData_EnablePaper(self_: *mut c_void, flag: bool);
    pub fn wxPageSetupDialogData_EnablePrinter(self_: *mut c_void, flag: bool);
    pub fn wxPageSetupDialogData_GetDefaultInfo(self_: *const c_void) -> bool;
    pub fn wxPageSetupDialogData_GetDefaultMinMargins(self_: *const c_void) -> bool;
    pub fn wxPageSetupDialogData_GetEnableHelp(self_: *const c_void) -> bool;
    pub fn wxPageSetupDialogData_GetEnableMargins(self_: *const c_void) -> bool;
    pub fn wxPageSetupDialogData_GetEnableOrientation(self_: *const c_void) -> bool;
    pub fn wxPageSetupDialogData_GetEnablePaper(self_: *const c_void) -> bool;
    pub fn wxPageSetupDialogData_GetEnablePrinter(self_: *const c_void) -> bool;
    pub fn wxPageSetupDialogData_GetMarginBottomRight(self_: *const c_void) -> *mut c_void;
    pub fn wxPageSetupDialogData_GetMarginTopLeft(self_: *const c_void) -> *mut c_void;
    pub fn wxPageSetupDialogData_GetMinMarginBottomRight(self_: *const c_void) -> *mut c_void;
    pub fn wxPageSetupDialogData_GetMinMarginTopLeft(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPageSetupDialogData_GetPaperId(self_: *const c_void) -> wxPaperSize;
    pub fn wxPageSetupDialogData_GetPaperSize(self_: *const c_void) -> *mut c_void;
    pub fn wxPageSetupDialogData_GetPrintData(self_: *mut c_void) -> *mut c_void;
    pub fn wxPageSetupDialogData_GetPrintData1(self_: *const c_void) -> *mut c_void;
    pub fn wxPageSetupDialogData_IsOk(self_: *const c_void) -> bool;
    pub fn wxPageSetupDialogData_SetDefaultInfo(self_: *mut c_void, flag: bool);
    pub fn wxPageSetupDialogData_SetDefaultMinMargins(self_: *mut c_void, flag: bool);
    pub fn wxPageSetupDialogData_SetMarginBottomRight(self_: *mut c_void, pt: *const c_void);
    pub fn wxPageSetupDialogData_SetMarginTopLeft(self_: *mut c_void, pt: *const c_void);
    pub fn wxPageSetupDialogData_SetMinMarginBottomRight(self_: *mut c_void, pt: *const c_void);
    pub fn wxPageSetupDialogData_SetMinMarginTopLeft(self_: *mut c_void, pt: *const c_void);
    // NOT_SUPPORTED: pub fn wxPageSetupDialogData_SetPaperId(self_: *mut c_void, id: wxPaperSize);
    pub fn wxPageSetupDialogData_SetPaperSize(self_: *mut c_void, size: *const c_void);
    pub fn wxPageSetupDialogData_SetPrintData(self_: *mut c_void, print_data: *const c_void);
    // BLOCKED: pub fn wxPageSetupDialogData_operator=(self_: *mut c_void, data: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPageSetupDialogData_operator=1(self_: *mut c_void, data: *const c_void) -> *mut c_void;

    // wxPaintDC
    pub fn wxPaintDC_CLASSINFO() -> *mut c_void;
    pub fn wxPaintDC_new(window: *mut c_void) -> *mut c_void;

    // wxPalette
    pub fn wxPalette_CLASSINFO() -> *mut c_void;
    pub fn wxPalette_new() -> *mut c_void;
    pub fn wxPalette_new1(palette: *const c_void) -> *mut c_void;
    pub fn wxPalette_new2(
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPalette_~wxPalette(self_: *mut c_void);
    pub fn wxPalette_Create(
        self_: *mut c_void,
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> bool;
    pub fn wxPalette_GetColoursCount(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxPalette_GetPixel(self_: *const c_void, red: unsigned char, green: unsigned char, blue: unsigned char) -> c_int;
    pub fn wxPalette_GetRGB(
        self_: *const c_void,
        pixel: c_int,
        red: *mut c_void,
        green: *mut c_void,
        blue: *mut c_void,
    ) -> bool;
    pub fn wxPalette_IsOk(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxPalette_operator=(self_: *mut c_void, palette: *const c_void) -> *mut c_void;

    // wxPanGestureEvent
    pub fn wxPanGestureEvent_CLASSINFO() -> *mut c_void;
    pub fn wxPanGestureEvent_new(winid: c_int) -> *mut c_void;
    pub fn wxPanGestureEvent_GetDelta(self_: *const c_void) -> *mut c_void;
    pub fn wxPanGestureEvent_SetDelta(self_: *mut c_void, delta: *const c_void);

    // wxPanel
    pub fn wxPanel_CLASSINFO() -> *mut c_void;
    pub fn wxPanel_new() -> *mut c_void;
    pub fn wxPanel_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPanel_~wxPanel(self_: *mut c_void);
    pub fn wxPanel_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxPanel_OnSysColourChanged(self_: *mut c_void, event: *mut c_void);
    pub fn wxPanel_SetFocusIgnoringChildren(self_: *mut c_void);

    // wxPen
    pub fn wxPen_CLASSINFO() -> *mut c_void;
    pub fn wxPen_new() -> *mut c_void;
    pub fn wxPen_new1(info: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPen_new2(colour: *const c_void, width: c_int, style: wxPenStyle) -> *mut c_void;
    pub fn wxPen_new3(stipple: *const c_void, width: c_int) -> *mut c_void;
    pub fn wxPen_new4(pen: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPen_~wxPen(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPen_GetCap(self_: *const c_void) -> wxPenCap;
    // NOT_SUPPORTED: pub fn wxPen_GetQuality(self_: *const c_void) -> wxPenQuality;
    pub fn wxPen_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxPen_GetDashes(self_: *const c_void, dashes: *mut c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxPen_GetJoin(self_: *const c_void) -> wxPenJoin;
    pub fn wxPen_GetStipple(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPen_GetStyle(self_: *const c_void) -> wxPenStyle;
    pub fn wxPen_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxPen_IsOk(self_: *const c_void) -> bool;
    pub fn wxPen_IsNonTransparent(self_: *const c_void) -> bool;
    pub fn wxPen_IsTransparent(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxPen_SetCap(self_: *mut c_void, cap_style: wxPenCap);
    // NOT_SUPPORTED: pub fn wxPen_SetQuality(self_: *mut c_void, quality: wxPenQuality);
    pub fn wxPen_SetColour(self_: *mut c_void, colour: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPen_SetColour1(self_: *mut c_void, red: unsigned char, green: unsigned char, blue: unsigned char);
    pub fn wxPen_SetDashes(self_: *mut c_void, n: c_int, dash: *const c_void);
    // NOT_SUPPORTED: pub fn wxPen_SetJoin(self_: *mut c_void, join_style: wxPenJoin);
    pub fn wxPen_SetStipple(self_: *mut c_void, stipple: *const c_void);
    // NOT_SUPPORTED: pub fn wxPen_SetStyle(self_: *mut c_void, style: wxPenStyle);
    pub fn wxPen_SetWidth(self_: *mut c_void, width: c_int);
    // BLOCKED: pub fn wxPen_operator!=(self_: *const c_void, pen: *const c_void) -> bool;
    // BLOCKED: pub fn wxPen_operator=(self_: *mut c_void, pen: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPen_operator==(self_: *const c_void, pen: *const c_void) -> bool;

    // wxPenList
    pub fn wxPenList_delete(self_: *mut c_void);
    pub fn wxPenList_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPenList_FindOrCreatePen(self_: *mut c_void, colour: *const c_void, width: c_int, style: wxPenStyle) -> *mut c_void;

    // wxPickerBase
    pub fn wxPickerBase_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxPickerBase_new() -> *mut c_void;
    // DTOR: pub fn wxPickerBase_~wxPickerBase(self_: *mut c_void);
    pub fn wxPickerBase_CreateBase(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        text: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxPickerBase_GetInternalMargin(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_GetPickerCtrlProportion(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_GetTextCtrl(self_: *mut c_void) -> *mut c_void;
    pub fn wxPickerBase_GetPickerCtrl(self_: *mut c_void) -> *mut c_void;
    pub fn wxPickerBase_GetTextCtrlProportion(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_HasTextCtrl(self_: *const c_void) -> bool;
    pub fn wxPickerBase_IsPickerCtrlGrowable(self_: *const c_void) -> bool;
    pub fn wxPickerBase_IsTextCtrlGrowable(self_: *const c_void) -> bool;
    pub fn wxPickerBase_SetInternalMargin(self_: *mut c_void, margin: c_int);
    pub fn wxPickerBase_SetPickerCtrlGrowable(self_: *mut c_void, grow: bool);
    pub fn wxPickerBase_SetPickerCtrlProportion(self_: *mut c_void, prop: c_int);
    pub fn wxPickerBase_SetTextCtrlGrowable(self_: *mut c_void, grow: bool);
    pub fn wxPickerBase_SetTextCtrlProportion(self_: *mut c_void, prop: c_int);
    pub fn wxPickerBase_SetTextCtrl(self_: *mut c_void, text: *mut c_void);
    pub fn wxPickerBase_SetPickerCtrl(self_: *mut c_void, picker: *mut c_void);
    pub fn wxPickerBase_UpdatePickerFromTextCtrl(self_: *mut c_void);
    pub fn wxPickerBase_UpdateTextCtrlFromPicker(self_: *mut c_void);

    // wxPoint
    pub fn wxPoint_delete(self_: *mut c_void);
    pub fn wxPoint_IsFullySpecified(self_: *const c_void) -> bool;
    pub fn wxPoint_SetDefaults(self_: *mut c_void, pt: *const c_void);
    // BLOCKED: pub fn wxPoint_operator=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator==(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxPoint_operator!=(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxPoint_operator+(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator-=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator+1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator-=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    pub fn wxPoint_new() -> *mut c_void;
    pub fn wxPoint_new1(x: c_int, y: c_int) -> *mut c_void;
    pub fn wxPoint_new2(pt: *const c_void) -> *mut c_void;

    // wxPopupTransientWindow
    pub fn wxPopupTransientWindow_CLASSINFO() -> *mut c_void;
    pub fn wxPopupTransientWindow_new() -> *mut c_void;
    pub fn wxPopupTransientWindow_new1(parent: *mut c_void, flags: c_int) -> *mut c_void;
    pub fn wxPopupTransientWindow_Popup(self_: *mut c_void, focus: *mut c_void);
    pub fn wxPopupTransientWindow_Dismiss(self_: *mut c_void);
    pub fn wxPopupTransientWindow_ProcessLeftDown(self_: *mut c_void, event: *mut c_void) -> bool;

    // wxPopupWindow
    pub fn wxPopupWindow_CLASSINFO() -> *mut c_void;
    pub fn wxPopupWindow_new() -> *mut c_void;
    pub fn wxPopupWindow_new1(parent: *mut c_void, flags: c_int) -> *mut c_void;
    pub fn wxPopupWindow_Create(self_: *mut c_void, parent: *mut c_void, flags: c_int) -> bool;
    pub fn wxPopupWindow_Position(
        self_: *mut c_void,
        pt_origin: *const c_void,
        size_popup: *const c_void,
    );

    // wxPreferencesEditor
    pub fn wxPreferencesEditor_delete(self_: *mut c_void);
    pub fn wxPreferencesEditor_new(title: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPreferencesEditor_~wxPreferencesEditor(self_: *mut c_void);
    pub fn wxPreferencesEditor_AddPage(self_: *mut c_void, page: *mut c_void);
    pub fn wxPreferencesEditor_Show(self_: *mut c_void, parent: *mut c_void);
    pub fn wxPreferencesEditor_Dismiss(self_: *mut c_void);
    pub fn wxPreferencesEditor_ShouldApplyChangesImmediately() -> bool;
    pub fn wxPreferencesEditor_ShownModally() -> bool;

    // wxPressAndTapEvent
    pub fn wxPressAndTapEvent_CLASSINFO() -> *mut c_void;
    pub fn wxPressAndTapEvent_new(windid: c_int) -> *mut c_void;

    // wxPreviewControlBar
    pub fn wxPreviewControlBar_CLASSINFO() -> *mut c_void;
    pub fn wxPreviewControlBar_new(
        preview: *mut c_void,
        buttons: c_long,
        parent: *mut c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPreviewControlBar_~wxPreviewControlBar(self_: *mut c_void);
    pub fn wxPreviewControlBar_CreateButtons(self_: *mut c_void);
    pub fn wxPreviewControlBar_GetPrintPreview(self_: *const c_void) -> *mut c_void;
    pub fn wxPreviewControlBar_GetZoomControl(self_: *mut c_void) -> c_int;
    pub fn wxPreviewControlBar_SetZoomControl(self_: *mut c_void, percent: c_int);

    // wxPreviewFrame
    pub fn wxPreviewFrame_CLASSINFO() -> *mut c_void;
    pub fn wxPreviewFrame_new(
        preview: *mut c_void,
        parent: *mut c_void,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPreviewFrame_~wxPreviewFrame(self_: *mut c_void);
    pub fn wxPreviewFrame_CreateCanvas(self_: *mut c_void);
    pub fn wxPreviewFrame_CreateControlBar(self_: *mut c_void);
    pub fn wxPreviewFrame_Initialize(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPreviewFrame_InitializeWithModality(self_: *mut c_void, kind: wxPreviewFrameModalityKind);
    pub fn wxPreviewFrame_OnCloseWindow(self_: *mut c_void, event: *mut c_void);

    // wxPrintData
    pub fn wxPrintData_CLASSINFO() -> *mut c_void;
    pub fn wxPrintData_new() -> *mut c_void;
    pub fn wxPrintData_new1(data: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPrintData_~wxPrintData(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPrintData_GetBin(self_: *const c_void) -> wxPrintBin;
    pub fn wxPrintData_GetCollate(self_: *const c_void) -> bool;
    pub fn wxPrintData_GetColour(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxPrintData_GetDuplex(self_: *const c_void) -> wxDuplexMode;
    pub fn wxPrintData_GetNoCopies(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxPrintData_GetOrientation(self_: *const c_void) -> wxPrintOrientation;
    // NOT_SUPPORTED: pub fn wxPrintData_GetPaperId(self_: *const c_void) -> wxPaperSize;
    pub fn wxPrintData_GetPrinterName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPrintData_GetQuality(self_: *const c_void) -> wxPrintQuality;
    pub fn wxPrintData_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxPrintData_SetBin(self_: *mut c_void, flag: wxPrintBin);
    pub fn wxPrintData_SetCollate(self_: *mut c_void, flag: bool);
    pub fn wxPrintData_SetColour(self_: *mut c_void, flag: bool);
    // NOT_SUPPORTED: pub fn wxPrintData_SetDuplex(self_: *mut c_void, mode: wxDuplexMode);
    pub fn wxPrintData_SetNoCopies(self_: *mut c_void, n: c_int);
    // NOT_SUPPORTED: pub fn wxPrintData_SetOrientation(self_: *mut c_void, orientation: wxPrintOrientation);
    // NOT_SUPPORTED: pub fn wxPrintData_SetPaperId(self_: *mut c_void, paper_id: wxPaperSize);
    pub fn wxPrintData_SetPaperSize(self_: *mut c_void, size: *const c_void);
    pub fn wxPrintData_SetPrinterName(self_: *mut c_void, printer_name: *const c_void);
    // NOT_SUPPORTED: pub fn wxPrintData_SetQuality(self_: *mut c_void, quality: wxPrintQuality);
    // BLOCKED: pub fn wxPrintData_operator=(self_: *mut c_void, data: *const c_void) -> *mut c_void;
    pub fn wxPrintData_GetFilename(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintData_SetFilename(self_: *mut c_void, filename: *const c_void);
    // NOT_SUPPORTED: pub fn wxPrintData_GetPrintMode(self_: *const c_void) -> wxPrintMode;
    // NOT_SUPPORTED: pub fn wxPrintData_SetPrintMode(self_: *mut c_void, print_mode: wxPrintMode);

    // wxPrintPreview
    pub fn wxPrintPreview_CLASSINFO() -> *mut c_void;
    pub fn wxPrintPreview_new(
        printout: *mut c_void,
        printout_for_printing: *mut c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxPrintPreview_new1(
        printout: *mut c_void,
        printout_for_printing: *mut c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPrintPreview_~wxPrintPreview(self_: *mut c_void);
    pub fn wxPrintPreview_GetCanvas(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintPreview_GetCurrentPage(self_: *const c_void) -> c_int;
    pub fn wxPrintPreview_GetFrame(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintPreview_GetMaxPage(self_: *const c_void) -> c_int;
    pub fn wxPrintPreview_GetMinPage(self_: *const c_void) -> c_int;
    pub fn wxPrintPreview_GetPrintout(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintPreview_GetPrintoutForPrinting(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintPreview_IsOk(self_: *const c_void) -> bool;
    pub fn wxPrintPreview_PaintPage(
        self_: *mut c_void,
        canvas: *mut c_void,
        dc: *mut c_void,
    ) -> bool;
    pub fn wxPrintPreview_Print(self_: *mut c_void, prompt: bool) -> bool;
    pub fn wxPrintPreview_RenderPage(self_: *mut c_void, page_num: c_int) -> bool;
    pub fn wxPrintPreview_SetCanvas(self_: *mut c_void, window: *mut c_void);
    pub fn wxPrintPreview_SetCurrentPage(self_: *mut c_void, page_num: c_int) -> bool;
    pub fn wxPrintPreview_SetFrame(self_: *mut c_void, frame: *mut c_void);
    pub fn wxPrintPreview_SetPrintout(self_: *mut c_void, printout: *mut c_void);
    pub fn wxPrintPreview_SetZoom(self_: *mut c_void, percent: c_int);

    // wxPrinterDC
    pub fn wxPrinterDC_CLASSINFO() -> *mut c_void;
    pub fn wxPrinterDC_new(print_data: *const c_void) -> *mut c_void;
    pub fn wxPrinterDC_GetPaperRect(self_: *const c_void) -> *mut c_void;

    // wxQuantize
    pub fn wxQuantize_CLASSINFO() -> *mut c_void;
    pub fn wxQuantize_new() -> *mut c_void;
    pub fn wxQuantize_DoQuantize(
        w: c_uint,
        h: c_uint,
        in_rows: *mut c_void,
        out_rows: *mut c_void,
        palette: *mut c_void,
        desired_no_colours: c_int,
    );
    pub fn wxQuantize_Quantize(
        src: *const c_void,
        dest: *mut c_void,
        p_palette: *mut c_void,
        desired_no_colours: c_int,
        eight_bit_data: *mut c_void,
        flags: c_int,
    ) -> bool;
    pub fn wxQuantize_Quantize1(
        src: *const c_void,
        dest: *mut c_void,
        desired_no_colours: c_int,
        eight_bit_data: *mut c_void,
        flags: c_int,
    ) -> bool;

    // wxQueryLayoutInfoEvent
    pub fn wxQueryLayoutInfoEvent_CLASSINFO() -> *mut c_void;
    pub fn wxQueryLayoutInfoEvent_new(id: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxQueryLayoutInfoEvent_GetAlignment(self_: *const c_void) -> wxLayoutAlignment;
    pub fn wxQueryLayoutInfoEvent_GetFlags(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxQueryLayoutInfoEvent_GetOrientation(self_: *const c_void) -> wxLayoutOrientation;
    pub fn wxQueryLayoutInfoEvent_GetRequestedLength(self_: *const c_void) -> c_int;
    pub fn wxQueryLayoutInfoEvent_GetSize(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxQueryLayoutInfoEvent_SetAlignment(self_: *mut c_void, alignment: wxLayoutAlignment);
    pub fn wxQueryLayoutInfoEvent_SetFlags(self_: *mut c_void, flags: c_int);
    // NOT_SUPPORTED: pub fn wxQueryLayoutInfoEvent_SetOrientation(self_: *mut c_void, orientation: wxLayoutOrientation);
    pub fn wxQueryLayoutInfoEvent_SetRequestedLength(self_: *mut c_void, length: c_int);
    pub fn wxQueryLayoutInfoEvent_SetSize(self_: *mut c_void, size: *const c_void);

    // wxRadioBox
    pub fn wxRadioBox_CLASSINFO() -> *mut c_void;
    pub fn wxRadioBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxRadioBox_new1(parent: *mut c_void, id: c_int, label: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, major_dimension: c_int, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxRadioBox_new2(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        major_dimension: c_int,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxRadioBox_~wxRadioBox(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxRadioBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, label: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, major_dimension: c_int, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxRadioBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        major_dimension: c_int,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxRadioBox_Enable(self_: *mut c_void, n: c_uint, enable: bool) -> bool;
    pub fn wxRadioBox_GetColumnCount(self_: *const c_void) -> c_uint;
    pub fn wxRadioBox_GetItemFromPoint(self_: *const c_void, pt: *const c_void) -> c_int;
    pub fn wxRadioBox_GetItemHelpText(self_: *const c_void, item: c_uint) -> *mut c_void;
    pub fn wxRadioBox_GetItemToolTip(self_: *const c_void, item: c_uint) -> *mut c_void;
    pub fn wxRadioBox_GetRowCount(self_: *const c_void) -> c_uint;
    pub fn wxRadioBox_IsItemEnabled(self_: *const c_void, n: c_uint) -> bool;
    pub fn wxRadioBox_IsItemShown(self_: *const c_void, n: c_uint) -> bool;
    pub fn wxRadioBox_SetItemHelpText(self_: *mut c_void, item: c_uint, helptext: *const c_void);
    pub fn wxRadioBox_SetItemToolTip(self_: *mut c_void, item: c_uint, text: *const c_void);
    pub fn wxRadioBox_Show(self_: *mut c_void, item: c_uint, show: bool) -> bool;
    // Mix-in(s) to wxRadioBox
    pub fn wxRadioBox_AsItemContainerImmutable(obj: *mut c_void) -> *mut c_void;

    // wxRadioButton
    pub fn wxRadioButton_CLASSINFO() -> *mut c_void;
    pub fn wxRadioButton_new() -> *mut c_void;
    pub fn wxRadioButton_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxRadioButton_~wxRadioButton(self_: *mut c_void);
    pub fn wxRadioButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxRadioButton_GetValue(self_: *const c_void) -> bool;
    pub fn wxRadioButton_SetValue(self_: *mut c_void, value: bool);
    pub fn wxRadioButton_GetFirstInGroup(self_: *const c_void) -> *mut c_void;
    pub fn wxRadioButton_GetLastInGroup(self_: *const c_void) -> *mut c_void;
    pub fn wxRadioButton_GetPreviousInGroup(self_: *const c_void) -> *mut c_void;
    pub fn wxRadioButton_GetNextInGroup(self_: *const c_void) -> *mut c_void;

    // wxRealPoint
    pub fn wxRealPoint_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxRealPoint_operator=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator==(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxRealPoint_operator!=(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxRealPoint_operator+(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator-(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator+=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator-=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator+1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator-1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator+2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator-2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator+=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator-=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxRealPoint_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxRealPoint_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxRealPoint_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    pub fn wxRealPoint_new() -> *mut c_void;
    pub fn wxRealPoint_new1(x: c_double, y: c_double) -> *mut c_void;
    pub fn wxRealPoint_new2(pt: *const c_void) -> *mut c_void;

    // wxRearrangeCtrl
    pub fn wxRearrangeCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxRearrangeCtrl_new() -> *mut c_void;
    pub fn wxRearrangeCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        order: *const c_void,
        items: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxRearrangeCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        order: *const c_void,
        items: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxRearrangeCtrl_GetList(self_: *const c_void) -> *mut c_void;

    // wxRearrangeList
    pub fn wxRearrangeList_CLASSINFO() -> *mut c_void;
    pub fn wxRearrangeList_new() -> *mut c_void;
    pub fn wxRearrangeList_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        order: *const c_void,
        items: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxRearrangeList_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        order: *const c_void,
        items: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxRearrangeList_GetCurrentOrder(self_: *const c_void) -> *mut c_void;
    pub fn wxRearrangeList_CanMoveCurrentUp(self_: *const c_void) -> bool;
    pub fn wxRearrangeList_CanMoveCurrentDown(self_: *const c_void) -> bool;
    pub fn wxRearrangeList_MoveCurrentUp(self_: *mut c_void) -> bool;
    pub fn wxRearrangeList_MoveCurrentDown(self_: *mut c_void) -> bool;
    // Mix-in(s) to wxRearrangeList
    pub fn wxRearrangeList_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxRect
    pub fn wxRect_delete(self_: *mut c_void);
    pub fn wxRect_new() -> *mut c_void;
    pub fn wxRect_new1(x: c_int, y: c_int, width: c_int, height: c_int) -> *mut c_void;
    pub fn wxRect_new2(top_left: *const c_void, bottom_right: *const c_void) -> *mut c_void;
    pub fn wxRect_new3(pos: *const c_void, size: *const c_void) -> *mut c_void;
    pub fn wxRect_new4(size: *const c_void) -> *mut c_void;
    pub fn wxRect_CentreIn(self_: *const c_void, r: *const c_void, dir: c_int) -> *mut c_void;
    pub fn wxRect_CenterIn(self_: *const c_void, r: *const c_void, dir: c_int) -> *mut c_void;
    pub fn wxRect_Contains(self_: *const c_void, x: c_int, y: c_int) -> bool;
    pub fn wxRect_Contains1(self_: *const c_void, pt: *const c_void) -> bool;
    pub fn wxRect_Contains2(self_: *const c_void, rect: *const c_void) -> bool;
    // BLOCKED: pub fn wxRect_Deflate(self_: *mut c_void, dx: c_int, dy: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Deflate1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Deflate2(self_: *mut c_void, diff: c_int) -> *mut c_void;
    pub fn wxRect_Deflate3(self_: *const c_void, dx: c_int, dy: c_int) -> *mut c_void;
    pub fn wxRect_GetBottom(self_: *const c_void) -> c_int;
    pub fn wxRect_GetBottomLeft(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetBottomRight(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxRect_GetLeft(self_: *const c_void) -> c_int;
    pub fn wxRect_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetRight(self_: *const c_void) -> c_int;
    pub fn wxRect_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetTop(self_: *const c_void) -> c_int;
    pub fn wxRect_GetTopLeft(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetTopRight(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxRect_GetX(self_: *const c_void) -> c_int;
    pub fn wxRect_GetY(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxRect_Inflate(self_: *mut c_void, dx: c_int, dy: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Inflate1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Inflate2(self_: *mut c_void, diff: c_int) -> *mut c_void;
    pub fn wxRect_Inflate3(self_: *const c_void, dx: c_int, dy: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Intersect(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxRect_Intersect1(self_: *const c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxRect_Intersects(self_: *const c_void, rect: *const c_void) -> bool;
    pub fn wxRect_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxRect_Offset(self_: *mut c_void, dx: c_int, dy: c_int);
    pub fn wxRect_Offset1(self_: *mut c_void, pt: *const c_void);
    pub fn wxRect_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxRect_SetPosition(self_: *mut c_void, pos: *const c_void);
    pub fn wxRect_SetSize(self_: *mut c_void, s: *const c_void);
    pub fn wxRect_SetWidth(self_: *mut c_void, width: c_int);
    pub fn wxRect_SetX(self_: *mut c_void, x: c_int);
    pub fn wxRect_SetY(self_: *mut c_void, y: c_int);
    pub fn wxRect_SetLeft(self_: *mut c_void, left: c_int);
    pub fn wxRect_SetRight(self_: *mut c_void, right: c_int);
    pub fn wxRect_SetTop(self_: *mut c_void, top: c_int);
    pub fn wxRect_SetBottom(self_: *mut c_void, bottom: c_int);
    pub fn wxRect_SetTopLeft(self_: *mut c_void, p: *const c_void);
    pub fn wxRect_SetBottomRight(self_: *mut c_void, p: *const c_void);
    pub fn wxRect_SetTopRight(self_: *mut c_void, p: *const c_void);
    pub fn wxRect_SetBottomLeft(self_: *mut c_void, p: *const c_void);
    pub fn wxRect_Union(self_: *const c_void, rect: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Union1(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_operator!=(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> bool;
    // BLOCKED: pub fn wxRect_operator+(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> wxRect;
    // BLOCKED: pub fn wxRect_operator+=(self_: *mut c_void, r: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_operator*(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> wxRect;
    // BLOCKED: pub fn wxRect_operator*=(self_: *mut c_void, r: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_operator=(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_operator==(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> bool;

    // wxRegionIterator
    pub fn wxRegionIterator_CLASSINFO() -> *mut c_void;
    pub fn wxRegionIterator_new() -> *mut c_void;
    pub fn wxRegionIterator_new1(region: *const c_void) -> *mut c_void;
    pub fn wxRegionIterator_GetH(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetRect(self_: *const c_void) -> *mut c_void;
    pub fn wxRegionIterator_GetW(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetX(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetY(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_HaveRects(self_: *const c_void) -> bool;
    pub fn wxRegionIterator_Reset(self_: *mut c_void);
    pub fn wxRegionIterator_Reset1(self_: *mut c_void, region: *const c_void);
    // BLOCKED: pub fn wxRegionIterator_operator++(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxRegionIterator_operator bool(self_: *const c_void);

    // wxRendererNative
    pub fn wxRendererNative_delete(self_: *mut c_void);
    // DTOR: pub fn wxRendererNative_~wxRendererNative(self_: *mut c_void);
    pub fn wxRendererNative_DrawCheckBox(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawComboBoxDropButton(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawDropArrow(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawFocusRect(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawGauge(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        value: c_int,
        max: c_int,
        flags: c_int,
    );
    // NOT_SUPPORTED: pub fn wxRendererNative_DrawHeaderButton(self_: *mut c_void, win: *mut c_void, dc: *mut c_void, rect: *const c_void, flags: c_int, sort_arrow: wxHeaderSortIconType, params: *mut c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxRendererNative_DrawHeaderButtonContents(self_: *mut c_void, win: *mut c_void, dc: *mut c_void, rect: *const c_void, flags: c_int, sort_arrow: wxHeaderSortIconType, params: *mut c_void) -> c_int;
    pub fn wxRendererNative_DrawItemSelectionRect(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawItemText(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        text: *const c_void,
        rect: *const c_void,
        align: c_int,
        flags: c_int,
        ellipsize_mode: c_int,
    );
    pub fn wxRendererNative_DrawPushButton(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawCollapseButton(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_GetCollapseButtonSize(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
    ) -> *mut c_void;
    pub fn wxRendererNative_DrawSplitterBorder(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    // NOT_SUPPORTED: pub fn wxRendererNative_DrawSplitterSash(self_: *mut c_void, win: *mut c_void, dc: *mut c_void, size: *const c_void, position: c_int, orient: wxOrientation, flags: c_int);
    pub fn wxRendererNative_DrawTreeItemButton(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawChoice(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawComboBox(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawTextCtrl(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawRadioBitmap(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    // NOT_SUPPORTED: pub fn wxRendererNative_DrawTitleBarBitmap(self_: *mut c_void, win: *mut c_void, dc: *mut c_void, rect: *const c_void, button: wxTitleBarButton, flags: c_int);
    pub fn wxRendererNative_DrawCheckMark(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_GetCheckBoxSize(
        self_: *mut c_void,
        win: *mut c_void,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxRendererNative_GetCheckMarkSize(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxRendererNative_GetExpanderSize(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxRendererNative_GetHeaderButtonHeight(self_: *mut c_void, win: *mut c_void) -> c_int;
    pub fn wxRendererNative_GetHeaderButtonMargin(self_: *mut c_void, win: *mut c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxRendererNative_GetSplitterParams(self_: *mut c_void, win: *const c_void) -> wxSplitterRenderParams;
    // NOT_SUPPORTED: pub fn wxRendererNative_GetVersion(self_: *const c_void) -> wxRendererVersion;
    pub fn wxRendererNative_Get() -> *mut c_void;
    pub fn wxRendererNative_GetDefault() -> *mut c_void;
    pub fn wxRendererNative_GetGeneric() -> *mut c_void;
    pub fn wxRendererNative_Load(name: *const c_void) -> *mut c_void;
    pub fn wxRendererNative_Set(renderer: *mut c_void) -> *mut c_void;

    // wxRichToolTip
    pub fn wxRichToolTip_delete(self_: *mut c_void);
    pub fn wxRichToolTip_new(title: *const c_void, message: *const c_void) -> *mut c_void;
    pub fn wxRichToolTip_SetBackgroundColour(
        self_: *mut c_void,
        col: *const c_void,
        col_end: *const c_void,
    );
    pub fn wxRichToolTip_SetIcon(self_: *mut c_void, icon: c_int);
    pub fn wxRichToolTip_SetIcon1(self_: *mut c_void, icon: *const c_void);
    // NOT_SUPPORTED: pub fn wxRichToolTip_SetTimeout(self_: *mut c_void, milliseconds_timeout: unsigned, milliseconds_delay: unsigned);
    // NOT_SUPPORTED: pub fn wxRichToolTip_SetTipKind(self_: *mut c_void, tip_kind: wxTipKind);
    pub fn wxRichToolTip_SetTitleFont(self_: *mut c_void, font: *const c_void);
    pub fn wxRichToolTip_ShowFor(self_: *mut c_void, win: *mut c_void, rect: *const c_void);
    // DTOR: pub fn wxRichToolTip_~wxRichToolTip(self_: *mut c_void);

    // wxRotateGestureEvent
    pub fn wxRotateGestureEvent_CLASSINFO() -> *mut c_void;
    pub fn wxRotateGestureEvent_new(windid: c_int) -> *mut c_void;
    pub fn wxRotateGestureEvent_GetRotationAngle(self_: *const c_void) -> c_double;
    pub fn wxRotateGestureEvent_SetRotationAngle(self_: *mut c_void, rotation_angle: c_double);

    // wxSVGBitmapEmbedHandler
    pub fn wxSVGBitmapEmbedHandler_delete(self_: *mut c_void);

    // wxSVGBitmapFileHandler
    pub fn wxSVGBitmapFileHandler_delete(self_: *mut c_void);
    pub fn wxSVGBitmapFileHandler_new(path: *const c_void) -> *mut c_void;

    // wxSVGBitmapHandler
    pub fn wxSVGBitmapHandler_delete(self_: *mut c_void);
    pub fn wxSVGBitmapHandler_ProcessBitmap(
        self_: *const c_void,
        bitmap: *const c_void,
        x: c_int,
        y: c_int,
        stream: *mut c_void,
    ) -> bool;

    // wxSVGFileDC
    pub fn wxSVGFileDC_CLASSINFO() -> *mut c_void;
    pub fn wxSVGFileDC_new(
        filename: *const c_void,
        width: c_int,
        height: c_int,
        dpi: c_double,
        title: *const c_void,
    ) -> *mut c_void;
    pub fn wxSVGFileDC_Clear(self_: *mut c_void);
    pub fn wxSVGFileDC_SetBitmapHandler(self_: *mut c_void, handler: *mut c_void);
    // NOT_SUPPORTED: pub fn wxSVGFileDC_SetShapeRenderingMode(self_: *mut c_void, rendering_mode: wxSVGShapeRenderingMode);
    pub fn wxSVGFileDC_DestroyClippingRegion(self_: *mut c_void);
    pub fn wxSVGFileDC_CrossHair(self_: *mut c_void, x: c_int, y: c_int);
    // NOT_SUPPORTED: pub fn wxSVGFileDC_FloodFill(self_: *mut c_void, x: c_int, y: c_int, colour: *const c_void, style: wxFloodFillStyle) -> bool;
    pub fn wxSVGFileDC_GetPixel(
        self_: *const c_void,
        x: c_int,
        y: c_int,
        colour: *mut c_void,
    ) -> bool;
    pub fn wxSVGFileDC_SetPalette(self_: *mut c_void, palette: *const c_void);
    pub fn wxSVGFileDC_GetDepth(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxSVGFileDC_SetLogicalFunction(self_: *mut c_void, function: wxRasterOperationMode);
    // NOT_SUPPORTED: pub fn wxSVGFileDC_GetLogicalFunction(self_: *const c_void) -> wxRasterOperationMode;
    pub fn wxSVGFileDC_StartDoc(self_: *mut c_void, message: *const c_void) -> bool;
    pub fn wxSVGFileDC_EndDoc(self_: *mut c_void);
    pub fn wxSVGFileDC_StartPage(self_: *mut c_void);
    pub fn wxSVGFileDC_EndPage(self_: *mut c_void);

    // wxSashEvent
    pub fn wxSashEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSashEvent_new(id: c_int, edge: wxSashEdgePosition) -> *mut c_void;
    pub fn wxSashEvent_GetDragRect(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSashEvent_GetDragStatus(self_: *const c_void) -> wxSashDragStatus;
    // NOT_SUPPORTED: pub fn wxSashEvent_GetEdge(self_: *const c_void) -> wxSashEdgePosition;
    // NOT_SUPPORTED: pub fn wxSashEvent_SetEdge(self_: *mut c_void, edge: wxSashEdgePosition);
    pub fn wxSashEvent_SetDragRect(self_: *mut c_void, rect: *const c_void);
    // NOT_SUPPORTED: pub fn wxSashEvent_SetDragStatus(self_: *mut c_void, status: wxSashDragStatus);

    // wxSashLayoutWindow
    pub fn wxSashLayoutWindow_CLASSINFO() -> *mut c_void;
    pub fn wxSashLayoutWindow_new() -> *mut c_void;
    pub fn wxSashLayoutWindow_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxSashLayoutWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxSashLayoutWindow_GetAlignment(self_: *const c_void) -> wxLayoutAlignment;
    // NOT_SUPPORTED: pub fn wxSashLayoutWindow_GetOrientation(self_: *const c_void) -> wxLayoutOrientation;
    pub fn wxSashLayoutWindow_OnCalculateLayout(self_: *mut c_void, event: *mut c_void);
    pub fn wxSashLayoutWindow_OnQueryLayoutInfo(self_: *mut c_void, event: *mut c_void);
    // NOT_SUPPORTED: pub fn wxSashLayoutWindow_SetAlignment(self_: *mut c_void, alignment: wxLayoutAlignment);
    pub fn wxSashLayoutWindow_SetDefaultSize(self_: *mut c_void, size: *const c_void);
    // NOT_SUPPORTED: pub fn wxSashLayoutWindow_SetOrientation(self_: *mut c_void, orientation: wxLayoutOrientation);

    // wxSashWindow
    pub fn wxSashWindow_CLASSINFO() -> *mut c_void;
    pub fn wxSashWindow_new() -> *mut c_void;
    pub fn wxSashWindow_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxSashWindow_~wxSashWindow(self_: *mut c_void);
    pub fn wxSashWindow_GetMaximumSizeX(self_: *const c_void) -> c_int;
    pub fn wxSashWindow_GetMaximumSizeY(self_: *const c_void) -> c_int;
    pub fn wxSashWindow_GetMinimumSizeX(self_: *const c_void) -> c_int;
    pub fn wxSashWindow_GetMinimumSizeY(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxSashWindow_GetSashVisible(self_: *const c_void, edge: wxSashEdgePosition) -> bool;
    pub fn wxSashWindow_SetMaximumSizeX(self_: *mut c_void, min: c_int);
    pub fn wxSashWindow_SetMaximumSizeY(self_: *mut c_void, min: c_int);
    pub fn wxSashWindow_SetMinimumSizeX(self_: *mut c_void, min: c_int);
    pub fn wxSashWindow_SetMinimumSizeY(self_: *mut c_void, min: c_int);
    // NOT_SUPPORTED: pub fn wxSashWindow_SetSashVisible(self_: *mut c_void, edge: wxSashEdgePosition, visible: bool);
    // NOT_SUPPORTED: pub fn wxSashWindow_GetEdgeMargin(self_: *const c_void, edge: wxSashEdgePosition) -> c_int;
    pub fn wxSashWindow_SetDefaultBorderSize(self_: *mut c_void, width: c_int);
    pub fn wxSashWindow_GetDefaultBorderSize(self_: *const c_void) -> c_int;
    pub fn wxSashWindow_SetExtraBorderSize(self_: *mut c_void, width: c_int);
    pub fn wxSashWindow_GetExtraBorderSize(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxSashWindow_SashHitTest(self_: *mut c_void, x: c_int, y: c_int, tolerance: c_int) -> wxSashEdgePosition;
    pub fn wxSashWindow_SizeWindows(self_: *mut c_void);

    // wxScreenDC
    pub fn wxScreenDC_CLASSINFO() -> *mut c_void;
    pub fn wxScreenDC_new() -> *mut c_void;
    pub fn wxScreenDC_EndDrawingOnTop() -> bool;
    pub fn wxScreenDC_StartDrawingOnTop(window: *mut c_void) -> bool;
    pub fn wxScreenDC_StartDrawingOnTop1(rect: *mut c_void) -> bool;

    // wxScrollBar
    pub fn wxScrollBar_CLASSINFO() -> *mut c_void;
    pub fn wxScrollBar_new() -> *mut c_void;
    pub fn wxScrollBar_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxScrollBar_~wxScrollBar(self_: *mut c_void);
    pub fn wxScrollBar_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxScrollBar_GetPageSize(self_: *const c_void) -> c_int;
    pub fn wxScrollBar_GetRange(self_: *const c_void) -> c_int;
    pub fn wxScrollBar_GetThumbPosition(self_: *const c_void) -> c_int;
    pub fn wxScrollBar_GetThumbSize(self_: *const c_void) -> c_int;
    pub fn wxScrollBar_SetThumbPosition(self_: *mut c_void, view_start: c_int);
    pub fn wxScrollBar_IsVertical(self_: *const c_void) -> bool;

    // wxScrollEvent
    pub fn wxScrollEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxScrollEvent_new(command_type: wxEventType, id: c_int, pos: c_int, orientation: c_int) -> *mut c_void;
    pub fn wxScrollEvent_GetOrientation(self_: *const c_void) -> c_int;
    pub fn wxScrollEvent_GetPosition(self_: *const c_void) -> c_int;
    pub fn wxScrollEvent_SetOrientation(self_: *mut c_void, orient: c_int);
    pub fn wxScrollEvent_SetPosition(self_: *mut c_void, pos: c_int);

    // wxScrollWinEvent
    pub fn wxScrollWinEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxScrollWinEvent_new(command_type: wxEventType, pos: c_int, orientation: c_int) -> *mut c_void;
    pub fn wxScrollWinEvent_GetOrientation(self_: *const c_void) -> c_int;
    pub fn wxScrollWinEvent_GetPosition(self_: *const c_void) -> c_int;
    pub fn wxScrollWinEvent_SetOrientation(self_: *mut c_void, orient: c_int);
    pub fn wxScrollWinEvent_SetPosition(self_: *mut c_void, pos: c_int);

    // wxSearchCtrl
    pub fn wxSearchCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxSearchCtrl_new() -> *mut c_void;
    pub fn wxSearchCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxSearchCtrl_~wxSearchCtrl(self_: *mut c_void);
    pub fn wxSearchCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxSearchCtrl_GetMenu(self_: *mut c_void) -> *mut c_void;
    pub fn wxSearchCtrl_IsSearchButtonVisible(self_: *const c_void) -> bool;
    pub fn wxSearchCtrl_IsCancelButtonVisible(self_: *const c_void) -> bool;
    pub fn wxSearchCtrl_SetMenu(self_: *mut c_void, menu: *mut c_void);
    pub fn wxSearchCtrl_ShowCancelButton(self_: *mut c_void, show: bool);
    pub fn wxSearchCtrl_ShowSearchButton(self_: *mut c_void, show: bool);
    pub fn wxSearchCtrl_SetDescriptiveText(self_: *mut c_void, text: *const c_void);
    pub fn wxSearchCtrl_GetDescriptiveText(self_: *const c_void) -> *mut c_void;
    // Mix-in(s) to wxSearchCtrl
    pub fn wxSearchCtrl_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxSetCursorEvent
    pub fn wxSetCursorEvent_CLASSINFO() -> *mut c_void;
    pub fn wxSetCursorEvent_new(x: c_int, y: c_int) -> *mut c_void;
    pub fn wxSetCursorEvent_GetCursor(self_: *const c_void) -> *mut c_void;
    pub fn wxSetCursorEvent_GetX(self_: *const c_void) -> c_int;
    pub fn wxSetCursorEvent_GetY(self_: *const c_void) -> c_int;
    pub fn wxSetCursorEvent_HasCursor(self_: *const c_void) -> bool;
    pub fn wxSetCursorEvent_SetCursor(self_: *mut c_void, cursor: *const c_void);

    // wxSettableHeaderColumn
    pub fn wxSettableHeaderColumn_delete(self_: *mut c_void);
    pub fn wxSettableHeaderColumn_SetTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxSettableHeaderColumn_SetBitmap(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxSettableHeaderColumn_SetWidth(self_: *mut c_void, width: c_int);
    pub fn wxSettableHeaderColumn_SetMinWidth(self_: *mut c_void, min_width: c_int);
    pub fn wxSettableHeaderColumn_SetAlignment(self_: *mut c_void, align: c_int);
    pub fn wxSettableHeaderColumn_SetFlags(self_: *mut c_void, flags: c_int);
    pub fn wxSettableHeaderColumn_ChangeFlag(self_: *mut c_void, flag: c_int, set: bool);
    pub fn wxSettableHeaderColumn_SetFlag(self_: *mut c_void, flag: c_int);
    pub fn wxSettableHeaderColumn_ClearFlag(self_: *mut c_void, flag: c_int);
    pub fn wxSettableHeaderColumn_ToggleFlag(self_: *mut c_void, flag: c_int);
    pub fn wxSettableHeaderColumn_SetResizeable(self_: *mut c_void, resizable: bool);
    pub fn wxSettableHeaderColumn_SetSortable(self_: *mut c_void, sortable: bool);
    pub fn wxSettableHeaderColumn_SetReorderable(self_: *mut c_void, reorderable: bool);
    pub fn wxSettableHeaderColumn_SetHidden(self_: *mut c_void, hidden: bool);
    pub fn wxSettableHeaderColumn_UnsetAsSortKey(self_: *mut c_void);
    pub fn wxSettableHeaderColumn_SetSortOrder(self_: *mut c_void, ascending: bool);
    pub fn wxSettableHeaderColumn_ToggleSortOrder(self_: *mut c_void);

    // wxSimpleHelpProvider
    pub fn wxSimpleHelpProvider_delete(self_: *mut c_void);

    // wxSimplebook
    pub fn wxSimplebook_CLASSINFO() -> *mut c_void;
    pub fn wxSimplebook_new() -> *mut c_void;
    pub fn wxSimplebook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxSimplebook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxSimplebook_SetEffects(self_: *mut c_void, show_effect: wxShowEffect, hide_effect: wxShowEffect);
    // NOT_SUPPORTED: pub fn wxSimplebook_SetEffect(self_: *mut c_void, effect: wxShowEffect);
    // NOT_SUPPORTED: pub fn wxSimplebook_SetEffectsTimeouts(self_: *mut c_void, show_timeout: unsigned, hide_timeout: unsigned);
    // NOT_SUPPORTED: pub fn wxSimplebook_SetEffectTimeout(self_: *mut c_void, timeout: unsigned);
    pub fn wxSimplebook_ShowNewPage(self_: *mut c_void, page: *mut c_void) -> bool;
    // Mix-in(s) to wxSimplebook
    pub fn wxSimplebook_AsWithImages(obj: *mut c_void) -> *mut c_void;

    // wxSize
    pub fn wxSize_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxSize_operator=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator==(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> bool;
    // BLOCKED: pub fn wxSize_operator!=(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> bool;
    // BLOCKED: pub fn wxSize_operator+(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator-(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator+=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator-=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxSize_operator/1(self_: *mut c_void, sz: *const c_void, factor: c_double) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*1(self_: *mut c_void, sz: *const c_void, factor: c_double) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*2(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*3(self_: *mut c_void, factor: c_double, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator/=1(self_: *mut c_void, factor: c_double) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator*=1(self_: *mut c_void, factor: c_double) -> *mut c_void;
    pub fn wxSize_new() -> *mut c_void;
    pub fn wxSize_new1(width: c_int, height: c_int) -> *mut c_void;
    pub fn wxSize_DecBy(self_: *mut c_void, pt: *const c_void);
    pub fn wxSize_DecBy1(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_DecBy2(self_: *mut c_void, dx: c_int, dy: c_int);
    pub fn wxSize_DecBy3(self_: *mut c_void, d: c_int);
    pub fn wxSize_DecTo(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_DecToIfSpecified(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxSize_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxSize_IncBy(self_: *mut c_void, pt: *const c_void);
    pub fn wxSize_IncBy1(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_IncBy2(self_: *mut c_void, dx: c_int, dy: c_int);
    pub fn wxSize_IncBy3(self_: *mut c_void, d: c_int);
    pub fn wxSize_IncTo(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_IsFullySpecified(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxSize_Scale(self_: *mut c_void, xscale: c_double, yscale: c_double) -> *mut c_void;
    pub fn wxSize_Set(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxSize_SetDefaults(self_: *mut c_void, size_default: *const c_void);
    pub fn wxSize_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxSize_SetWidth(self_: *mut c_void, width: c_int);

    // wxSizeEvent
    pub fn wxSizeEvent_CLASSINFO() -> *mut c_void;
    pub fn wxSizeEvent_new(sz: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxSizeEvent_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxSizeEvent_SetSize(self_: *mut c_void, size: wxSize);
    pub fn wxSizeEvent_GetRect(self_: *const c_void) -> *mut c_void;
    pub fn wxSizeEvent_SetRect(self_: *mut c_void, rect: wxRect);

    // wxSizer
    pub fn wxSizer_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxSizer_new() -> *mut c_void;
    // DTOR: pub fn wxSizer_~wxSizer(self_: *mut c_void);
    pub fn wxSizer_Add(
        self_: *mut c_void,
        window: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add1(
        self_: *mut c_void,
        window: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add2(
        self_: *mut c_void,
        sizer: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add3(
        self_: *mut c_void,
        sizer: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add4(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add5(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add6(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxSizer_AddSpacer(self_: *mut c_void, size: c_int) -> *mut c_void;
    pub fn wxSizer_AddStretchSpacer(self_: *mut c_void, prop: c_int) -> *mut c_void;
    pub fn wxSizer_CalcMin(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizer_Clear(self_: *mut c_void, delete_windows: bool);
    pub fn wxSizer_ComputeFittingClientSize(self_: *mut c_void, window: *mut c_void)
        -> *mut c_void;
    pub fn wxSizer_ComputeFittingWindowSize(self_: *mut c_void, window: *mut c_void)
        -> *mut c_void;
    pub fn wxSizer_Detach(self_: *mut c_void, window: *mut c_void) -> bool;
    pub fn wxSizer_Detach1(self_: *mut c_void, sizer: *mut c_void) -> bool;
    pub fn wxSizer_Detach2(self_: *mut c_void, index: c_int) -> bool;
    pub fn wxSizer_Fit(self_: *mut c_void, window: *mut c_void) -> *mut c_void;
    pub fn wxSizer_FitInside(self_: *mut c_void, window: *mut c_void);
    pub fn wxSizer_InformFirstDirection(
        self_: *mut c_void,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool;
    pub fn wxSizer_GetChildren(self_: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSizer_GetChildren1(self_: *const c_void) -> *mut c_void;
    pub fn wxSizer_GetContainingWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxSizer_SetContainingWindow(self_: *mut c_void, window: *mut c_void);
    pub fn wxSizer_GetItemCount(self_: *const c_void) -> usize;
    pub fn wxSizer_GetItem(self_: *mut c_void, window: *mut c_void, recursive: bool)
        -> *mut c_void;
    pub fn wxSizer_GetItem1(self_: *mut c_void, sizer: *mut c_void, recursive: bool)
        -> *mut c_void;
    pub fn wxSizer_GetItem2(self_: *mut c_void, index: usize) -> *mut c_void;
    pub fn wxSizer_GetItemById(self_: *mut c_void, id: c_int, recursive: bool) -> *mut c_void;
    pub fn wxSizer_GetMinSize(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizer_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxSizer_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxSizer_Hide(self_: *mut c_void, window: *mut c_void, recursive: bool) -> bool;
    pub fn wxSizer_Hide1(self_: *mut c_void, sizer: *mut c_void, recursive: bool) -> bool;
    pub fn wxSizer_Hide2(self_: *mut c_void, index: usize) -> bool;
    pub fn wxSizer_Insert(
        self_: *mut c_void,
        index: usize,
        window: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert1(
        self_: *mut c_void,
        index: usize,
        window: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert2(
        self_: *mut c_void,
        index: usize,
        sizer: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert3(
        self_: *mut c_void,
        index: usize,
        sizer: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert4(
        self_: *mut c_void,
        index: usize,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert5(
        self_: *mut c_void,
        index: usize,
        width: c_int,
        height: c_int,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert6(self_: *mut c_void, index: usize, item: *mut c_void) -> *mut c_void;
    pub fn wxSizer_InsertSpacer(self_: *mut c_void, index: usize, size: c_int) -> *mut c_void;
    pub fn wxSizer_InsertStretchSpacer(
        self_: *mut c_void,
        index: usize,
        prop: c_int,
    ) -> *mut c_void;
    pub fn wxSizer_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxSizer_IsShown(self_: *const c_void, window: *mut c_void) -> bool;
    pub fn wxSizer_IsShown1(self_: *const c_void, sizer: *mut c_void) -> bool;
    pub fn wxSizer_IsShown2(self_: *const c_void, index: usize) -> bool;
    pub fn wxSizer_Layout(self_: *mut c_void);
    pub fn wxSizer_Prepend(
        self_: *mut c_void,
        window: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend1(
        self_: *mut c_void,
        window: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend2(
        self_: *mut c_void,
        sizer: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend3(
        self_: *mut c_void,
        sizer: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend4(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend5(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend6(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxSizer_PrependSpacer(self_: *mut c_void, size: c_int) -> *mut c_void;
    pub fn wxSizer_PrependStretchSpacer(self_: *mut c_void, prop: c_int) -> *mut c_void;
    pub fn wxSizer_RepositionChildren(self_: *mut c_void, min_size: *const c_void);
    // BLOCKED: pub fn wxSizer_Remove(self_: *mut c_void, window: *mut c_void) -> bool;
    pub fn wxSizer_Remove1(self_: *mut c_void, sizer: *mut c_void) -> bool;
    pub fn wxSizer_Remove2(self_: *mut c_void, index: c_int) -> bool;
    pub fn wxSizer_Replace(
        self_: *mut c_void,
        oldwin: *mut c_void,
        newwin: *mut c_void,
        recursive: bool,
    ) -> bool;
    pub fn wxSizer_Replace1(
        self_: *mut c_void,
        oldsz: *mut c_void,
        newsz: *mut c_void,
        recursive: bool,
    ) -> bool;
    pub fn wxSizer_Replace2(self_: *mut c_void, index: usize, newitem: *mut c_void) -> bool;
    pub fn wxSizer_SetDimension(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn wxSizer_SetDimension1(self_: *mut c_void, pos: *const c_void, size: *const c_void);
    pub fn wxSizer_SetItemMinSize(
        self_: *mut c_void,
        window: *mut c_void,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize1(
        self_: *mut c_void,
        window: *mut c_void,
        size: *const c_void,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize2(
        self_: *mut c_void,
        sizer: *mut c_void,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize3(
        self_: *mut c_void,
        sizer: *mut c_void,
        size: *const c_void,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize4(
        self_: *mut c_void,
        index: usize,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize5(self_: *mut c_void, index: usize, size: *const c_void) -> bool;
    pub fn wxSizer_SetMinSize(self_: *mut c_void, size: *const c_void);
    pub fn wxSizer_SetMinSize1(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxSizer_SetSizeHints(self_: *mut c_void, window: *mut c_void);
    // BLOCKED: pub fn wxSizer_SetVirtualSizeHints(self_: *mut c_void, window: *mut c_void);
    pub fn wxSizer_Show(
        self_: *mut c_void,
        window: *mut c_void,
        show: bool,
        recursive: bool,
    ) -> bool;
    pub fn wxSizer_Show1(
        self_: *mut c_void,
        sizer: *mut c_void,
        show: bool,
        recursive: bool,
    ) -> bool;
    pub fn wxSizer_Show2(self_: *mut c_void, index: usize, show: bool) -> bool;
    pub fn wxSizer_ShowItems(self_: *mut c_void, show: bool);

    // wxSizerFlags
    pub fn wxSizerFlags_delete(self_: *mut c_void);
    pub fn wxSizerFlags_new(proportion: c_int) -> *mut c_void;
    pub fn wxSizerFlags_Align(self_: *mut c_void, alignment: c_int) -> *mut c_void;
    pub fn wxSizerFlags_Border(
        self_: *mut c_void,
        direction: c_int,
        borderinpixels: c_int,
    ) -> *mut c_void;
    pub fn wxSizerFlags_Border1(self_: *mut c_void, direction: c_int) -> *mut c_void;
    pub fn wxSizerFlags_Bottom(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Center(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Centre(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_CenterHorizontal(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_CenterVertical(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_CentreHorizontal(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_CentreVertical(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_DoubleBorder(self_: *mut c_void, direction: c_int) -> *mut c_void;
    pub fn wxSizerFlags_DoubleHorzBorder(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Expand(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_FixedMinSize(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_ReserveSpaceEvenIfHidden(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Left(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Proportion(self_: *mut c_void, proportion: c_int) -> *mut c_void;
    pub fn wxSizerFlags_Right(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Shaped(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Top(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_TripleBorder(self_: *mut c_void, direction: c_int) -> *mut c_void;
    pub fn wxSizerFlags_DisableConsistencyChecks();
    pub fn wxSizerFlags_GetDefaultBorder() -> c_int;
    // NOT_SUPPORTED: pub fn wxSizerFlags_GetDefaultBorderFractional() -> float;

    // wxSlider
    pub fn wxSlider_CLASSINFO() -> *mut c_void;
    pub fn wxSlider_new() -> *mut c_void;
    pub fn wxSlider_new1(
        parent: *mut c_void,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxSlider_~wxSlider(self_: *mut c_void);
    pub fn wxSlider_ClearSel(self_: *mut c_void);
    pub fn wxSlider_ClearTicks(self_: *mut c_void);
    pub fn wxSlider_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        point: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxSlider_GetLineSize(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetMax(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetMin(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetPageSize(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetSelEnd(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetSelStart(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetThumbLength(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetTickFreq(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetValue(self_: *const c_void) -> c_int;
    pub fn wxSlider_SetLineSize(self_: *mut c_void, line_size: c_int);
    pub fn wxSlider_SetMin(self_: *mut c_void, min_value: c_int);
    pub fn wxSlider_SetMax(self_: *mut c_void, max_value: c_int);
    pub fn wxSlider_SetPageSize(self_: *mut c_void, page_size: c_int);
    pub fn wxSlider_SetRange(self_: *mut c_void, min_value: c_int, max_value: c_int);
    pub fn wxSlider_SetSelection(self_: *mut c_void, start_pos: c_int, end_pos: c_int);
    pub fn wxSlider_SetThumbLength(self_: *mut c_void, len: c_int);
    pub fn wxSlider_SetTick(self_: *mut c_void, tick_pos: c_int);
    pub fn wxSlider_SetTickFreq(self_: *mut c_void, freq: c_int);
    pub fn wxSlider_SetValue(self_: *mut c_void, value: c_int);

    // wxSound
    pub fn wxSound_CLASSINFO() -> *mut c_void;
    pub fn wxSound_new() -> *mut c_void;
    pub fn wxSound_new1(file_name: *const c_void, is_resource: bool) -> *mut c_void;
    pub fn wxSound_new2(size: usize, data: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxSound_~wxSound(self_: *mut c_void);
    pub fn wxSound_Create(self_: *mut c_void, file_name: *const c_void, is_resource: bool) -> bool;
    pub fn wxSound_Create1(self_: *mut c_void, size: usize, data: *const c_void) -> bool;
    pub fn wxSound_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxSound_Play(self_: *const c_void, flags: unsigned) -> bool;
    pub fn wxSound_IsPlaying() -> bool;
    // NOT_SUPPORTED: pub fn wxSound_Play1(filename: *const c_void, flags: unsigned) -> bool;
    pub fn wxSound_Stop();

    // wxSpinButton
    pub fn wxSpinButton_CLASSINFO() -> *mut c_void;
    pub fn wxSpinButton_new() -> *mut c_void;
    pub fn wxSpinButton_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxSpinButton_~wxSpinButton(self_: *mut c_void);
    pub fn wxSpinButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxSpinButton_GetIncrement(self_: *const c_void) -> c_int;
    pub fn wxSpinButton_GetMax(self_: *const c_void) -> c_int;
    pub fn wxSpinButton_GetMin(self_: *const c_void) -> c_int;
    pub fn wxSpinButton_GetValue(self_: *const c_void) -> c_int;
    pub fn wxSpinButton_SetIncrement(self_: *mut c_void, value: c_int);
    pub fn wxSpinButton_SetRange(self_: *mut c_void, min: c_int, max: c_int);
    pub fn wxSpinButton_SetValue(self_: *mut c_void, value: c_int);

    // wxSpinCtrl
    pub fn wxSpinCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxSpinCtrl_new() -> *mut c_void;
    pub fn wxSpinCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        min: c_int,
        max: c_int,
        initial: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxSpinCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        min: c_int,
        max: c_int,
        initial: c_int,
        name: *const c_void,
    ) -> bool;
    pub fn wxSpinCtrl_GetBase(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_GetMax(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_GetMin(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_GetTextValue(self_: *const c_void) -> *mut c_void;
    pub fn wxSpinCtrl_GetValue(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_GetIncrement(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_SetBase(self_: *mut c_void, base: c_int) -> bool;
    pub fn wxSpinCtrl_SetRange(self_: *mut c_void, min_val: c_int, max_val: c_int);
    pub fn wxSpinCtrl_SetSelection(self_: *mut c_void, from: c_long, to: c_long);
    pub fn wxSpinCtrl_SetValue(self_: *mut c_void, text: *const c_void);
    pub fn wxSpinCtrl_SetValue1(self_: *mut c_void, value: c_int);
    pub fn wxSpinCtrl_SetIncrement(self_: *mut c_void, value: c_int);

    // wxSpinCtrlDouble
    pub fn wxSpinCtrlDouble_CLASSINFO() -> *mut c_void;
    pub fn wxSpinCtrlDouble_new() -> *mut c_void;
    pub fn wxSpinCtrlDouble_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        min: c_double,
        max: c_double,
        initial: c_double,
        inc: c_double,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxSpinCtrlDouble_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        min: c_double,
        max: c_double,
        initial: c_double,
        inc: c_double,
        name: *const c_void,
    ) -> bool;
    pub fn wxSpinCtrlDouble_GetDigits(self_: *const c_void) -> c_uint;
    pub fn wxSpinCtrlDouble_GetIncrement(self_: *const c_void) -> c_double;
    pub fn wxSpinCtrlDouble_GetMax(self_: *const c_void) -> c_double;
    pub fn wxSpinCtrlDouble_GetMin(self_: *const c_void) -> c_double;
    pub fn wxSpinCtrlDouble_GetTextValue(self_: *const c_void) -> *mut c_void;
    pub fn wxSpinCtrlDouble_GetValue(self_: *const c_void) -> c_double;
    pub fn wxSpinCtrlDouble_SetDigits(self_: *mut c_void, digits: c_uint);
    pub fn wxSpinCtrlDouble_SetIncrement(self_: *mut c_void, inc: c_double);
    pub fn wxSpinCtrlDouble_SetRange(self_: *mut c_void, min_val: c_double, max_val: c_double);
    pub fn wxSpinCtrlDouble_SetValue(self_: *mut c_void, text: *const c_void);
    pub fn wxSpinCtrlDouble_SetValue1(self_: *mut c_void, value: c_double);

    // wxSpinDoubleEvent
    pub fn wxSpinDoubleEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSpinDoubleEvent_new(command_type: wxEventType, winid: c_int, value: c_double) -> *mut c_void;
    pub fn wxSpinDoubleEvent_new1(event: *const c_void) -> *mut c_void;
    pub fn wxSpinDoubleEvent_GetValue(self_: *const c_void) -> c_double;
    pub fn wxSpinDoubleEvent_SetValue(self_: *mut c_void, value: c_double);

    // wxSpinEvent
    pub fn wxSpinEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSpinEvent_new(command_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxSpinEvent_GetPosition(self_: *const c_void) -> c_int;
    pub fn wxSpinEvent_SetPosition(self_: *mut c_void, pos: c_int);

    // wxSplashScreen
    pub fn wxSplashScreen_CLASSINFO() -> *mut c_void;
    pub fn wxSplashScreen_new(
        bitmap: *const c_void,
        splash_style: c_long,
        milliseconds: c_int,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
    ) -> *mut c_void;
    // DTOR: pub fn wxSplashScreen_~wxSplashScreen(self_: *mut c_void);
    pub fn wxSplashScreen_GetSplashStyle(self_: *const c_void) -> c_long;
    pub fn wxSplashScreen_GetSplashWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxSplashScreen_GetTimeout(self_: *const c_void) -> c_int;
    pub fn wxSplashScreen_OnCloseWindow(self_: *mut c_void, event: *mut c_void);

    // wxSplitterEvent
    pub fn wxSplitterEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSplitterEvent_new(event_type: wxEventType, splitter: *mut c_void) -> *mut c_void;
    pub fn wxSplitterEvent_GetSashPosition(self_: *const c_void) -> c_int;
    pub fn wxSplitterEvent_GetWindowBeingRemoved(self_: *const c_void) -> *mut c_void;
    pub fn wxSplitterEvent_GetX(self_: *const c_void) -> c_int;
    pub fn wxSplitterEvent_GetY(self_: *const c_void) -> c_int;
    pub fn wxSplitterEvent_SetSashPosition(self_: *mut c_void, pos: c_int);
    pub fn wxSplitterEvent_SetSize(self_: *mut c_void, old_size: c_int, new_size: c_int);
    pub fn wxSplitterEvent_GetOldSize(self_: *const c_void) -> c_int;

    // wxSplitterWindow
    pub fn wxSplitterWindow_CLASSINFO() -> *mut c_void;
    pub fn wxSplitterWindow_new() -> *mut c_void;
    pub fn wxSplitterWindow_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxSplitterWindow_~wxSplitterWindow(self_: *mut c_void);
    pub fn wxSplitterWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        point: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxSplitterWindow_GetMinimumPaneSize(self_: *const c_void) -> c_int;
    pub fn wxSplitterWindow_GetSashGravity(self_: *const c_void) -> c_double;
    pub fn wxSplitterWindow_GetSashPosition(self_: *const c_void) -> c_int;
    pub fn wxSplitterWindow_GetSashSize(self_: *const c_void) -> c_int;
    pub fn wxSplitterWindow_GetDefaultSashSize(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxSplitterWindow_GetSplitMode(self_: *const c_void) -> wxSplitMode;
    pub fn wxSplitterWindow_GetWindow1(self_: *const c_void) -> *mut c_void;
    pub fn wxSplitterWindow_GetWindow2(self_: *const c_void) -> *mut c_void;
    pub fn wxSplitterWindow_Initialize(self_: *mut c_void, window: *mut c_void);
    pub fn wxSplitterWindow_IsSashInvisible(self_: *const c_void) -> bool;
    pub fn wxSplitterWindow_IsSplit(self_: *const c_void) -> bool;
    pub fn wxSplitterWindow_OnDoubleClickSash(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxSplitterWindow_OnSashPositionChange(
        self_: *mut c_void,
        new_sash_position: c_int,
    ) -> bool;
    pub fn wxSplitterWindow_OnUnsplit(self_: *mut c_void, removed: *mut c_void);
    pub fn wxSplitterWindow_ReplaceWindow(
        self_: *mut c_void,
        win_old: *mut c_void,
        win_new: *mut c_void,
    ) -> bool;
    pub fn wxSplitterWindow_SetMinimumPaneSize(self_: *mut c_void, pane_size: c_int);
    pub fn wxSplitterWindow_SetSashGravity(self_: *mut c_void, gravity: c_double);
    pub fn wxSplitterWindow_SetSashPosition(self_: *mut c_void, position: c_int, redraw: bool);
    pub fn wxSplitterWindow_SetSplitMode(self_: *mut c_void, mode: c_int);
    pub fn wxSplitterWindow_SetSashInvisible(self_: *mut c_void, invisible: bool);
    pub fn wxSplitterWindow_SplitHorizontally(
        self_: *mut c_void,
        window1: *mut c_void,
        window2: *mut c_void,
        sash_position: c_int,
    ) -> bool;
    pub fn wxSplitterWindow_SplitVertically(
        self_: *mut c_void,
        window1: *mut c_void,
        window2: *mut c_void,
        sash_position: c_int,
    ) -> bool;
    pub fn wxSplitterWindow_Unsplit(self_: *mut c_void, to_remove: *mut c_void) -> bool;
    pub fn wxSplitterWindow_UpdateSize(self_: *mut c_void);

    // wxStaticBitmap
    pub fn wxStaticBitmap_CLASSINFO() -> *mut c_void;
    pub fn wxStaticBitmap_new() -> *mut c_void;
    pub fn wxStaticBitmap_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxStaticBitmap_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxStaticBitmap_GetBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxStaticBitmap_GetIcon(self_: *const c_void) -> *mut c_void;
    pub fn wxStaticBitmap_SetBitmap(self_: *mut c_void, label: *const c_void);
    pub fn wxStaticBitmap_SetIcon(self_: *mut c_void, label: *const c_void);
    // NOT_SUPPORTED: pub fn wxStaticBitmap_SetScaleMode(self_: *mut c_void, scale_mode: ScaleMode);
    // NOT_SUPPORTED: pub fn wxStaticBitmap_GetScaleMode(self_: *const c_void) -> ScaleMode;

    // wxStaticBox
    pub fn wxStaticBox_CLASSINFO() -> *mut c_void;
    pub fn wxStaticBox_new() -> *mut c_void;
    pub fn wxStaticBox_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // BLOCKED: pub fn wxStaticBox_new2(parent: *mut c_void, id: c_int, label: *mut c_void, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxStaticBox_~wxStaticBox(self_: *mut c_void);
    pub fn wxStaticBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    // BLOCKED: pub fn wxStaticBox_Create1(self_: *mut c_void, parent: *mut c_void, id: c_int, label: *mut c_void, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> bool;

    // wxStaticBoxSizer
    pub fn wxStaticBoxSizer_CLASSINFO() -> *mut c_void;
    pub fn wxStaticBoxSizer_new(box_: *mut c_void, orient: c_int) -> *mut c_void;
    pub fn wxStaticBoxSizer_new1(
        orient: c_int,
        parent: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxStaticBoxSizer_GetStaticBox(self_: *const c_void) -> *mut c_void;

    // wxStaticLine
    pub fn wxStaticLine_CLASSINFO() -> *mut c_void;
    pub fn wxStaticLine_new() -> *mut c_void;
    pub fn wxStaticLine_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxStaticLine_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxStaticLine_IsVertical(self_: *const c_void) -> bool;
    pub fn wxStaticLine_GetDefaultSize() -> c_int;

    // wxStaticText
    pub fn wxStaticText_CLASSINFO() -> *mut c_void;
    pub fn wxStaticText_new() -> *mut c_void;
    pub fn wxStaticText_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxStaticText_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxStaticText_IsEllipsized(self_: *const c_void) -> bool;
    pub fn wxStaticText_Wrap(self_: *mut c_void, width: c_int);

    // wxStatusBar
    pub fn wxStatusBar_CLASSINFO() -> *mut c_void;
    pub fn wxStatusBar_new() -> *mut c_void;
    pub fn wxStatusBar_new1(
        parent: *mut c_void,
        id: c_int,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxStatusBar_~wxStatusBar(self_: *mut c_void);
    pub fn wxStatusBar_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxStatusBar_GetFieldRect(self_: *const c_void, i: c_int, rect: *mut c_void) -> bool;
    pub fn wxStatusBar_GetFieldsCount(self_: *const c_void) -> c_int;
    pub fn wxStatusBar_GetField(self_: *const c_void, n: c_int) -> *mut c_void;
    pub fn wxStatusBar_GetBorders(self_: *const c_void) -> *mut c_void;
    pub fn wxStatusBar_GetStatusText(self_: *const c_void, i: c_int) -> *mut c_void;
    pub fn wxStatusBar_GetStatusWidth(self_: *const c_void, n: c_int) -> c_int;
    pub fn wxStatusBar_GetStatusStyle(self_: *const c_void, n: c_int) -> c_int;
    pub fn wxStatusBar_PopStatusText(self_: *mut c_void, field: c_int);
    pub fn wxStatusBar_PushStatusText(self_: *mut c_void, string: *const c_void, field: c_int);
    pub fn wxStatusBar_SetFieldsCount(self_: *mut c_void, number: c_int, widths: *const c_void);
    pub fn wxStatusBar_SetMinHeight(self_: *mut c_void, height: c_int);
    pub fn wxStatusBar_SetStatusStyles(self_: *mut c_void, n: c_int, styles: *const c_void);
    pub fn wxStatusBar_SetStatusText(self_: *mut c_void, text: *const c_void, i: c_int);
    pub fn wxStatusBar_SetStatusWidths(self_: *mut c_void, n: c_int, widths_field: *const c_void);

    // wxStatusBarPane
    pub fn wxStatusBarPane_delete(self_: *mut c_void);
    pub fn wxStatusBarPane_new(style: c_int, width: c_int) -> *mut c_void;
    pub fn wxStatusBarPane_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxStatusBarPane_GetStyle(self_: *const c_void) -> c_int;
    pub fn wxStatusBarPane_GetText(self_: *const c_void) -> *mut c_void;

    // wxStdDialogButtonSizer
    pub fn wxStdDialogButtonSizer_CLASSINFO() -> *mut c_void;
    pub fn wxStdDialogButtonSizer_new() -> *mut c_void;
    pub fn wxStdDialogButtonSizer_AddButton(self_: *mut c_void, button: *mut c_void);
    pub fn wxStdDialogButtonSizer_Realize(self_: *mut c_void);
    pub fn wxStdDialogButtonSizer_SetAffirmativeButton(self_: *mut c_void, button: *mut c_void);
    pub fn wxStdDialogButtonSizer_SetCancelButton(self_: *mut c_void, button: *mut c_void);
    pub fn wxStdDialogButtonSizer_SetNegativeButton(self_: *mut c_void, button: *mut c_void);

    // wxSysColourChangedEvent
    pub fn wxSysColourChangedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxSysColourChangedEvent_new() -> *mut c_void;

    // wxSystemSettings
    pub fn wxSystemSettings_delete(self_: *mut c_void);
    pub fn wxSystemSettings_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSystemSettings_GetColour(index: wxSystemColour) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSystemSettings_GetFont(index: wxSystemFont) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSystemSettings_GetMetric(index: wxSystemMetric, win: *mut c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxSystemSettings_GetScreenType() -> wxSystemScreenType;
    pub fn wxSystemSettings_GetAppearance() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxSystemSettings_HasFeature(index: wxSystemFeature) -> bool;

    // wxTaskBarIcon
    pub fn wxTaskBarIcon_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTaskBarIcon_new(icon_type: wxTaskBarIconType) -> *mut c_void;
    // DTOR: pub fn wxTaskBarIcon_~wxTaskBarIcon(self_: *mut c_void);
    pub fn wxTaskBarIcon_Destroy(self_: *mut c_void);
    pub fn wxTaskBarIcon_IsIconInstalled(self_: *const c_void) -> bool;
    pub fn wxTaskBarIcon_IsOk(self_: *const c_void) -> bool;
    pub fn wxTaskBarIcon_PopupMenu(self_: *mut c_void, menu: *mut c_void) -> bool;
    pub fn wxTaskBarIcon_RemoveIcon(self_: *mut c_void) -> bool;
    pub fn wxTaskBarIcon_SetIcon(
        self_: *mut c_void,
        icon: *const c_void,
        tooltip: *const c_void,
    ) -> bool;
    pub fn wxTaskBarIcon_IsAvailable() -> bool;

    // wxTaskBarIconEvent
    pub fn wxTaskBarIconEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTaskBarIconEvent_new(evt_type: wxEventType, tb_icon: *mut c_void) -> *mut c_void;

    // wxTextAttr
    pub fn wxTextAttr_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxTextAttr_GetAlignment(self_: *const c_void) -> wxTextAttrAlignment;
    pub fn wxTextAttr_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetBulletFont(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetBulletName(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetBulletNumber(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetBulletStyle(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetBulletText(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetCharacterStyleName(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetFlags(self_: *const c_void) -> c_long;
    pub fn wxTextAttr_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetFontAttributes(
        self_: *mut c_void,
        font: *const c_void,
        flags: c_int,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetFontEncoding(self_: *const c_void) -> wxFontEncoding;
    pub fn wxTextAttr_GetFontFaceName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetFontFamily(self_: *const c_void) -> wxFontFamily;
    pub fn wxTextAttr_GetFontSize(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetFontStyle(self_: *const c_void) -> wxFontStyle;
    pub fn wxTextAttr_GetFontUnderlined(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetUnderlineType(self_: *const c_void) -> wxTextAttrUnderlineType;
    pub fn wxTextAttr_GetUnderlineColour(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetFontWeight(self_: *const c_void) -> wxFontWeight;
    pub fn wxTextAttr_GetLeftIndent(self_: *const c_void) -> c_long;
    pub fn wxTextAttr_GetLeftSubIndent(self_: *const c_void) -> c_long;
    pub fn wxTextAttr_GetLineSpacing(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetListStyleName(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetOutlineLevel(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetParagraphSpacingAfter(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetParagraphSpacingBefore(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetParagraphStyleName(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetRightIndent(self_: *const c_void) -> c_long;
    pub fn wxTextAttr_GetTabs(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetTextEffectFlags(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetTextEffects(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetURL(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_HasAlignment(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBulletName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBulletNumber(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBulletStyle(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBulletText(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasCharacterStyleName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFlag(self_: *const c_void, flag: c_long) -> bool;
    pub fn wxTextAttr_HasFont(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontEncoding(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontFaceName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontFamily(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontItalic(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontSize(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontPointSize(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontPixelSize(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontUnderlined(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontWeight(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasLeftIndent(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasLineSpacing(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasListStyleName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasOutlineLevel(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasPageBreak(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasParagraphSpacingAfter(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasParagraphSpacingBefore(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasParagraphStyleName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasRightIndent(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasTabs(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasTextColour(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasTextEffects(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasURL(self_: *const c_void) -> bool;
    pub fn wxTextAttr_IsCharacterStyle(self_: *const c_void) -> bool;
    pub fn wxTextAttr_IsDefault(self_: *const c_void) -> bool;
    pub fn wxTextAttr_IsParagraphStyle(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTextAttr_SetAlignment(self_: *mut c_void, alignment: wxTextAttrAlignment);
    pub fn wxTextAttr_SetBackgroundColour(self_: *mut c_void, col_back: *const c_void);
    pub fn wxTextAttr_SetBulletFont(self_: *mut c_void, font: *const c_void);
    pub fn wxTextAttr_SetBulletName(self_: *mut c_void, name: *const c_void);
    pub fn wxTextAttr_SetBulletNumber(self_: *mut c_void, n: c_int);
    pub fn wxTextAttr_SetBulletStyle(self_: *mut c_void, style: c_int);
    pub fn wxTextAttr_SetBulletText(self_: *mut c_void, text: *const c_void);
    pub fn wxTextAttr_SetCharacterStyleName(self_: *mut c_void, name: *const c_void);
    pub fn wxTextAttr_SetFlags(self_: *mut c_void, flags: c_long);
    pub fn wxTextAttr_SetFont(self_: *mut c_void, font: *const c_void, flags: c_int);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontEncoding(self_: *mut c_void, encoding: wxFontEncoding);
    pub fn wxTextAttr_SetFontFaceName(self_: *mut c_void, face_name: *const c_void);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontFamily(self_: *mut c_void, family: wxFontFamily);
    pub fn wxTextAttr_SetFontSize(self_: *mut c_void, point_size: c_int);
    pub fn wxTextAttr_SetFontPointSize(self_: *mut c_void, point_size: c_int);
    pub fn wxTextAttr_SetFontPixelSize(self_: *mut c_void, pixel_size: c_int);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontStyle(self_: *mut c_void, font_style: wxFontStyle);
    pub fn wxTextAttr_SetFontUnderlined(self_: *mut c_void, underlined: bool);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontUnderlined1(self_: *mut c_void, type_: wxTextAttrUnderlineType, colour: *const c_void);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontWeight(self_: *mut c_void, font_weight: wxFontWeight);
    pub fn wxTextAttr_SetLeftIndent(self_: *mut c_void, indent: c_int, sub_indent: c_int);
    pub fn wxTextAttr_SetLineSpacing(self_: *mut c_void, spacing: c_int);
    pub fn wxTextAttr_SetListStyleName(self_: *mut c_void, name: *const c_void);
    pub fn wxTextAttr_SetOutlineLevel(self_: *mut c_void, level: c_int);
    pub fn wxTextAttr_SetPageBreak(self_: *mut c_void, page_break: bool);
    pub fn wxTextAttr_SetParagraphSpacingAfter(self_: *mut c_void, spacing: c_int);
    pub fn wxTextAttr_SetParagraphSpacingBefore(self_: *mut c_void, spacing: c_int);
    pub fn wxTextAttr_SetParagraphStyleName(self_: *mut c_void, name: *const c_void);
    pub fn wxTextAttr_SetRightIndent(self_: *mut c_void, indent: c_int);
    pub fn wxTextAttr_SetTabs(self_: *mut c_void, tabs: *const c_void);
    pub fn wxTextAttr_SetTextColour(self_: *mut c_void, col_text: *const c_void);
    pub fn wxTextAttr_SetTextEffectFlags(self_: *mut c_void, flags: c_int);
    pub fn wxTextAttr_SetTextEffects(self_: *mut c_void, effects: c_int);
    pub fn wxTextAttr_SetURL(self_: *mut c_void, url: *const c_void);
    // BLOCKED: pub fn wxTextAttr_operator=(self_: *mut c_void, attr: *const c_void);
    pub fn wxTextAttr_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextAttr_new1(col_text: *const c_void, col_back: *const c_void, font: *const c_void, alignment: wxTextAttrAlignment) -> *mut c_void;
    pub fn wxTextAttr_new2(attr: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_Apply(
        self_: *mut c_void,
        style: *const c_void,
        compare_with: *const c_void,
    ) -> bool;
    pub fn wxTextAttr_Merge(self_: *mut c_void, overlay: *const c_void);
    pub fn wxTextAttr_EqPartial(self_: *const c_void, attr: *const c_void, weak_test: bool)
        -> bool;
    pub fn wxTextAttr_Merge1(base: *const c_void, overlay: *const c_void) -> *mut c_void;

    // wxTextCompleterSimple
    pub fn wxTextCompleterSimple_delete(self_: *mut c_void);
    pub fn wxTextCompleterSimple_GetCompletions(
        self_: *mut c_void,
        prefix: *const c_void,
        res: *mut c_void,
    );

    // wxTextCtrl
    pub fn wxTextCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxTextCtrl_OSXEnableNewLineReplacement(self_: *mut c_void, enable: bool);
    // BLOCKED: pub fn wxTextCtrl_operator<<(self_: *mut c_void, s: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxTextCtrl_operator<<1(self_: *mut c_void, i: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxTextCtrl_operator<<2(self_: *mut c_void, i: c_long) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextCtrl_operator<<3(self_: *mut c_void, f: float) -> *mut c_void;
    // BLOCKED: pub fn wxTextCtrl_operator<<4(self_: *mut c_void, d: c_double) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextCtrl_operator<<5(self_: *mut c_void, c: char) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextCtrl_operator<<6(self_: *mut c_void, c: wchar_t) -> *mut c_void;
    pub fn wxTextCtrl_new() -> *mut c_void;
    pub fn wxTextCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTextCtrl_~wxTextCtrl(self_: *mut c_void);
    pub fn wxTextCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxTextCtrl_DiscardEdits(self_: *mut c_void);
    pub fn wxTextCtrl_EmptyUndoBuffer(self_: *mut c_void);
    pub fn wxTextCtrl_EmulateKeyPress(self_: *mut c_void, event: *const c_void) -> bool;
    pub fn wxTextCtrl_EnableProofCheck(self_: *mut c_void, options: *const c_void) -> bool;
    pub fn wxTextCtrl_GetDefaultStyle(self_: *const c_void) -> *mut c_void;
    pub fn wxTextCtrl_GetLineLength(self_: *const c_void, line_no: c_long) -> c_int;
    pub fn wxTextCtrl_GetLineText(self_: *const c_void, line_no: c_long) -> *mut c_void;
    pub fn wxTextCtrl_GetNumberOfLines(self_: *const c_void) -> c_int;
    pub fn wxTextCtrl_GetStyle(self_: *mut c_void, position: c_long, style: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTextCtrl_HitTest(self_: *const c_void, pt: *const c_void, pos: *mut c_void) -> wxTextCtrlHitTestResult;
    // NOT_SUPPORTED: pub fn wxTextCtrl_HitTest1(self_: *const c_void, pt: *const c_void, col: *mut c_void, row: *mut c_void) -> wxTextCtrlHitTestResult;
    pub fn wxTextCtrl_IsModified(self_: *const c_void) -> bool;
    pub fn wxTextCtrl_IsMultiLine(self_: *const c_void) -> bool;
    pub fn wxTextCtrl_IsSingleLine(self_: *const c_void) -> bool;
    pub fn wxTextCtrl_GetProofCheckOptions(self_: *mut c_void) -> *mut c_void;
    pub fn wxTextCtrl_LoadFile(
        self_: *mut c_void,
        filename: *const c_void,
        file_type: c_int,
    ) -> bool;
    pub fn wxTextCtrl_MarkDirty(self_: *mut c_void);
    pub fn wxTextCtrl_OnDropFiles(self_: *mut c_void, event: *mut c_void);
    pub fn wxTextCtrl_PositionToXY(
        self_: *const c_void,
        pos: c_long,
        x: *mut c_void,
        y: *mut c_void,
    ) -> bool;
    pub fn wxTextCtrl_PositionToCoords(self_: *const c_void, pos: c_long) -> *mut c_void;
    pub fn wxTextCtrl_SaveFile(
        self_: *mut c_void,
        filename: *const c_void,
        file_type: c_int,
    ) -> bool;
    pub fn wxTextCtrl_SetDefaultStyle(self_: *mut c_void, style: *const c_void) -> bool;
    pub fn wxTextCtrl_SetModified(self_: *mut c_void, modified: bool);
    pub fn wxTextCtrl_SetStyle(
        self_: *mut c_void,
        start: c_long,
        end: c_long,
        style: *const c_void,
    ) -> bool;
    pub fn wxTextCtrl_ShowPosition(self_: *mut c_void, pos: c_long);
    pub fn wxTextCtrl_XYToPosition(self_: *const c_void, x: c_long, y: c_long) -> c_long;
    // Mix-in(s) to wxTextCtrl
    pub fn wxTextCtrl_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxTextEntry
    pub fn wxTextEntry_delete(self_: *mut c_void);
    pub fn wxTextEntry_AppendText(self_: *mut c_void, text: *const c_void);
    pub fn wxTextEntry_AutoComplete(self_: *mut c_void, choices: *const c_void) -> bool;
    pub fn wxTextEntry_AutoComplete1(self_: *mut c_void, completer: *mut c_void) -> bool;
    pub fn wxTextEntry_AutoCompleteFileNames(self_: *mut c_void) -> bool;
    pub fn wxTextEntry_AutoCompleteDirectories(self_: *mut c_void) -> bool;
    pub fn wxTextEntry_CanCopy(self_: *const c_void) -> bool;
    pub fn wxTextEntry_CanCut(self_: *const c_void) -> bool;
    pub fn wxTextEntry_CanPaste(self_: *const c_void) -> bool;
    pub fn wxTextEntry_CanRedo(self_: *const c_void) -> bool;
    pub fn wxTextEntry_CanUndo(self_: *const c_void) -> bool;
    pub fn wxTextEntry_ChangeValue(self_: *mut c_void, value: *const c_void);
    pub fn wxTextEntry_Clear(self_: *mut c_void);
    pub fn wxTextEntry_Copy(self_: *mut c_void);
    pub fn wxTextEntry_Cut(self_: *mut c_void);
    pub fn wxTextEntry_ForceUpper(self_: *mut c_void);
    pub fn wxTextEntry_GetInsertionPoint(self_: *const c_void) -> c_long;
    // NOT_SUPPORTED: pub fn wxTextEntry_GetLastPosition(self_: *const c_void) -> wxTextPos;
    pub fn wxTextEntry_GetRange(self_: *const c_void, from: c_long, to: c_long) -> *mut c_void;
    pub fn wxTextEntry_GetSelection(self_: *const c_void, from: *mut c_void, to: *mut c_void);
    pub fn wxTextEntry_GetStringSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntry_GetValue(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntry_IsEditable(self_: *const c_void) -> bool;
    pub fn wxTextEntry_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxTextEntry_Paste(self_: *mut c_void);
    pub fn wxTextEntry_Redo(self_: *mut c_void);
    pub fn wxTextEntry_Remove(self_: *mut c_void, from: c_long, to: c_long);
    pub fn wxTextEntry_Replace(self_: *mut c_void, from: c_long, to: c_long, value: *const c_void);
    pub fn wxTextEntry_SetEditable(self_: *mut c_void, editable: bool);
    pub fn wxTextEntry_SetInsertionPoint(self_: *mut c_void, pos: c_long);
    pub fn wxTextEntry_SetInsertionPointEnd(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxTextEntry_SetMaxLength(self_: *mut c_void, len: unsigned long);
    pub fn wxTextEntry_SetSelection(self_: *mut c_void, from: c_long, to: c_long);
    pub fn wxTextEntry_SelectAll(self_: *mut c_void);
    pub fn wxTextEntry_SelectNone(self_: *mut c_void);
    pub fn wxTextEntry_SetHint(self_: *mut c_void, hint: *const c_void) -> bool;
    pub fn wxTextEntry_GetHint(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntry_SetMargins(self_: *mut c_void, pt: *const c_void) -> bool;
    pub fn wxTextEntry_SetMargins1(self_: *mut c_void, left: c_int, top: c_int) -> bool;
    pub fn wxTextEntry_GetMargins(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntry_SetValue(self_: *mut c_void, value: *const c_void);
    pub fn wxTextEntry_Undo(self_: *mut c_void);
    pub fn wxTextEntry_WriteText(self_: *mut c_void, text: *const c_void);

    // wxTextValidator
    pub fn wxTextValidator_CLASSINFO() -> *mut c_void;
    pub fn wxTextValidator_new(validator: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_new1(style: c_long, val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxTextValidator_GetCharExcludes(self_: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_GetCharIncludes(self_: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_GetExcludes(self_: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_GetIncludes(self_: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_GetStyle(self_: *const c_void) -> c_long;
    // NOT_SUPPORTED: pub fn wxTextValidator_HasFlag(self_: *const c_void, style: wxTextValidatorStyle) -> bool;
    pub fn wxTextValidator_OnChar(self_: *mut c_void, event: *mut c_void);
    pub fn wxTextValidator_SetExcludes(self_: *mut c_void, string_list: *const c_void);
    pub fn wxTextValidator_SetCharExcludes(self_: *mut c_void, chars: *const c_void);
    pub fn wxTextValidator_SetIncludes(self_: *mut c_void, string_list: *const c_void);
    pub fn wxTextValidator_SetCharIncludes(self_: *mut c_void, chars: *const c_void);
    pub fn wxTextValidator_AddExclude(self_: *mut c_void, exclude: *const c_void);
    pub fn wxTextValidator_AddInclude(self_: *mut c_void, include: *const c_void);
    pub fn wxTextValidator_AddCharExcludes(self_: *mut c_void, chars: *const c_void);
    pub fn wxTextValidator_AddCharIncludes(self_: *mut c_void, chars: *const c_void);
    pub fn wxTextValidator_SetStyle(self_: *mut c_void, style: c_long);
    pub fn wxTextValidator_IsValid(self_: *const c_void, val: *const c_void) -> *mut c_void;

    // wxTimePickerCtrl
    pub fn wxTimePickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxTimePickerCtrl_new() -> *mut c_void;
    pub fn wxTimePickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxTimePickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxTimePickerCtrl_GetTime(
        self_: *const c_void,
        hour: *mut c_void,
        min: *mut c_void,
        sec: *mut c_void,
    ) -> bool;
    pub fn wxTimePickerCtrl_GetValue(self_: *const c_void) -> *mut c_void;
    pub fn wxTimePickerCtrl_SetTime(
        self_: *mut c_void,
        hour: c_int,
        min: c_int,
        sec: c_int,
    ) -> bool;
    pub fn wxTimePickerCtrl_SetValue(self_: *mut c_void, dt: *const c_void);

    // wxTipWindow
    pub fn wxTipWindow_CLASSINFO() -> *mut c_void;
    pub fn wxTipWindow_new(
        parent: *mut c_void,
        text: *const c_void,
        max_length: c_int,
        window_ptr: *mut c_void,
        rect_bounds: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTipWindow_SetBoundingRect(self_: *mut c_void, rect_bound: *const c_void);
    pub fn wxTipWindow_SetTipWindowPtr(self_: *mut c_void, window_ptr: *mut c_void);

    // wxToggleButton
    pub fn wxToggleButton_CLASSINFO() -> *mut c_void;
    pub fn wxToggleButton_new() -> *mut c_void;
    pub fn wxToggleButton_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        val: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxToggleButton_~wxToggleButton(self_: *mut c_void);
    pub fn wxToggleButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        val: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxToggleButton_GetValue(self_: *const c_void) -> bool;
    pub fn wxToggleButton_SetValue(self_: *mut c_void, state: bool);

    // wxToolBar
    pub fn wxToolBar_CLASSINFO() -> *mut c_void;
    pub fn wxToolBar_new() -> *mut c_void;
    pub fn wxToolBar_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxToolBar_~wxToolBar(self_: *mut c_void);
    pub fn wxToolBar_AddCheckTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap1: *const c_void,
        bmp_disabled: *const c_void,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddControl(
        self_: *mut c_void,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddRadioTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap1: *const c_void,
        bmp_disabled: *const c_void,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddSeparator(self_: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddStretchableSpace(self_: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddTool(self_: *mut c_void, tool: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddTool1(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        short_help: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxToolBar_AddTool2(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_ClearTools(self_: *mut c_void);
    pub fn wxToolBar_DeleteTool(self_: *mut c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_DeleteToolByPos(self_: *mut c_void, pos: usize) -> bool;
    pub fn wxToolBar_EnableTool(self_: *mut c_void, tool_id: c_int, enable: bool);
    pub fn wxToolBar_FindById(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_FindControl(self_: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_FindToolForPosition(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxToolBar_GetMargins(self_: *const c_void) -> *mut c_void;
    pub fn wxToolBar_GetToolBitmapSize(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxToolBar_GetToolByPos(self_: *mut c_void, pos: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolByPos1(self_: *const c_void, pos: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolClientData(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolEnabled(self_: *const c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_GetToolLongHelp(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolPacking(self_: *const c_void) -> c_int;
    pub fn wxToolBar_GetToolPos(self_: *const c_void, tool_id: c_int) -> c_int;
    pub fn wxToolBar_GetToolSeparation(self_: *const c_void) -> c_int;
    pub fn wxToolBar_GetToolShortHelp(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolSize(self_: *const c_void) -> *mut c_void;
    pub fn wxToolBar_GetToolState(self_: *const c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_GetToolsCount(self_: *const c_void) -> usize;
    pub fn wxToolBar_InsertControl(
        self_: *mut c_void,
        pos: usize,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_InsertSeparator(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxToolBar_InsertStretchableSpace(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxToolBar_InsertTool(
        self_: *mut c_void,
        pos: usize,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_InsertTool1(self_: *mut c_void, pos: usize, tool: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_OnLeftClick(self_: *mut c_void, tool_id: c_int, toggle_down: bool) -> bool;
    pub fn wxToolBar_OnMouseEnter(self_: *mut c_void, tool_id: c_int);
    pub fn wxToolBar_OnRightClick(self_: *mut c_void, tool_id: c_int, x: c_long, y: c_long);
    pub fn wxToolBar_Realize(self_: *mut c_void) -> bool;
    pub fn wxToolBar_RemoveTool(self_: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_SetDropdownMenu(self_: *mut c_void, id: c_int, menu: *mut c_void) -> bool;
    pub fn wxToolBar_SetMargins(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxToolBar_SetMargins1(self_: *mut c_void, size: *const c_void);
    pub fn wxToolBar_SetToolBitmapSize(self_: *mut c_void, size: *const c_void);
    pub fn wxToolBar_SetToolClientData(self_: *mut c_void, id: c_int, client_data: *mut c_void);
    pub fn wxToolBar_SetToolDisabledBitmap(self_: *mut c_void, id: c_int, bitmap: *const c_void);
    pub fn wxToolBar_SetToolLongHelp(
        self_: *mut c_void,
        tool_id: c_int,
        help_string: *const c_void,
    );
    pub fn wxToolBar_SetToolNormalBitmap(self_: *mut c_void, id: c_int, bitmap: *const c_void);
    pub fn wxToolBar_SetToolPacking(self_: *mut c_void, packing: c_int);
    pub fn wxToolBar_SetToolSeparation(self_: *mut c_void, separation: c_int);
    pub fn wxToolBar_SetToolShortHelp(
        self_: *mut c_void,
        tool_id: c_int,
        help_string: *const c_void,
    );
    pub fn wxToolBar_ToggleTool(self_: *mut c_void, tool_id: c_int, toggle: bool);
    pub fn wxToolBar_CreateTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bmp_normal: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        client_data: *mut c_void,
        short_help: *const c_void,
        long_help: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_CreateTool1(
        self_: *mut c_void,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_CreateSeparator(self_: *mut c_void) -> *mut c_void;

    // wxToolbook
    pub fn wxToolbook_CLASSINFO() -> *mut c_void;
    pub fn wxToolbook_new() -> *mut c_void;
    pub fn wxToolbook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolbook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxToolbook_GetToolBar(self_: *const c_void) -> *mut c_void;
    pub fn wxToolbook_EnablePage(self_: *mut c_void, page: usize, enable: bool) -> bool;
    pub fn wxToolbook_EnablePage1(self_: *mut c_void, page: *mut c_void, enable: bool) -> bool;
    // Mix-in(s) to wxToolbook
    pub fn wxToolbook_AsWithImages(obj: *mut c_void) -> *mut c_void;

    // wxTopLevelWindow
    pub fn wxTopLevelWindow_CLASSINFO() -> *mut c_void;
    pub fn wxTopLevelWindow_new() -> *mut c_void;
    pub fn wxTopLevelWindow_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTopLevelWindow_~wxTopLevelWindow(self_: *mut c_void);
    pub fn wxTopLevelWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxTopLevelWindow_CenterOnScreen(self_: *mut c_void, direction: c_int);
    pub fn wxTopLevelWindow_CentreOnScreen(self_: *mut c_void, direction: c_int);
    pub fn wxTopLevelWindow_EnableCloseButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_EnableMaximizeButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_EnableMinimizeButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_GetDefaultItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_GetIcon(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxTopLevelWindow_GetIcons(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_GetTitle(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_Iconize(self_: *mut c_void, iconize: bool);
    pub fn wxTopLevelWindow_IsActive(self_: *mut c_void) -> bool;
    pub fn wxTopLevelWindow_IsAlwaysMaximized(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsFullScreen(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsIconized(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsMaximized(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_IsUsingNativeDecorations(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_Maximize(self_: *mut c_void, maximize: bool);
    pub fn wxTopLevelWindow_MSWGetSystemMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_RequestUserAttention(self_: *mut c_void, flags: c_int);
    pub fn wxTopLevelWindow_Restore(self_: *mut c_void);
    // BLOCKED: pub fn wxTopLevelWindow_RestoreToGeometry(self_: *mut c_void, ser: *mut c_void) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_SaveGeometry(self_: *const c_void, ser: *const c_void) -> bool;
    pub fn wxTopLevelWindow_SetDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_SetTmpDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_GetTmpDefaultItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxTopLevelWindow_SetIcons(self_: *mut c_void, icons: *const c_void);
    pub fn wxTopLevelWindow_SetTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxTopLevelWindow_ShouldPreventAppExit(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_OSXSetModified(self_: *mut c_void, modified: bool);
    pub fn wxTopLevelWindow_OSXIsModified(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_SetRepresentedFilename(self_: *mut c_void, filename: *const c_void);
    pub fn wxTopLevelWindow_ShowWithoutActivating(self_: *mut c_void);
    pub fn wxTopLevelWindow_EnableFullScreenView(
        self_: *mut c_void,
        enable: bool,
        style: c_long,
    ) -> bool;
    pub fn wxTopLevelWindow_ShowFullScreen(self_: *mut c_void, show: bool, style: c_long) -> bool;
    // NOT_SUPPORTED: pub fn wxTopLevelWindow_GetContentProtection(self_: *const c_void) -> wxContentProtection;
    // NOT_SUPPORTED: pub fn wxTopLevelWindow_SetContentProtection(self_: *mut c_void, content_protection: wxContentProtection) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_UseNativeDecorations(self_: *mut c_void, native: bool);
    // BLOCKED: pub fn wxTopLevelWindow_UseNativeDecorationsByDefault(self_: *mut c_void, native: bool);
    pub fn wxTopLevelWindow_GetDefaultSize() -> *mut c_void;

    // wxTreeCtrl
    pub fn wxTreeCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxTreeCtrl_new() -> *mut c_void;
    pub fn wxTreeCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTreeCtrl_~wxTreeCtrl(self_: *mut c_void);
    pub fn wxTreeCtrl_AddRoot(
        self_: *mut c_void,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_AppendItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_AssignButtonsImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeCtrl_AssignStateImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeCtrl_Collapse(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_CollapseAll(self_: *mut c_void);
    pub fn wxTreeCtrl_CollapseAllChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_CollapseAndReset(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxTreeCtrl_Delete(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_DeleteAllItems(self_: *mut c_void);
    pub fn wxTreeCtrl_DeleteChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_EditLabel(
        self_: *mut c_void,
        item: *const c_void,
        text_ctrl_class: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_EnableBellOnNoMatch(self_: *mut c_void, on: bool);
    pub fn wxTreeCtrl_EndEditLabel(self_: *mut c_void, item: *const c_void, discard_changes: bool);
    pub fn wxTreeCtrl_EnsureVisible(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_Expand(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_ExpandAll(self_: *mut c_void);
    pub fn wxTreeCtrl_ExpandAllChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_GetBoundingRect(
        self_: *const c_void,
        item: *const c_void,
        rect: *mut c_void,
        text_only: bool,
    ) -> bool;
    pub fn wxTreeCtrl_GetButtonsImageList(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetChildrenCount(
        self_: *const c_void,
        item: *const c_void,
        recursively: bool,
    ) -> usize;
    pub fn wxTreeCtrl_GetCount(self_: *const c_void) -> c_uint;
    pub fn wxTreeCtrl_GetEditControl(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetFirstChild(
        self_: *const c_void,
        item: *const c_void,
        cookie: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_GetFirstVisibleItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetFocusedItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_ClearFocusedItem(self_: *mut c_void);
    pub fn wxTreeCtrl_SetFocusedItem(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_GetIndent(self_: *const c_void) -> c_uint;
    pub fn wxTreeCtrl_GetSpacing(self_: *const c_void) -> c_uint;
    pub fn wxTreeCtrl_GetItemBackgroundColour(
        self_: *const c_void,
        item: *const c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_GetItemData(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetItemFont(self_: *const c_void, item: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeCtrl_GetItemImage(self_: *const c_void, item: *const c_void, which: wxTreeItemIcon) -> c_int;
    pub fn wxTreeCtrl_GetItemParent(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetItemState(self_: *const c_void, item: *const c_void) -> c_int;
    pub fn wxTreeCtrl_GetItemText(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetItemTextColour(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetLastChild(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetNextChild(
        self_: *const c_void,
        item: *const c_void,
        cookie: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_GetNextSibling(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetNextVisible(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetPrevSibling(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetPrevVisible(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetQuickBestSize(self_: *const c_void) -> bool;
    pub fn wxTreeCtrl_GetRootItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetSelections(self_: *const c_void, selection: *mut c_void) -> usize;
    pub fn wxTreeCtrl_GetStateImageList(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_HitTest(
        self_: *const c_void,
        point: *const c_void,
        flags: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_InsertItem(
        self_: *mut c_void,
        parent: *const c_void,
        previous: *const c_void,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_InsertItem1(
        self_: *mut c_void,
        parent: *const c_void,
        pos: usize,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_IsBold(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxTreeCtrl_IsExpanded(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_IsSelected(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_IsVisible(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_ItemHasChildren(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_OnCompareItems(
        self_: *mut c_void,
        item1: *const c_void,
        item2: *const c_void,
    ) -> c_int;
    pub fn wxTreeCtrl_PrependItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_ScrollTo(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_SelectItem(self_: *mut c_void, item: *const c_void, select: bool);
    pub fn wxTreeCtrl_SetButtonsImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeCtrl_SetIndent(self_: *mut c_void, indent: c_uint);
    pub fn wxTreeCtrl_SetSpacing(self_: *mut c_void, spacing: c_uint);
    pub fn wxTreeCtrl_SetItemBackgroundColour(
        self_: *mut c_void,
        item: *const c_void,
        col: *const c_void,
    );
    pub fn wxTreeCtrl_SetItemBold(self_: *mut c_void, item: *const c_void, bold: bool);
    pub fn wxTreeCtrl_SetItemData(self_: *mut c_void, item: *const c_void, data: *mut c_void);
    pub fn wxTreeCtrl_SetItemDropHighlight(
        self_: *mut c_void,
        item: *const c_void,
        highlight: bool,
    );
    pub fn wxTreeCtrl_SetItemFont(self_: *mut c_void, item: *const c_void, font: *const c_void);
    pub fn wxTreeCtrl_SetItemHasChildren(
        self_: *mut c_void,
        item: *const c_void,
        has_children: bool,
    );
    // NOT_SUPPORTED: pub fn wxTreeCtrl_SetItemImage(self_: *mut c_void, item: *const c_void, image: c_int, which: wxTreeItemIcon);
    pub fn wxTreeCtrl_SetItemState(self_: *mut c_void, item: *const c_void, state: c_int);
    pub fn wxTreeCtrl_SetItemText(self_: *mut c_void, item: *const c_void, text: *const c_void);
    pub fn wxTreeCtrl_SetItemTextColour(
        self_: *mut c_void,
        item: *const c_void,
        col: *const c_void,
    );
    pub fn wxTreeCtrl_SetQuickBestSize(self_: *mut c_void, quick_best_size: bool);
    pub fn wxTreeCtrl_SetStateImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeCtrl_SetWindowStyle(self_: *mut c_void, styles: c_long);
    pub fn wxTreeCtrl_SortChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_Toggle(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_ToggleItemSelection(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_Unselect(self_: *mut c_void);
    pub fn wxTreeCtrl_UnselectAll(self_: *mut c_void);
    pub fn wxTreeCtrl_UnselectItem(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_SelectChildren(self_: *mut c_void, parent: *const c_void);
    // Mix-in(s) to wxTreeCtrl
    pub fn wxTreeCtrl_AsWithImages(obj: *mut c_void) -> *mut c_void;

    // wxTreeEvent
    pub fn wxTreeEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeEvent_new(command_type: wxEventType, tree: *mut c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetKeyCode(self_: *const c_void) -> c_int;
    pub fn wxTreeEvent_GetKeyEvent(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetOldItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetPoint(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_IsEditCancelled(self_: *const c_void) -> bool;
    pub fn wxTreeEvent_SetToolTip(self_: *mut c_void, tooltip: *const c_void);

    // wxTreeItemData
    pub fn wxTreeItemData_delete(self_: *mut c_void);
    pub fn wxTreeItemData_new() -> *mut c_void;
    // DTOR: pub fn wxTreeItemData_~wxTreeItemData(self_: *mut c_void);
    pub fn wxTreeItemData_GetId(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeItemData_SetId(self_: *mut c_void, id: *const c_void);

    // wxTreeItemId
    pub fn wxTreeItemId_delete(self_: *mut c_void);
    pub fn wxTreeItemId_new() -> *mut c_void;
    pub fn wxTreeItemId_IsOk(self_: *const c_void) -> bool;
    pub fn wxTreeItemId_GetID(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeItemId_Unset(self_: *mut c_void);

    // wxTreeListCtrl
    pub fn wxTreeListCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxTreeListCtrl_AssignImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeListCtrl_SetImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeListCtrl_AppendColumn(
        self_: *mut c_void,
        title: *const c_void,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> c_int;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_GetColumnCount(self_: *const c_void) -> unsigned;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_DeleteColumn(self_: *mut c_void, col: unsigned) -> bool;
    pub fn wxTreeListCtrl_ClearColumns(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_SetColumnWidth(self_: *mut c_void, col: unsigned, width: c_int);
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_GetColumnWidth(self_: *const c_void, col: unsigned) -> c_int;
    pub fn wxTreeListCtrl_WidthFor(self_: *const c_void, text: *const c_void) -> c_int;
    pub fn wxTreeListCtrl_AppendItem(
        self_: *mut c_void,
        parent: wxTreeListItem,
        text: *const c_void,
        image_closed: c_int,
        image_opened: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeListCtrl_InsertItem(
        self_: *mut c_void,
        parent: wxTreeListItem,
        previous: wxTreeListItem,
        text: *const c_void,
        image_closed: c_int,
        image_opened: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeListCtrl_PrependItem(
        self_: *mut c_void,
        parent: wxTreeListItem,
        text: *const c_void,
        image_closed: c_int,
        image_opened: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeListCtrl_DeleteItem(self_: *mut c_void, item: wxTreeListItem);
    pub fn wxTreeListCtrl_DeleteAllItems(self_: *mut c_void);
    pub fn wxTreeListCtrl_GetRootItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeListCtrl_GetItemParent(self_: *const c_void, item: wxTreeListItem) -> *mut c_void;
    pub fn wxTreeListCtrl_GetFirstChild(self_: *const c_void, item: wxTreeListItem) -> *mut c_void;
    pub fn wxTreeListCtrl_GetNextSibling(self_: *const c_void, item: wxTreeListItem)
        -> *mut c_void;
    pub fn wxTreeListCtrl_GetFirstItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeListCtrl_GetNextItem(self_: *const c_void, item: wxTreeListItem) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_GetItemText(self_: *const c_void, item: wxTreeListItem, col: unsigned) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_SetItemText(self_: *mut c_void, item: wxTreeListItem, col: unsigned, text: *const c_void);
    pub fn wxTreeListCtrl_SetItemText1(
        self_: *mut c_void,
        item: wxTreeListItem,
        text: *const c_void,
    );
    pub fn wxTreeListCtrl_SetItemImage(
        self_: *mut c_void,
        item: wxTreeListItem,
        closed: c_int,
        opened: c_int,
    );
    pub fn wxTreeListCtrl_GetItemData(self_: *const c_void, item: wxTreeListItem) -> *mut c_void;
    pub fn wxTreeListCtrl_SetItemData(self_: *mut c_void, item: wxTreeListItem, data: *mut c_void);
    pub fn wxTreeListCtrl_Expand(self_: *mut c_void, item: wxTreeListItem);
    pub fn wxTreeListCtrl_Collapse(self_: *mut c_void, item: wxTreeListItem);
    pub fn wxTreeListCtrl_IsExpanded(self_: *const c_void, item: wxTreeListItem) -> bool;
    pub fn wxTreeListCtrl_GetSelection(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_GetSelections(self_: *const c_void, selections: *mut c_void) -> unsigned;
    pub fn wxTreeListCtrl_Select(self_: *mut c_void, item: wxTreeListItem);
    pub fn wxTreeListCtrl_Unselect(self_: *mut c_void, item: wxTreeListItem);
    pub fn wxTreeListCtrl_IsSelected(self_: *const c_void, item: wxTreeListItem) -> bool;
    pub fn wxTreeListCtrl_SelectAll(self_: *mut c_void);
    pub fn wxTreeListCtrl_UnselectAll(self_: *mut c_void);
    pub fn wxTreeListCtrl_EnsureVisible(self_: *mut c_void, item: wxTreeListItem);
    pub fn wxTreeListCtrl_CheckItem(self_: *mut c_void, item: wxTreeListItem, state: c_int);
    pub fn wxTreeListCtrl_CheckItemRecursively(
        self_: *mut c_void,
        item: wxTreeListItem,
        state: c_int,
    );
    pub fn wxTreeListCtrl_UncheckItem(self_: *mut c_void, item: wxTreeListItem);
    pub fn wxTreeListCtrl_UpdateItemParentStateRecursively(
        self_: *mut c_void,
        item: wxTreeListItem,
    );
    pub fn wxTreeListCtrl_GetCheckedState(self_: *const c_void, item: wxTreeListItem) -> c_int;
    pub fn wxTreeListCtrl_AreAllChildrenInState(
        self_: *const c_void,
        item: wxTreeListItem,
        state: c_int,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_SetSortColumn(self_: *mut c_void, col: unsigned, ascending_order: bool);
    pub fn wxTreeListCtrl_GetSortColumn(
        self_: *mut c_void,
        col: *mut c_void,
        ascending_order: *mut c_void,
    ) -> bool;
    pub fn wxTreeListCtrl_SetItemComparator(self_: *mut c_void, comparator: *mut c_void);
    pub fn wxTreeListCtrl_GetView(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeListCtrl_GetDataView(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeListCtrl_new() -> *mut c_void;
    pub fn wxTreeListCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxTreeListCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxTreeListItem
    pub fn wxTreeListItem_delete(self_: *mut c_void);
    pub fn wxTreeListItem_new() -> *mut c_void;
    pub fn wxTreeListItem_IsOk(self_: *const c_void) -> bool;

    // wxTreebook
    pub fn wxTreebook_CLASSINFO() -> *mut c_void;
    pub fn wxTreebook_new() -> *mut c_void;
    pub fn wxTreebook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTreebook_~wxTreebook(self_: *mut c_void);
    pub fn wxTreebook_AddSubPage(
        self_: *mut c_void,
        page: *mut c_void,
        text: *const c_void,
        b_select: bool,
        image_id: c_int,
    ) -> bool;
    pub fn wxTreebook_CollapseNode(self_: *mut c_void, page_id: usize) -> bool;
    pub fn wxTreebook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxTreebook_ExpandNode(self_: *mut c_void, page_id: usize, expand: bool) -> bool;
    pub fn wxTreebook_GetPageParent(self_: *const c_void, page: usize) -> c_int;
    pub fn wxTreebook_InsertSubPage(
        self_: *mut c_void,
        page_pos: usize,
        page: *mut c_void,
        text: *const c_void,
        b_select: bool,
        image_id: c_int,
    ) -> bool;
    pub fn wxTreebook_IsNodeExpanded(self_: *const c_void, page_id: usize) -> bool;
    // Mix-in(s) to wxTreebook
    pub fn wxTreebook_AsWithImages(obj: *mut c_void) -> *mut c_void;

    // wxTwoFingerTapEvent
    pub fn wxTwoFingerTapEvent_CLASSINFO() -> *mut c_void;
    pub fn wxTwoFingerTapEvent_new(windid: c_int) -> *mut c_void;

    // wxUIActionSimulator
    pub fn wxUIActionSimulator_delete(self_: *mut c_void);
    pub fn wxUIActionSimulator_new() -> *mut c_void;
    pub fn wxUIActionSimulator_MouseMove(self_: *mut c_void, x: c_long, y: c_long) -> bool;
    pub fn wxUIActionSimulator_MouseMove1(self_: *mut c_void, point: *const c_void) -> bool;
    pub fn wxUIActionSimulator_MouseDown(self_: *mut c_void, button: c_int) -> bool;
    pub fn wxUIActionSimulator_MouseUp(self_: *mut c_void, button: c_int) -> bool;
    pub fn wxUIActionSimulator_MouseClick(self_: *mut c_void, button: c_int) -> bool;
    pub fn wxUIActionSimulator_MouseDblClick(self_: *mut c_void, button: c_int) -> bool;
    pub fn wxUIActionSimulator_MouseDragDrop(
        self_: *mut c_void,
        x1: c_long,
        y1: c_long,
        x2: c_long,
        y2: c_long,
        button: c_int,
    ) -> bool;
    pub fn wxUIActionSimulator_KeyDown(
        self_: *mut c_void,
        keycode: c_int,
        modifiers: c_int,
    ) -> bool;
    pub fn wxUIActionSimulator_KeyUp(self_: *mut c_void, keycode: c_int, modifiers: c_int) -> bool;
    pub fn wxUIActionSimulator_Char(self_: *mut c_void, keycode: c_int, modifiers: c_int) -> bool;
    pub fn wxUIActionSimulator_Select(self_: *mut c_void, text: *const c_void) -> bool;
    pub fn wxUIActionSimulator_Text(self_: *mut c_void, text: *const c_void) -> bool;

    // wxUpdateUIEvent
    pub fn wxUpdateUIEvent_CLASSINFO() -> *mut c_void;
    pub fn wxUpdateUIEvent_new(command_id: c_int) -> *mut c_void;
    pub fn wxUpdateUIEvent_Check(self_: *mut c_void, check: bool);
    pub fn wxUpdateUIEvent_Enable(self_: *mut c_void, enable: bool);
    pub fn wxUpdateUIEvent_GetChecked(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetEnabled(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_IsCheckable(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetSetChecked(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetSetEnabled(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetSetShown(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetSetText(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetShown(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxUpdateUIEvent_SetText(self_: *mut c_void, text: *const c_void);
    pub fn wxUpdateUIEvent_Show(self_: *mut c_void, show: bool);
    pub fn wxUpdateUIEvent_CanUpdate(window: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxUpdateUIEvent_GetMode() -> wxUpdateUIMode;
    pub fn wxUpdateUIEvent_GetUpdateInterval() -> c_long;
    pub fn wxUpdateUIEvent_ResetUpdateTime();
    // NOT_SUPPORTED: pub fn wxUpdateUIEvent_SetMode(mode: wxUpdateUIMode);
    pub fn wxUpdateUIEvent_SetUpdateInterval(update_interval: c_long);

    // wxValidator
    pub fn wxValidator_CLASSINFO() -> *mut c_void;
    pub fn wxValidator_new() -> *mut c_void;
    // DTOR: pub fn wxValidator_~wxValidator(self_: *mut c_void);
    pub fn wxValidator_Clone(self_: *const c_void) -> *mut c_void;
    pub fn wxValidator_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxValidator_SetWindow(self_: *mut c_void, window: *mut c_void);
    pub fn wxValidator_TransferFromWindow(self_: *mut c_void) -> bool;
    pub fn wxValidator_TransferToWindow(self_: *mut c_void) -> bool;
    pub fn wxValidator_Validate(self_: *mut c_void, parent: *mut c_void) -> bool;
    pub fn wxValidator_SuppressBellOnError(suppress: bool);
    pub fn wxValidator_IsSilent() -> bool;

    // wxWindow
    pub fn wxWindow_CLASSINFO() -> *mut c_void;
    pub fn wxWindow_AcceptsFocus(self_: *const c_void) -> bool;
    pub fn wxWindow_AcceptsFocusFromKeyboard(self_: *const c_void) -> bool;
    pub fn wxWindow_AcceptsFocusRecursively(self_: *const c_void) -> bool;
    pub fn wxWindow_DisableFocusFromKeyboard(self_: *mut c_void);
    pub fn wxWindow_IsFocusable(self_: *const c_void) -> bool;
    pub fn wxWindow_CanAcceptFocus(self_: *const c_void) -> bool;
    pub fn wxWindow_CanAcceptFocusFromKeyboard(self_: *const c_void) -> bool;
    pub fn wxWindow_HasFocus(self_: *const c_void) -> bool;
    pub fn wxWindow_SetCanFocus(self_: *mut c_void, can_focus: bool);
    pub fn wxWindow_EnableVisibleFocus(self_: *mut c_void, enable: bool);
    pub fn wxWindow_SetFocus(self_: *mut c_void);
    pub fn wxWindow_SetFocusFromKbd(self_: *mut c_void);
    pub fn wxWindow_AddChild(self_: *mut c_void, child: *mut c_void);
    pub fn wxWindow_DestroyChildren(self_: *mut c_void) -> bool;
    pub fn wxWindow_FindWindow(self_: *const c_void, id: c_long) -> *mut c_void;
    pub fn wxWindow_FindWindow1(self_: *const c_void, name: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxWindow_GetChildren(self_: *mut c_void) -> *mut c_void;
    pub fn wxWindow_GetChildren1(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_RemoveChild(self_: *mut c_void, child: *mut c_void);
    pub fn wxWindow_GetGrandParent(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetNextSibling(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetParent(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetPrevSibling(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_IsDescendant(self_: *const c_void, win: *mut c_void) -> bool;
    pub fn wxWindow_Reparent(self_: *mut c_void, new_parent: *mut c_void) -> bool;
    pub fn wxWindow_AlwaysShowScrollbars(self_: *mut c_void, hflag: bool, vflag: bool);
    pub fn wxWindow_GetScrollPos(self_: *const c_void, orientation: c_int) -> c_int;
    pub fn wxWindow_GetScrollRange(self_: *const c_void, orientation: c_int) -> c_int;
    pub fn wxWindow_GetScrollThumb(self_: *const c_void, orientation: c_int) -> c_int;
    pub fn wxWindow_CanScroll(self_: *const c_void, orient: c_int) -> bool;
    pub fn wxWindow_HasScrollbar(self_: *const c_void, orient: c_int) -> bool;
    pub fn wxWindow_IsScrollbarAlwaysShown(self_: *const c_void, orient: c_int) -> bool;
    pub fn wxWindow_ScrollLines(self_: *mut c_void, lines: c_int) -> bool;
    pub fn wxWindow_ScrollPages(self_: *mut c_void, pages: c_int) -> bool;
    pub fn wxWindow_ScrollWindow(self_: *mut c_void, dx: c_int, dy: c_int, rect: *const c_void);
    pub fn wxWindow_LineUp(self_: *mut c_void) -> bool;
    pub fn wxWindow_LineDown(self_: *mut c_void) -> bool;
    pub fn wxWindow_PageUp(self_: *mut c_void) -> bool;
    pub fn wxWindow_PageDown(self_: *mut c_void) -> bool;
    pub fn wxWindow_SetScrollPos(self_: *mut c_void, orientation: c_int, pos: c_int, refresh: bool);
    pub fn wxWindow_SetScrollbar(
        self_: *mut c_void,
        orientation: c_int,
        position: c_int,
        thumb_size: c_int,
        range: c_int,
        refresh: bool,
    );
    pub fn wxWindow_BeginRepositioningChildren(self_: *mut c_void) -> bool;
    pub fn wxWindow_EndRepositioningChildren(self_: *mut c_void);
    pub fn wxWindow_CacheBestSize(self_: *const c_void, size: *const c_void);
    pub fn wxWindow_ClientToWindowSize(self_: *const c_void, size: *const c_void) -> *mut c_void;
    pub fn wxWindow_WindowToClientSize(self_: *const c_void, size: *const c_void) -> *mut c_void;
    pub fn wxWindow_Fit(self_: *mut c_void);
    pub fn wxWindow_FitInside(self_: *mut c_void);
    pub fn wxWindow_FromDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromDIP2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxWindow_ToDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToDIP2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxWindow_FromPhys(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromPhys1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromPhys2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxWindow_ToPhys(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToPhys1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToPhys2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxWindow_GetBestSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetBestHeight(self_: *const c_void, width: c_int) -> c_int;
    pub fn wxWindow_GetBestWidth(self_: *const c_void, height: c_int) -> c_int;
    pub fn wxWindow_GetClientSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxWindow_GetClientSize1(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetEffectiveMinSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetMaxClientSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetMaxSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetMinClientSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetMinSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetMinWidth(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetMinHeight(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetMaxWidth(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetMaxHeight(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxWindow_GetSize1(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetVirtualSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetVirtualSize1(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxWindow_GetBestVirtualSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetContentScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxWindow_GetDPIScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxWindow_GetWindowBorderSize(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_InformFirstDirection(
        self_: *mut c_void,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool;
    pub fn wxWindow_InvalidateBestSize(self_: *mut c_void);
    pub fn wxWindow_PostSizeEvent(self_: *mut c_void);
    pub fn wxWindow_PostSizeEventToParent(self_: *mut c_void);
    pub fn wxWindow_SendSizeEvent(self_: *mut c_void, flags: c_int);
    pub fn wxWindow_SendSizeEventToParent(self_: *mut c_void, flags: c_int);
    pub fn wxWindow_SetClientSize(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxWindow_SetClientSize1(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_SetClientSize2(self_: *mut c_void, rect: *const c_void);
    pub fn wxWindow_SetContainingSizer(self_: *mut c_void, sizer: *mut c_void);
    pub fn wxWindow_SetInitialSize(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_SetMaxClientSize(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_SetMaxSize(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_SetMinClientSize(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_SetMinSize(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_SetSize(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        size_flags: c_int,
    );
    pub fn wxWindow_SetSize1(self_: *mut c_void, rect: *const c_void);
    pub fn wxWindow_SetSize2(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_SetSize3(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxWindow_SetSizeHints(
        self_: *mut c_void,
        min_size: *const c_void,
        max_size: *const c_void,
        inc_size: *const c_void,
    );
    pub fn wxWindow_SetSizeHints1(
        self_: *mut c_void,
        min_w: c_int,
        min_h: c_int,
        max_w: c_int,
        max_h: c_int,
        inc_w: c_int,
        inc_h: c_int,
    );
    pub fn wxWindow_SetVirtualSize(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxWindow_SetVirtualSize1(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_FromDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromDIP5(d: c_int, w: *const c_void) -> c_int;
    pub fn wxWindow_ToDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToDIP5(d: c_int, w: *const c_void) -> c_int;
    pub fn wxWindow_FromPhys3(sz: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromPhys4(pt: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromPhys5(d: c_int, w: *const c_void) -> c_int;
    pub fn wxWindow_ToPhys3(sz: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToPhys4(pt: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToPhys5(d: c_int, w: *const c_void) -> c_int;
    pub fn wxWindow_Center(self_: *mut c_void, dir: c_int);
    pub fn wxWindow_CenterOnParent(self_: *mut c_void, dir: c_int);
    pub fn wxWindow_Centre(self_: *mut c_void, direction: c_int);
    pub fn wxWindow_CentreOnParent(self_: *mut c_void, direction: c_int);
    pub fn wxWindow_GetPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxWindow_GetPosition1(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetRect(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetScreenPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxWindow_GetScreenPosition1(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetScreenRect(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetClientAreaOrigin(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetClientRect(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_Move(self_: *mut c_void, x: c_int, y: c_int, flags: c_int);
    pub fn wxWindow_Move1(self_: *mut c_void, pt: *const c_void, flags: c_int);
    pub fn wxWindow_SetPosition(self_: *mut c_void, pt: *const c_void);
    pub fn wxWindow_ClientToScreen(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxWindow_ClientToScreen1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_ConvertDialogToPixels(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_ConvertDialogToPixels1(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxWindow_ConvertPixelsToDialog(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_ConvertPixelsToDialog1(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxWindow_ScreenToClient(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxWindow_ScreenToClient1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_ClearBackground(self_: *mut c_void);
    pub fn wxWindow_Freeze(self_: *mut c_void);
    pub fn wxWindow_Thaw(self_: *mut c_void);
    pub fn wxWindow_IsFrozen(self_: *const c_void) -> bool;
    pub fn wxWindow_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxWindow_GetBackgroundStyle(self_: *const c_void) -> wxBackgroundStyle;
    pub fn wxWindow_GetCharHeight(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetCharWidth(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxWindow_GetDefaultAttributes(self_: *const c_void) -> wxVisualAttributes;
    pub fn wxWindow_GetDPI(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetForegroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetTextExtent(
        self_: *const c_void,
        string: *const c_void,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: *const c_void,
    );
    pub fn wxWindow_GetTextExtent1(self_: *const c_void, string: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxWindow_GetUpdateRegion(self_: *const c_void) -> *const c_void;
    pub fn wxWindow_GetUpdateClientRect(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_HasTransparentBackground(self_: *mut c_void) -> bool;
    pub fn wxWindow_Refresh(self_: *mut c_void, erase_background: bool, rect: *const c_void);
    pub fn wxWindow_RefreshRect(self_: *mut c_void, rect: *const c_void, erase_background: bool);
    pub fn wxWindow_Update(self_: *mut c_void);
    pub fn wxWindow_SetBackgroundColour(self_: *mut c_void, colour: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxWindow_SetBackgroundStyle(self_: *mut c_void, style: wxBackgroundStyle) -> bool;
    pub fn wxWindow_IsTransparentBackgroundSupported(
        self_: *const c_void,
        reason: *mut c_void,
    ) -> bool;
    pub fn wxWindow_SetFont(self_: *mut c_void, font: *const c_void) -> bool;
    pub fn wxWindow_SetForegroundColour(self_: *mut c_void, colour: *const c_void) -> bool;
    pub fn wxWindow_SetOwnBackgroundColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxWindow_InheritsBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxWindow_UseBgCol(self_: *const c_void) -> bool;
    pub fn wxWindow_UseBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxWindow_SetOwnFont(self_: *mut c_void, font: *const c_void);
    pub fn wxWindow_SetOwnForegroundColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxWindow_UseForegroundColour(self_: *const c_void) -> bool;
    pub fn wxWindow_InheritsForegroundColour(self_: *const c_void) -> bool;
    pub fn wxWindow_SetPalette(self_: *mut c_void, pal: *const c_void);
    pub fn wxWindow_ShouldInheritColours(self_: *const c_void) -> bool;
    pub fn wxWindow_SetThemeEnabled(self_: *mut c_void, enable: bool);
    pub fn wxWindow_GetThemeEnabled(self_: *const c_void) -> bool;
    pub fn wxWindow_CanSetTransparent(self_: *mut c_void) -> bool;
    pub fn wxWindow_SetTransparent(self_: *mut c_void, alpha: c_uchar) -> bool;
    pub fn wxWindow_GetEventHandler(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_HandleAsNavigationKey(self_: *mut c_void, event: *const c_void) -> bool;
    pub fn wxWindow_HandleWindowEvent(self_: *const c_void, event: *mut c_void) -> bool;
    pub fn wxWindow_ProcessWindowEvent(self_: *mut c_void, event: *mut c_void) -> bool;
    pub fn wxWindow_ProcessWindowEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
    pub fn wxWindow_PopEventHandler(self_: *mut c_void, delete_handler: bool) -> *mut c_void;
    pub fn wxWindow_PushEventHandler(self_: *mut c_void, handler: *mut c_void);
    pub fn wxWindow_RemoveEventHandler(self_: *mut c_void, handler: *mut c_void) -> bool;
    pub fn wxWindow_SetEventHandler(self_: *mut c_void, handler: *mut c_void);
    pub fn wxWindow_GetExtraStyle(self_: *const c_void) -> c_long;
    pub fn wxWindow_GetWindowStyleFlag(self_: *const c_void) -> c_long;
    pub fn wxWindow_GetWindowStyle(self_: *const c_void) -> c_long;
    pub fn wxWindow_HasExtraStyle(self_: *const c_void, ex_flag: c_int) -> bool;
    pub fn wxWindow_HasFlag(self_: *const c_void, flag: c_int) -> bool;
    pub fn wxWindow_SetExtraStyle(self_: *mut c_void, ex_style: c_long);
    pub fn wxWindow_SetWindowStyleFlag(self_: *mut c_void, style: c_long);
    pub fn wxWindow_SetWindowStyle(self_: *mut c_void, style: c_long);
    pub fn wxWindow_ToggleWindowStyle(self_: *mut c_void, flag: c_int) -> bool;
    pub fn wxWindow_MoveAfterInTabOrder(self_: *mut c_void, win: *mut c_void);
    pub fn wxWindow_MoveBeforeInTabOrder(self_: *mut c_void, win: *mut c_void);
    pub fn wxWindow_Navigate(self_: *mut c_void, flags: c_int) -> bool;
    pub fn wxWindow_NavigateIn(self_: *mut c_void, flags: c_int) -> bool;
    pub fn wxWindow_Lower(self_: *mut c_void);
    pub fn wxWindow_Raise(self_: *mut c_void);
    pub fn wxWindow_Hide(self_: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxWindow_HideWithEffect(self_: *mut c_void, effect: wxShowEffect, timeout: c_uint) -> bool;
    pub fn wxWindow_IsEnabled(self_: *const c_void) -> bool;
    pub fn wxWindow_IsExposed(self_: *const c_void, x: c_int, y: c_int) -> bool;
    pub fn wxWindow_IsExposed1(self_: *const c_void, pt: *mut c_void) -> bool;
    pub fn wxWindow_IsExposed2(
        self_: *const c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
    ) -> bool;
    pub fn wxWindow_IsExposed3(self_: *const c_void, rect: *mut c_void) -> bool;
    pub fn wxWindow_IsShown(self_: *const c_void) -> bool;
    pub fn wxWindow_IsShownOnScreen(self_: *const c_void) -> bool;
    pub fn wxWindow_Disable(self_: *mut c_void) -> bool;
    pub fn wxWindow_Enable(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxWindow_Show(self_: *mut c_void, show: bool) -> bool;
    // NOT_SUPPORTED: pub fn wxWindow_ShowWithEffect(self_: *mut c_void, effect: wxShowEffect, timeout: c_uint) -> bool;
    pub fn wxWindow_GetHelpText(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_SetHelpText(self_: *mut c_void, help_text: *const c_void);
    // NOT_SUPPORTED: pub fn wxWindow_GetHelpTextAtPoint(self_: *const c_void, point: *const c_void, origin: wxHelpEvent::Origin) -> *mut c_void;
    pub fn wxWindow_GetToolTip(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetToolTipText(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_SetToolTip(self_: *mut c_void, tip_string: *const c_void);
    pub fn wxWindow_SetToolTip1(self_: *mut c_void, tip: *mut c_void);
    pub fn wxWindow_UnsetToolTip(self_: *mut c_void);
    pub fn wxWindow_GetPopupMenuSelectionFromUser(
        self_: *mut c_void,
        menu: *mut c_void,
        pos: *const c_void,
    ) -> c_int;
    pub fn wxWindow_GetPopupMenuSelectionFromUser1(
        self_: *mut c_void,
        menu: *mut c_void,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn wxWindow_PopupMenu(self_: *mut c_void, menu: *mut c_void, pos: *const c_void) -> bool;
    pub fn wxWindow_PopupMenu1(self_: *mut c_void, menu: *mut c_void, x: c_int, y: c_int) -> bool;
    pub fn wxWindow_GetValidator(self_: *mut c_void) -> *mut c_void;
    pub fn wxWindow_SetValidator(self_: *mut c_void, validator: *const c_void);
    pub fn wxWindow_TransferDataFromWindow(self_: *mut c_void) -> bool;
    pub fn wxWindow_TransferDataToWindow(self_: *mut c_void) -> bool;
    pub fn wxWindow_Validate(self_: *mut c_void) -> bool;
    pub fn wxWindow_GetId(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetLayoutDirection(self_: *const c_void) -> c_int;
    pub fn wxWindow_AdjustForLayoutDirection(
        self_: *const c_void,
        x: c_int,
        width: c_int,
        width_total: c_int,
    ) -> c_int;
    pub fn wxWindow_GetName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxWindow_GetWindowVariant(self_: *const c_void) -> wxWindowVariant;
    pub fn wxWindow_SetId(self_: *mut c_void, winid: c_int);
    pub fn wxWindow_SetLabel(self_: *mut c_void, label: *const c_void);
    pub fn wxWindow_SetLayoutDirection(self_: *mut c_void, dir: c_int);
    pub fn wxWindow_SetName(self_: *mut c_void, name: *const c_void);
    // NOT_SUPPORTED: pub fn wxWindow_SetWindowVariant(self_: *mut c_void, variant: wxWindowVariant);
    pub fn wxWindow_GetAcceleratorTable(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxWindow_GetAccessible(self_: *mut c_void) -> *mut c_void;
    pub fn wxWindow_SetAcceleratorTable(self_: *mut c_void, accel: *const c_void);
    // NOT_SUPPORTED: pub fn wxWindow_SetAccessible(self_: *mut c_void, accessible: *mut c_void);
    pub fn wxWindow_Close(self_: *mut c_void, force: bool) -> bool;
    pub fn wxWindow_Destroy(self_: *mut c_void) -> bool;
    pub fn wxWindow_IsBeingDeleted(self_: *const c_void) -> bool;
    pub fn wxWindow_GetDropTarget(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_SetDropTarget(self_: *mut c_void, target: *mut c_void);
    pub fn wxWindow_DragAcceptFiles(self_: *mut c_void, accept: bool);
    pub fn wxWindow_GetContainingSizer(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetSizer(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_SetSizer(self_: *mut c_void, sizer: *mut c_void, delete_old: bool);
    pub fn wxWindow_SetSizerAndFit(self_: *mut c_void, sizer: *mut c_void, delete_old: bool);
    pub fn wxWindow_GetConstraints(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_SetConstraints(self_: *mut c_void, constraints: *mut c_void);
    pub fn wxWindow_Layout(self_: *mut c_void) -> bool;
    pub fn wxWindow_SetAutoLayout(self_: *mut c_void, auto_layout: bool);
    pub fn wxWindow_GetAutoLayout(self_: *const c_void) -> bool;
    pub fn wxWindow_CaptureMouse(self_: *mut c_void);
    pub fn wxWindow_GetCaret(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxWindow_GetCursor(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_HasCapture(self_: *const c_void) -> bool;
    pub fn wxWindow_ReleaseMouse(self_: *mut c_void);
    pub fn wxWindow_SetCaret(self_: *mut c_void, caret: *mut c_void);
    pub fn wxWindow_SetCursor(self_: *mut c_void, cursor: *const c_void) -> bool;
    pub fn wxWindow_WarpPointer(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxWindow_EnableTouchEvents(self_: *mut c_void, events_mask: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxWindow_HitTest(self_: *const c_void, x: c_int, y: c_int) -> wxHitTest;
    // NOT_SUPPORTED: pub fn wxWindow_HitTest1(self_: *const c_void, pt: *const c_void) -> wxHitTest;
    // NOT_SUPPORTED: pub fn wxWindow_GetBorder(self_: *const c_void, flags: c_long) -> wxBorder;
    // NOT_SUPPORTED: pub fn wxWindow_GetBorder1(self_: *const c_void) -> wxBorder;
    pub fn wxWindow_DoUpdateWindowUI(self_: *mut c_void, event: *mut c_void);
    // NOT_SUPPORTED: pub fn wxWindow_GetHandle(self_: *const c_void) -> WXWidget;
    pub fn wxWindow_HasMultiplePages(self_: *const c_void) -> bool;
    pub fn wxWindow_InheritAttributes(self_: *mut c_void);
    pub fn wxWindow_InitDialog(self_: *mut c_void);
    pub fn wxWindow_IsDoubleBuffered(self_: *const c_void) -> bool;
    pub fn wxWindow_SetDoubleBuffered(self_: *mut c_void, on: bool);
    pub fn wxWindow_IsRetained(self_: *const c_void) -> bool;
    pub fn wxWindow_IsThisEnabled(self_: *const c_void) -> bool;
    pub fn wxWindow_IsTopLevel(self_: *const c_void) -> bool;
    pub fn wxWindow_OnInternalIdle(self_: *mut c_void);
    pub fn wxWindow_SendIdleEvents(self_: *mut c_void, event: *mut c_void) -> bool;
    pub fn wxWindow_RegisterHotKey(
        self_: *mut c_void,
        hotkey_id: c_int,
        modifiers: c_int,
        virtual_key_code: c_int,
    ) -> bool;
    pub fn wxWindow_UnregisterHotKey(self_: *mut c_void, hotkey_id: c_int) -> bool;
    pub fn wxWindow_UpdateWindowUI(self_: *mut c_void, flags: c_long);
    // NOT_SUPPORTED: pub fn wxWindow_GetClassDefaultAttributes(variant: wxWindowVariant) -> wxVisualAttributes;
    pub fn wxWindow_FindFocus() -> *mut c_void;
    pub fn wxWindow_FindWindowById(id: c_long, parent: *const c_void) -> *mut c_void;
    pub fn wxWindow_FindWindowByLabel(label: *const c_void, parent: *const c_void) -> *mut c_void;
    pub fn wxWindow_FindWindowByName(name: *const c_void, parent: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetCapture() -> *mut c_void;
    pub fn wxWindow_NewControlId(count: c_int) -> c_int;
    pub fn wxWindow_UnreserveControlId(id: c_int, count: c_int);
    pub fn wxWindow_new() -> *mut c_void;
    pub fn wxWindow_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxWindow_~wxWindow(self_: *mut c_void);
    pub fn wxWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxWindowCreateEvent
    pub fn wxWindowCreateEvent_CLASSINFO() -> *mut c_void;
    pub fn wxWindowCreateEvent_new(win: *mut c_void) -> *mut c_void;
    pub fn wxWindowCreateEvent_GetWindow(self_: *const c_void) -> *mut c_void;

    // wxWindowDC
    pub fn wxWindowDC_CLASSINFO() -> *mut c_void;
    pub fn wxWindowDC_new(window: *mut c_void) -> *mut c_void;

    // wxWindowDestroyEvent
    pub fn wxWindowDestroyEvent_CLASSINFO() -> *mut c_void;
    pub fn wxWindowDestroyEvent_new(win: *mut c_void) -> *mut c_void;
    pub fn wxWindowDestroyEvent_GetWindow(self_: *const c_void) -> *mut c_void;

    // wxWindowDisabler
    pub fn wxWindowDisabler_delete(self_: *mut c_void);
    pub fn wxWindowDisabler_new(disable: bool) -> *mut c_void;
    pub fn wxWindowDisabler_new1(
        win_to_skip: *mut c_void,
        win_to_skip2: *mut c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxWindowDisabler_~wxWindowDisabler(self_: *mut c_void);

    // wxWizardEvent
    pub fn wxWizardEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxWizardEvent_new(type_: wxEventType, id: c_int, direction: bool, page: *mut c_void) -> *mut c_void;
    pub fn wxWizardEvent_GetDirection(self_: *const c_void) -> bool;
    pub fn wxWizardEvent_GetPage(self_: *const c_void) -> *mut c_void;

    // wxWrapSizer
    pub fn wxWrapSizer_CLASSINFO() -> *mut c_void;
    pub fn wxWrapSizer_new(orient: c_int, flags: c_int) -> *mut c_void;

    // wxZoomGestureEvent
    pub fn wxZoomGestureEvent_CLASSINFO() -> *mut c_void;
    pub fn wxZoomGestureEvent_new(windid: c_int) -> *mut c_void;
    pub fn wxZoomGestureEvent_GetZoomFactor(self_: *const c_void) -> c_double;
    pub fn wxZoomGestureEvent_SetZoomFactor(self_: *mut c_void, zoom_factor: c_double);

}
