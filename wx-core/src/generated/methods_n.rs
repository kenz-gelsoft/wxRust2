use super::*;

// wxNativeFontInfo
/// This trait represents C++ [`wxNativeFontInfo`](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html) class's methods and inheritance.
///
/// See [`NativeFontInfoIsOwned`] documentation for the class usage.
pub trait NativeFontInfoMethods: WxRustMethods {
    // DTOR: fn ~wxNativeFontInfo()
    // BLOCKED: fn operator=()
    ///
    /// [See `wxNativeFontInfo::Init()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a0d23b1700ea3c1b115be09f522af3f58)
    fn init(&self) {
        unsafe { ffi::wxNativeFontInfo_Init(self.as_ptr()) }
    }
    ///
    /// [See `wxNativeFontInfo::InitFromFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a7e1279441ba88730ec844e422cb6ab8b)
    fn init_from_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxNativeFontInfo_InitFromFont(self.as_ptr(), font)
        }
    }
    ///
    /// [See `wxNativeFontInfo::GetPointSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a05a1305d080a8d415e278a9b4417d1b3)
    fn get_point_size(&self) -> c_int {
        unsafe { ffi::wxNativeFontInfo_GetPointSize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFractionalPointSize()
    ///
    /// [See `wxNativeFontInfo::GetPixelSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a234c05c5794ec1eac0c7daf53fea7f3e)
    fn get_pixel_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxNativeFontInfo_GetPixelSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    ///
    /// [See `wxNativeFontInfo::GetNumericWeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#ab23583cfa8209a4a5dd4b05e3253d79c)
    fn get_numeric_weight(&self) -> c_int {
        unsafe { ffi::wxNativeFontInfo_GetNumericWeight(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWeight()
    ///
    /// [See `wxNativeFontInfo::GetUnderlined()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#ac12bb56cfe22d373826515a27b94cce1)
    fn get_underlined(&self) -> bool {
        unsafe { ffi::wxNativeFontInfo_GetUnderlined(self.as_ptr()) }
    }
    ///
    /// [See `wxNativeFontInfo::GetFaceName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a6639077c3ea2c3c8939ffd2d4d17abea)
    fn get_face_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_GetFaceName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFamily()
    // NOT_SUPPORTED: fn GetEncoding()
    ///
    /// [See `wxNativeFontInfo::SetPointSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a0042740b8192433d0e6e8f05b700e513)
    fn set_point_size(&self, pointsize: c_int) {
        unsafe { ffi::wxNativeFontInfo_SetPointSize(self.as_ptr(), pointsize) }
    }
    // NOT_SUPPORTED: fn SetFractionalPointSize()
    ///
    /// [See `wxNativeFontInfo::SetPixelSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a121e8354ba0e6c84622d3ed00922e289)
    fn set_pixel_size<S: SizeMethods>(&self, pixel_size: &S) {
        unsafe {
            let pixel_size = pixel_size.as_ptr();
            ffi::wxNativeFontInfo_SetPixelSize(self.as_ptr(), pixel_size)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    ///
    /// [See `wxNativeFontInfo::SetNumericWeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#ac207278a6ce8e8f9e1827e429e76f9d5)
    fn set_numeric_weight(&self, weight: c_int) {
        unsafe { ffi::wxNativeFontInfo_SetNumericWeight(self.as_ptr(), weight) }
    }
    // NOT_SUPPORTED: fn SetWeight()
    ///
    /// [See `wxNativeFontInfo::SetUnderlined()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a945abb6c70aee5256168c3de05968661)
    fn set_underlined(&self, underlined: bool) {
        unsafe { ffi::wxNativeFontInfo_SetUnderlined(self.as_ptr(), underlined) }
    }
    ///
    /// [See `wxNativeFontInfo::SetFaceName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#aba2f08b96bc40d67db5f539ee00c67df)
    fn set_face_name_str(&self, facename: &str) -> bool {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ffi::wxNativeFontInfo_SetFaceName(self.as_ptr(), facename)
        }
    }
    // NOT_SUPPORTED: fn SetFamily()
    // NOT_SUPPORTED: fn SetEncoding()
    ///
    /// [See `wxNativeFontInfo::SetFaceName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a92f0a96a3ef7b45035bacdb8432e2c4f)
    fn set_face_name_arraystring<A: ArrayStringMethods>(&self, facenames: &A) {
        unsafe {
            let facenames = facenames.as_ptr();
            ffi::wxNativeFontInfo_SetFaceName1(self.as_ptr(), facenames)
        }
    }
    ///
    /// [See `wxNativeFontInfo::FromString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#ab3004b4a00752735ca499f7ad861a21c)
    fn from_string(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxNativeFontInfo_FromString(self.as_ptr(), s)
        }
    }
    ///
    /// [See `wxNativeFontInfo::ToString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#ad99188dfbece2e9b26751d4536e275f8)
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_ToString(self.as_ptr())).into() }
    }
    ///
    /// [See `wxNativeFontInfo::FromUserString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a17b900c6fc3dd02f6883c162b90090c9)
    fn from_user_string(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxNativeFontInfo_FromUserString(self.as_ptr(), s)
        }
    }
    ///
    /// [See `wxNativeFontInfo::ToUserString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a79f7cde44955ec44b029fe39de047605)
    fn to_user_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_ToUserString(self.as_ptr())).into() }
    }
}

// wxNavigationKeyEvent
/// This trait represents C++ [`wxNavigationKeyEvent`](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html) class's methods and inheritance.
///
/// See [`NavigationKeyEventIsOwned`] documentation for the class usage.
pub trait NavigationKeyEventMethods: EventMethods {
    /// Returns the child that has the focus, or NULL.
    ///
    /// [See `wxNavigationKeyEvent::GetCurrentFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a1121082afcf4cb364394754207c5be45)
    fn get_current_focus(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxNavigationKeyEvent_GetCurrentFocus(self.as_ptr())) }
    }
    /// Returns true if the navigation was in the forward direction.
    ///
    /// [See `wxNavigationKeyEvent::GetDirection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a75da085de483ba8c2e38da09129b159e)
    fn get_direction(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_GetDirection(self.as_ptr()) }
    }
    /// Returns true if the navigation event was from a tab key.
    ///
    /// [See `wxNavigationKeyEvent::IsFromTab()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a93efc37f1b0782e426cd3478ffac7704)
    fn is_from_tab(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_IsFromTab(self.as_ptr()) }
    }
    /// Returns true if the navigation event represents a window change (for example, from Ctrl-Page Down in a notebook).
    ///
    /// [See `wxNavigationKeyEvent::IsWindowChange()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a8122774d94842478514ab41377136542)
    fn is_window_change(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_IsWindowChange(self.as_ptr()) }
    }
    /// Sets the current focus window member.
    ///
    /// [See `wxNavigationKeyEvent::SetCurrentFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a0b477f5fe9e89d241fba97010c60b7da)
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
    ///
    /// [See `wxNavigationKeyEvent::SetDirection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a675370a7c37cc27c43b6b6c2122184fb)
    fn set_direction(&self, direction: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetDirection(self.as_ptr(), direction) }
    }
    /// Sets the flags for this event.
    ///
    /// [See `wxNavigationKeyEvent::SetFlags()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#aace60aab1dad2b0e1e0671cef98925ad)
    fn set_flags(&self, flags: c_long) {
        unsafe { ffi::wxNavigationKeyEvent_SetFlags(self.as_ptr(), flags) }
    }
    /// Marks the navigation event as from a tab key.
    ///
    /// [See `wxNavigationKeyEvent::SetFromTab()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a282313445152c58d55702cca26422bb5)
    fn set_from_tab(&self, from_tab: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetFromTab(self.as_ptr(), from_tab) }
    }
    /// Marks the event as a window change event.
    ///
    /// [See `wxNavigationKeyEvent::SetWindowChange()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a355fccd120e343b6ebea4c75b39f098f)
    fn set_window_change(&self, window_change: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetWindowChange(self.as_ptr(), window_change) }
    }
}

// wxNonOwnedWindow
/// This trait represents C++ [`wxNonOwnedWindow`](https://docs.wxwidgets.org/3.2/classwx_non_owned_window.html) class's methods and inheritance.
///
/// See [`NonOwnedWindowIsOwned`] documentation for the class usage.
pub trait NonOwnedWindowMethods: WindowMethods {
    /// If the platform supports it, sets the shape of the window to that depicted by region.
    ///
    /// [See `wxNonOwnedWindow::SetShape()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_non_owned_window.html#a7f223eadbd72caea66efcb0b55e89613)
    fn set_shape_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxNonOwnedWindow_SetShape(self.as_ptr(), region)
        }
    }
    /// Set the window shape to the given path.
    ///
    /// [See `wxNonOwnedWindow::SetShape()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_non_owned_window.html#aa3c3e3fe90bd1b20407262157a593de7)
    fn set_shape_graphicspath<G: GraphicsPathMethods>(&self, path: &G) -> bool {
        unsafe {
            let path = path.as_ptr();
            ffi::wxNonOwnedWindow_SetShape1(self.as_ptr(), path)
        }
    }
}

// wxNotebook
/// This trait represents C++ [`wxNotebook`](https://docs.wxwidgets.org/3.2/classwx_notebook.html) class's methods and inheritance.
///
/// See [`NotebookIsOwned`] documentation for the class usage.
pub trait NotebookMethods: BookCtrlBaseMethods {
    // DTOR: fn ~wxNotebook()
    /// Returns the number of rows in the notebook control.
    ///
    /// [See `wxNotebook::GetRowCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notebook.html#aad3b293cccad6c70b4175f4977ae0bea)
    fn get_row_count(&self) -> c_int {
        unsafe { ffi::wxNotebook_GetRowCount(self.as_ptr()) }
    }
    /// If running under Windows and themes are enabled for the application, this function returns a suitable colour for painting the background of a notebook page, and can be passed to SetBackgroundColour().
    ///
    /// [See `wxNotebook::GetThemeBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notebook.html#a96aab3702683109bc7cabf478b887ae4)
    fn get_theme_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxNotebook_GetThemeBackgroundColour(self.as_ptr())) }
    }
    // BLOCKED: fn OnSelChange()
    /// Sets the amount of space around each page's icon and label, in pixels.
    ///
    /// [See `wxNotebook::SetPadding()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notebook.html#ad04a94e412e514319f187e7ec79e0815)
    fn set_padding<S: SizeMethods>(&self, padding: &S) {
        unsafe {
            let padding = padding.as_ptr();
            ffi::wxNotebook_SetPadding(self.as_ptr(), padding)
        }
    }
}

// wxNotificationMessage
/// This trait represents C++ [`wxNotificationMessage`](https://docs.wxwidgets.org/3.2/classwx_notification_message.html) class's methods and inheritance.
///
/// See [`NotificationMessageIsOwned`] documentation for the class usage.
pub trait NotificationMessageMethods: EvtHandlerMethods {
    // DTOR: fn ~wxNotificationMessage()
    /// Add an action to the notification.
    ///
    /// [See `wxNotificationMessage::AddAction()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#ac68288b8bedc6f0e35dd001e7d5a94f2)
    fn add_action(&self, actionid: c_int, label: &str) -> bool {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxNotificationMessage_AddAction(self.as_ptr(), actionid, label)
        }
    }
    /// Hides the notification.
    ///
    /// [See `wxNotificationMessage::Close()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#a657f2f18e7e72fc40d4993c44ae2f1dc)
    fn close(&self) -> bool {
        unsafe { ffi::wxNotificationMessage_Close(self.as_ptr()) }
    }
    /// This parameter can be currently used to specify the icon to show in the notification.
    ///
    /// [See `wxNotificationMessage::SetFlags()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#af55f3983bfd60ae746411dafed65257a)
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxNotificationMessage_SetFlags(self.as_ptr(), flags) }
    }
    /// Specify a custom icon to be displayed in the notification.
    ///
    /// [See `wxNotificationMessage::SetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#a94cff840f2cd59a5ed1178d978a17fbd)
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxNotificationMessage_SetIcon(self.as_ptr(), icon)
        }
    }
    /// Set the main text of the notification.
    ///
    /// [See `wxNotificationMessage::SetMessage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#acdc0e245b4391be704cced4c790e9e35)
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxNotificationMessage_SetMessage(self.as_ptr(), message)
        }
    }
    /// Set the parent for this notification: the notification will be associated with the top level parent of this window or, if this method is not called, with the main application window by default.
    ///
    /// [See `wxNotificationMessage::SetParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#a5b966d65b4c90b36c86fc80782cf8459)
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
    ///
    /// [See `wxNotificationMessage::SetTitle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#a7654f8e0764ac66c5dcf58b057453045)
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxNotificationMessage_SetTitle(self.as_ptr(), title)
        }
    }
    /// Show the notification to the user and hides it after timeout seconds are elapsed.
    ///
    /// [See `wxNotificationMessage::Show()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#a353452157bcf940406c1650c83fa3654)
    fn show(&self, timeout: c_int) -> bool {
        unsafe { ffi::wxNotificationMessage_Show(self.as_ptr(), timeout) }
    }
    /// If the application already uses a wxTaskBarIcon, it should be connected to notifications by using this method.
    ///
    /// [See `wxNotificationMessage::UseTaskBarIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#a05341a374a1e8537bf585862ac639482)
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
    ///
    /// [See `wxNotificationMessage::MSWUseToasts()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#a52c56483b083d771de4d0819f4795242)
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
/// This trait represents C++ [`wxNotifyEvent`](https://docs.wxwidgets.org/3.2/classwx_notify_event.html) class's methods and inheritance.
///
/// See [`NotifyEventIsOwned`] documentation for the class usage.
pub trait NotifyEventMethods: CommandEventMethods {
    /// This is the opposite of Veto(): it explicitly allows the event to be processed.
    ///
    /// [See `wxNotifyEvent::Allow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notify_event.html#a7b23515f827b2ce1da1e6990d9aafe80)
    fn allow(&self) {
        unsafe { ffi::wxNotifyEvent_Allow(self.as_ptr()) }
    }
    /// Returns true if the change is allowed (Veto() hasn't been called) or false otherwise (if it was).
    ///
    /// [See `wxNotifyEvent::IsAllowed()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notify_event.html#a907334f4b322d93cf49304a939bba143)
    fn is_allowed(&self) -> bool {
        unsafe { ffi::wxNotifyEvent_IsAllowed(self.as_ptr()) }
    }
    /// Prevents the change announced by this event from happening.
    ///
    /// [See `wxNotifyEvent::Veto()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_notify_event.html#a49e4502414a11e273f51a34a21d2acd3)
    fn veto(&self) {
        unsafe { ffi::wxNotifyEvent_Veto(self.as_ptr()) }
    }
}

// wxNumberEntryDialog
/// This trait represents C++ [`wxNumberEntryDialog`](https://docs.wxwidgets.org/3.2/classwx_number_entry_dialog.html) class's methods and inheritance.
///
/// See [`NumberEntryDialogIsOwned`] documentation for the class usage.
pub trait NumberEntryDialogMethods: DialogMethods {
    ///
    /// [See `wxNumberEntryDialog::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_number_entry_dialog.html#ab7dd25c596d540a0d31b96d04c87f08e)
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
    ///
    /// [See `wxNumberEntryDialog::GetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_number_entry_dialog.html#aefb7a8fdccb643da680265d028a9385d)
    fn get_value(&self) -> c_long {
        unsafe { ffi::wxNumberEntryDialog_GetValue(self.as_ptr()) }
    }
}
