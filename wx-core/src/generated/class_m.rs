use super::*;

// wxMask
wxwidgets! {
    /// This class encapsulates a monochrome mask bitmap, where the masked area is black and the unmasked area is white.
    ///
    /// [See `wxMask`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html)
    #[doc(alias = "wxMask")]
    #[doc(alias = "Mask")]
    class Mask
        = MaskIsOwned<true>(wxMask) impl
        MaskMethods,
        ObjectMethods
}
impl<const OWNED: bool> MaskIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxMask::wxMask()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html#aea603d0c86c663653d9c8d44db6b5a52)
    pub fn new() -> MaskIsOwned<OWNED> {
        unsafe { MaskIsOwned(ffi::wxMask_new()) }
    }
    /// Constructs a mask from a bitmap and a palette index that indicates the background.
    ///
    /// [See `wxMask::wxMask()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html#a79aeae2ab235833def3e83bc8284bf10)
    pub fn new_with_bitmap_int<B: BitmapMethods>(bitmap: &B, index: c_int) -> MaskIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskIsOwned(ffi::wxMask_new1(bitmap, index))
        }
    }
    /// Constructs a mask from a monochrome bitmap.
    ///
    /// [See `wxMask::wxMask()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html#a42712a00244539d25bbc2535c0e6c6c4)
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MaskIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskIsOwned(ffi::wxMask_new2(bitmap))
        }
    }
    /// Constructs a mask from a bitmap and a colour that indicates the background.
    ///
    /// [See `wxMask::wxMask()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html#a4ee5ba148d135c1c512aab5d099b5dc5)
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
    /// An event being sent when a top level window is maximized.
    ///
    /// [See `wxMaximizeEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_maximize_event.html)
    #[doc(alias = "wxMaximizeEvent")]
    #[doc(alias = "MaximizeEvent")]
    class MaximizeEvent
        = MaximizeEventIsOwned<true>(wxMaximizeEvent) impl
        MaximizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MaximizeEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxMaximizeEvent::wxMaximizeEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_maximize_event.html#ac10b3d281efd119efcd700af7ed902f5)
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
    /// A memory device context provides a means to draw graphics onto a bitmap.
    ///
    /// [See `wxMemoryDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html)
    #[doc(alias = "wxMemoryDC")]
    #[doc(alias = "MemoryDC")]
    class MemoryDC
        = MemoryDCIsOwned<true>(wxMemoryDC) impl
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MemoryDCIsOwned<OWNED> {
    /// Constructs a new memory device context.
    ///
    /// [See `wxMemoryDC::wxMemoryDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#a381e4c13a2df5b4915789b26fe43fd8b)
    pub fn new() -> MemoryDCIsOwned<OWNED> {
        unsafe { MemoryDCIsOwned(ffi::wxMemoryDC_new()) }
    }
    /// Constructs a new memory device context having the same characteristics as the given existing device context.
    ///
    /// [See `wxMemoryDC::wxMemoryDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#aaa30d5e8f9caa2a3a727296226488a8d)
    pub fn new_with_dc<D: DCMethods>(dc: Option<&D>) -> MemoryDCIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MemoryDCIsOwned(ffi::wxMemoryDC_new1(dc))
        }
    }
    /// Constructs a new memory device context and calls SelectObject() with the given bitmap.
    ///
    /// [See `wxMemoryDC::wxMemoryDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#ab6d6febe55bb6fbbac655cdaf1a719d2)
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
    /// A menu is a popup (or pull down) list of items, one of which may be selected before the menu goes away (clicking elsewhere dismisses the menu).
    ///
    /// [See `wxMenu`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html)
    #[doc(alias = "wxMenu")]
    #[doc(alias = "Menu")]
    class Menu
        = MenuIsOwned<true>(wxMenu) impl
        MenuMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuIsOwned<OWNED> {
    /// Constructs a wxMenu object.
    ///
    /// [See `wxMenu::wxMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#af8b5588bdd1ef7c94ac542adf91915ed)
    pub fn new() -> MenuIsOwned<OWNED> {
        unsafe { MenuIsOwned(ffi::wxMenu_new()) }
    }
    /// Constructs a wxMenu object.
    ///
    /// [See `wxMenu::wxMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a462f0ad758ef3d32b8a6a2a1d9c5c43f)
    pub fn new_with_long(style: c_long) -> MenuIsOwned<OWNED> {
        unsafe { MenuIsOwned(ffi::wxMenu_new1(style)) }
    }
    /// Constructs a wxMenu object with a title.
    ///
    /// [See `wxMenu::wxMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a9a6afd553257ebd8040305f21ec6094e)
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
    /// A menu bar is a series of menus accessible from the top of a frame.
    ///
    /// [See `wxMenuBar`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html)
    #[doc(alias = "wxMenuBar")]
    #[doc(alias = "MenuBar")]
    class MenuBar
        = MenuBarIsOwned<true>(wxMenuBar) impl
        MenuBarMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuBarIsOwned<OWNED> {
    /// Construct an empty menu bar.
    ///
    /// [See `wxMenuBar::wxMenuBar()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#aebc5627ed35e364d6a9785e22c0dde85)
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
    /// This class is used for a variety of menu-related events.
    ///
    /// [See `wxMenuEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_event.html)
    #[doc(alias = "wxMenuEvent")]
    #[doc(alias = "MenuEvent")]
    class MenuEvent
        = MenuEventIsOwned<true>(wxMenuEvent) impl
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
    /// A menu item represents an item in a menu.
    ///
    /// [See `wxMenuItem`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html)
    #[doc(alias = "wxMenuItem")]
    #[doc(alias = "MenuItem")]
    class MenuItem
        = MenuItemIsOwned<true>(wxMenuItem) impl
        MenuItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuItemIsOwned<OWNED> {
    /// Constructs a wxMenuItem object.
    ///
    /// [See `wxMenuItem::wxMenuItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a6e9b0e1b786fa84250a42c88d84aed2b)
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
    /// This class represents a dialog that shows a single or multi-line message, with a choice of OK, Yes, No and Cancel buttons.
    ///
    /// [See `wxMessageDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html)
    #[doc(alias = "wxMessageDialog")]
    #[doc(alias = "MessageDialog")]
    class MessageDialog
        = MessageDialogIsOwned<true>(wxMessageDialog) impl
        MessageDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MessageDialogIsOwned<OWNED> {
    /// Constructor specifying the message box properties.
    ///
    /// [See `wxMessageDialog::wxMessageDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a444cfa18ba33da6cfe81d6a93c737d24)
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
    /// Output messages by showing them in a message box.
    ///
    /// [See `wxMessageOutputMessageBox`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_output_message_box.html)
    #[doc(alias = "wxMessageOutputMessageBox")]
    #[doc(alias = "MessageOutputMessageBox")]
    class MessageOutputMessageBox
        = MessageOutputMessageBoxIsOwned<true>(wxMessageOutputMessageBox) impl
        MessageOutputMessageBoxMethods,
        MessageOutputMethods
}
impl<const OWNED: bool> MessageOutputMessageBoxIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxMessageOutputMessageBox::wxMessageOutputMessageBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_output_message_box.html#a3ce6041a24205714d3a446f43850d08b)
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
    /// A miniframe is a frame with a small title bar.
    ///
    /// [See `wxMiniFrame`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html)
    #[doc(alias = "wxMiniFrame")]
    #[doc(alias = "MiniFrame")]
    class MiniFrame
        = MiniFrameIsOwned<true>(wxMiniFrame) impl
        MiniFrameMethods,
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MiniFrameIsOwned<OWNED> {
    /// Default ctor.
    ///
    /// [See `wxMiniFrame::wxMiniFrame()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html#a10802f0d5e950024ea3d234974675679)
    pub fn new_2step() -> MiniFrameIsOwned<OWNED> {
        unsafe { MiniFrameIsOwned(ffi::wxMiniFrame_new()) }
    }
    /// Constructor, creating the window.
    ///
    /// [See `wxMiniFrame::wxMiniFrame()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html#a19c1a9008b86260bd8e95c824df39cb9)
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
    /// Used in two-step frame construction.
    ///
    /// [See `wxMiniFrame::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html#a6091c19ff1f45bf4c0315b8bfbecc4a8)
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
    /// wxMirrorDC is a simple wrapper class which is always associated with a real wxDC object and either forwards all of its operations to it without changes (no mirroring takes place) or exchanges x and y coordinates which makes it possible to reuse the same code to draw a figure and its mirror  i.e.
    ///
    /// [See `wxMirrorDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mirror_d_c.html)
    #[doc(alias = "wxMirrorDC")]
    #[doc(alias = "MirrorDC")]
    class MirrorDC
        = MirrorDCIsOwned<true>(wxMirrorDC) impl
        MirrorDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MirrorDCIsOwned<OWNED> {
    /// Creates a (maybe) mirrored DC associated with the real dc.
    ///
    /// [See `wxMirrorDC::wxMirrorDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mirror_d_c.html#a9d3738dc3c28391d9bdcfaabf7bc1ed4)
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
    /// A mouse capture changed event is sent to a window that loses its mouse capture.
    ///
    /// [See `wxMouseCaptureChangedEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_changed_event.html)
    #[doc(alias = "wxMouseCaptureChangedEvent")]
    #[doc(alias = "MouseCaptureChangedEvent")]
    class MouseCaptureChangedEvent
        = MouseCaptureChangedEventIsOwned<true>(wxMouseCaptureChangedEvent) impl
        MouseCaptureChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseCaptureChangedEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxMouseCaptureChangedEvent::wxMouseCaptureChangedEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_changed_event.html#aefe355de68756bc9e396bca92d9a65c7)
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
    /// A mouse capture lost event is sent to a window that had obtained mouse capture, which was subsequently lost due to an "external" event (for example, when a dialog box is shown or if another application captures the mouse).
    ///
    /// [See `wxMouseCaptureLostEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_lost_event.html)
    #[doc(alias = "wxMouseCaptureLostEvent")]
    #[doc(alias = "MouseCaptureLostEvent")]
    class MouseCaptureLostEvent
        = MouseCaptureLostEventIsOwned<true>(wxMouseCaptureLostEvent) impl
        MouseCaptureLostEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseCaptureLostEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxMouseCaptureLostEvent::wxMouseCaptureLostEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_lost_event.html#aa769021cc73f8d63a752143f2e6467e9)
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
    /// This event class contains information about the events generated by the mouse: they include mouse buttons press and release events and mouse move events.
    ///
    /// [See `wxMouseEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html)
    #[doc(alias = "wxMouseEvent")]
    #[doc(alias = "MouseEvent")]
    class MouseEvent
        = MouseEventIsOwned<true>(wxMouseEvent) impl
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
    /// Helper for handling mouse input events in windows containing multiple items.
    ///
    /// [See `wxMouseEventsManager`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_events_manager.html)
    #[doc(alias = "wxMouseEventsManager")]
    #[doc(alias = "MouseEventsManager")]
    class MouseEventsManager
        = MouseEventsManagerIsOwned<true>(wxMouseEventsManager) impl
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
    /// A move event holds information about window position change.
    ///
    /// [See `wxMoveEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_move_event.html)
    #[doc(alias = "wxMoveEvent")]
    #[doc(alias = "MoveEvent")]
    class MoveEvent
        = MoveEventIsOwned<true>(wxMoveEvent) impl
        MoveEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MoveEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxMoveEvent::wxMoveEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_move_event.html#a420c53b7c6d48578a638f9b3d3a208ad)
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
