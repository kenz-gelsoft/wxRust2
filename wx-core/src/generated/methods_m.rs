use super::*;

// wxMask
pub trait MaskMethods: ObjectMethods {
    // DTOR: fn ~wxMask()
    fn create_int<B: BitmapMethods>(&self, bitmap: &B, index: c_int) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMask_Create(self.as_ptr(), bitmap, index)
        }
    }
    fn create<B: BitmapMethods>(&self, bitmap: &B) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMask_Create1(self.as_ptr(), bitmap)
        }
    }
    fn create_colour<B: BitmapMethods, C: ColourMethods>(&self, bitmap: &B, colour: &C) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxMask_Create2(self.as_ptr(), bitmap, colour)
        }
    }
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMask_GetBitmap(self.as_ptr())) }
    }
}

// wxMaximizeEvent
pub trait MaximizeEventMethods: EventMethods {}

// wxMemoryDC
pub trait MemoryDCMethods: DCMethods {
    fn select_object<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMemoryDC_SelectObject(self.as_ptr(), bitmap)
        }
    }
    fn select_object_as_source<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMemoryDC_SelectObjectAsSource(self.as_ptr(), bitmap)
        }
    }
    fn get_selected_bitmap(&self) -> BitmapIsOwned<false> {
        unsafe { BitmapIsOwned::from_ptr(ffi::wxMemoryDC_GetSelectedBitmap(self.as_ptr())) }
    }
    // BLOCKED: fn GetSelectedBitmap1()
}

// wxMenu
pub trait MenuMethods: EvtHandlerMethods {
    // DTOR: fn ~wxMenu()
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
    fn append_separator(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_AppendSeparator(self.as_ptr())) }
    }
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
    fn break_(&self) {
        unsafe { ffi::wxMenu_Break(self.as_ptr()) }
    }
    fn check(&self, id: c_int, check: bool) {
        unsafe { ffi::wxMenu_Check(self.as_ptr(), id, check) }
    }
    fn delete_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_Delete(self.as_ptr(), id) }
    }
    fn delete_menuitem<M: MenuItemMethods>(&self, item: Option<&M>) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Delete1(self.as_ptr(), item)
        }
    }
    fn destroy_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_Destroy(self.as_ptr(), id) }
    }
    fn destroy_menuitem<M: MenuItemMethods>(&self, item: Option<&M>) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Destroy1(self.as_ptr(), item)
        }
    }
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { ffi::wxMenu_Enable(self.as_ptr(), id, enable) }
    }
    fn find_child_item(&self, id: c_int, pos: *mut c_void) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_FindChildItem(self.as_ptr(), id, pos)) }
    }
    fn find_item_str(&self, item_string: &str) -> c_int {
        unsafe {
            let item_string = WxString::from(item_string);
            let item_string = item_string.as_ptr();
            ffi::wxMenu_FindItem(self.as_ptr(), item_string)
        }
    }
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
    fn find_item_by_position(&self, position: usize) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_FindItemByPosition(self.as_ptr(), position)) }
    }
    fn get_help_string(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetHelpString(self.as_ptr(), id)).into() }
    }
    fn get_label(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetLabel(self.as_ptr(), id)).into() }
    }
    fn get_label_text(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetLabelText(self.as_ptr(), id)).into() }
    }
    fn get_menu_item_count(&self) -> usize {
        unsafe { ffi::wxMenu_GetMenuItemCount(self.as_ptr()) }
    }
    // BLOCKED: fn GetMenuItems()
    // BLOCKED: fn GetMenuItems1()
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetTitle(self.as_ptr())).into() }
    }
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
    fn insert_separator(&self, pos: usize) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_InsertSeparator(self.as_ptr(), pos)) }
    }
    fn is_checked(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_IsChecked(self.as_ptr(), id) }
    }
    fn is_enabled(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_IsEnabled(self.as_ptr(), id) }
    }
    // NOT_SUPPORTED: fn MSWCommand()
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
    fn prepend_separator(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_PrependSeparator(self.as_ptr())) }
    }
    fn remove_int(&self, id: c_int) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_Remove(self.as_ptr(), id)) }
    }
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
    fn set_help_string(&self, id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenu_SetHelpString(self.as_ptr(), id, help_string)
        }
    }
    fn set_label(&self, id: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenu_SetLabel(self.as_ptr(), id, label)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenu_SetTitle(self.as_ptr(), title)
        }
    }
    fn update_ui<E: EvtHandlerMethods>(&self, source: Option<&E>) {
        unsafe {
            let source = match source {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_UpdateUI(self.as_ptr(), source)
        }
    }
    fn set_invoking_window<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_SetInvokingWindow(self.as_ptr(), win)
        }
    }
    fn get_invoking_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxMenu_GetInvokingWindow(self.as_ptr())) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxMenu_GetWindow(self.as_ptr())) }
    }
    fn get_style(&self) -> c_long {
        unsafe { ffi::wxMenu_GetStyle(self.as_ptr()) }
    }
    fn set_parent<M: MenuMethods>(&self, parent: Option<&M>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_SetParent(self.as_ptr(), parent)
        }
    }
    fn get_parent(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenu_GetParent(self.as_ptr())) }
    }
    fn attach<M: MenuBarMethods>(&self, menubar: Option<&M>) {
        unsafe {
            let menubar = match menubar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Attach(self.as_ptr(), menubar)
        }
    }
    fn detach(&self) {
        unsafe { ffi::wxMenu_Detach(self.as_ptr()) }
    }
    fn is_attached(&self) -> bool {
        unsafe { ffi::wxMenu_IsAttached(self.as_ptr()) }
    }
}

// wxMenuBar
pub trait MenuBarMethods: WindowMethods {
    // DTOR: fn ~wxMenuBar()
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
    fn check(&self, id: c_int, check: bool) {
        unsafe { ffi::wxMenuBar_Check(self.as_ptr(), id, check) }
    }
    fn enable_int(&self, id: c_int, enable: bool) {
        unsafe { ffi::wxMenuBar_Enable(self.as_ptr(), id, enable) }
    }
    fn is_enabled_top(&self, pos: usize) -> bool {
        unsafe { ffi::wxMenuBar_IsEnabledTop(self.as_ptr(), pos) }
    }
    fn enable_top(&self, pos: usize, enable: bool) {
        unsafe { ffi::wxMenuBar_EnableTop(self.as_ptr(), pos, enable) }
    }
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
    fn find_menu(&self, title: &str) -> c_int {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenuBar_FindMenu(self.as_ptr(), title)
        }
    }
    fn find_menu_item(&self, menu_string: &str, item_string: &str) -> c_int {
        unsafe {
            let menu_string = WxString::from(menu_string);
            let menu_string = menu_string.as_ptr();
            let item_string = WxString::from(item_string);
            let item_string = item_string.as_ptr();
            ffi::wxMenuBar_FindMenuItem(self.as_ptr(), menu_string, item_string)
        }
    }
    fn get_help_string(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetHelpString(self.as_ptr(), id)).into() }
    }
    fn get_label_int(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetLabel(self.as_ptr(), id)).into() }
    }
    // BLOCKED: fn GetLabelTop()
    fn get_menu(&self, menu_index: usize) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_GetMenu(self.as_ptr(), menu_index)) }
    }
    fn get_menu_count(&self) -> usize {
        unsafe { ffi::wxMenuBar_GetMenuCount(self.as_ptr()) }
    }
    fn get_menu_label(&self, pos: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetMenuLabel(self.as_ptr(), pos)).into() }
    }
    fn get_menu_label_text(&self, pos: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetMenuLabelText(self.as_ptr(), pos)).into() }
    }
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
    fn is_checked(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenuBar_IsChecked(self.as_ptr(), id) }
    }
    fn is_enabled_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenuBar_IsEnabled(self.as_ptr(), id) }
    }
    fn remove(&self, pos: usize) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_Remove(self.as_ptr(), pos)) }
    }
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
    fn set_help_string(&self, id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenuBar_SetHelpString(self.as_ptr(), id, help_string)
        }
    }
    fn set_label_int(&self, id: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuBar_SetLabel(self.as_ptr(), id, label)
        }
    }
    // BLOCKED: fn SetLabelTop()
    fn set_menu_label(&self, pos: usize, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuBar_SetMenuLabel(self.as_ptr(), pos, label)
        }
    }
    fn osx_get_apple_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_OSXGetAppleMenu(self.as_ptr())) }
    }
    fn get_frame(&self) -> WeakRef<Frame> {
        unsafe { WeakRef::<Frame>::from(ffi::wxMenuBar_GetFrame(self.as_ptr())) }
    }
    fn is_attached(&self) -> bool {
        unsafe { ffi::wxMenuBar_IsAttached(self.as_ptr()) }
    }
    fn attach<F: FrameMethods>(&self, frame: Option<&F>) {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuBar_Attach(self.as_ptr(), frame)
        }
    }
    fn detach(&self) {
        unsafe { ffi::wxMenuBar_Detach(self.as_ptr()) }
    }
    fn mac_set_common_menu_bar<M: MenuBarMethods>(menubar: Option<&M>) {
        unsafe {
            let menubar = match menubar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuBar_MacSetCommonMenuBar(menubar)
        }
    }
    fn mac_get_common_menu_bar() -> WeakRef<MenuBar> {
        unsafe { WeakRef::<MenuBar>::from(ffi::wxMenuBar_MacGetCommonMenuBar()) }
    }
}

// wxMenuEvent
pub trait MenuEventMethods: EventMethods {
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuEvent_GetMenu(self.as_ptr())) }
    }
    fn get_menu_id(&self) -> c_int {
        unsafe { ffi::wxMenuEvent_GetMenuId(self.as_ptr()) }
    }
    fn is_popup(&self) -> bool {
        unsafe { ffi::wxMenuEvent_IsPopup(self.as_ptr()) }
    }
}

// wxMenuItem
pub trait MenuItemMethods: ObjectMethods {
    // BLOCKED: fn GetBackgroundColour()
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetBitmap(self.as_ptr())) }
    }
    fn get_bitmap_bool(&self, checked: bool) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetBitmap1(self.as_ptr(), checked)) }
    }
    fn get_bitmap_bundle(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxMenuItem_GetBitmapBundle(self.as_ptr())) }
    }
    fn get_disabled_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetDisabledBitmap(self.as_ptr())) }
    }
    // BLOCKED: fn GetFont()
    fn get_help(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetHelp(self.as_ptr())).into() }
    }
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetId(self.as_ptr()) }
    }
    fn get_item_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetItemLabel(self.as_ptr())).into() }
    }
    fn get_item_label_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetItemLabelText(self.as_ptr())).into() }
    }
    fn get_kind(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetKind(self.as_ptr()) }
    }
    // BLOCKED: fn GetLabel()
    fn get_margin_width(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetMarginWidth(self.as_ptr()) }
    }
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuItem_GetMenu(self.as_ptr())) }
    }
    // BLOCKED: fn GetName()
    fn get_sub_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuItem_GetSubMenu(self.as_ptr())) }
    }
    // BLOCKED: fn GetText()
    // BLOCKED: fn GetTextColour()
    fn get_accel(&self) -> Option<AcceleratorEntryIsOwned<false>> {
        unsafe { AcceleratorEntry::option_from(ffi::wxMenuItem_GetAccel(self.as_ptr())) }
    }
    // BLOCKED: fn GetAccelFromString()
    fn is_check(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsCheck(self.as_ptr()) }
    }
    fn is_checkable(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsCheckable(self.as_ptr()) }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsChecked(self.as_ptr()) }
    }
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsEnabled(self.as_ptr()) }
    }
    fn is_radio(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsRadio(self.as_ptr()) }
    }
    fn is_separator(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsSeparator(self.as_ptr()) }
    }
    fn is_sub_menu(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsSubMenu(self.as_ptr()) }
    }
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxMenuItem_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxMenuItem_SetBitmap(self.as_ptr(), bmp)
        }
    }
    fn set_bitmap_bool<B: BitmapBundleMethods>(&self, bmp: &B, checked: bool) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxMenuItem_SetBitmap1(self.as_ptr(), bmp, checked)
        }
    }
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
    fn set_disabled_bitmap<B: BitmapBundleMethods>(&self, disabled: &B) {
        unsafe {
            let disabled = disabled.as_ptr();
            ffi::wxMenuItem_SetDisabledBitmap(self.as_ptr(), disabled)
        }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxMenuItem_SetFont(self.as_ptr(), font)
        }
    }
    fn set_help(&self, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenuItem_SetHelp(self.as_ptr(), help_string)
        }
    }
    fn set_item_label(&self, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuItem_SetItemLabel(self.as_ptr(), label)
        }
    }
    fn set_margin_width(&self, width: c_int) {
        unsafe { ffi::wxMenuItem_SetMarginWidth(self.as_ptr(), width) }
    }
    fn set_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetMenu(self.as_ptr(), menu)
        }
    }
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
    fn set_text_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxMenuItem_SetTextColour(self.as_ptr(), colour)
        }
    }
    fn set_accel<A: AcceleratorEntryMethods>(&self, accel: Option<&A>) {
        unsafe {
            let accel = match accel {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetAccel(self.as_ptr(), accel)
        }
    }
    fn add_extra_accel<A: AcceleratorEntryMethods>(&self, accel: &A) {
        unsafe {
            let accel = accel.as_ptr();
            ffi::wxMenuItem_AddExtraAccel(self.as_ptr(), accel)
        }
    }
    fn clear_extra_accels(&self) {
        unsafe { ffi::wxMenuItem_ClearExtraAccels(self.as_ptr()) }
    }
    // DTOR: fn ~wxMenuItem()
    fn check(&self, check: bool) {
        unsafe { ffi::wxMenuItem_Check(self.as_ptr(), check) }
    }
    fn enable(&self, enable: bool) {
        unsafe { ffi::wxMenuItem_Enable(self.as_ptr(), enable) }
    }
    // BLOCKED: fn GetLabelFromText()
    fn get_label_text(text: &str) -> String {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            WxString::from_ptr(ffi::wxMenuItem_GetLabelText(text)).into()
        }
    }
}

// wxMessageDialog
pub trait MessageDialogMethods: DialogMethods {
    fn set_extended_message(&self, extended_message: &str) {
        unsafe {
            let extended_message = WxString::from(extended_message);
            let extended_message = extended_message.as_ptr();
            ffi::wxMessageDialog_SetExtendedMessage(self.as_ptr(), extended_message)
        }
    }
    fn set_help_label(&self, help: *const c_void) -> bool {
        unsafe { ffi::wxMessageDialog_SetHelpLabel(self.as_ptr(), help) }
    }
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxMessageDialog_SetMessage(self.as_ptr(), message)
        }
    }
    fn set_ok_cancel_labels(&self, ok: *const c_void, cancel: *const c_void) -> bool {
        unsafe { ffi::wxMessageDialog_SetOKCancelLabels(self.as_ptr(), ok, cancel) }
    }
    fn set_ok_label(&self, ok: *const c_void) -> bool {
        unsafe { ffi::wxMessageDialog_SetOKLabel(self.as_ptr(), ok) }
    }
    fn set_yes_no_cancel_labels(
        &self,
        yes: *const c_void,
        no: *const c_void,
        cancel: *const c_void,
    ) -> bool {
        unsafe { ffi::wxMessageDialog_SetYesNoCancelLabels(self.as_ptr(), yes, no, cancel) }
    }
    fn set_yes_no_labels(&self, yes: *const c_void, no: *const c_void) -> bool {
        unsafe { ffi::wxMessageDialog_SetYesNoLabels(self.as_ptr(), yes, no) }
    }
    fn get_caption(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetCaption(self.as_ptr())).into() }
    }
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetMessage(self.as_ptr())).into() }
    }
    fn get_extended_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetExtendedMessage(self.as_ptr())).into() }
    }
    fn get_message_dialog_style(&self) -> c_long {
        unsafe { ffi::wxMessageDialog_GetMessageDialogStyle(self.as_ptr()) }
    }
    fn has_custom_labels(&self) -> bool {
        unsafe { ffi::wxMessageDialog_HasCustomLabels(self.as_ptr()) }
    }
    fn get_yes_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetYesLabel(self.as_ptr())).into() }
    }
    fn get_no_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetNoLabel(self.as_ptr())).into() }
    }
    fn get_ok_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetOKLabel(self.as_ptr())).into() }
    }
    fn get_cancel_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetCancelLabel(self.as_ptr())).into() }
    }
    fn get_help_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMessageDialog_GetHelpLabel(self.as_ptr())).into() }
    }
    fn get_effective_icon(&self) -> c_long {
        unsafe { ffi::wxMessageDialog_GetEffectiveIcon(self.as_ptr()) }
    }
}

// wxMessageOutputMessageBox
pub trait MessageOutputMessageBoxMethods: MessageOutputMethods {}

// wxMiniFrame
pub trait MiniFrameMethods: FrameMethods {
    // DTOR: fn ~wxMiniFrame()
}

// wxMirrorDC
pub trait MirrorDCMethods: DCMethods {}

// wxMouseCaptureChangedEvent
pub trait MouseCaptureChangedEventMethods: EventMethods {
    fn get_captured_window(&self) -> WeakRef<Window> {
        unsafe {
            WeakRef::<Window>::from(ffi::wxMouseCaptureChangedEvent_GetCapturedWindow(
                self.as_ptr(),
            ))
        }
    }
}

// wxMouseCaptureLostEvent
pub trait MouseCaptureLostEventMethods: EventMethods {}

// wxMouseEvent
pub trait MouseEventMethods: EventMethods {
    fn aux1_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1DClick(self.as_ptr()) }
    }
    fn aux1_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1Down(self.as_ptr()) }
    }
    fn aux1_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1Up(self.as_ptr()) }
    }
    fn aux2_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2DClick(self.as_ptr()) }
    }
    fn aux2_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2Down(self.as_ptr()) }
    }
    fn aux2_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2Up(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Button()
    // NOT_SUPPORTED: fn ButtonDClick()
    // NOT_SUPPORTED: fn ButtonDown()
    // NOT_SUPPORTED: fn ButtonUp()
    fn dragging(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Dragging(self.as_ptr()) }
    }
    fn entering(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Entering(self.as_ptr()) }
    }
    fn get_button(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetButton(self.as_ptr()) }
    }
    fn get_click_count(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetClickCount(self.as_ptr()) }
    }
    fn get_lines_per_action(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetLinesPerAction(self.as_ptr()) }
    }
    fn get_columns_per_action(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetColumnsPerAction(self.as_ptr()) }
    }
    fn get_logical_position<D: DCMethods>(&self, dc: &D) -> Point {
        unsafe {
            let dc = dc.as_ptr();
            Point::from_ptr(ffi::wxMouseEvent_GetLogicalPosition(self.as_ptr(), dc))
        }
    }
    // NOT_SUPPORTED: fn GetMagnification()
    fn get_wheel_delta(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetWheelDelta(self.as_ptr()) }
    }
    fn is_wheel_inverted(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsWheelInverted(self.as_ptr()) }
    }
    fn get_wheel_rotation(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetWheelRotation(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWheelAxis()
    fn is_button(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsButton(self.as_ptr()) }
    }
    fn is_page_scroll(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsPageScroll(self.as_ptr()) }
    }
    fn leaving(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Leaving(self.as_ptr()) }
    }
    fn left_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftDClick(self.as_ptr()) }
    }
    fn left_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftDown(self.as_ptr()) }
    }
    fn left_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftUp(self.as_ptr()) }
    }
    fn magnify(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Magnify(self.as_ptr()) }
    }
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MetaDown(self.as_ptr()) }
    }
    fn middle_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleDClick(self.as_ptr()) }
    }
    fn middle_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleDown(self.as_ptr()) }
    }
    fn middle_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleUp(self.as_ptr()) }
    }
    fn moving(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Moving(self.as_ptr()) }
    }
    fn right_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightDClick(self.as_ptr()) }
    }
    fn right_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightDown(self.as_ptr()) }
    }
    fn right_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightUp(self.as_ptr()) }
    }
}

// wxMouseEventsManager
pub trait MouseEventsManagerMethods: EvtHandlerMethods {
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
pub trait MoveEventMethods: EventMethods {
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxMoveEvent_GetPosition(self.as_ptr())) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxMoveEvent_GetRect(self.as_ptr())) }
    }
    fn set_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxMoveEvent_SetRect(self.as_ptr(), rect)
        }
    }
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxMoveEvent_SetPosition(self.as_ptr(), pos)
        }
    }
}
