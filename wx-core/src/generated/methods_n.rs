use super::*;

// wxNativeFontInfo
pub trait NativeFontInfoMethods: WxRustMethods {
    // DTOR: fn ~wxNativeFontInfo()
    // BLOCKED: fn operator=()
    fn init(&self) {
        unsafe { ffi::wxNativeFontInfo_Init(self.as_ptr()) }
    }
    fn init_from_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxNativeFontInfo_InitFromFont(self.as_ptr(), font)
        }
    }
    fn get_point_size(&self) -> c_int {
        unsafe { ffi::wxNativeFontInfo_GetPointSize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFractionalPointSize()
    fn get_pixel_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxNativeFontInfo_GetPixelSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    fn get_numeric_weight(&self) -> c_int {
        unsafe { ffi::wxNativeFontInfo_GetNumericWeight(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWeight()
    fn get_underlined(&self) -> bool {
        unsafe { ffi::wxNativeFontInfo_GetUnderlined(self.as_ptr()) }
    }
    fn get_face_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_GetFaceName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFamily()
    // NOT_SUPPORTED: fn GetEncoding()
    fn set_point_size(&self, pointsize: c_int) {
        unsafe { ffi::wxNativeFontInfo_SetPointSize(self.as_ptr(), pointsize) }
    }
    // NOT_SUPPORTED: fn SetFractionalPointSize()
    fn set_pixel_size<S: SizeMethods>(&self, pixel_size: &S) {
        unsafe {
            let pixel_size = pixel_size.as_ptr();
            ffi::wxNativeFontInfo_SetPixelSize(self.as_ptr(), pixel_size)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    fn set_numeric_weight(&self, weight: c_int) {
        unsafe { ffi::wxNativeFontInfo_SetNumericWeight(self.as_ptr(), weight) }
    }
    // NOT_SUPPORTED: fn SetWeight()
    fn set_underlined(&self, underlined: bool) {
        unsafe { ffi::wxNativeFontInfo_SetUnderlined(self.as_ptr(), underlined) }
    }
    fn set_face_name_str(&self, facename: &str) -> bool {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ffi::wxNativeFontInfo_SetFaceName(self.as_ptr(), facename)
        }
    }
    // NOT_SUPPORTED: fn SetFamily()
    // NOT_SUPPORTED: fn SetEncoding()
    fn set_face_name_arraystring<A: ArrayStringMethods>(&self, facenames: &A) {
        unsafe {
            let facenames = facenames.as_ptr();
            ffi::wxNativeFontInfo_SetFaceName1(self.as_ptr(), facenames)
        }
    }
    fn from_string(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxNativeFontInfo_FromString(self.as_ptr(), s)
        }
    }
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_ToString(self.as_ptr())).into() }
    }
    fn from_user_string(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxNativeFontInfo_FromUserString(self.as_ptr(), s)
        }
    }
    fn to_user_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_ToUserString(self.as_ptr())).into() }
    }
}

// wxNavigationKeyEvent
pub trait NavigationKeyEventMethods: EventMethods {
    fn get_current_focus(&self) -> WeakRef<Window> {
        unsafe {
            WeakRef::<Window>::from_ptr(ffi::wxNavigationKeyEvent_GetCurrentFocus(self.as_ptr()))
        }
    }
    fn get_direction(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_GetDirection(self.as_ptr()) }
    }
    fn is_from_tab(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_IsFromTab(self.as_ptr()) }
    }
    fn is_window_change(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_IsWindowChange(self.as_ptr()) }
    }
    fn set_current_focus<W: WindowMethods>(&self, current_focus: Option<&W>) {
        unsafe {
            let current_focus = match current_focus {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxNavigationKeyEvent_SetCurrentFocus(self.as_ptr(), current_focus)
        }
    }
    fn set_direction(&self, direction: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetDirection(self.as_ptr(), direction) }
    }
    fn set_flags(&self, flags: c_long) {
        unsafe { ffi::wxNavigationKeyEvent_SetFlags(self.as_ptr(), flags) }
    }
    fn set_from_tab(&self, from_tab: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetFromTab(self.as_ptr(), from_tab) }
    }
    fn set_window_change(&self, window_change: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetWindowChange(self.as_ptr(), window_change) }
    }
}

// wxNonOwnedWindow
pub trait NonOwnedWindowMethods: WindowMethods {
    fn set_shape_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxNonOwnedWindow_SetShape(self.as_ptr(), region)
        }
    }
    fn set_shape_graphicspath<G: GraphicsPathMethods>(&self, path: &G) -> bool {
        unsafe {
            let path = path.as_ptr();
            ffi::wxNonOwnedWindow_SetShape1(self.as_ptr(), path)
        }
    }
}

// wxNotebook
pub trait NotebookMethods: BookCtrlBaseMethods {
    // DTOR: fn ~wxNotebook()
    fn get_row_count(&self) -> c_int {
        unsafe { ffi::wxNotebook_GetRowCount(self.as_ptr()) }
    }
    fn get_theme_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxNotebook_GetThemeBackgroundColour(self.as_ptr())) }
    }
    // BLOCKED: fn OnSelChange()
    fn set_padding<S: SizeMethods>(&self, padding: &S) {
        unsafe {
            let padding = padding.as_ptr();
            ffi::wxNotebook_SetPadding(self.as_ptr(), padding)
        }
    }
}

// wxNotificationMessage
pub trait NotificationMessageMethods: EvtHandlerMethods {
    // DTOR: fn ~wxNotificationMessage()
    fn add_action(&self, actionid: c_int, label: &str) -> bool {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxNotificationMessage_AddAction(self.as_ptr(), actionid, label)
        }
    }
    fn close(&self) -> bool {
        unsafe { ffi::wxNotificationMessage_Close(self.as_ptr()) }
    }
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxNotificationMessage_SetFlags(self.as_ptr(), flags) }
    }
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxNotificationMessage_SetIcon(self.as_ptr(), icon)
        }
    }
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxNotificationMessage_SetMessage(self.as_ptr(), message)
        }
    }
    fn set_parent<W: WindowMethods>(&self, parent: Option<&W>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxNotificationMessage_SetParent(self.as_ptr(), parent)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxNotificationMessage_SetTitle(self.as_ptr(), title)
        }
    }
    fn show(&self, timeout: c_int) -> bool {
        unsafe { ffi::wxNotificationMessage_Show(self.as_ptr(), timeout) }
    }
    fn use_task_bar_icon<T: TaskBarIconMethods>(icon: Option<&T>) -> WeakRef<TaskBarIcon> {
        unsafe {
            let icon = match icon {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<TaskBarIcon>::from_ptr(ffi::wxNotificationMessage_UseTaskBarIcon(icon))
        }
    }
    fn msw_use_toasts(shortcut_path: &str, app_id: &str) -> bool {
        unsafe {
            let shortcut_path = WxString::from(shortcut_path);
            let shortcut_path = shortcut_path.as_ptr();
            let app_id = WxString::from(app_id);
            let app_id = app_id.as_ptr();
            ffi::wxNotificationMessage_MSWUseToasts(shortcut_path, app_id)
        }
    }
}

// wxNotifyEvent
pub trait NotifyEventMethods: CommandEventMethods {
    fn allow(&self) {
        unsafe { ffi::wxNotifyEvent_Allow(self.as_ptr()) }
    }
    fn is_allowed(&self) -> bool {
        unsafe { ffi::wxNotifyEvent_IsAllowed(self.as_ptr()) }
    }
    fn veto(&self) {
        unsafe { ffi::wxNotifyEvent_Veto(self.as_ptr()) }
    }
}

// wxNumberEntryDialog
pub trait NumberEntryDialogMethods: DialogMethods {
    fn create_str<W: WindowMethods, P: PointMethods>(
        &self,
        parent: Option<&W>,
        message: &str,
        prompt: &str,
        caption: &str,
        value: c_long,
        min: c_long,
        max: c_long,
        pos: &P,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let prompt = WxString::from(prompt);
            let prompt = prompt.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxNumberEntryDialog_Create(
                self.as_ptr(),
                parent,
                message,
                prompt,
                caption,
                value,
                min,
                max,
                pos,
            )
        }
    }
    fn get_value(&self) -> c_long {
        unsafe { ffi::wxNumberEntryDialog_GetValue(self.as_ptr()) }
    }
}
