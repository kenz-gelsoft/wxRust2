use super::*;

// wxMask
/// This class encapsulates a monochrome mask bitmap, where the masked area is black and the unmasked area is white.
///
/// [See `wxMask`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html)
pub trait MaskMethods: ObjectMethods {
    // DTOR: fn ~wxMask()
    /// Constructs a mask from a bitmap and a palette index that indicates the background.
    ///
    /// [See `wxMask::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html#a1caffe6896159b669c11c691dbf0142f)
    fn create_int<B: BitmapMethods>(&self, bitmap: &B, index: c_int) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMask_Create(self.as_ptr(), bitmap, index)
        }
    }
    /// Constructs a mask from a monochrome bitmap.
    ///
    /// [See `wxMask::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html#aa10e8c239e67f5daba39046a2959b483)
    fn create<B: BitmapMethods>(&self, bitmap: &B) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMask_Create1(self.as_ptr(), bitmap)
        }
    }
    /// Constructs a mask from a bitmap and a colour that indicates the background.
    ///
    /// [See `wxMask::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html#ac4181b0bb83866525004d94264cf9803)
    fn create_colour<B: BitmapMethods, C: ColourMethods>(&self, bitmap: &B, colour: &C) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxMask_Create2(self.as_ptr(), bitmap, colour)
        }
    }
    /// Returns the mask as a monochrome bitmap.
    ///
    /// [See `wxMask::GetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mask.html#af066abbe7a46fa595fbbce6e3b76a2d0)
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMask_GetBitmap(self.as_ptr())) }
    }
}

// wxMaximizeEvent
/// An event being sent when a top level window is maximized.
///
/// [See `wxMaximizeEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_maximize_event.html)
pub trait MaximizeEventMethods: EventMethods {}

// wxMemoryDC
/// A memory device context provides a means to draw graphics onto a bitmap.
///
/// [See `wxMemoryDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html)
pub trait MemoryDCMethods: DCMethods {
    /// Allow using this device context object to modify the given bitmap contents.
    ///
    /// [See `wxMemoryDC::SelectObject()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#a93d218796ba9359eb4aec2ae46a813e6)
    fn select_object<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMemoryDC_SelectObject(self.as_ptr(), bitmap)
        }
    }
    /// Selects the given bitmap into the device context, to use as the memory bitmap.
    ///
    /// [See `wxMemoryDC::SelectObjectAsSource()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#a148ceba1c29d4a78fca6026a90e2ee5f)
    fn select_object_as_source<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMemoryDC_SelectObjectAsSource(self.as_ptr(), bitmap)
        }
    }
    /// Get the selected bitmap.
    ///
    /// [See `wxMemoryDC::GetSelectedBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_memory_d_c.html#abfc29bdb24e4dba3dad63e8022030c11)
    fn get_selected_bitmap(&self) -> BitmapIsOwned<false> {
        unsafe { BitmapIsOwned::from_ptr(ffi::wxMemoryDC_GetSelectedBitmap(self.as_ptr())) }
    }
    // BLOCKED: fn GetSelectedBitmap1()
}

// wxMenu
/// A menu is a popup (or pull down) list of items, one of which may be selected before the menu goes away (clicking elsewhere dismisses the menu).
///
/// [See `wxMenu`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html)
pub trait MenuMethods: EvtHandlerMethods {
    // DTOR: fn ~wxMenu()
    /// Adds a menu item.
    ///
    /// [See `wxMenu::Append()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a5a71098cee12cd7ad9c26d92e218c590)
    fn append_int_str(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Append(
                self.as_ptr(),
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    /// Adds a submenu.
    ///
    /// [See `wxMenu::Append()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#afcd6a1a05161bd4c6c02656eca01eb1b)
    fn append_int_menu<M: MenuMethods>(
        &self,
        id: c_int,
        item: &str,
        sub_menu: Option<&M>,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let sub_menu = match sub_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Append1(
                self.as_ptr(),
                id,
                item,
                sub_menu,
                help_string,
            ))
        }
    }
    /// Adds a menu item object.
    ///
    /// [See `wxMenu::Append()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a00d10cb282bd06aca0243c02a2153ac6)
    fn append_menuitem<M: MenuItemMethods>(
        &self,
        menu_item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu_item = match menu_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Append2(self.as_ptr(), menu_item))
        }
    }
    /// Adds a checkable item to the end of the menu.
    ///
    /// [See `wxMenu::AppendCheckItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a24a34c612911df27e0bb3cae81bbdda0)
    fn append_check_item(
        &self,
        id: c_int,
        item: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_AppendCheckItem(self.as_ptr(), id, item, help))
        }
    }
    /// Adds a radio item to the end of the menu.
    ///
    /// [See `wxMenu::AppendRadioItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a341af26a4d59740d99d17aa1a6ed6e48)
    fn append_radio_item(
        &self,
        id: c_int,
        item: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_AppendRadioItem(self.as_ptr(), id, item, help))
        }
    }
    /// Adds a separator to the end of the menu.
    ///
    /// [See `wxMenu::AppendSeparator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#afdc206877228ad8cc2c9de5b71f5b633)
    fn append_separator(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_AppendSeparator(self.as_ptr())) }
    }
    /// Adds the given submenu to this menu.
    ///
    /// [See `wxMenu::AppendSubMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a778d7e9b333967e5ae3954135fc730c8)
    fn append_sub_menu<M: MenuMethods>(
        &self,
        submenu: Option<&M>,
        text: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_AppendSubMenu(
                self.as_ptr(),
                submenu,
                text,
                help,
            ))
        }
    }
    /// Inserts a break in a menu, causing the next appended item to appear in a new column.
    ///
    /// [See `wxMenu::Break()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a3728d2f96ee825b042e0b6d0e08d21d3)
    fn break_(&self) {
        unsafe { ffi::wxMenu_Break(self.as_ptr()) }
    }
    /// Checks or unchecks the menu item.
    ///
    /// [See `wxMenu::Check()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a3a5a76d1ee332a40919941870529b49f)
    fn check(&self, id: c_int, check: bool) {
        unsafe { ffi::wxMenu_Check(self.as_ptr(), id, check) }
    }
    /// Deletes the menu item from the menu.
    ///
    /// [See `wxMenu::Delete()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#ad0478a386aa0f55b568686fc8837d00e)
    fn delete_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_Delete(self.as_ptr(), id) }
    }
    /// Deletes the menu item from the menu.
    ///
    /// [See `wxMenu::Delete()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a8d88695161c09b8a43d2b1cf98defd85)
    fn delete_menuitem<M: MenuItemMethods>(&self, item: Option<&M>) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Delete1(self.as_ptr(), item)
        }
    }
    /// Deletes the menu item from the menu.
    ///
    /// [See `wxMenu::Destroy()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#ab30d479e9c8651d64b7c8fc30701aa8d)
    fn destroy_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_Destroy(self.as_ptr(), id) }
    }
    /// Deletes the menu item from the menu.
    ///
    /// [See `wxMenu::Destroy()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a72bf5c0fa9af6227245930a06afdc19b)
    fn destroy_menuitem<M: MenuItemMethods>(&self, item: Option<&M>) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Destroy1(self.as_ptr(), item)
        }
    }
    /// Enables or disables (greys out) a menu item.
    ///
    /// [See `wxMenu::Enable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#ae2a468532c454371d556d61efca7c002)
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { ffi::wxMenu_Enable(self.as_ptr(), id, enable) }
    }
    /// Finds the menu item object associated with the given menu item identifier and, optionally, the position of the item in the menu.
    ///
    /// [See `wxMenu::FindChildItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a7cf7f66a395364aed26779cbf0d387d5)
    fn find_child_item(&self, id: c_int, pos: *mut c_void) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_FindChildItem(self.as_ptr(), id, pos)) }
    }
    /// Finds the menu id for a menu item string.
    ///
    /// [See `wxMenu::FindItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a7e8bf27fe85df1e81905b544cbf2021a)
    fn find_item_str(&self, item_string: &str) -> c_int {
        unsafe {
            let item_string = WxString::from(item_string);
            let item_string = item_string.as_ptr();
            ffi::wxMenu_FindItem(self.as_ptr(), item_string)
        }
    }
    /// Finds the menu item object associated with the given menu item identifier and, optionally, the (sub)menu it belongs to.
    ///
    /// [See `wxMenu::FindItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#aeaebdafea19d63fef51096d934b4c8e5)
    fn find_item_int<M: MenuMethods>(
        &self,
        id: c_int,
        menu: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_FindItem1(self.as_ptr(), id, menu))
        }
    }
    /// Returns the wxMenuItem given a position in the menu.
    ///
    /// [See `wxMenu::FindItemByPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a8d91d9b706cd13df799ad8391cb28a2c)
    fn find_item_by_position(&self, position: usize) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_FindItemByPosition(self.as_ptr(), position)) }
    }
    /// Returns the help string associated with a menu item.
    ///
    /// [See `wxMenu::GetHelpString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#ada059de657d426ef97435926b69fd88d)
    fn get_help_string(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetHelpString(self.as_ptr(), id)).into() }
    }
    /// Returns a menu item label.
    ///
    /// [See `wxMenu::GetLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#adf97a53174e150c94b459fe9aad06f36)
    fn get_label(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetLabel(self.as_ptr(), id)).into() }
    }
    /// Returns a menu item label, without any of the original mnemonics and accelerators.
    ///
    /// [See `wxMenu::GetLabelText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a096a14747b7f1516795cb9d492973e21)
    fn get_label_text(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetLabelText(self.as_ptr(), id)).into() }
    }
    /// Returns the number of items in the menu.
    ///
    /// [See `wxMenu::GetMenuItemCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a0865b6be2b9067c38a177d5394460f5b)
    fn get_menu_item_count(&self) -> usize {
        unsafe { ffi::wxMenu_GetMenuItemCount(self.as_ptr()) }
    }
    // BLOCKED: fn GetMenuItems()
    // BLOCKED: fn GetMenuItems1()
    /// Returns the title of the menu.
    ///
    /// [See `wxMenu::GetTitle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a5e18712fde92a7d24da6fe5747cfd22e)
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetTitle(self.as_ptr())).into() }
    }
    /// Inserts the given item before the position pos.
    ///
    /// [See `wxMenu::Insert()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#adacabf86ad46528b776cb72aa0883e19)
    fn insert_menuitem<M: MenuItemMethods>(
        &self,
        pos: usize,
        menu_item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu_item = match menu_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Insert(self.as_ptr(), pos, menu_item))
        }
    }
    /// Inserts the given item before the position pos.
    ///
    /// [See `wxMenu::Insert()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a6d307c16972b3e8a815b891973d55f16)
    fn insert_int_str(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Insert1(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    /// Inserts the given submenu before the position pos.
    ///
    /// [See `wxMenu::Insert()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#acc220068554fcd46c2d395a56fd5b30b)
    fn insert_int_menu<M: MenuMethods>(
        &self,
        pos: usize,
        id: c_int,
        text: &str,
        submenu: Option<&M>,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Insert2(
                self.as_ptr(),
                pos,
                id,
                text,
                submenu,
                help,
            ))
        }
    }
    /// Inserts a checkable item at the given position.
    ///
    /// [See `wxMenu::InsertCheckItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a69128701ada8f6d85fdba7b7cfef94cc)
    fn insert_check_item(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_InsertCheckItem(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
            ))
        }
    }
    /// Inserts a radio item at the given position.
    ///
    /// [See `wxMenu::InsertRadioItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a51ac95b8d82e61ff0ccdffeb1dba19cd)
    fn insert_radio_item(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_InsertRadioItem(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
            ))
        }
    }
    /// Inserts a separator at the given position.
    ///
    /// [See `wxMenu::InsertSeparator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a968684b5237ebdf2d51bedcf66adb2d8)
    fn insert_separator(&self, pos: usize) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_InsertSeparator(self.as_ptr(), pos)) }
    }
    /// Determines whether a menu item is checked.
    ///
    /// [See `wxMenu::IsChecked()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a753d11b69d263d7c4560af82454e27f1)
    fn is_checked(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_IsChecked(self.as_ptr(), id) }
    }
    /// Determines whether a menu item is enabled.
    ///
    /// [See `wxMenu::IsEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#aea543f11f8ccfb3c340a4ac45984c5c8)
    fn is_enabled(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_IsEnabled(self.as_ptr(), id) }
    }
    // NOT_SUPPORTED: fn MSWCommand()
    /// Inserts the given item at position 0, i.e. before all the other existing items.
    ///
    /// [See `wxMenu::Prepend()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a2e24871e09f50a8075289ff3d6ad4076)
    fn prepend_menuitem<M: MenuItemMethods>(
        &self,
        item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Prepend(self.as_ptr(), item))
        }
    }
    /// Inserts the given item at position 0, i.e. before all the other existing items.
    ///
    /// [See `wxMenu::Prepend()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a78813c14e46be2029da3c9ea46e17a7f)
    fn prepend_int_str(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Prepend1(
                self.as_ptr(),
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    /// Inserts the given submenu at position 0.
    ///
    /// [See `wxMenu::Prepend()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a2750a6036cb0f7d0d6d5fc9521736329)
    fn prepend_int_menu<M: MenuMethods>(
        &self,
        id: c_int,
        text: &str,
        submenu: Option<&M>,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Prepend2(self.as_ptr(), id, text, submenu, help))
        }
    }
    /// Inserts a checkable item at position 0.
    ///
    /// [See `wxMenu::PrependCheckItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a102bf56de810eecf74f83acf46b87009)
    fn prepend_check_item(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_PrependCheckItem(
                self.as_ptr(),
                id,
                item,
                help_string,
            ))
        }
    }
    /// Inserts a radio item at position 0.
    ///
    /// [See `wxMenu::PrependRadioItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a3f1570860f9613f7e4d31c0d29d79e11)
    fn prepend_radio_item(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_PrependRadioItem(
                self.as_ptr(),
                id,
                item,
                help_string,
            ))
        }
    }
    /// Inserts a separator at position 0.
    ///
    /// [See `wxMenu::PrependSeparator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a13a427147256b2809a103a2874ea5e69)
    fn prepend_separator(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_PrependSeparator(self.as_ptr())) }
    }
    /// Removes the menu item from the menu but doesn't delete the associated C++ object.
    ///
    /// [See `wxMenu::Remove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a873acedf8a664b574d1f97de9a100bf1)
    fn remove_int(&self, id: c_int) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_Remove(self.as_ptr(), id)) }
    }
    /// Removes the menu item from the menu but doesn't delete the associated C++ object.
    ///
    /// [See `wxMenu::Remove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#acbb314ac2e3a4b20d0fc54abb8ea2d41)
    fn remove_menuitem<M: MenuItemMethods>(
        &self,
        item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Remove1(self.as_ptr(), item))
        }
    }
    /// Sets an item's help string.
    ///
    /// [See `wxMenu::SetHelpString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#afaad5f31ec3abcf830ecdf2532c6e98b)
    fn set_help_string(&self, id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenu_SetHelpString(self.as_ptr(), id, help_string)
        }
    }
    /// Sets the label of a menu item.
    ///
    /// [See `wxMenu::SetLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#aec7c82f4c6fa18187c25b3797590a9d2)
    fn set_label(&self, id: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenu_SetLabel(self.as_ptr(), id, label)
        }
    }
    /// Sets the title of the menu.
    ///
    /// [See `wxMenu::SetTitle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#aa8b3117231c244d4952a641fb3e9a272)
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenu_SetTitle(self.as_ptr(), title)
        }
    }
    /// Update the state of all menu items, recursively, by generating wxEVT_UPDATE_UI events for them.
    ///
    /// [See `wxMenu::UpdateUI()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#aedd1c1536176ffec144f7e8bea85bd89)
    fn update_ui<E: EvtHandlerMethods>(&self, source: Option<&E>) {
        unsafe {
            let source = match source {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_UpdateUI(self.as_ptr(), source)
        }
    }
    ///
    /// [See `wxMenu::SetInvokingWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a1c459fa87f5c52afc7b5d717e846cce4)
    fn set_invoking_window<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_SetInvokingWindow(self.as_ptr(), win)
        }
    }
    ///
    /// [See `wxMenu::GetInvokingWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a21b4283f7debebca8b4a335f7209ab97)
    fn get_invoking_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxMenu_GetInvokingWindow(self.as_ptr())) }
    }
    ///
    /// [See `wxMenu::GetWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a6357304cd381232b374fbdbfd2f61c0b)
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxMenu_GetWindow(self.as_ptr())) }
    }
    ///
    /// [See `wxMenu::GetStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a8042c4419e395495e41f181bca057558)
    fn get_style(&self) -> c_long {
        unsafe { ffi::wxMenu_GetStyle(self.as_ptr()) }
    }
    ///
    /// [See `wxMenu::SetParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a7873255a269c036348d6af33629d118b)
    fn set_parent<M: MenuMethods>(&self, parent: Option<&M>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_SetParent(self.as_ptr(), parent)
        }
    }
    ///
    /// [See `wxMenu::GetParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#afcf1a7b77318e8a3affb4d2e42b5b3e6)
    fn get_parent(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenu_GetParent(self.as_ptr())) }
    }
    ///
    /// [See `wxMenu::Attach()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a15faea06e13e6751f561d4edd6b1052e)
    fn attach<M: MenuBarMethods>(&self, menubar: Option<&M>) {
        unsafe {
            let menubar = match menubar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Attach(self.as_ptr(), menubar)
        }
    }
    ///
    /// [See `wxMenu::Detach()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a19a7de8fd10b2cd24c6387b02762fecd)
    fn detach(&self) {
        unsafe { ffi::wxMenu_Detach(self.as_ptr()) }
    }
    ///
    /// [See `wxMenu::IsAttached()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu.html#a1e980cc5464e02baa51bd3db636625a7)
    fn is_attached(&self) -> bool {
        unsafe { ffi::wxMenu_IsAttached(self.as_ptr()) }
    }
}

// wxMenuBar
/// A menu bar is a series of menus accessible from the top of a frame.
///
/// [See `wxMenuBar`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html)
pub trait MenuBarMethods: WindowMethods {
    // DTOR: fn ~wxMenuBar()
    /// Adds the item to the end of the menu bar.
    ///
    /// [See `wxMenuBar::Append()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a342fec5ec2d96789cfc7b82557dfa646)
    fn append<M: MenuMethods>(&self, menu: Option<&M>, title: &str) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenuBar_Append(self.as_ptr(), menu, title)
        }
    }
    /// Checks or unchecks a menu item.
    ///
    /// [See `wxMenuBar::Check()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#ae52945fab916e8031fdb14b8124f3cd9)
    fn check(&self, id: c_int, check: bool) {
        unsafe { ffi::wxMenuBar_Check(self.as_ptr(), id, check) }
    }
    /// Enables or disables (greys out) a menu item.
    ///
    /// [See `wxMenuBar::Enable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#ab53b19f2e8b0e7b39ae551d51f2f6f6b)
    fn enable_int(&self, id: c_int, enable: bool) {
        unsafe { ffi::wxMenuBar_Enable(self.as_ptr(), id, enable) }
    }
    /// Returns true if the menu with the given index is enabled.
    ///
    /// [See `wxMenuBar::IsEnabledTop()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a5fc45b5e36edc176235c7ee385e92e2f)
    fn is_enabled_top(&self, pos: usize) -> bool {
        unsafe { ffi::wxMenuBar_IsEnabledTop(self.as_ptr(), pos) }
    }
    /// Enables or disables a whole menu.
    ///
    /// [See `wxMenuBar::EnableTop()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#ae995701578fa19cc6ae528cfb13a4d7d)
    fn enable_top(&self, pos: usize, enable: bool) {
        unsafe { ffi::wxMenuBar_EnableTop(self.as_ptr(), pos, enable) }
    }
    /// Finds the menu item object associated with the given menu item identifier.
    ///
    /// [See `wxMenuBar::FindItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a7bc48a9a0e6ec8268d3281e401183552)
    fn find_item<M: MenuMethods>(
        &self,
        id: c_int,
        menu: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenuBar_FindItem(self.as_ptr(), id, menu))
        }
    }
    /// Returns the index of the menu with the given title or wxNOT_FOUND if no such menu exists in this menubar.
    ///
    /// [See `wxMenuBar::FindMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a459b49b70c955145fbb202c258188309)
    fn find_menu(&self, title: &str) -> c_int {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenuBar_FindMenu(self.as_ptr(), title)
        }
    }
    /// Finds the menu item id for a menu name/menu item string pair.
    ///
    /// [See `wxMenuBar::FindMenuItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a56e9010ba84f0ac5303cd52a34ac0dc7)
    fn find_menu_item(&self, menu_string: &str, item_string: &str) -> c_int {
        unsafe {
            let menu_string = WxString::from(menu_string);
            let menu_string = menu_string.as_ptr();
            let item_string = WxString::from(item_string);
            let item_string = item_string.as_ptr();
            ffi::wxMenuBar_FindMenuItem(self.as_ptr(), menu_string, item_string)
        }
    }
    /// Gets the help string associated with the menu item identifier.
    ///
    /// [See `wxMenuBar::GetHelpString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#ad3e511976bb620ce9a7accfe32fe6073)
    fn get_help_string(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetHelpString(self.as_ptr(), id)).into() }
    }
    /// Gets the label associated with a menu item.
    ///
    /// [See `wxMenuBar::GetLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a4d82ad199d1c348a144637ca92bf6b6f)
    fn get_label_int(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetLabel(self.as_ptr(), id)).into() }
    }
    // BLOCKED: fn GetLabelTop()
    /// Returns the menu at menuIndex (zero-based).
    ///
    /// [See `wxMenuBar::GetMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a54ea388d67b44a06e80b68a5b1b7f4c3)
    fn get_menu(&self, menu_index: usize) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_GetMenu(self.as_ptr(), menu_index)) }
    }
    /// Returns the number of menus in this menubar.
    ///
    /// [See `wxMenuBar::GetMenuCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#add65e03ec1f868486e541426381a17f7)
    fn get_menu_count(&self) -> usize {
        unsafe { ffi::wxMenuBar_GetMenuCount(self.as_ptr()) }
    }
    /// Returns the label of a top-level menu.
    ///
    /// [See `wxMenuBar::GetMenuLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#ae177b639c9a6281a4bb0f202f8758b61)
    fn get_menu_label(&self, pos: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetMenuLabel(self.as_ptr(), pos)).into() }
    }
    /// Returns the label of a top-level menu.
    ///
    /// [See `wxMenuBar::GetMenuLabelText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a9a9b0bf227387c5c2b9ab291bfdf3027)
    fn get_menu_label_text(&self, pos: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetMenuLabelText(self.as_ptr(), pos)).into() }
    }
    /// Inserts the menu at the given position into the menu bar.
    ///
    /// [See `wxMenuBar::Insert()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a6bb349afc87743a78f25159bc68a4087)
    fn insert<M: MenuMethods>(&self, pos: usize, menu: Option<&M>, title: &str) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenuBar_Insert(self.as_ptr(), pos, menu, title)
        }
    }
    /// Determines whether an item is checked.
    ///
    /// [See `wxMenuBar::IsChecked()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#aea128698af773f0fc6024fecfbe39281)
    fn is_checked(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenuBar_IsChecked(self.as_ptr(), id) }
    }
    /// Determines whether an item is enabled.
    ///
    /// [See `wxMenuBar::IsEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a43e4c04827f790a5bfeacef95b53e461)
    fn is_enabled_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenuBar_IsEnabled(self.as_ptr(), id) }
    }
    /// Removes the menu from the menu bar and returns the menu object - the caller is responsible for deleting it.
    ///
    /// [See `wxMenuBar::Remove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a05d7e4c19d2c98aca6bd6cf81af2f0b4)
    fn remove(&self, pos: usize) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_Remove(self.as_ptr(), pos)) }
    }
    /// Replaces the menu at the given position with another one.
    ///
    /// [See `wxMenuBar::Replace()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a77bd71f6c9dc1bb21742c07c4683e789)
    fn replace<M: MenuMethods>(&self, pos: usize, menu: Option<&M>, title: &str) -> WeakRef<Menu> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            WeakRef::<Menu>::from(ffi::wxMenuBar_Replace(self.as_ptr(), pos, menu, title))
        }
    }
    /// Sets the help string associated with a menu item.
    ///
    /// [See `wxMenuBar::SetHelpString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a4379fba6f5a0a3e62b6271735f2b4841)
    fn set_help_string(&self, id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenuBar_SetHelpString(self.as_ptr(), id, help_string)
        }
    }
    /// Sets the label of a menu item.
    ///
    /// [See `wxMenuBar::SetLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a4b3669c33dfafeb1c8468a0ba3c09d5f)
    fn set_label_int(&self, id: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuBar_SetLabel(self.as_ptr(), id, label)
        }
    }
    // BLOCKED: fn SetLabelTop()
    /// Sets the label of a top-level menu.
    ///
    /// [See `wxMenuBar::SetMenuLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a66a14aec7caeff702f964cd35085cf06)
    fn set_menu_label(&self, pos: usize, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuBar_SetMenuLabel(self.as_ptr(), pos, label)
        }
    }
    /// Returns the Apple menu.
    ///
    /// [See `wxMenuBar::OSXGetAppleMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a61209f2ef16fa67bf8d5216732437431)
    fn osx_get_apple_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_OSXGetAppleMenu(self.as_ptr())) }
    }
    ///
    /// [See `wxMenuBar::GetFrame()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a464b1576b87f6f5806379e523b0e95c3)
    fn get_frame(&self) -> WeakRef<Frame> {
        unsafe { WeakRef::<Frame>::from(ffi::wxMenuBar_GetFrame(self.as_ptr())) }
    }
    ///
    /// [See `wxMenuBar::IsAttached()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a1ef964f2241f7d0eff30c3dd6d65c97d)
    fn is_attached(&self) -> bool {
        unsafe { ffi::wxMenuBar_IsAttached(self.as_ptr()) }
    }
    ///
    /// [See `wxMenuBar::Attach()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a4ca9c3e53c3ff18f7f74e3d2aeea3dab)
    fn attach<F: FrameMethods>(&self, frame: Option<&F>) {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuBar_Attach(self.as_ptr(), frame)
        }
    }
    ///
    /// [See `wxMenuBar::Detach()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a3fc40b9272a0ef7ff1bcbea04d3557c7)
    fn detach(&self) {
        unsafe { ffi::wxMenuBar_Detach(self.as_ptr()) }
    }
    /// Enables you to set the global menubar on Mac, that is, the menubar displayed when the app is running without any frames open.
    ///
    /// [See `wxMenuBar::MacSetCommonMenuBar()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a1fd28115be9437d0dd7b1f99049ec905)
    fn mac_set_common_menu_bar<M: MenuBarMethods>(menubar: Option<&M>) {
        unsafe {
            let menubar = match menubar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuBar_MacSetCommonMenuBar(menubar)
        }
    }
    /// Enables you to get the global menubar on Mac, that is, the menubar displayed when the app is running without any frames open.
    ///
    /// [See `wxMenuBar::MacGetCommonMenuBar()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_bar.html#a1c373424e34a9a1a0f40b29f30f6fecb)
    fn mac_get_common_menu_bar() -> WeakRef<MenuBar> {
        unsafe { WeakRef::<MenuBar>::from(ffi::wxMenuBar_MacGetCommonMenuBar()) }
    }
}

// wxMenuEvent
/// This class is used for a variety of menu-related events.
///
/// [See `wxMenuEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_event.html)
pub trait MenuEventMethods: EventMethods {
    /// Returns the menu which is being opened or closed, or the menu containing the highlighted item.
    ///
    /// [See `wxMenuEvent::GetMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_event.html#a1b44ad5a80dbce004b9d304813c91f29)
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuEvent_GetMenu(self.as_ptr())) }
    }
    /// Returns the menu identifier associated with the event.
    ///
    /// [See `wxMenuEvent::GetMenuId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_event.html#af59809f4b21a8136ebb84395756c7323)
    fn get_menu_id(&self) -> c_int {
        unsafe { ffi::wxMenuEvent_GetMenuId(self.as_ptr()) }
    }
    /// Returns true if the menu which is being opened or closed is a popup menu, false if it is a normal one.
    ///
    /// [See `wxMenuEvent::IsPopup()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_event.html#a1a38c42493a996ea43bcc8a55fe4174c)
    fn is_popup(&self) -> bool {
        unsafe { ffi::wxMenuEvent_IsPopup(self.as_ptr()) }
    }
}

// wxMenuItem
/// A menu item represents an item in a menu.
///
/// [See `wxMenuItem`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html)
pub trait MenuItemMethods: ObjectMethods {
    // BLOCKED: fn GetBackgroundColour()
    /// Returns the item bitmap.
    ///
    /// [See `wxMenuItem::GetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#acad864491fe941af3718115a79daed8d)
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetBitmap(self.as_ptr())) }
    }
    /// Returns the checked or unchecked bitmap.
    ///
    /// [See `wxMenuItem::GetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a2fdaf18c01423ccd46c22dfe0c4ac821)
    fn get_bitmap_bool(&self, checked: bool) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetBitmap1(self.as_ptr(), checked)) }
    }
    /// Returns the bitmap bundle containing the bitmap used for this item.
    ///
    /// [See `wxMenuItem::GetBitmapBundle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#ab22eaea1e24d1de0186bd2f496c67905)
    fn get_bitmap_bundle(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxMenuItem_GetBitmapBundle(self.as_ptr())) }
    }
    /// Returns the bitmap used for disabled items.
    ///
    /// [See `wxMenuItem::GetDisabledBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#ad159d100aa8852d5a3d90bd07e3db0b8)
    fn get_disabled_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetDisabledBitmap(self.as_ptr())) }
    }
    // BLOCKED: fn GetFont()
    /// Returns the help string associated with the menu item.
    ///
    /// [See `wxMenuItem::GetHelp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a0e5cf57080e9239d54b1145b00449332)
    fn get_help(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetHelp(self.as_ptr())).into() }
    }
    /// Returns the menu item identifier.
    ///
    /// [See `wxMenuItem::GetId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a86d0d34d65d1d115ea951870298b4330)
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetId(self.as_ptr()) }
    }
    /// Returns the text associated with the menu item including any accelerator characters that were passed to the constructor or SetItemLabel().
    ///
    /// [See `wxMenuItem::GetItemLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a7ca185aa5c527d13b8c62d3e01c3471a)
    fn get_item_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetItemLabel(self.as_ptr())).into() }
    }
    /// Returns the text associated with the menu item, without any accelerator characters.
    ///
    /// [See `wxMenuItem::GetItemLabelText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a8f8c25b7e46072f5f6e51856cae53584)
    fn get_item_label_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetItemLabelText(self.as_ptr())).into() }
    }
    /// Returns the item kind, one of wxITEM_SEPARATOR, wxITEM_NORMAL, wxITEM_CHECK or wxITEM_RADIO.
    ///
    /// [See `wxMenuItem::GetKind()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a7020d6b58487acb2cc691d2230938fe5)
    fn get_kind(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetKind(self.as_ptr()) }
    }
    // BLOCKED: fn GetLabel()
    /// Gets the width of the menu item checkmark bitmap.
    ///
    /// [See `wxMenuItem::GetMarginWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a5d49fce4aa0446c647906bd46e86c134)
    fn get_margin_width(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetMarginWidth(self.as_ptr()) }
    }
    /// Returns the menu this menu item is in, or NULL if this menu item is not attached.
    ///
    /// [See `wxMenuItem::GetMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#ab3acd9e96ff430293380b53ec817dad2)
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuItem_GetMenu(self.as_ptr())) }
    }
    // BLOCKED: fn GetName()
    /// Returns the submenu associated with the menu item, or NULL if there isn't one.
    ///
    /// [See `wxMenuItem::GetSubMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a41d4139faeb7323ad8a0059fa06f3eec)
    fn get_sub_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuItem_GetSubMenu(self.as_ptr())) }
    }
    // BLOCKED: fn GetText()
    // BLOCKED: fn GetTextColour()
    /// Get our accelerator or NULL (caller must delete the pointer)
    ///
    /// [See `wxMenuItem::GetAccel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a011e5b7a9945b43acc32848612376c2e)
    fn get_accel(&self) -> Option<AcceleratorEntryIsOwned<false>> {
        unsafe { AcceleratorEntry::option_from(ffi::wxMenuItem_GetAccel(self.as_ptr())) }
    }
    // BLOCKED: fn GetAccelFromString()
    /// Returns true if the item is a check item.
    ///
    /// [See `wxMenuItem::IsCheck()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#ad130ff84162b411524b8fd67e4fa4766)
    fn is_check(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsCheck(self.as_ptr()) }
    }
    /// Returns true if the item is checkable.
    ///
    /// [See `wxMenuItem::IsCheckable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a0c10c9514cdc1a719b5e53fe75c1e764)
    fn is_checkable(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsCheckable(self.as_ptr()) }
    }
    /// Returns true if the item is checked.
    ///
    /// [See `wxMenuItem::IsChecked()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a8727ee892b87d44b4c23566dd81e71db)
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsChecked(self.as_ptr()) }
    }
    /// Returns true if the item is enabled.
    ///
    /// [See `wxMenuItem::IsEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a9f5e3a06bf0d596098ffdf2b48779911)
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsEnabled(self.as_ptr()) }
    }
    /// Returns true if the item is a radio button.
    ///
    /// [See `wxMenuItem::IsRadio()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a680da3cb33645cac12057a62a3207268)
    fn is_radio(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsRadio(self.as_ptr()) }
    }
    /// Returns true if the item is a separator.
    ///
    /// [See `wxMenuItem::IsSeparator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#acda66a8035c0e880ba996da2da6c7371)
    fn is_separator(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsSeparator(self.as_ptr()) }
    }
    /// Returns true if the item is a submenu.
    ///
    /// [See `wxMenuItem::IsSubMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#ab400e6250bcab892205bb22e703d024e)
    fn is_sub_menu(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsSubMenu(self.as_ptr()) }
    }
    /// Sets the background colour associated with the menu item.
    ///
    /// [See `wxMenuItem::SetBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a5a4462c00517c5d8470b99fd021c6a9d)
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxMenuItem_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    /// Sets the bitmap for the menu item.
    ///
    /// [See `wxMenuItem::SetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#abf175a94aa2b9ce76ab8bf4e223e609d)
    fn set_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxMenuItem_SetBitmap(self.as_ptr(), bmp)
        }
    }
    /// Sets the checked or unchecked bitmap for the menu item.
    ///
    /// [See `wxMenuItem::SetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a8e935bd7d6b0c93b243dfdfd102421b3)
    fn set_bitmap_bool<B: BitmapBundleMethods>(&self, bmp: &B, checked: bool) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxMenuItem_SetBitmap1(self.as_ptr(), bmp, checked)
        }
    }
    /// Sets the checked/unchecked bitmaps for the menu item.
    ///
    /// [See `wxMenuItem::SetBitmaps()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a8024cfb73724cc156049571cc23bd9a0)
    fn set_bitmaps<B: BitmapBundleMethods, B2: BitmapBundleMethods>(
        &self,
        checked: &B,
        unchecked: &B2,
    ) {
        unsafe {
            let checked = checked.as_ptr();
            let unchecked = unchecked.as_ptr();
            ffi::wxMenuItem_SetBitmaps(self.as_ptr(), checked, unchecked)
        }
    }
    /// Sets the to be used for disabled menu items.
    ///
    /// [See `wxMenuItem::SetDisabledBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#ac6304d34c075ed4d7fc25581440f7b43)
    fn set_disabled_bitmap<B: BitmapBundleMethods>(&self, disabled: &B) {
        unsafe {
            let disabled = disabled.as_ptr();
            ffi::wxMenuItem_SetDisabledBitmap(self.as_ptr(), disabled)
        }
    }
    /// Sets the font associated with the menu item.
    ///
    /// [See `wxMenuItem::SetFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a5deda3866e4308a965caa2fb78c1ca93)
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxMenuItem_SetFont(self.as_ptr(), font)
        }
    }
    /// Sets the help string.
    ///
    /// [See `wxMenuItem::SetHelp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#af20e2cb1c73892e1c8e86e69b40d9040)
    fn set_help(&self, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenuItem_SetHelp(self.as_ptr(), help_string)
        }
    }
    /// Sets the label associated with the menu item.
    ///
    /// [See `wxMenuItem::SetItemLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a8b0517fb35e3eada66b51568aa87f261)
    fn set_item_label(&self, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuItem_SetItemLabel(self.as_ptr(), label)
        }
    }
    /// Sets the width of the menu item checkmark bitmap.
    ///
    /// [See `wxMenuItem::SetMarginWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#aac2257b03dd2485c72f9398ceb1a76cf)
    fn set_margin_width(&self, width: c_int) {
        unsafe { ffi::wxMenuItem_SetMarginWidth(self.as_ptr(), width) }
    }
    /// Sets the parent menu which will contain this menu item.
    ///
    /// [See `wxMenuItem::SetMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a41fb81219430a81e7e2a668831868296)
    fn set_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetMenu(self.as_ptr(), menu)
        }
    }
    /// Sets the submenu of this menu item.
    ///
    /// [See `wxMenuItem::SetSubMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#ab222651b8b47653d8890cd3469d5ff5a)
    fn set_sub_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetSubMenu(self.as_ptr(), menu)
        }
    }
    // BLOCKED: fn SetText()
    /// Sets the text colour associated with the menu item.
    ///
    /// [See `wxMenuItem::SetTextColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a695b5f2f1c2325ec01dba5cdd83dd3b5)
    fn set_text_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxMenuItem_SetTextColour(self.as_ptr(), colour)
        }
    }
    /// Set the accel for this item - this may also be done indirectly with SetText()
    ///
    /// [See `wxMenuItem::SetAccel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a1deb59611abda8b68cd06c848f48884d)
    fn set_accel<A: AcceleratorEntryMethods>(&self, accel: Option<&A>) {
        unsafe {
            let accel = match accel {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetAccel(self.as_ptr(), accel)
        }
    }
    /// Add an extra accelerator for this menu item.
    ///
    /// [See `wxMenuItem::AddExtraAccel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#ac289400c23e28ff4b094a4263d093c66)
    fn add_extra_accel<A: AcceleratorEntryMethods>(&self, accel: &A) {
        unsafe {
            let accel = accel.as_ptr();
            ffi::wxMenuItem_AddExtraAccel(self.as_ptr(), accel)
        }
    }
    /// Clear the extra accelerators list.
    ///
    /// [See `wxMenuItem::ClearExtraAccels()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a80f57a2bf185823729bf1cba73aef43d)
    fn clear_extra_accels(&self) {
        unsafe { ffi::wxMenuItem_ClearExtraAccels(self.as_ptr()) }
    }
    // DTOR: fn ~wxMenuItem()
    /// Checks or unchecks the menu item.
    ///
    /// [See `wxMenuItem::Check()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#afcfbb12e302c0528e55efcb1c6f5f7fc)
    fn check(&self, check: bool) {
        unsafe { ffi::wxMenuItem_Check(self.as_ptr(), check) }
    }
    /// Enables or disables the menu item.
    ///
    /// [See `wxMenuItem::Enable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#a1e399da82f486adea3893480bcf66b21)
    fn enable(&self, enable: bool) {
        unsafe { ffi::wxMenuItem_Enable(self.as_ptr(), enable) }
    }
    // BLOCKED: fn GetLabelFromText()
    /// Strips all accelerator characters and mnemonics from the given text.
    ///
    /// [See `wxMenuItem::GetLabelText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_menu_item.html#afed23d53a97171076763385c93207fc0)
    fn get_label_text(text: &str) -> String {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            WxString::from_ptr(ffi::wxMenuItem_GetLabelText(text)).into()
        }
    }
}

// wxMessageDialog
/// This class represents a dialog that shows a single or multi-line message, with a choice of OK, Yes, No and Cancel buttons.
///
/// [See `wxMessageDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html)
pub trait MessageDialogMethods: DialogMethods {
    /// Sets the extended message for the dialog: this message is usually an extension of the short message specified in the constructor or set with SetMessage().
    ///
    /// [See `wxMessageDialog::SetExtendedMessage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a6085d6b273139c655af06874abf03ea3)
    fn set_extended_message(&self, extended_message: &str) {
        unsafe {
            let extended_message = WxString::from(extended_message);
            let extended_message = extended_message.as_ptr();
            ffi::wxMessageDialog_SetExtendedMessage(self.as_ptr(), extended_message)
        }
    }
    /// Sets the label for the Help button.
    ///
    /// [See `wxMessageDialog::SetHelpLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a85dd2dfd88baa9af9de185bae6642c0e)
    fn set_help_label(&self, help: *const c_void) -> bool {
        unsafe { ffi::wxMessageDialog_SetHelpLabel(self.as_ptr(), help) }
    }
    /// Sets the message shown by the dialog.
    ///
    /// [See `wxMessageDialog::SetMessage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a43fd42a28d0008f5de3c8bd88aa47a87)
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxMessageDialog_SetMessage(self.as_ptr(), message)
        }
    }
    /// Overrides the default labels of the OK and Cancel buttons.
    ///
    /// [See `wxMessageDialog::SetOKCancelLabels()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#ad8fdf1426f7363bd1e1e07af5f1e7adc)
    fn set_ok_cancel_labels(&self, ok: *const c_void, cancel: *const c_void) -> bool {
        unsafe { ffi::wxMessageDialog_SetOKCancelLabels(self.as_ptr(), ok, cancel) }
    }
    /// Overrides the default label of the OK button.
    ///
    /// [See `wxMessageDialog::SetOKLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a9790fa9c1f2fc473539b57d9c1e862a9)
    fn set_ok_label(&self, ok: *const c_void) -> bool {
        unsafe { ffi::wxMessageDialog_SetOKLabel(self.as_ptr(), ok) }
    }
    /// Overrides the default labels of the Yes, No and Cancel buttons.
    ///
    /// [See `wxMessageDialog::SetYesNoCancelLabels()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a69d5facbe995eee8e92035423e30f534)
    fn set_yes_no_cancel_labels(
        &self,
        yes: *const c_void,
        no: *const c_void,
        cancel: *const c_void,
    ) -> bool {
        unsafe { ffi::wxMessageDialog_SetYesNoCancelLabels(self.as_ptr(), yes, no, cancel) }
    }
    /// Overrides the default labels of the Yes and No buttons.
    ///
    /// [See `wxMessageDialog::SetYesNoLabels()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#aa23b25ebf18bf0915d438cc13b55cb23)
    fn set_yes_no_labels(&self, yes: *const c_void, no: *const c_void) -> bool {
        unsafe { ffi::wxMessageDialog_SetYesNoLabels(self.as_ptr(), yes, no) }
    }
    ///
    /// [See `wxMessageDialog::GetCaption()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a380e4f2718064c3c52654a6f53545223)
    fn get_caption(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetCaption(self.as_ptr())).into() }
    }
    ///
    /// [See `wxMessageDialog::GetMessage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#acab6c3bb053033eca71b1676b12af9ee)
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetMessage(self.as_ptr())).into() }
    }
    ///
    /// [See `wxMessageDialog::GetExtendedMessage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a332328991aadf9dab5c0694a66d23a5d)
    fn get_extended_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetExtendedMessage(self.as_ptr())).into() }
    }
    ///
    /// [See `wxMessageDialog::GetMessageDialogStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a29fb358b32048e35d2baf2dda0c1da51)
    fn get_message_dialog_style(&self) -> c_long {
        unsafe { ffi::wxMessageDialog_GetMessageDialogStyle(self.as_ptr()) }
    }
    ///
    /// [See `wxMessageDialog::HasCustomLabels()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a3004e1d69c6d468af486ed716e1b8bbb)
    fn has_custom_labels(&self) -> bool {
        unsafe { ffi::wxMessageDialog_HasCustomLabels(self.as_ptr()) }
    }
    ///
    /// [See `wxMessageDialog::GetYesLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a2a47c30c4a7387bc97634f1a5fb02407)
    fn get_yes_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetYesLabel(self.as_ptr())).into() }
    }
    ///
    /// [See `wxMessageDialog::GetNoLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a4c91306d0b35371ad55572659b974f19)
    fn get_no_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetNoLabel(self.as_ptr())).into() }
    }
    ///
    /// [See `wxMessageDialog::GetOKLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#aaff522bee896f22fb41a58a75124df7f)
    fn get_ok_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetOKLabel(self.as_ptr())).into() }
    }
    ///
    /// [See `wxMessageDialog::GetCancelLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a01862f641296cb49c813bc7d3718435a)
    fn get_cancel_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetCancelLabel(self.as_ptr())).into() }
    }
    ///
    /// [See `wxMessageDialog::GetHelpLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a6dab1b25eeea11ded4e4bfaed376f5eb)
    fn get_help_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetHelpLabel(self.as_ptr())).into() }
    }
    ///
    /// [See `wxMessageDialog::GetEffectiveIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_dialog.html#a5b9e20e8335e1799356d8326342f8a24)
    fn get_effective_icon(&self) -> c_long {
        unsafe { ffi::wxMessageDialog_GetEffectiveIcon(self.as_ptr()) }
    }
}

// wxMessageOutputMessageBox
/// Output messages by showing them in a message box.
///
/// [See `wxMessageOutputMessageBox`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_output_message_box.html)
pub trait MessageOutputMessageBoxMethods: MessageOutputMethods {}

// wxMiniFrame
/// A miniframe is a frame with a small title bar.
///
/// [See `wxMiniFrame`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mini_frame.html)
pub trait MiniFrameMethods: FrameMethods {
    // DTOR: fn ~wxMiniFrame()
}

// wxMirrorDC
/// wxMirrorDC is a simple wrapper class which is always associated with a real wxDC object and either forwards all of its operations to it without changes (no mirroring takes place) or exchanges x and y coordinates which makes it possible to reuse the same code to draw a figure and its mirror  i.e.
///
/// [See `wxMirrorDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mirror_d_c.html)
pub trait MirrorDCMethods: DCMethods {}

// wxMouseCaptureChangedEvent
/// A mouse capture changed event is sent to a window that loses its mouse capture.
///
/// [See `wxMouseCaptureChangedEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_changed_event.html)
pub trait MouseCaptureChangedEventMethods: EventMethods {
    /// Returns the window that gained the capture, or NULL if it was a non-wxWidgets window.
    ///
    /// [See `wxMouseCaptureChangedEvent::GetCapturedWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_changed_event.html#a403f9904cb2e746ae5cb052fdb328f4a)
    fn get_captured_window(&self) -> WeakRef<Window> {
        unsafe {
            WeakRef::<Window>::from(ffi::wxMouseCaptureChangedEvent_GetCapturedWindow(
                self.as_ptr(),
            ))
        }
    }
}

// wxMouseCaptureLostEvent
/// A mouse capture lost event is sent to a window that had obtained mouse capture, which was subsequently lost due to an "external" event (for example, when a dialog box is shown or if another application captures the mouse).
///
/// [See `wxMouseCaptureLostEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_capture_lost_event.html)
pub trait MouseCaptureLostEventMethods: EventMethods {}

// wxMouseEvent
/// This event class contains information about the events generated by the mouse: they include mouse buttons press and release events and mouse move events.
///
/// [See `wxMouseEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html)
pub trait MouseEventMethods: EventMethods {
    /// Returns true if the event was a first extra button double click.
    ///
    /// [See `wxMouseEvent::Aux1DClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#ae2c51596a8608ac2028911140efca9f4)
    fn aux1_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1DClick(self.as_ptr()) }
    }
    /// Returns true if the first extra button mouse button changed to down.
    ///
    /// [See `wxMouseEvent::Aux1Down()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#afe1a57591865fe8454a21bd3699a01f9)
    fn aux1_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1Down(self.as_ptr()) }
    }
    /// Returns true if the first extra button mouse button changed to up.
    ///
    /// [See `wxMouseEvent::Aux1Up()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#add041b97305586a2305b3b532dc65123)
    fn aux1_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1Up(self.as_ptr()) }
    }
    /// Returns true if the event was a second extra button double click.
    ///
    /// [See `wxMouseEvent::Aux2DClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a2be62a363b8da0a5ecfe06929e3ad4bb)
    fn aux2_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2DClick(self.as_ptr()) }
    }
    /// Returns true if the second extra button mouse button changed to down.
    ///
    /// [See `wxMouseEvent::Aux2Down()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a5494f60d0424dbed2886853207ca24b4)
    fn aux2_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2Down(self.as_ptr()) }
    }
    /// Returns true if the second extra button mouse button changed to up.
    ///
    /// [See `wxMouseEvent::Aux2Up()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#aae659084143a6485d5dd95fb81d1ffaa)
    fn aux2_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2Up(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Button()
    // NOT_SUPPORTED: fn ButtonDClick()
    // NOT_SUPPORTED: fn ButtonDown()
    // NOT_SUPPORTED: fn ButtonUp()
    /// Returns true if this was a dragging event (motion while a button is depressed).
    ///
    /// [See `wxMouseEvent::Dragging()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#aa08074a2555618c63e2a53f38e56f18f)
    fn dragging(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Dragging(self.as_ptr()) }
    }
    /// Returns true if the mouse was entering the window.
    ///
    /// [See `wxMouseEvent::Entering()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#aba23d806bfad21911a5e1e4f21bdf283)
    fn entering(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Entering(self.as_ptr()) }
    }
    /// Returns the mouse button which generated this event or wxMOUSE_BTN_NONE if no button is involved (for mouse move, enter or leave event, for example).
    ///
    /// [See `wxMouseEvent::GetButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a33653a3f59d68405361f3ae65d2936eb)
    fn get_button(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetButton(self.as_ptr()) }
    }
    /// Returns the number of mouse clicks for this event: 1 for a simple click, 2 for a double-click, 3 for a triple-click and so on.
    ///
    /// [See `wxMouseEvent::GetClickCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a392c9bac6c152016ecf09c05fa2d8f9c)
    fn get_click_count(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetClickCount(self.as_ptr()) }
    }
    /// Returns the configured number of lines (or whatever) to be scrolled per wheel action.
    ///
    /// [See `wxMouseEvent::GetLinesPerAction()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a0af4db62efdd4e3a51d5ea120aeadda2)
    fn get_lines_per_action(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetLinesPerAction(self.as_ptr()) }
    }
    /// Returns the configured number of columns (or whatever) to be scrolled per wheel action.
    ///
    /// [See `wxMouseEvent::GetColumnsPerAction()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a84ffae36f25eb784975ccbd4b8774b01)
    fn get_columns_per_action(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetColumnsPerAction(self.as_ptr()) }
    }
    /// Returns the logical mouse position in pixels (i.e. translated according to the translation set for the DC, which usually indicates that the window has been scrolled).
    ///
    /// [See `wxMouseEvent::GetLogicalPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#ada3567c0ac444667e328c7c733496b58)
    fn get_logical_position<D: DCMethods>(&self, dc: &D) -> Point {
        unsafe {
            let dc = dc.as_ptr();
            Point::from_ptr(ffi::wxMouseEvent_GetLogicalPosition(self.as_ptr(), dc))
        }
    }
    // NOT_SUPPORTED: fn GetMagnification()
    /// Get wheel delta, normally 120.
    ///
    /// [See `wxMouseEvent::GetWheelDelta()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a5fa028a09896b6ddb894b12caacc32fb)
    fn get_wheel_delta(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetWheelDelta(self.as_ptr()) }
    }
    /// On Mac, has the user selected "Natural" scrolling in their System Preferences? Currently false on all other OS's.
    ///
    /// [See `wxMouseEvent::IsWheelInverted()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#ae69c1c2d061961457432976e73856af9)
    fn is_wheel_inverted(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsWheelInverted(self.as_ptr()) }
    }
    /// Get wheel rotation, positive or negative indicates direction of rotation.
    ///
    /// [See `wxMouseEvent::GetWheelRotation()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#af25c634cf45cb3d4633d45c472f5b2b9)
    fn get_wheel_rotation(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetWheelRotation(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWheelAxis()
    /// Returns true if the event was a mouse button event (not necessarily a button down event - that may be tested using ButtonDown()).
    ///
    /// [See `wxMouseEvent::IsButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#aabd0abde2a268f185577e5cdad70804e)
    fn is_button(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsButton(self.as_ptr()) }
    }
    /// Returns true if the system has been setup to do page scrolling with the mouse wheel instead of line scrolling.
    ///
    /// [See `wxMouseEvent::IsPageScroll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#ab00378989ac7f5e8c56bcc4bd5e4251a)
    fn is_page_scroll(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsPageScroll(self.as_ptr()) }
    }
    /// Returns true if the mouse was leaving the window.
    ///
    /// [See `wxMouseEvent::Leaving()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a40c80ae3b1c99525711c5c68aa31d917)
    fn leaving(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Leaving(self.as_ptr()) }
    }
    /// Returns true if the event was a left double click.
    ///
    /// [See `wxMouseEvent::LeftDClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#ad34fad1c0f5a1e1912edcc3a8dec1c6f)
    fn left_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftDClick(self.as_ptr()) }
    }
    /// Returns true if the left mouse button changed to down.
    ///
    /// [See `wxMouseEvent::LeftDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#af24cf14161da22d4d0d8bc80ffe33150)
    fn left_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftDown(self.as_ptr()) }
    }
    /// Returns true if the left mouse button changed to up.
    ///
    /// [See `wxMouseEvent::LeftUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a47552c7cc416599eed216be1948da5c8)
    fn left_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftUp(self.as_ptr()) }
    }
    /// Returns true if the event is a magnify (i.e. pinch to zoom) event.
    ///
    /// [See `wxMouseEvent::Magnify()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a661103e003b464a6af90fef888a4bcc1)
    fn magnify(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Magnify(self.as_ptr()) }
    }
    /// Returns true if the Meta key was down at the time of the event.
    ///
    /// [See `wxMouseEvent::MetaDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#afe6f69d3384ce92e42f4c57ea625303d)
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MetaDown(self.as_ptr()) }
    }
    /// Returns true if the event was a middle double click.
    ///
    /// [See `wxMouseEvent::MiddleDClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a82a4878e915c35d2a71fe2d882c5e7fd)
    fn middle_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleDClick(self.as_ptr()) }
    }
    /// Returns true if the middle mouse button changed to down.
    ///
    /// [See `wxMouseEvent::MiddleDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a3e1baf28a5a73bf16d4a567fdb05fa25)
    fn middle_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleDown(self.as_ptr()) }
    }
    /// Returns true if the middle mouse button changed to up.
    ///
    /// [See `wxMouseEvent::MiddleUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a53904a7f504db15ad3978d270bee2ebf)
    fn middle_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleUp(self.as_ptr()) }
    }
    /// Returns true if this was a motion event and no mouse buttons were pressed.
    ///
    /// [See `wxMouseEvent::Moving()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a03707fb904deb70ab5cfe2e893046465)
    fn moving(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Moving(self.as_ptr()) }
    }
    /// Returns true if the event was a right double click.
    ///
    /// [See `wxMouseEvent::RightDClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a3f47f47ce99e00138b8d4f31390192f9)
    fn right_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightDClick(self.as_ptr()) }
    }
    /// Returns true if the right mouse button changed to down.
    ///
    /// [See `wxMouseEvent::RightDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#ae3da81deb2d240ad6a43b8b3255e00b9)
    fn right_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightDown(self.as_ptr()) }
    }
    /// Returns true if the right mouse button changed to up.
    ///
    /// [See `wxMouseEvent::RightUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_event.html#a345a41200f4fb64724e54b2fd85e7770)
    fn right_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightUp(self.as_ptr()) }
    }
}

// wxMouseEventsManager
/// Helper for handling mouse input events in windows containing multiple items.
///
/// [See `wxMouseEventsManager`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_events_manager.html)
pub trait MouseEventsManagerMethods: EvtHandlerMethods {
    /// Finishes initialization of the object created using default constructor.
    ///
    /// [See `wxMouseEventsManager::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_mouse_events_manager.html#a8627443b743f13fb4a486ee9831f6f89)
    fn create<W: WindowMethods>(&self, win: Option<&W>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMouseEventsManager_Create(self.as_ptr(), win)
        }
    }
}

// wxMoveEvent
/// A move event holds information about window position change.
///
/// [See `wxMoveEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_move_event.html)
pub trait MoveEventMethods: EventMethods {
    /// Returns the position of the window generating the move change event.
    ///
    /// [See `wxMoveEvent::GetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_move_event.html#a4d49085d2cc9c758e8520048cdd33bee)
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxMoveEvent_GetPosition(self.as_ptr())) }
    }
    ///
    /// [See `wxMoveEvent::GetRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_move_event.html#ae759cbaaa2f269271bcc3ec2f27f15fb)
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxMoveEvent_GetRect(self.as_ptr())) }
    }
    ///
    /// [See `wxMoveEvent::SetRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_move_event.html#a8a497be8943285a6cbc031b466dda914)
    fn set_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxMoveEvent_SetRect(self.as_ptr(), rect)
        }
    }
    ///
    /// [See `wxMoveEvent::SetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_move_event.html#a4abd7f5e82edbc613cbddca16d54f4ce)
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxMoveEvent_SetPosition(self.as_ptr(), pos)
        }
    }
}
