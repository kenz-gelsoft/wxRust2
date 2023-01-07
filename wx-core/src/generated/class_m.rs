use super::*;

// wxMask
wxwidgets! {
    /// This class encapsulates a monochrome mask bitmap, where the masked area is black and the unmasked area is white.
    /// - [`Mask`] represents a C++ `wxMask` class instance which your code has ownership, [`MaskInRust`]`<false>` represents one which don't own.
    /// - Use [`Mask`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMask` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html) for more details.
    #[doc(alias = "wxMask")]
    #[doc(alias = "Mask")]
    class Mask
        = MaskInRust<true>(wxMask) impl
        MaskMethods,
        ObjectMethods
}
impl<const OWNED: bool> MaskInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxMask::wxMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html#aea603d0c86c663653d9c8d44db6b5a52).
    pub fn new() -> MaskInRust<OWNED> {
        unsafe { MaskInRust(ffi::wxMask_new()) }
    }
    /// Constructs a mask from a bitmap and a palette index that indicates the background.
    ///
    /// See [C++ `wxMask::wxMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html#a79aeae2ab235833def3e83bc8284bf10).
    pub fn new_with_bitmap_int<B: BitmapMethods>(bitmap: &B, index: c_int) -> MaskInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskInRust(ffi::wxMask_new1(bitmap, index))
        }
    }
    /// Constructs a mask from a monochrome bitmap.
    ///
    /// See [C++ `wxMask::wxMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html#a42712a00244539d25bbc2535c0e6c6c4).
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MaskInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskInRust(ffi::wxMask_new2(bitmap))
        }
    }
    /// Constructs a mask from a bitmap and a colour that indicates the background.
    ///
    /// See [C++ `wxMask::wxMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html#a4ee5ba148d135c1c512aab5d099b5dc5).
    pub fn new_with_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        bitmap: &B,
        colour: &C,
    ) -> MaskInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let colour = colour.as_ptr();
            MaskInRust(ffi::wxMask_new3(bitmap, colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MaskInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MaskInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MaskInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MaskInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMask_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MaskInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMaximizeEvent
wxwidgets! {
    /// An event being sent when a top level window is maximized.
    /// - [`MaximizeEvent`] represents a C++ `wxMaximizeEvent` class instance which your code has ownership, [`MaximizeEventInRust`]`<false>` represents one which don't own.
    /// - Use [`MaximizeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMaximizeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_maximize_event.html) for more details.
    #[doc(alias = "wxMaximizeEvent")]
    #[doc(alias = "MaximizeEvent")]
    class MaximizeEvent
        = MaximizeEventInRust<true>(wxMaximizeEvent) impl
        MaximizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MaximizeEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxMaximizeEvent::wxMaximizeEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_maximize_event.html#ac10b3d281efd119efcd700af7ed902f5).
    pub fn new(id: c_int) -> MaximizeEventInRust<OWNED> {
        unsafe { MaximizeEventInRust(ffi::wxMaximizeEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MaximizeEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MaximizeEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: MaximizeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MaximizeEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MaximizeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MaximizeEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMaximizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MaximizeEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMemoryDC
wxwidgets! {
    /// A memory device context provides a means to draw graphics onto a bitmap.
    /// - [`MemoryDC`] represents a C++ `wxMemoryDC` class instance which your code has ownership, [`MemoryDCInRust`]`<false>` represents one which don't own.
    /// - Use [`MemoryDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMemoryDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html) for more details.
    #[doc(alias = "wxMemoryDC")]
    #[doc(alias = "MemoryDC")]
    class MemoryDC
        = MemoryDCInRust<true>(wxMemoryDC) impl
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MemoryDCInRust<OWNED> {
    /// Constructs a new memory device context.
    ///
    /// See [C++ `wxMemoryDC::wxMemoryDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#a381e4c13a2df5b4915789b26fe43fd8b).
    pub fn new() -> MemoryDCInRust<OWNED> {
        unsafe { MemoryDCInRust(ffi::wxMemoryDC_new()) }
    }
    /// Constructs a new memory device context having the same characteristics as the given existing device context.
    ///
    /// See [C++ `wxMemoryDC::wxMemoryDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#aaa30d5e8f9caa2a3a727296226488a8d).
    pub fn new_with_dc<D: DCMethods>(dc: Option<&D>) -> MemoryDCInRust<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MemoryDCInRust(ffi::wxMemoryDC_new1(dc))
        }
    }
    /// Constructs a new memory device context and calls SelectObject() with the given bitmap.
    ///
    /// See [C++ `wxMemoryDC::wxMemoryDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#ab6d6febe55bb6fbbac655cdaf1a719d2).
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MemoryDCInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MemoryDCInRust(ffi::wxMemoryDC_new2(bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MemoryDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MemoryDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: MemoryDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MemoryDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MemoryDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MemoryDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMemoryDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MemoryDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMenu
wxwidgets! {
    /// A menu is a popup (or pull down) list of items, one of which may be selected before the menu goes away (clicking elsewhere dismisses the menu).
    /// - [`Menu`] represents a C++ `wxMenu` class instance which your code has ownership, [`MenuInRust`]`<false>` represents one which don't own.
    /// - Use [`Menu`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMenu` class's documentation](https://docs.wxwidgets.org/3.2/classwx_menu.html) for more details.
    #[doc(alias = "wxMenu")]
    #[doc(alias = "Menu")]
    class Menu
        = MenuInRust<true>(wxMenu) impl
        MenuMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuInRust<OWNED> {
    /// Constructs a wxMenu object.
    ///
    /// See [C++ `wxMenu::wxMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu.html#af8b5588bdd1ef7c94ac542adf91915ed).
    pub fn new() -> MenuInRust<OWNED> {
        unsafe { MenuInRust(ffi::wxMenu_new()) }
    }
    /// Constructs a wxMenu object.
    ///
    /// See [C++ `wxMenu::wxMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu.html#a462f0ad758ef3d32b8a6a2a1d9c5c43f).
    pub fn new_with_long(style: c_long) -> MenuInRust<OWNED> {
        unsafe { MenuInRust(ffi::wxMenu_new1(style)) }
    }
    /// Constructs a wxMenu object with a title.
    ///
    /// See [C++ `wxMenu::wxMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu.html#a9a6afd553257ebd8040305f21ec6094e).
    pub fn new_with_str(title: &str, style: c_long) -> MenuInRust<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            MenuInRust(ffi::wxMenu_new2(title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MenuInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MenuInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: MenuInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MenuInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMenu_CLASSINFO()) }
    }
}

// wxMenuBar
wxwidgets! {
    /// A menu bar is a series of menus accessible from the top of a frame.
    /// - [`MenuBar`] represents a C++ `wxMenuBar` class instance which your code has ownership, [`MenuBarInRust`]`<false>` represents one which don't own.
    /// - Use [`MenuBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMenuBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html) for more details.
    #[doc(alias = "wxMenuBar")]
    #[doc(alias = "MenuBar")]
    class MenuBar
        = MenuBarInRust<true>(wxMenuBar) impl
        MenuBarMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuBarInRust<OWNED> {
    /// Construct an empty menu bar.
    ///
    /// See [C++ `wxMenuBar::wxMenuBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#aebc5627ed35e364d6a9785e22c0dde85).
    pub fn new(style: c_long) -> MenuBarInRust<OWNED> {
        unsafe { MenuBarInRust(ffi::wxMenuBar_new(style)) }
    }
    // NOT_SUPPORTED: fn wxMenuBar1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MenuBarInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MenuBarInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: MenuBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuBarInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: MenuBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuBarInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MenuBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuBarInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMenuBar_CLASSINFO()) }
    }
}

// wxMenuEvent
wxwidgets! {
    /// This class is used for a variety of menu-related events.
    /// - [`MenuEvent`] represents a C++ `wxMenuEvent` class instance which your code has ownership, [`MenuEventInRust`]`<false>` represents one which don't own.
    /// - Use [`MenuEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMenuEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_menu_event.html) for more details.
    #[doc(alias = "wxMenuEvent")]
    #[doc(alias = "MenuEvent")]
    class MenuEvent
        = MenuEventInRust<true>(wxMenuEvent) impl
        MenuEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxMenuEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MenuEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MenuEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: MenuEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MenuEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMenuEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MenuEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMenuItem
wxwidgets! {
    /// A menu item represents an item in a menu.
    /// - [`MenuItem`] represents a C++ `wxMenuItem` class instance which your code has ownership, [`MenuItemInRust`]`<false>` represents one which don't own.
    /// - Use [`MenuItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMenuItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_menu_item.html) for more details.
    #[doc(alias = "wxMenuItem")]
    #[doc(alias = "MenuItem")]
    class MenuItem
        = MenuItemInRust<true>(wxMenuItem) impl
        MenuItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuItemInRust<OWNED> {
    /// Constructs a wxMenuItem object.
    ///
    /// See [C++ `wxMenuItem::wxMenuItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a6e9b0e1b786fa84250a42c88d84aed2b).
    pub fn new<M: MenuMethods, M2: MenuMethods>(
        parent_menu: Option<&M>,
        id: c_int,
        text: &str,
        help_string: &str,
        kind: c_int,
        sub_menu: Option<&M2>,
    ) -> MenuItemInRust<OWNED> {
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
            MenuItemInRust(ffi::wxMenuItem_new(
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
impl Clone for MenuItemInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MenuItemInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MenuItemInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuItemInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMenuItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MenuItemInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMessageDialog
wxwidgets! {
    /// This class represents a dialog that shows a single or multi-line message, with a choice of OK, Yes, No and Cancel buttons.
    /// - [`MessageDialog`] represents a C++ `wxMessageDialog` class instance which your code has ownership, [`MessageDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`MessageDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMessageDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html) for more details.
    #[doc(alias = "wxMessageDialog")]
    #[doc(alias = "MessageDialog")]
    class MessageDialog
        = MessageDialogInRust<true>(wxMessageDialog) impl
        MessageDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MessageDialogInRust<OWNED> {
    /// Constructor specifying the message box properties.
    ///
    /// See [C++ `wxMessageDialog::wxMessageDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a444cfa18ba33da6cfe81d6a93c737d24).
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        style: c_long,
        pos: &P,
    ) -> MessageDialogInRust<OWNED> {
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
            MessageDialogInRust(ffi::wxMessageDialog_new(
                parent, message, caption, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MessageDialogInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MessageDialogInRust<OWNED>> for DialogInRust<OWNED> {
    fn from(o: MessageDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: MessageDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: MessageDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: MessageDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: MessageDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MessageDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MessageDialogInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMessageDialog_CLASSINFO()) }
    }
}

// wxMessageOutputMessageBox
wxwidgets! {
    /// Output messages by showing them in a message box.
    /// - [`MessageOutputMessageBox`] represents a C++ `wxMessageOutputMessageBox` class instance which your code has ownership, [`MessageOutputMessageBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`MessageOutputMessageBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMessageOutputMessageBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_message_output_message_box.html) for more details.
    #[doc(alias = "wxMessageOutputMessageBox")]
    #[doc(alias = "MessageOutputMessageBox")]
    class MessageOutputMessageBox
        = MessageOutputMessageBoxInRust<true>(wxMessageOutputMessageBox) impl
        MessageOutputMessageBoxMethods,
        MessageOutputMethods
}
impl<const OWNED: bool> MessageOutputMessageBoxInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxMessageOutputMessageBox::wxMessageOutputMessageBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_message_output_message_box.html#a3ce6041a24205714d3a446f43850d08b).
    pub fn new() -> MessageOutputMessageBoxInRust<OWNED> {
        unsafe { MessageOutputMessageBoxInRust(ffi::wxMessageOutputMessageBox_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MessageOutputMessageBoxInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MessageOutputMessageBoxInRust<OWNED>> for MessageOutputInRust<OWNED> {
    fn from(o: MessageOutputMessageBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for MessageOutputMessageBoxInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMessageOutputMessageBox_delete(self.0) }
        }
    }
}

// wxMiniFrame
wxwidgets! {
    /// A miniframe is a frame with a small title bar.
    /// - [`MiniFrame`] represents a C++ `wxMiniFrame` class instance which your code has ownership, [`MiniFrameInRust`]`<false>` represents one which don't own.
    /// - Use [`MiniFrame`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMiniFrame` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html) for more details.
    #[doc(alias = "wxMiniFrame")]
    #[doc(alias = "MiniFrame")]
    class MiniFrame
        = MiniFrameInRust<true>(wxMiniFrame) impl
        MiniFrameMethods,
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MiniFrameInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxMiniFrame::wxMiniFrame()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html#a10802f0d5e950024ea3d234974675679).
    pub fn new_2step() -> MiniFrameInRust<OWNED> {
        unsafe { MiniFrameInRust(ffi::wxMiniFrame_new()) }
    }
    /// Constructor, creating the window.
    ///
    /// See [C++ `wxMiniFrame::wxMiniFrame()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html#a19c1a9008b86260bd8e95c824df39cb9).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> MiniFrameInRust<OWNED> {
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
            MiniFrameInRust(ffi::wxMiniFrame_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MiniFrameInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MiniFrameInRust<OWNED>> for FrameInRust<OWNED> {
    fn from(o: MiniFrameInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: MiniFrameInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: MiniFrameInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: MiniFrameInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: MiniFrameInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MiniFrameInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MiniFrameInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMiniFrame_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for MiniFrameInRust<OWNED> {
    /// Used in two-step frame construction.
    ///
    /// See [C++ `wxMiniFrame::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html#a6091c19ff1f45bf4c0315b8bfbecc4a8).
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
    /// - [`MirrorDC`] represents a C++ `wxMirrorDC` class instance which your code has ownership, [`MirrorDCInRust`]`<false>` represents one which don't own.
    /// - Use [`MirrorDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMirrorDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mirror_d_c.html) for more details.
    #[doc(alias = "wxMirrorDC")]
    #[doc(alias = "MirrorDC")]
    class MirrorDC
        = MirrorDCInRust<true>(wxMirrorDC) impl
        MirrorDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MirrorDCInRust<OWNED> {
    /// Creates a (maybe) mirrored DC associated with the real dc.
    ///
    /// See [C++ `wxMirrorDC::wxMirrorDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mirror_d_c.html#a9d3738dc3c28391d9bdcfaabf7bc1ed4).
    pub fn new<D: DCMethods>(dc: &D, mirror: bool) -> MirrorDCInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            MirrorDCInRust(ffi::wxMirrorDC_new(dc, mirror))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MirrorDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MirrorDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: MirrorDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MirrorDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MirrorDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MirrorDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMirrorDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MirrorDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseCaptureChangedEvent
wxwidgets! {
    /// A mouse capture changed event is sent to a window that loses its mouse capture.
    /// - [`MouseCaptureChangedEvent`] represents a C++ `wxMouseCaptureChangedEvent` class instance which your code has ownership, [`MouseCaptureChangedEventInRust`]`<false>` represents one which don't own.
    /// - Use [`MouseCaptureChangedEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMouseCaptureChangedEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_changed_event.html) for more details.
    #[doc(alias = "wxMouseCaptureChangedEvent")]
    #[doc(alias = "MouseCaptureChangedEvent")]
    class MouseCaptureChangedEvent
        = MouseCaptureChangedEventInRust<true>(wxMouseCaptureChangedEvent) impl
        MouseCaptureChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseCaptureChangedEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxMouseCaptureChangedEvent::wxMouseCaptureChangedEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_changed_event.html#aefe355de68756bc9e396bca92d9a65c7).
    pub fn new<W: WindowMethods>(
        window_id: c_int,
        gained_capture: Option<&W>,
    ) -> MouseCaptureChangedEventInRust<OWNED> {
        unsafe {
            let gained_capture = match gained_capture {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MouseCaptureChangedEventInRust(ffi::wxMouseCaptureChangedEvent_new(
                window_id,
                gained_capture,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseCaptureChangedEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MouseCaptureChangedEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: MouseCaptureChangedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseCaptureChangedEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MouseCaptureChangedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseCaptureChangedEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMouseCaptureChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseCaptureChangedEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseCaptureLostEvent
wxwidgets! {
    /// A mouse capture lost event is sent to a window that had obtained mouse capture, which was subsequently lost due to an "external" event (for example, when a dialog box is shown or if another application captures the mouse).
    /// - [`MouseCaptureLostEvent`] represents a C++ `wxMouseCaptureLostEvent` class instance which your code has ownership, [`MouseCaptureLostEventInRust`]`<false>` represents one which don't own.
    /// - Use [`MouseCaptureLostEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMouseCaptureLostEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_lost_event.html) for more details.
    #[doc(alias = "wxMouseCaptureLostEvent")]
    #[doc(alias = "MouseCaptureLostEvent")]
    class MouseCaptureLostEvent
        = MouseCaptureLostEventInRust<true>(wxMouseCaptureLostEvent) impl
        MouseCaptureLostEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseCaptureLostEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxMouseCaptureLostEvent::wxMouseCaptureLostEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_lost_event.html#aa769021cc73f8d63a752143f2e6467e9).
    pub fn new(window_id: c_int) -> MouseCaptureLostEventInRust<OWNED> {
        unsafe { MouseCaptureLostEventInRust(ffi::wxMouseCaptureLostEvent_new(window_id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseCaptureLostEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MouseCaptureLostEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: MouseCaptureLostEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseCaptureLostEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MouseCaptureLostEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseCaptureLostEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMouseCaptureLostEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseCaptureLostEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseEvent
wxwidgets! {
    /// This event class contains information about the events generated by the mouse: they include mouse buttons press and release events and mouse move events.
    /// - [`MouseEvent`] represents a C++ `wxMouseEvent` class instance which your code has ownership, [`MouseEventInRust`]`<false>` represents one which don't own.
    /// - Use [`MouseEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMouseEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html) for more details.
    #[doc(alias = "wxMouseEvent")]
    #[doc(alias = "MouseEvent")]
    class MouseEvent
        = MouseEventInRust<true>(wxMouseEvent) impl
        MouseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxMouseEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MouseEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: MouseEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MouseEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMouseEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseEventsManager
wxwidgets! {
    /// Helper for handling mouse input events in windows containing multiple items.
    /// - [`MouseEventsManager`] represents a C++ `wxMouseEventsManager` class instance which your code has ownership, [`MouseEventsManagerInRust`]`<false>` represents one which don't own.
    /// - Use [`MouseEventsManager`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMouseEventsManager` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_events_manager.html) for more details.
    #[doc(alias = "wxMouseEventsManager")]
    #[doc(alias = "MouseEventsManager")]
    class MouseEventsManager
        = MouseEventsManagerInRust<true>(wxMouseEventsManager) impl
        MouseEventsManagerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseEventsManagerInRust<OWNED> {
    // BLOCKED: fn wxMouseEventsManager()
    // BLOCKED: fn wxMouseEventsManager1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for MouseEventsManagerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MouseEventsManagerInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: MouseEventsManagerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseEventsManagerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MouseEventsManagerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseEventsManagerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMouseEventsManager_CLASSINFO()) }
    }
}

// wxMoveEvent
wxwidgets! {
    /// A move event holds information about window position change.
    /// - [`MoveEvent`] represents a C++ `wxMoveEvent` class instance which your code has ownership, [`MoveEventInRust`]`<false>` represents one which don't own.
    /// - Use [`MoveEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMoveEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_move_event.html) for more details.
    #[doc(alias = "wxMoveEvent")]
    #[doc(alias = "MoveEvent")]
    class MoveEvent
        = MoveEventInRust<true>(wxMoveEvent) impl
        MoveEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MoveEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxMoveEvent::wxMoveEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_move_event.html#a420c53b7c6d48578a638f9b3d3a208ad).
    pub fn new<P: PointMethods>(pt: &P, id: c_int) -> MoveEventInRust<OWNED> {
        unsafe {
            let pt = pt.as_ptr();
            MoveEventInRust(ffi::wxMoveEvent_new(pt, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MoveEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<MoveEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: MoveEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MoveEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: MoveEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MoveEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxMoveEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MoveEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
