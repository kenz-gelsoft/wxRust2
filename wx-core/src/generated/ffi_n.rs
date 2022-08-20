use super::*;

extern "C" {

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
    pub fn wxNotebook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxNotebook_GetRowCount(self_: *const c_void) -> c_int;
    pub fn wxNotebook_GetThemeBackgroundColour(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxNotebook_OnSelChange(self_: *mut c_void, event: *mut c_void);
    pub fn wxNotebook_SetPadding(self_: *mut c_void, padding: *const c_void);

    // wxNotificationMessage
    pub fn wxNotificationMessage_CLASSINFO() -> *mut c_void;
    pub fn wxNotificationMessage_new() -> *mut c_void;
    pub fn wxNotificationMessage_new1(
        title: *const c_void,
        message: *const c_void,
        parent: *mut c_void,
        flags: c_int,
    ) -> *mut c_void;
    // DTOR: pub fn wxNotificationMessage_~wxNotificationMessage(self_: *mut c_void);
    pub fn wxNotificationMessage_AddAction(
        self_: *mut c_void,
        actionid: c_int,
        label: *const c_void,
    ) -> bool;
    pub fn wxNotificationMessage_Close(self_: *mut c_void) -> bool;
    pub fn wxNotificationMessage_SetFlags(self_: *mut c_void, flags: c_int);
    pub fn wxNotificationMessage_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxNotificationMessage_SetMessage(self_: *mut c_void, message: *const c_void);
    pub fn wxNotificationMessage_SetParent(self_: *mut c_void, parent: *mut c_void);
    pub fn wxNotificationMessage_SetTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxNotificationMessage_Show(self_: *mut c_void, timeout: c_int) -> bool;
    pub fn wxNotificationMessage_UseTaskBarIcon(icon: *mut c_void) -> *mut c_void;
    pub fn wxNotificationMessage_MSWUseToasts(
        shortcut_path: *const c_void,
        app_id: *const c_void,
    ) -> bool;

    // wxNotifyEvent
    pub fn wxNotifyEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxNotifyEvent_new(event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxNotifyEvent_Allow(self_: *mut c_void);
    pub fn wxNotifyEvent_IsAllowed(self_: *const c_void) -> bool;
    pub fn wxNotifyEvent_Veto(self_: *mut c_void);

    // wxNumberEntryDialog
    pub fn wxNumberEntryDialog_CLASSINFO() -> *mut c_void;
    pub fn wxNumberEntryDialog_new() -> *mut c_void;
    pub fn wxNumberEntryDialog_new1(
        parent: *mut c_void,
        message: *const c_void,
        prompt: *const c_void,
        caption: *const c_void,
        value: c_long,
        min: c_long,
        max: c_long,
        pos: *const c_void,
    ) -> *mut c_void;
    pub fn wxNumberEntryDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        message: *const c_void,
        prompt: *const c_void,
        caption: *const c_void,
        value: c_long,
        min: c_long,
        max: c_long,
        pos: *const c_void,
    ) -> bool;
    pub fn wxNumberEntryDialog_GetValue(self_: *const c_void) -> c_long;

}
