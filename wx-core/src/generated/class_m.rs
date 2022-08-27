use super::*;

// wxMask
wxwidgets! {
    #[doc(alias = "wxMask")]
    #[doc(alias = "Mask")]
    class Mask = MaskIsOwned<true>(wxMask) impl
        MaskMethods,
        ObjectMethods
}
impl<const OWNED: bool> MaskIsOwned<OWNED> {
    pub fn new() -> MaskIsOwned<OWNED> {
        unsafe { MaskIsOwned(ffi::wxMask_new()) }
    }
    pub fn new_with_bitmap_int<B: BitmapMethods>(bitmap: &B, index: c_int) -> MaskIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskIsOwned(ffi::wxMask_new1(bitmap, index))
        }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MaskIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskIsOwned(ffi::wxMask_new2(bitmap))
        }
    }
    pub fn new_with_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        bitmap: &B,
        colour: &C,
    ) -> MaskIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let colour = colour.as_ptr();
            MaskIsOwned(ffi::wxMask_new3(bitmap, colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MaskIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MaskIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MaskIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MaskIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMask_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MaskIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMaximizeEvent
wxwidgets! {
    #[doc(alias = "wxMaximizeEvent")]
    #[doc(alias = "MaximizeEvent")]
    class MaximizeEvent = MaximizeEventIsOwned<true>(wxMaximizeEvent) impl
        MaximizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MaximizeEventIsOwned<OWNED> {
    pub fn new(id: c_int) -> MaximizeEventIsOwned<OWNED> {
        unsafe { MaximizeEventIsOwned(ffi::wxMaximizeEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MaximizeEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MaximizeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MaximizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MaximizeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MaximizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MaximizeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMaximizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MaximizeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMemoryDC
wxwidgets! {
    #[doc(alias = "wxMemoryDC")]
    #[doc(alias = "MemoryDC")]
    class MemoryDC = MemoryDCIsOwned<true>(wxMemoryDC) impl
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MemoryDCIsOwned<OWNED> {
    pub fn new() -> MemoryDCIsOwned<OWNED> {
        unsafe { MemoryDCIsOwned(ffi::wxMemoryDC_new()) }
    }
    pub fn new_with_dc<D: DCMethods>(dc: Option<&D>) -> MemoryDCIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MemoryDCIsOwned(ffi::wxMemoryDC_new1(dc))
        }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MemoryDCIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MemoryDCIsOwned(ffi::wxMemoryDC_new2(bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MemoryDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MemoryDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: MemoryDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MemoryDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MemoryDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MemoryDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMemoryDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MemoryDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMenu
wxwidgets! {
    #[doc(alias = "wxMenu")]
    #[doc(alias = "Menu")]
    class Menu = MenuIsOwned<true>(wxMenu) impl
        MenuMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuIsOwned<OWNED> {
    pub fn new() -> MenuIsOwned<OWNED> {
        unsafe { MenuIsOwned(ffi::wxMenu_new()) }
    }
    pub fn new_with_long(style: c_long) -> MenuIsOwned<OWNED> {
        unsafe { MenuIsOwned(ffi::wxMenu_new1(style)) }
    }
    pub fn new_with_str(title: &str, style: c_long) -> MenuIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            MenuIsOwned(ffi::wxMenu_new2(title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MenuIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MenuIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MenuIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MenuIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMenu_CLASSINFO()) }
    }
}

// wxMenuBar
wxwidgets! {
    #[doc(alias = "wxMenuBar")]
    #[doc(alias = "MenuBar")]
    class MenuBar = MenuBarIsOwned<true>(wxMenuBar) impl
        MenuBarMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuBarIsOwned<OWNED> {
    pub fn new(style: c_long) -> MenuBarIsOwned<OWNED> {
        unsafe { MenuBarIsOwned(ffi::wxMenuBar_new(style)) }
    }
    // NOT_SUPPORTED: fn wxMenuBar1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MenuBarIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MenuBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MenuBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MenuBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MenuBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMenuBar_CLASSINFO()) }
    }
}

// wxMenuEvent
wxwidgets! {
    #[doc(alias = "wxMenuEvent")]
    #[doc(alias = "MenuEvent")]
    class MenuEvent = MenuEventIsOwned<true>(wxMenuEvent) impl
        MenuEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxMenuEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MenuEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MenuEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMenuEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MenuEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMenuItem
wxwidgets! {
    #[doc(alias = "wxMenuItem")]
    #[doc(alias = "MenuItem")]
    class MenuItem = MenuItemIsOwned<true>(wxMenuItem) impl
        MenuItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuItemIsOwned<OWNED> {
    pub fn new<M: MenuMethods, M2: MenuMethods>(
        parent_menu: Option<&M>,
        id: c_int,
        text: &str,
        help_string: &str,
        kind: c_int,
        sub_menu: Option<&M2>,
    ) -> MenuItemIsOwned<OWNED> {
        unsafe {
            let parent_menu = match parent_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            let sub_menu = match sub_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItemIsOwned(ffi::wxMenuItem_new(
                parent_menu,
                id,
                text,
                help_string,
                kind,
                sub_menu,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MenuItemIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MenuItemIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MenuItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuItemIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMenuItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MenuItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMessageDialog
wxwidgets! {
    #[doc(alias = "wxMessageDialog")]
    #[doc(alias = "MessageDialog")]
    class MessageDialog = MessageDialogIsOwned<true>(wxMessageDialog) impl
        MessageDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MessageDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        style: c_long,
        pos: &P,
    ) -> MessageDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let pos = pos.as_ptr();
            MessageDialogIsOwned(ffi::wxMessageDialog_new(
                parent, message, caption, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MessageDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MessageDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMessageDialog_CLASSINFO()) }
    }
}

// wxMessageOutputMessageBox
wxwidgets! {
    #[doc(alias = "wxMessageOutputMessageBox")]
    #[doc(alias = "MessageOutputMessageBox")]
    class MessageOutputMessageBox = MessageOutputMessageBoxIsOwned<true>(wxMessageOutputMessageBox) impl
        MessageOutputMessageBoxMethods,
        MessageOutputMethods
}
impl<const OWNED: bool> MessageOutputMessageBoxIsOwned<OWNED> {
    pub fn new() -> MessageOutputMessageBoxIsOwned<OWNED> {
        unsafe { MessageOutputMessageBoxIsOwned(ffi::wxMessageOutputMessageBox_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MessageOutputMessageBoxIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MessageOutputMessageBoxIsOwned<OWNED>>
    for MessageOutputIsOwned<OWNED>
{
    fn from(o: MessageOutputMessageBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for MessageOutputMessageBoxIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMessageOutputMessageBox_delete(self.0) }
        }
    }
}

// wxMiniFrame
wxwidgets! {
    #[doc(alias = "wxMiniFrame")]
    #[doc(alias = "MiniFrame")]
    class MiniFrame = MiniFrameIsOwned<true>(wxMiniFrame) impl
        MiniFrameMethods,
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MiniFrameIsOwned<OWNED> {
    pub fn new_2step() -> MiniFrameIsOwned<OWNED> {
        unsafe { MiniFrameIsOwned(ffi::wxMiniFrame_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> MiniFrameIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            MiniFrameIsOwned(ffi::wxMiniFrame_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MiniFrameIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MiniFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMiniFrame_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for MiniFrameIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxMiniFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}

// wxMirrorDC
wxwidgets! {
    #[doc(alias = "wxMirrorDC")]
    #[doc(alias = "MirrorDC")]
    class MirrorDC = MirrorDCIsOwned<true>(wxMirrorDC) impl
        MirrorDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MirrorDCIsOwned<OWNED> {
    pub fn new<D: DCMethods>(dc: &D, mirror: bool) -> MirrorDCIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            MirrorDCIsOwned(ffi::wxMirrorDC_new(dc, mirror))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MirrorDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MirrorDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: MirrorDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MirrorDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MirrorDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MirrorDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMirrorDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MirrorDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseCaptureChangedEvent
wxwidgets! {
    #[doc(alias = "wxMouseCaptureChangedEvent")]
    #[doc(alias = "MouseCaptureChangedEvent")]
    class MouseCaptureChangedEvent = MouseCaptureChangedEventIsOwned<true>(wxMouseCaptureChangedEvent) impl
        MouseCaptureChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseCaptureChangedEventIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(
        window_id: c_int,
        gained_capture: Option<&W>,
    ) -> MouseCaptureChangedEventIsOwned<OWNED> {
        unsafe {
            let gained_capture = match gained_capture {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MouseCaptureChangedEventIsOwned(ffi::wxMouseCaptureChangedEvent_new(
                window_id,
                gained_capture,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseCaptureChangedEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MouseCaptureChangedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MouseCaptureChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseCaptureChangedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MouseCaptureChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseCaptureChangedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMouseCaptureChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseCaptureChangedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseCaptureLostEvent
wxwidgets! {
    #[doc(alias = "wxMouseCaptureLostEvent")]
    #[doc(alias = "MouseCaptureLostEvent")]
    class MouseCaptureLostEvent = MouseCaptureLostEventIsOwned<true>(wxMouseCaptureLostEvent) impl
        MouseCaptureLostEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseCaptureLostEventIsOwned<OWNED> {
    pub fn new(window_id: c_int) -> MouseCaptureLostEventIsOwned<OWNED> {
        unsafe { MouseCaptureLostEventIsOwned(ffi::wxMouseCaptureLostEvent_new(window_id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseCaptureLostEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MouseCaptureLostEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MouseCaptureLostEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseCaptureLostEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MouseCaptureLostEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseCaptureLostEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMouseCaptureLostEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseCaptureLostEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseEvent
wxwidgets! {
    #[doc(alias = "wxMouseEvent")]
    #[doc(alias = "MouseEvent")]
    class MouseEvent = MouseEventIsOwned<true>(wxMouseEvent) impl
        MouseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxMouseEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MouseEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MouseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MouseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMouseEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseEventsManager
wxwidgets! {
    #[doc(alias = "wxMouseEventsManager")]
    #[doc(alias = "MouseEventsManager")]
    class MouseEventsManager = MouseEventsManagerIsOwned<true>(wxMouseEventsManager) impl
        MouseEventsManagerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseEventsManagerIsOwned<OWNED> {
    // BLOCKED: fn wxMouseEventsManager()
    // BLOCKED: fn wxMouseEventsManager1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MouseEventsManagerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MouseEventsManagerIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MouseEventsManagerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseEventsManagerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MouseEventsManagerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseEventsManagerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMouseEventsManager_CLASSINFO()) }
    }
}

// wxMoveEvent
wxwidgets! {
    #[doc(alias = "wxMoveEvent")]
    #[doc(alias = "MoveEvent")]
    class MoveEvent = MoveEventIsOwned<true>(wxMoveEvent) impl
        MoveEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MoveEventIsOwned<OWNED> {
    pub fn new<P: PointMethods>(pt: &P, id: c_int) -> MoveEventIsOwned<OWNED> {
        unsafe {
            let pt = pt.as_ptr();
            MoveEventIsOwned(ffi::wxMoveEvent_new(pt, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MoveEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MoveEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MoveEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MoveEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MoveEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MoveEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMoveEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MoveEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
