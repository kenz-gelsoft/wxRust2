#![allow(non_upper_case_globals)]

use super::*;

// wxMDIChildFrame
wx_class! { MDIChildFrame =
    MDIChildFrameIsOwned<true>(wxMDIChildFrame) impl
        MDIChildFrameMethods,
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MDIChildFrameIsOwned<OWNED> {
    pub fn new_2step() -> MDIChildFrameIsOwned<OWNED> {
        unsafe { MDIChildFrameIsOwned(ffi::wxMDIChildFrame_new()) }
    }
    pub fn new<M: MDIParentFrameMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&M>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> MDIChildFrameIsOwned<OWNED> {
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
            MDIChildFrameIsOwned(ffi::wxMDIChildFrame_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MDIChildFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMDIChildFrame_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for MDIChildFrameIsOwned<OWNED> {
    fn restore(&self) {
        unsafe { ffi::wxMDIChildFrame_Restore(self.as_ptr()) }
    }
}

// wxMDIParentFrame
wx_class! { MDIParentFrame =
    MDIParentFrameIsOwned<true>(wxMDIParentFrame) impl
        MDIParentFrameMethods,
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MDIParentFrameIsOwned<OWNED> {
    pub fn new_2step() -> MDIParentFrameIsOwned<OWNED> {
        unsafe { MDIParentFrameIsOwned(ffi::wxMDIParentFrame_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> MDIParentFrameIsOwned<OWNED> {
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
            MDIParentFrameIsOwned(ffi::wxMDIParentFrame_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MDIParentFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMDIParentFrame_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for MDIParentFrameIsOwned<OWNED> {
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
            ffi::wxMDIParentFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}

// wxMenu
wx_class! { Menu =
    MenuIsOwned<true>(wxMenu) impl
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
wx_class! { MenuBar =
    MenuBarIsOwned<true>(wxMenuBar) impl
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

// wxMenuItem
wx_class! { MenuItem =
    MenuItemIsOwned<true>(wxMenuItem) impl
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
