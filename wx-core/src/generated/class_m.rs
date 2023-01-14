use super::*;

// wxMask
wxwidgets! {
    /// This class encapsulates a monochrome mask bitmap, where the masked area is black and the unmasked area is white.
    /// - [`Mask`] represents a C++ `wxMask` class instance which your code has ownership, [`MaskFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Mask`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMask` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html) for more details.
    #[doc(alias = "wxMask")]
    #[doc(alias = "Mask")]
    class Mask
        = MaskFromCpp<false>(wxMask) impl
        MaskMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MaskFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxMask::wxMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html#aea603d0c86c663653d9c8d44db6b5a52).
    pub fn new() -> MaskFromCpp<FROM_CPP> {
        unsafe { MaskFromCpp(ffi::wxMask_new()) }
    }
    /// Constructs a mask from a bitmap and a palette index that indicates the background.
    ///
    /// See [C++ `wxMask::wxMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html#a79aeae2ab235833def3e83bc8284bf10).
    pub fn new_with_bitmap_int<B: BitmapMethods>(
        bitmap: &B,
        index: c_int,
    ) -> MaskFromCpp<FROM_CPP> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskFromCpp(ffi::wxMask_new1(bitmap, index))
        }
    }
    /// Constructs a mask from a monochrome bitmap.
    ///
    /// See [C++ `wxMask::wxMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html#a42712a00244539d25bbc2535c0e6c6c4).
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MaskFromCpp<FROM_CPP> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskFromCpp(ffi::wxMask_new2(bitmap))
        }
    }
    /// Constructs a mask from a bitmap and a colour that indicates the background.
    ///
    /// See [C++ `wxMask::wxMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mask.html#a4ee5ba148d135c1c512aab5d099b5dc5).
    pub fn new_with_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        bitmap: &B,
        colour: &C,
    ) -> MaskFromCpp<FROM_CPP> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let colour = colour.as_ptr();
            MaskFromCpp(ffi::wxMask_new3(bitmap, colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MaskFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MaskFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MaskFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MaskFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMask_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MaskFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMaximizeEvent
wxwidgets! {
    /// An event being sent when a top level window is maximized.
    /// - [`MaximizeEvent`] represents a C++ `wxMaximizeEvent` class instance which your code has ownership, [`MaximizeEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MaximizeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMaximizeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_maximize_event.html) for more details.
    #[doc(alias = "wxMaximizeEvent")]
    #[doc(alias = "MaximizeEvent")]
    class MaximizeEvent
        = MaximizeEventFromCpp<false>(wxMaximizeEvent) impl
        MaximizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MaximizeEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxMaximizeEvent::wxMaximizeEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_maximize_event.html#ac10b3d281efd119efcd700af7ed902f5).
    pub fn new(id: c_int) -> MaximizeEventFromCpp<FROM_CPP> {
        unsafe { MaximizeEventFromCpp(ffi::wxMaximizeEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MaximizeEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MaximizeEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: MaximizeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MaximizeEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MaximizeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MaximizeEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMaximizeEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MaximizeEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMemoryDC
wxwidgets! {
    /// A memory device context provides a means to draw graphics onto a bitmap.
    /// - [`MemoryDC`] represents a C++ `wxMemoryDC` class instance which your code has ownership, [`MemoryDCFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MemoryDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMemoryDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html) for more details.
    #[doc(alias = "wxMemoryDC")]
    #[doc(alias = "MemoryDC")]
    class MemoryDC
        = MemoryDCFromCpp<false>(wxMemoryDC) impl
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MemoryDCFromCpp<FROM_CPP> {
    /// Constructs a new memory device context.
    ///
    /// See [C++ `wxMemoryDC::wxMemoryDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#a381e4c13a2df5b4915789b26fe43fd8b).
    pub fn new() -> MemoryDCFromCpp<FROM_CPP> {
        unsafe { MemoryDCFromCpp(ffi::wxMemoryDC_new()) }
    }
    /// Constructs a new memory device context having the same characteristics as the given existing device context.
    ///
    /// See [C++ `wxMemoryDC::wxMemoryDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#aaa30d5e8f9caa2a3a727296226488a8d).
    pub fn new_with_dc<D: DCMethods>(dc: Option<&D>) -> MemoryDCFromCpp<FROM_CPP> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MemoryDCFromCpp(ffi::wxMemoryDC_new1(dc))
        }
    }
    /// Constructs a new memory device context and calls SelectObject() with the given bitmap.
    ///
    /// See [C++ `wxMemoryDC::wxMemoryDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#ab6d6febe55bb6fbbac655cdaf1a719d2).
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MemoryDCFromCpp<FROM_CPP> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MemoryDCFromCpp(ffi::wxMemoryDC_new2(bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MemoryDCFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MemoryDCFromCpp<FROM_CPP>> for DCFromCpp<FROM_CPP> {
    fn from(o: MemoryDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MemoryDCFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MemoryDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MemoryDCFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMemoryDC_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MemoryDCFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMenu
wxwidgets! {
    /// A menu is a popup (or pull down) list of items, one of which may be selected before the menu goes away (clicking elsewhere dismisses the menu).
    /// - [`Menu`] represents a C++ `wxMenu` class instance which your code has ownership, [`MenuFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Menu`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMenu` class's documentation](https://docs.wxwidgets.org/3.2/classwx_menu.html) for more details.
    #[doc(alias = "wxMenu")]
    #[doc(alias = "Menu")]
    class Menu
        = MenuFromCpp<false>(wxMenu) impl
        MenuMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MenuFromCpp<FROM_CPP> {
    /// Constructs a wxMenu object.
    ///
    /// See [C++ `wxMenu::wxMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu.html#af8b5588bdd1ef7c94ac542adf91915ed).
    pub fn new() -> MenuFromCpp<FROM_CPP> {
        unsafe { MenuFromCpp(ffi::wxMenu_new()) }
    }
    /// Constructs a wxMenu object.
    ///
    /// See [C++ `wxMenu::wxMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu.html#a462f0ad758ef3d32b8a6a2a1d9c5c43f).
    pub fn new_with_long(style: c_long) -> MenuFromCpp<FROM_CPP> {
        unsafe { MenuFromCpp(ffi::wxMenu_new1(style)) }
    }
    /// Constructs a wxMenu object with a title.
    ///
    /// See [C++ `wxMenu::wxMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu.html#a9a6afd553257ebd8040305f21ec6094e).
    pub fn new_with_str(title: &str, style: c_long) -> MenuFromCpp<FROM_CPP> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            MenuFromCpp(ffi::wxMenu_new2(title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for MenuFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MenuFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: MenuFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MenuFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MenuFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MenuFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMenu_CLASSINFO()) }
    }
}

// wxMenuBar
wxwidgets! {
    /// A menu bar is a series of menus accessible from the top of a frame.
    /// - [`MenuBar`] represents a C++ `wxMenuBar` class instance which your code has ownership, [`MenuBarFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MenuBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMenuBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html) for more details.
    #[doc(alias = "wxMenuBar")]
    #[doc(alias = "MenuBar")]
    class MenuBar
        = MenuBarFromCpp<false>(wxMenuBar) impl
        MenuBarMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MenuBarFromCpp<FROM_CPP> {
    /// Construct an empty menu bar.
    ///
    /// See [C++ `wxMenuBar::wxMenuBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#aebc5627ed35e364d6a9785e22c0dde85).
    pub fn new(style: c_long) -> MenuBarFromCpp<FROM_CPP> {
        unsafe { MenuBarFromCpp(ffi::wxMenuBar_new(style)) }
    }
    // NOT_SUPPORTED: fn wxMenuBar1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for MenuBarFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MenuBarFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: MenuBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MenuBarFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: MenuBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MenuBarFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MenuBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MenuBarFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMenuBar_CLASSINFO()) }
    }
}

// wxMenuEvent
wxwidgets! {
    /// This class is used for a variety of menu-related events.
    /// - [`MenuEvent`] represents a C++ `wxMenuEvent` class instance which your code has ownership, [`MenuEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MenuEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMenuEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_menu_event.html) for more details.
    #[doc(alias = "wxMenuEvent")]
    #[doc(alias = "MenuEvent")]
    class MenuEvent
        = MenuEventFromCpp<false>(wxMenuEvent) impl
        MenuEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MenuEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxMenuEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MenuEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MenuEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: MenuEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MenuEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MenuEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MenuEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMenuEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MenuEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMenuItem
wxwidgets! {
    /// A menu item represents an item in a menu.
    /// - [`MenuItem`] represents a C++ `wxMenuItem` class instance which your code has ownership, [`MenuItemFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MenuItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMenuItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_menu_item.html) for more details.
    #[doc(alias = "wxMenuItem")]
    #[doc(alias = "MenuItem")]
    class MenuItem
        = MenuItemFromCpp<false>(wxMenuItem) impl
        MenuItemMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MenuItemFromCpp<FROM_CPP> {
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
    ) -> MenuItemFromCpp<FROM_CPP> {
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
            MenuItemFromCpp(ffi::wxMenuItem_new(
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
impl Clone for MenuItemFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MenuItemFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MenuItemFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MenuItemFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMenuItem_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MenuItemFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMessageDialog
wxwidgets! {
    /// This class represents a dialog that shows a single or multi-line message, with a choice of OK, Yes, No and Cancel buttons.
    /// - [`MessageDialog`] represents a C++ `wxMessageDialog` class instance which your code has ownership, [`MessageDialogFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MessageDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMessageDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html) for more details.
    #[doc(alias = "wxMessageDialog")]
    #[doc(alias = "MessageDialog")]
    class MessageDialog
        = MessageDialogFromCpp<false>(wxMessageDialog) impl
        MessageDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MessageDialogFromCpp<FROM_CPP> {
    /// Constructor specifying the message box properties.
    ///
    /// See [C++ `wxMessageDialog::wxMessageDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a444cfa18ba33da6cfe81d6a93c737d24).
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        style: c_long,
        pos: &P,
    ) -> MessageDialogFromCpp<FROM_CPP> {
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
            MessageDialogFromCpp(ffi::wxMessageDialog_new(
                parent, message, caption, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for MessageDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MessageDialogFromCpp<FROM_CPP>> for DialogFromCpp<FROM_CPP> {
    fn from(o: MessageDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MessageDialogFromCpp<FROM_CPP>>
    for TopLevelWindowFromCpp<FROM_CPP>
{
    fn from(o: MessageDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MessageDialogFromCpp<FROM_CPP>>
    for NonOwnedWindowFromCpp<FROM_CPP>
{
    fn from(o: MessageDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MessageDialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: MessageDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MessageDialogFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: MessageDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MessageDialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MessageDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MessageDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMessageDialog_CLASSINFO()) }
    }
}

// wxMessageOutputMessageBox
wxwidgets! {
    /// Output messages by showing them in a message box.
    /// - [`MessageOutputMessageBox`] represents a C++ `wxMessageOutputMessageBox` class instance which your code has ownership, [`MessageOutputMessageBoxFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MessageOutputMessageBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMessageOutputMessageBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_message_output_message_box.html) for more details.
    #[doc(alias = "wxMessageOutputMessageBox")]
    #[doc(alias = "MessageOutputMessageBox")]
    class MessageOutputMessageBox
        = MessageOutputMessageBoxFromCpp<false>(wxMessageOutputMessageBox) impl
        MessageOutputMessageBoxMethods,
        MessageOutputMethods
}
impl<const FROM_CPP: bool> MessageOutputMessageBoxFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxMessageOutputMessageBox::wxMessageOutputMessageBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_message_output_message_box.html#a3ce6041a24205714d3a446f43850d08b).
    pub fn new() -> MessageOutputMessageBoxFromCpp<FROM_CPP> {
        unsafe { MessageOutputMessageBoxFromCpp(ffi::wxMessageOutputMessageBox_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MessageOutputMessageBoxFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MessageOutputMessageBoxFromCpp<FROM_CPP>>
    for MessageOutputFromCpp<FROM_CPP>
{
    fn from(o: MessageOutputMessageBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for MessageOutputMessageBoxFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxMessageOutputMessageBox_delete(self.0) }
        }
    }
}

// wxMiniFrame
wxwidgets! {
    /// A miniframe is a frame with a small title bar.
    /// - [`MiniFrame`] represents a C++ `wxMiniFrame` class instance which your code has ownership, [`MiniFrameFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MiniFrame`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMiniFrame` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html) for more details.
    #[doc(alias = "wxMiniFrame")]
    #[doc(alias = "MiniFrame")]
    class MiniFrame
        = MiniFrameFromCpp<false>(wxMiniFrame) impl
        MiniFrameMethods,
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MiniFrameFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxMiniFrame::wxMiniFrame()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html#a10802f0d5e950024ea3d234974675679).
    pub fn new_2step() -> MiniFrameFromCpp<FROM_CPP> {
        unsafe { MiniFrameFromCpp(ffi::wxMiniFrame_new()) }
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
    ) -> MiniFrameFromCpp<FROM_CPP> {
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
            MiniFrameFromCpp(ffi::wxMiniFrame_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for MiniFrameFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MiniFrameFromCpp<FROM_CPP>> for FrameFromCpp<FROM_CPP> {
    fn from(o: MiniFrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MiniFrameFromCpp<FROM_CPP>> for TopLevelWindowFromCpp<FROM_CPP> {
    fn from(o: MiniFrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MiniFrameFromCpp<FROM_CPP>> for NonOwnedWindowFromCpp<FROM_CPP> {
    fn from(o: MiniFrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MiniFrameFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: MiniFrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MiniFrameFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: MiniFrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MiniFrameFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MiniFrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MiniFrameFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMiniFrame_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> TopLevelWindowMethods for MiniFrameFromCpp<FROM_CPP> {
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
    /// - [`MirrorDC`] represents a C++ `wxMirrorDC` class instance which your code has ownership, [`MirrorDCFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MirrorDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMirrorDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mirror_d_c.html) for more details.
    #[doc(alias = "wxMirrorDC")]
    #[doc(alias = "MirrorDC")]
    class MirrorDC
        = MirrorDCFromCpp<false>(wxMirrorDC) impl
        MirrorDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MirrorDCFromCpp<FROM_CPP> {
    /// Creates a (maybe) mirrored DC associated with the real dc.
    ///
    /// See [C++ `wxMirrorDC::wxMirrorDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mirror_d_c.html#a9d3738dc3c28391d9bdcfaabf7bc1ed4).
    pub fn new<D: DCMethods>(dc: &D, mirror: bool) -> MirrorDCFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            MirrorDCFromCpp(ffi::wxMirrorDC_new(dc, mirror))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MirrorDCFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MirrorDCFromCpp<FROM_CPP>> for DCFromCpp<FROM_CPP> {
    fn from(o: MirrorDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MirrorDCFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MirrorDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MirrorDCFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMirrorDC_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MirrorDCFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseCaptureChangedEvent
wxwidgets! {
    /// A mouse capture changed event is sent to a window that loses its mouse capture.
    /// - [`MouseCaptureChangedEvent`] represents a C++ `wxMouseCaptureChangedEvent` class instance which your code has ownership, [`MouseCaptureChangedEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MouseCaptureChangedEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMouseCaptureChangedEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_changed_event.html) for more details.
    #[doc(alias = "wxMouseCaptureChangedEvent")]
    #[doc(alias = "MouseCaptureChangedEvent")]
    class MouseCaptureChangedEvent
        = MouseCaptureChangedEventFromCpp<false>(wxMouseCaptureChangedEvent) impl
        MouseCaptureChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MouseCaptureChangedEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxMouseCaptureChangedEvent::wxMouseCaptureChangedEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_changed_event.html#aefe355de68756bc9e396bca92d9a65c7).
    pub fn new<W: WindowMethods>(
        window_id: c_int,
        gained_capture: Option<&W>,
    ) -> MouseCaptureChangedEventFromCpp<FROM_CPP> {
        unsafe {
            let gained_capture = match gained_capture {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MouseCaptureChangedEventFromCpp(ffi::wxMouseCaptureChangedEvent_new(
                window_id,
                gained_capture,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseCaptureChangedEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MouseCaptureChangedEventFromCpp<FROM_CPP>>
    for EventFromCpp<FROM_CPP>
{
    fn from(o: MouseCaptureChangedEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MouseCaptureChangedEventFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: MouseCaptureChangedEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MouseCaptureChangedEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMouseCaptureChangedEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MouseCaptureChangedEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseCaptureLostEvent
wxwidgets! {
    /// A mouse capture lost event is sent to a window that had obtained mouse capture, which was subsequently lost due to an "external" event (for example, when a dialog box is shown or if another application captures the mouse).
    /// - [`MouseCaptureLostEvent`] represents a C++ `wxMouseCaptureLostEvent` class instance which your code has ownership, [`MouseCaptureLostEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MouseCaptureLostEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMouseCaptureLostEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_lost_event.html) for more details.
    #[doc(alias = "wxMouseCaptureLostEvent")]
    #[doc(alias = "MouseCaptureLostEvent")]
    class MouseCaptureLostEvent
        = MouseCaptureLostEventFromCpp<false>(wxMouseCaptureLostEvent) impl
        MouseCaptureLostEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MouseCaptureLostEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxMouseCaptureLostEvent::wxMouseCaptureLostEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_lost_event.html#aa769021cc73f8d63a752143f2e6467e9).
    pub fn new(window_id: c_int) -> MouseCaptureLostEventFromCpp<FROM_CPP> {
        unsafe { MouseCaptureLostEventFromCpp(ffi::wxMouseCaptureLostEvent_new(window_id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseCaptureLostEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MouseCaptureLostEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: MouseCaptureLostEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MouseCaptureLostEventFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: MouseCaptureLostEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MouseCaptureLostEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMouseCaptureLostEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MouseCaptureLostEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseEvent
wxwidgets! {
    /// This event class contains information about the events generated by the mouse: they include mouse buttons press and release events and mouse move events.
    /// - [`MouseEvent`] represents a C++ `wxMouseEvent` class instance which your code has ownership, [`MouseEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MouseEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMouseEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html) for more details.
    #[doc(alias = "wxMouseEvent")]
    #[doc(alias = "MouseEvent")]
    class MouseEvent
        = MouseEventFromCpp<false>(wxMouseEvent) impl
        MouseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MouseEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxMouseEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MouseEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MouseEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: MouseEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MouseEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MouseEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MouseEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMouseEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MouseEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseEventsManager
wxwidgets! {
    /// Helper for handling mouse input events in windows containing multiple items.
    /// - [`MouseEventsManager`] represents a C++ `wxMouseEventsManager` class instance which your code has ownership, [`MouseEventsManagerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MouseEventsManager`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMouseEventsManager` class's documentation](https://docs.wxwidgets.org/3.2/classwx_mouse_events_manager.html) for more details.
    #[doc(alias = "wxMouseEventsManager")]
    #[doc(alias = "MouseEventsManager")]
    class MouseEventsManager
        = MouseEventsManagerFromCpp<false>(wxMouseEventsManager) impl
        MouseEventsManagerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MouseEventsManagerFromCpp<FROM_CPP> {
    // BLOCKED: fn wxMouseEventsManager()
    // BLOCKED: fn wxMouseEventsManager1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for MouseEventsManagerFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MouseEventsManagerFromCpp<FROM_CPP>>
    for EvtHandlerFromCpp<FROM_CPP>
{
    fn from(o: MouseEventsManagerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MouseEventsManagerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MouseEventsManagerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MouseEventsManagerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMouseEventsManager_CLASSINFO()) }
    }
}

// wxMoveEvent
wxwidgets! {
    /// A move event holds information about window position change.
    /// - [`MoveEvent`] represents a C++ `wxMoveEvent` class instance which your code has ownership, [`MoveEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`MoveEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxMoveEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_move_event.html) for more details.
    #[doc(alias = "wxMoveEvent")]
    #[doc(alias = "MoveEvent")]
    class MoveEvent
        = MoveEventFromCpp<false>(wxMoveEvent) impl
        MoveEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> MoveEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxMoveEvent::wxMoveEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_move_event.html#a420c53b7c6d48578a638f9b3d3a208ad).
    pub fn new<P: PointMethods>(pt: &P, id: c_int) -> MoveEventFromCpp<FROM_CPP> {
        unsafe {
            let pt = pt.as_ptr();
            MoveEventFromCpp(ffi::wxMoveEvent_new(pt, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MoveEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<MoveEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: MoveEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<MoveEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: MoveEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for MoveEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxMoveEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for MoveEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
