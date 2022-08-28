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
    /// Returns the child that has the focus, or NULL.
    fn get_current_focus(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxNavigationKeyEvent_GetCurrentFocus(self.as_ptr())) }
    }
    /// Returns true if the navigation was in the forward direction.
    fn get_direction(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_GetDirection(self.as_ptr()) }
    }
    /// Returns true if the navigation event was from a tab key.
    fn is_from_tab(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_IsFromTab(self.as_ptr()) }
    }
    /// Returns true if the navigation event represents a window change (for example, from Ctrl-Page Down in a notebook).
    fn is_window_change(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_IsWindowChange(self.as_ptr()) }
    }
    /// Sets the current focus window member.
    fn set_current_focus<W: WindowMethods>(&self, current_focus: Option<&W>) {
        unsafe {
            let current_focus = match current_focus {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxNavigationKeyEvent_SetCurrentFocus(self.as_ptr(), current_focus)
        }
    }
    /// Sets the direction to forward if direction is true, or backward if false.
    fn set_direction(&self, direction: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetDirection(self.as_ptr(), direction) }
    }
    /// Sets the flags for this event.
    fn set_flags(&self, flags: c_long) {
        unsafe { ffi::wxNavigationKeyEvent_SetFlags(self.as_ptr(), flags) }
    }
    /// Marks the navigation event as from a tab key.
    fn set_from_tab(&self, from_tab: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetFromTab(self.as_ptr(), from_tab) }
    }
    /// Marks the event as a window change event.
    fn set_window_change(&self, window_change: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetWindowChange(self.as_ptr(), window_change) }
    }
}

// wxNonOwnedWindow
pub trait NonOwnedWindowMethods: WindowMethods {
    /// If the platform supports it, sets the shape of the window to that depicted by region.
    fn set_shape_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxNonOwnedWindow_SetShape(self.as_ptr(), region)
        }
    }
    /// Set the window shape to the given path.
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
    /// Returns the number of rows in the notebook control.
    fn get_row_count(&self) -> c_int {
        unsafe { ffi::wxNotebook_GetRowCount(self.as_ptr()) }
    }
    /// If running under Windows and themes are enabled for the application, this function returns a suitable colour for painting the background of a notebook page, and can be passed to SetBackgroundColour().
    fn get_theme_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxNotebook_GetThemeBackgroundColour(self.as_ptr())) }
    }
    // BLOCKED: fn OnSelChange()
    /// Sets the amount of space around each page's icon and label, in pixels.
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
    /// Add an action to the notification.
    fn add_action(&self, actionid: c_int, label: &str) -> bool {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxNotificationMessage_AddAction(self.as_ptr(), actionid, label)
        }
    }
    /// Hides the notification.
    fn close(&self) -> bool {
        unsafe { ffi::wxNotificationMessage_Close(self.as_ptr()) }
    }
    /// This parameter can be currently used to specify the icon to show in the notification.
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxNotificationMessage_SetFlags(self.as_ptr(), flags) }
    }
    /// Specify a custom icon to be displayed in the notification.
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxNotificationMessage_SetIcon(self.as_ptr(), icon)
        }
    }
    /// Set the main text of the notification.
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxNotificationMessage_SetMessage(self.as_ptr(), message)
        }
    }
    /// Set the parent for this notification: the notification will be associated with the top level parent of this window or, if this method is not called, with the main application window by default.
    fn set_parent<W: WindowMethods>(&self, parent: Option<&W>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxNotificationMessage_SetParent(self.as_ptr(), parent)
        }
    }
    /// Set the title, it must be a concise string (not more than 64 characters), use SetMessage() to give the user more details.
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxNotificationMessage_SetTitle(self.as_ptr(), title)
        }
    }
    /// Show the notification to the user and hides it after timeout seconds are elapsed.
    fn show(&self, timeout: c_int) -> bool {
        unsafe { ffi::wxNotificationMessage_Show(self.as_ptr(), timeout) }
    }
    /// If the application already uses a wxTaskBarIcon, it should be connected to notifications by using this method.
    fn use_task_bar_icon<T: TaskBarIconMethods>(icon: Option<&T>) -> WeakRef<TaskBarIcon> {
        unsafe {
            let icon = match icon {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<TaskBarIcon>::from(ffi::wxNotificationMessage_UseTaskBarIcon(icon))
        }
    }
    /// Enables toast notifications available since Windows 8 and suppresses the additional icon in the notification area on Windows 10.
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
    /// This is the opposite of Veto(): it explicitly allows the event to be processed.
    fn allow(&self) {
        unsafe { ffi::wxNotifyEvent_Allow(self.as_ptr()) }
    }
    /// Returns true if the change is allowed (Veto() hasn't been called) or false otherwise (if it was).
    fn is_allowed(&self) -> bool {
        unsafe { ffi::wxNotifyEvent_IsAllowed(self.as_ptr()) }
    }
    /// Prevents the change announced by this event from happening.
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
    /// Returns the value that the user has entered if the user has pressed OK, or the original value if the user has pressed Cancel.
    fn get_value(&self) -> c_long {
        unsafe { ffi::wxNumberEntryDialog_GetValue(self.as_ptr()) }
    }
}
