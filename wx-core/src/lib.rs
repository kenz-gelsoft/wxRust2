use std::mem;
use std::os::raw::{c_int, c_long, c_void};
use std::ptr;

mod generated;
pub use generated::*;

// re-export wx_base
pub use wx_base::*;

#[doc(hidden)]
pub mod methods {
    use std::os::raw::{c_int, c_uint};

    pub use super::generated::methods::*;
    use super::*;

    // re-export wx_base::methods
    pub use wx_base::methods::*;

    pub trait Buildable<'a, P, B> {
        fn builder(parent: Option<&'a P>) -> B;
    }

    pub trait SizerItemListMethods: WxRustMethods {
        fn get_count(&self) -> usize {
            unsafe { super::ffi::wxSizerItemList_GetCount(self.as_ptr()) }
        }
        fn is_empty(&self) -> bool {
            unsafe { super::ffi::wxSizerItemList_IsEmpty(self.as_ptr()) }
        }
    }
    pub trait WindowListMethods: WxRustMethods {
        fn get_count(&self) -> usize {
            unsafe { super::ffi::wxWindowList_GetCount(self.as_ptr()) }
        }
        fn is_empty(&self) -> bool {
            unsafe { super::ffi::wxWindowList_IsEmpty(self.as_ptr()) }
        }
    }

    pub trait MenuItemBuilder {
        fn item<ID: Into<c_int>>(self, id: ID, s: &str) -> Self;
        fn item_h<ID: Into<c_int>>(self, id: ID, s: &str, h: &str) -> Self;
        fn check_item<ID: Into<c_int>>(self, id: ID, s: &str) -> Self;
        fn radio_item<ID: Into<c_int>>(self, id: ID, s: &str) -> Self;
        fn sub_menu<M: MenuMethods>(self, s: &str, submenu: &M) -> Self;
        fn separator(self) -> Self;
    }

    // wxItemContainer
    pub trait ItemContainerMethods: ItemContainerImmutableMethods {
        fn as_item_container(&self) -> *mut c_void;
        fn append_str(&self, item: &str) -> c_int {
            unsafe {
                let item = wx_base::wx_string_from(item);
                ffi::wxItemContainer_Append(self.as_item_container(), item)
            }
        }
        fn append_str_void(&self, item: &str, client_data: *mut c_void) -> c_int {
            unsafe {
                let item = wx_base::wx_string_from(item);
                ffi::wxItemContainer_Append1(self.as_item_container(), item, client_data)
            }
        }
        fn append_str_clientdata(&self, item: &str, client_data: *mut c_void) -> c_int {
            unsafe {
                let item = wx_base::wx_string_from(item);
                ffi::wxItemContainer_Append2(self.as_item_container(), item, client_data)
            }
        }
        fn append_arraystring<A: ArrayStringMethods>(&self, items: &A) -> c_int {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Append3(self.as_item_container(), items)
            }
        }
        // BLOCKED: fn Append4()
        fn append_arraystring_void<A: ArrayStringMethods>(
            &self,
            items: &A,
            client_data: *mut c_void,
        ) -> c_int {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Append5(self.as_item_container(), items, client_data)
            }
        }
        fn append_arraystring_clientdata<A: ArrayStringMethods>(
            &self,
            items: &A,
            client_data: *mut c_void,
        ) -> c_int {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Append6(self.as_item_container(), items, client_data)
            }
        }
        // BLOCKED: fn Append7()
        // BLOCKED: fn Append8()
        // BLOCKED: fn Append9()
        fn clear(&self) {
            unsafe { ffi::wxItemContainer_Clear(self.as_item_container()) }
        }
        fn delete(&self, n: c_uint) {
            unsafe { ffi::wxItemContainer_Delete(self.as_item_container(), n) }
        }
        fn detach_client_object(&self, n: c_uint) -> *mut c_void {
            unsafe { ffi::wxItemContainer_DetachClientObject(self.as_item_container(), n) }
        }
        fn has_client_data(&self) -> bool {
            unsafe { ffi::wxItemContainer_HasClientData(self.as_item_container()) }
        }
        fn has_client_object_data(&self) -> bool {
            unsafe { ffi::wxItemContainer_HasClientObjectData(self.as_item_container()) }
        }
        fn has_client_untyped_data(&self) -> bool {
            unsafe { ffi::wxItemContainer_HasClientUntypedData(self.as_item_container()) }
        }
        fn get_client_data(&self, n: c_uint) -> *mut c_void {
            unsafe { ffi::wxItemContainer_GetClientData(self.as_item_container(), n) }
        }
        fn get_client_object(&self, n: c_uint) -> *mut c_void {
            unsafe { ffi::wxItemContainer_GetClientObject(self.as_item_container(), n) }
        }
        fn set_client_data(&self, n: c_uint, data: *mut c_void) {
            unsafe { ffi::wxItemContainer_SetClientData(self.as_item_container(), n, data) }
        }
        fn set_client_object(&self, n: c_uint, data: *mut c_void) {
            unsafe { ffi::wxItemContainer_SetClientObject(self.as_item_container(), n, data) }
        }
        fn insert_str(&self, item: &str, pos: c_uint) -> c_int {
            unsafe {
                let item = wx_base::wx_string_from(item);
                ffi::wxItemContainer_Insert(self.as_item_container(), item, pos)
            }
        }
        fn insert_str_void(&self, item: &str, pos: c_uint, client_data: *mut c_void) -> c_int {
            unsafe {
                let item = wx_base::wx_string_from(item);
                ffi::wxItemContainer_Insert1(self.as_item_container(), item, pos, client_data)
            }
        }
        fn insert_str_clientdata(
            &self,
            item: &str,
            pos: c_uint,
            client_data: *mut c_void,
        ) -> c_int {
            unsafe {
                let item = wx_base::wx_string_from(item);
                ffi::wxItemContainer_Insert2(self.as_item_container(), item, pos, client_data)
            }
        }
        fn insert_arraystring<A: ArrayStringMethods>(&self, items: &A, pos: c_uint) -> c_int {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Insert3(self.as_item_container(), items, pos)
            }
        }
        // BLOCKED: fn Insert4()
        fn insert_arraystring_void<A: ArrayStringMethods>(
            &self,
            items: &A,
            pos: c_uint,
            client_data: *mut c_void,
        ) -> c_int {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Insert5(self.as_item_container(), items, pos, client_data)
            }
        }
        fn insert_arraystring_clientdata<A: ArrayStringMethods>(
            &self,
            items: &A,
            pos: c_uint,
            client_data: *mut c_void,
        ) -> c_int {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Insert6(self.as_item_container(), items, pos, client_data)
            }
        }
        // BLOCKED: fn Insert7()
        // BLOCKED: fn Insert8()
        // BLOCKED: fn Insert9()
        fn set<A: ArrayStringMethods>(&self, items: &A) {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Set(self.as_item_container(), items)
            }
        }
        // BLOCKED: fn Set1()
        fn set_void<A: ArrayStringMethods>(&self, items: &A, client_data: *mut c_void) {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Set2(self.as_item_container(), items, client_data)
            }
        }
        fn set_clientdata<A: ArrayStringMethods>(&self, items: &A, client_data: *mut c_void) {
            unsafe {
                let items = items.as_ptr();
                ffi::wxItemContainer_Set3(self.as_item_container(), items, client_data)
            }
        }
        // BLOCKED: fn Set4()
        // BLOCKED: fn Set5()
        // BLOCKED: fn Set6()
    }

    // wxItemContainerImmutable
    pub trait ItemContainerImmutableMethods: WxRustMethods {
        fn as_item_container_immutable(&self) -> *mut c_void;
        fn set_selection(&self, n: c_int) {
            unsafe {
                ffi::wxItemContainerImmutable_SetSelection(self.as_item_container_immutable(), n)
            }
        }
        fn get_selection(&self) -> c_int {
            unsafe {
                ffi::wxItemContainerImmutable_GetSelection(self.as_item_container_immutable())
            }
        }
        fn set_string_selection(&self, string: &str) -> bool {
            unsafe {
                let string = wx_base::wx_string_from(string);
                ffi::wxItemContainerImmutable_SetStringSelection(
                    self.as_item_container_immutable(),
                    string,
                )
            }
        }
        fn get_string_selection(&self) -> String {
            unsafe {
                wx_base::from_wx_string(ffi::wxItemContainerImmutable_GetStringSelection(
                    self.as_item_container_immutable(),
                ))
            }
        }
        fn select(&self, n: c_int) {
            unsafe { ffi::wxItemContainerImmutable_Select(self.as_item_container_immutable(), n) }
        }
        fn get_count(&self) -> c_uint {
            unsafe { ffi::wxItemContainerImmutable_GetCount(self.as_item_container_immutable()) }
        }
        fn is_empty(&self) -> bool {
            unsafe { ffi::wxItemContainerImmutable_IsEmpty(self.as_item_container_immutable()) }
        }
        fn get_string(&self, n: c_uint) -> String {
            unsafe {
                wx_base::from_wx_string(ffi::wxItemContainerImmutable_GetString(
                    self.as_item_container_immutable(),
                    n,
                ))
            }
        }
        fn get_strings(&self) -> ArrayString {
            unsafe {
                ArrayStringIsOwned::from_ptr(ffi::wxItemContainerImmutable_GetStrings(
                    self.as_item_container_immutable(),
                ))
            }
        }
        fn set_string(&self, n: c_uint, string: &str) {
            unsafe {
                let string = wx_base::wx_string_from(string);
                ffi::wxItemContainerImmutable_SetString(
                    self.as_item_container_immutable(),
                    n,
                    string,
                )
            }
        }
        fn find_string(&self, string: &str, case_sensitive: bool) -> c_int {
            unsafe {
                let string = wx_base::wx_string_from(string);
                ffi::wxItemContainerImmutable_FindString(
                    self.as_item_container_immutable(),
                    string,
                    case_sensitive,
                )
            }
        }
    }
}
use methods::*;

mod ffi {
    use std::os::raw::{c_int, c_long, c_uint, c_void};
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);

        // SizerItemList
        pub fn wxSizerItemList_new() -> *mut c_void;
        pub fn wxSizerItemList_delete(self_: *mut c_void);
        pub fn wxSizerItemList_GetCount(self_: *mut c_void) -> usize;
        pub fn wxSizerItemList_IsEmpty(self_: *mut c_void) -> bool;

        // WindowList
        pub fn wxWindowList_new() -> *mut c_void;
        pub fn wxWindowList_delete(self_: *mut c_void);
        pub fn wxWindowList_GetCount(self_: *mut c_void) -> usize;
        pub fn wxWindowList_IsEmpty(self_: *mut c_void) -> bool;

        pub fn wxRustMessageBox(
            message: *const c_void,
            caption: *const c_void,
            style: c_int,
            parent: *mut c_void,
            x: c_int,
            y: c_int,
        );

        pub fn wxChoice_AsItemContainer(obj: *mut c_void) -> *mut c_void;
        // wxItemContainer
        pub fn wxItemContainer_delete(self_: *mut c_void);
        pub fn wxItemContainer_Append(self_: *mut c_void, item: *const c_void) -> c_int;
        pub fn wxItemContainer_Append1(
            self_: *mut c_void,
            item: *const c_void,
            client_data: *mut c_void,
        ) -> c_int;
        pub fn wxItemContainer_Append2(
            self_: *mut c_void,
            item: *const c_void,
            client_data: *mut c_void,
        ) -> c_int;
        pub fn wxItemContainer_Append3(self_: *mut c_void, items: *const c_void) -> c_int;
        // BLOCKED: pub fn wxItemContainer_Append4(self_: *mut c_void, items: *const c_void) -> c_int;
        pub fn wxItemContainer_Append5(
            self_: *mut c_void,
            items: *const c_void,
            client_data: *mut c_void,
        ) -> c_int;
        pub fn wxItemContainer_Append6(
            self_: *mut c_void,
            items: *const c_void,
            client_data: *mut c_void,
        ) -> c_int;
        // BLOCKED: pub fn wxItemContainer_Append7(self_: *mut c_void, n: c_uint, items: *const c_void) -> c_int;
        // BLOCKED: pub fn wxItemContainer_Append8(self_: *mut c_void, n: c_uint, items: *const c_void, client_data: *mut c_void) -> c_int;
        // BLOCKED: pub fn wxItemContainer_Append9(self_: *mut c_void, n: c_uint, items: *const c_void, client_data: *mut c_void) -> c_int;
        pub fn wxItemContainer_Clear(self_: *mut c_void);
        pub fn wxItemContainer_Delete(self_: *mut c_void, n: c_uint);
        pub fn wxItemContainer_DetachClientObject(self_: *mut c_void, n: c_uint) -> *mut c_void;
        pub fn wxItemContainer_HasClientData(self_: *const c_void) -> bool;
        pub fn wxItemContainer_HasClientObjectData(self_: *const c_void) -> bool;
        pub fn wxItemContainer_HasClientUntypedData(self_: *const c_void) -> bool;
        pub fn wxItemContainer_GetClientData(self_: *const c_void, n: c_uint) -> *mut c_void;
        pub fn wxItemContainer_GetClientObject(self_: *const c_void, n: c_uint) -> *mut c_void;
        pub fn wxItemContainer_SetClientData(self_: *mut c_void, n: c_uint, data: *mut c_void);
        pub fn wxItemContainer_SetClientObject(self_: *mut c_void, n: c_uint, data: *mut c_void);
        pub fn wxItemContainer_Insert(
            self_: *mut c_void,
            item: *const c_void,
            pos: c_uint,
        ) -> c_int;
        pub fn wxItemContainer_Insert1(
            self_: *mut c_void,
            item: *const c_void,
            pos: c_uint,
            client_data: *mut c_void,
        ) -> c_int;
        pub fn wxItemContainer_Insert2(
            self_: *mut c_void,
            item: *const c_void,
            pos: c_uint,
            client_data: *mut c_void,
        ) -> c_int;
        pub fn wxItemContainer_Insert3(
            self_: *mut c_void,
            items: *const c_void,
            pos: c_uint,
        ) -> c_int;
        // BLOCKED: pub fn wxItemContainer_Insert4(self_: *mut c_void, items: *const c_void) -> c_int;
        pub fn wxItemContainer_Insert5(
            self_: *mut c_void,
            items: *const c_void,
            pos: c_uint,
            client_data: *mut c_void,
        ) -> c_int;
        pub fn wxItemContainer_Insert6(
            self_: *mut c_void,
            items: *const c_void,
            pos: c_uint,
            client_data: *mut c_void,
        ) -> c_int;
        // BLOCKED: pub fn wxItemContainer_Insert7(self_: *mut c_void, n: c_uint, items: *const c_void, pos: c_uint) -> c_int;
        // BLOCKED: pub fn wxItemContainer_Insert8(self_: *mut c_void, n: c_uint, items: *const c_void, pos: c_uint, client_data: *mut c_void) -> c_int;
        // BLOCKED: pub fn wxItemContainer_Insert9(self_: *mut c_void, n: c_uint, items: *const c_void, pos: c_uint, client_data: *mut c_void) -> c_int;
        pub fn wxItemContainer_Set(self_: *mut c_void, items: *const c_void);
        // BLOCKED: pub fn wxItemContainer_Set1(self_: *mut c_void, items: *const c_void);
        pub fn wxItemContainer_Set2(
            self_: *mut c_void,
            items: *const c_void,
            client_data: *mut c_void,
        );
        pub fn wxItemContainer_Set3(
            self_: *mut c_void,
            items: *const c_void,
            client_data: *mut c_void,
        );
        // BLOCKED: pub fn wxItemContainer_Set4(self_: *mut c_void, n: c_uint, items: *const c_void);
        // BLOCKED: pub fn wxItemContainer_Set5(self_: *mut c_void, n: c_uint, items: *const c_void, client_data: *mut c_void);
        // BLOCKED: pub fn wxItemContainer_Set6(self_: *mut c_void, n: c_uint, items: *const c_void, client_data: *mut c_void);

        pub fn wxRadioBox_AsItemContainerImmutable(obj: *mut c_void) -> *mut c_void;
        // wxItemContainerImmutable
        pub fn wxItemContainerImmutable_delete(self_: *mut c_void);
        pub fn wxItemContainerImmutable_SetSelection(self_: *mut c_void, n: c_int);
        pub fn wxItemContainerImmutable_GetSelection(self_: *const c_void) -> c_int;
        pub fn wxItemContainerImmutable_SetStringSelection(
            self_: *mut c_void,
            string: *const c_void,
        ) -> bool;
        pub fn wxItemContainerImmutable_GetStringSelection(self_: *const c_void) -> *mut c_void;
        pub fn wxItemContainerImmutable_Select(self_: *mut c_void, n: c_int);
        pub fn wxItemContainerImmutable_new() -> *mut c_void;
        pub fn wxItemContainerImmutable_GetCount(self_: *const c_void) -> c_uint;
        pub fn wxItemContainerImmutable_IsEmpty(self_: *const c_void) -> bool;
        pub fn wxItemContainerImmutable_GetString(self_: *const c_void, n: c_uint) -> *mut c_void;
        pub fn wxItemContainerImmutable_GetStrings(self_: *const c_void) -> *mut c_void;
        pub fn wxItemContainerImmutable_SetString(
            self_: *mut c_void,
            n: c_uint,
            string: *const c_void,
        );
        pub fn wxItemContainerImmutable_FindString(
            self_: *const c_void,
            string: *const c_void,
            case_sensitive: bool,
        ) -> c_int;

        pub fn wxTextCtrl_AsTextEntry(obj: *mut c_void) -> *mut c_void;
    }
}

// Mix-in wxItemContainer manually
impl<const OWNED: bool> ItemContainerMethods for ChoiceIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}

// Mix-in wxItemContainerImmutable manually
impl<const OWNED: bool> ItemContainerImmutableMethods for ChoiceIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for RadioBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxRadioBox_AsItemContainerImmutable(self.as_ptr()) }
    }
}

// Mix-in wxTextEntry manually
// impl<const OWNED: bool> TextEntryMethods for TextCtrlIsOwned<OWNED> {
//     fn as_text_entry(&self) -> *mut c_void {
//         unsafe { ffi::wxTextCtrl_AsTextEntry(self.as_ptr()) }
//     }
// }

pub struct FrameBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    title: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, FrameBuilder<'a, P>> for Frame {
    fn builder(parent: Option<&'a P>) -> FrameBuilder<'a, P> {
        FrameBuilder {
            parent: parent,
            id: ID_ANY,
            title: "".to_string(),
            pos: None,
            size: None,
            style: DEFAULT_FRAME_STYLE,
        }
    }
}
impl<'a, P: WindowMethods> FrameBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> Frame {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Frame::new(
            self.parent,
            self.id,
            &self.title,
            &pos,
            &size,
            self.style,
            "",
        )
    }
}

pub struct PanelBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, PanelBuilder<'a, P>> for Panel {
    fn builder(parent: Option<&'a P>) -> PanelBuilder<'a, P> {
        PanelBuilder {
            parent: parent,
            pos: None,
            size: None,
            style: TAB_TRAVERSAL,
        }
    }
}
impl<'a, P: WindowMethods> PanelBuilder<'a, P> {
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> Panel {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Panel::new(self.parent, ID_ANY, &pos, &size, self.style, "")
    }
}

// pub struct ActivityIndicatorBuilder<'a, P: WindowMethods> {
//     parent: Option<&'a P>,
//     id: c_int,
//     pos: Option<Point>,
//     size: Option<Size>,
//     style: c_long,
// }
// impl<'a, P: WindowMethods> Buildable<'a, P, ActivityIndicatorBuilder<'a, P>> for ActivityIndicator {
//     fn builder(parent: Option<&'a P>) -> ActivityIndicatorBuilder<'a, P> {
//         ActivityIndicatorBuilder {
//             parent: parent,
//             id: ID_ANY,
//             pos: None,
//             size: None,
//             style: 0,
//         }
//     }
// }
// impl<'a, P: WindowMethods> ActivityIndicatorBuilder<'a, P> {
//     pub fn id(&mut self, id: c_int) -> &mut Self {
//         self.id = id;
//         self
//     }
//     pub fn pos(&mut self, pos: Point) -> &mut Self {
//         self.pos = Some(pos);
//         self
//     }
//     pub fn size(&mut self, size: Size) -> &mut Self {
//         self.size = Some(size);
//         self
//     }
//     pub fn style(&mut self, style: c_long) -> &mut Self {
//         self.style = style;
//         self
//     }
//     pub fn build(&mut self) -> ActivityIndicator {
//         let pos = self.pos.take().unwrap_or_else(|| Point::default());
//         let size = self.size.take().unwrap_or_else(|| Size::default());
//         ActivityIndicator::new(self.parent, self.id, &pos, &size, self.style, "")
//     }
// }

pub struct BitmapButtonBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, BitmapButtonBuilder<'a, P>> for BitmapButton {
    fn builder(parent: Option<&'a P>) -> BitmapButtonBuilder<'a, P> {
        BitmapButtonBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> BitmapButtonBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build<B: BitmapMethods>(&mut self, bitmap: &B) -> BitmapButton {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        BitmapButton::new(
            self.parent,
            self.id,
            bitmap,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct ButtonBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ButtonBuilder<'a, P>> for Button {
    fn builder(parent: Option<&'a P>) -> ButtonBuilder<'a, P> {
        ButtonBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> ButtonBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> Button {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        Button::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct CheckBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, CheckBoxBuilder<'a, P>> for CheckBox {
    fn builder(parent: Option<&'a P>) -> CheckBoxBuilder<'a, P> {
        CheckBoxBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> CheckBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> CheckBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        CheckBox::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct ChoiceBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    choices: Option<ArrayString>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ChoiceBuilder<'a, P>> for Choice {
    fn builder(parent: Option<&'a P>) -> ChoiceBuilder<'a, P> {
        ChoiceBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            choices: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> ChoiceBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn choices(&mut self, choices: ArrayString) -> &mut Self {
        self.choices = Some(choices);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> Choice {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let choices = self.choices.take().unwrap_or_else(|| ArrayString::new());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        Choice::new(
            self.parent,
            self.id,
            &pos,
            &size,
            &choices,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct ListBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    choices: Option<ArrayString>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ListBoxBuilder<'a, P>> for ListBox {
    fn builder(parent: Option<&'a P>) -> ListBoxBuilder<'a, P> {
        ListBoxBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            choices: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> ListBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn choices(&mut self, choices: ArrayString) -> &mut Self {
        self.choices = Some(choices);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> ListBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let choices = self.choices.take().unwrap_or_else(|| ArrayString::new());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        ListBox::new(
            self.parent,
            self.id,
            &pos,
            &size,
            &choices,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct NotebookBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, NotebookBuilder<'a, P>> for Notebook {
    fn builder(parent: Option<&'a P>) -> NotebookBuilder<'a, P> {
        NotebookBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            style: 0,
        }
    }
}
impl<'a, P: WindowMethods> NotebookBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> Notebook {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Notebook::new(self.parent, self.id, &pos, &size, self.style, "")
    }
}

pub struct RadioBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    choices: Option<ArrayString>,
    major_dimension: c_int,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, RadioBoxBuilder<'a, P>> for RadioBox {
    fn builder(parent: Option<&'a P>) -> RadioBoxBuilder<'a, P> {
        RadioBoxBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            choices: None,
            major_dimension: 0,
            style: RA_SPECIFY_COLS.into(),
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> RadioBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn choices(&mut self, choices: ArrayString) -> &mut Self {
        self.choices = Some(choices);
        self
    }
    pub fn major_dimension(&mut self, major_dimension: c_int) -> &mut Self {
        self.major_dimension = major_dimension;
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> RadioBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let choices = self.choices.take().unwrap_or_else(|| ArrayString::new());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        RadioBox::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            &choices,
            self.major_dimension,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct StaticBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, StaticBoxBuilder<'a, P>> for StaticBox {
    fn builder(parent: Option<&'a P>) -> StaticBoxBuilder<'a, P> {
        StaticBoxBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            style: 0,
        }
    }
}
impl<'a, P: WindowMethods> StaticBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> StaticBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        StaticBox::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            self.style,
            "",
        )
    }
}

pub struct StaticTextBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, StaticTextBuilder<'a, P>> for StaticText {
    fn builder(parent: Option<&'a P>) -> StaticTextBuilder<'a, P> {
        StaticTextBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            style: 0,
        }
    }
}
impl<'a, P: WindowMethods> StaticTextBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> StaticText {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        StaticText::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            self.style,
            "",
        )
    }
}

pub struct TextCtrlBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    value: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, TextCtrlBuilder<'a, P>> for TextCtrl {
    fn builder(parent: Option<&'a P>) -> TextCtrlBuilder<'a, P> {
        TextCtrlBuilder {
            parent: parent,
            id: ID_ANY,
            value: "".to_string(),
            pos: None,
            size: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> TextCtrlBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn value(&mut self, value: &str) -> &mut Self {
        self.value = value.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> TextCtrl {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        TextCtrl::new(
            self.parent,
            self.id,
            &self.value,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct ToolBarBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ToolBarBuilder<'a, P>> for ToolBar {
    fn builder(parent: Option<&'a P>) -> ToolBarBuilder<'a, P> {
        ToolBarBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            style: TB_HORIZONTAL.into(),
        }
    }
}
impl<'a, P: WindowMethods> ToolBarBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> ToolBar {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        ToolBar::new(self.parent, self.id, &pos, &size, self.style, "")
    }
}

impl MenuItemBuilder for Menu {
    fn item<ID: Into<c_int>>(self, id: ID, item: &str) -> Self {
        self.item_h(id, item, "")
    }
    fn item_h<ID: Into<c_int>>(self, id: ID, item: &str, help: &str) -> Self {
        self.append_int_str(id.into(), item, help, ITEM_NORMAL);
        self
    }
    fn check_item<ID: Into<c_int>>(self, id: ID, item: &str) -> Self {
        self.append_check_item(id.into(), item, "");
        self
    }
    fn radio_item<ID: Into<c_int>>(self, id: ID, item: &str) -> Self {
        self.append_radio_item(id.into(), item, "");
        self
    }
    fn sub_menu<M: MenuMethods>(self, text: &str, submenu: &M) -> Self {
        self.append_sub_menu(Some(submenu), text, "");
        self
    }
    fn separator(self) -> Self {
        self.append_separator();
        self
    }
}

// wxDefaultPosition
impl<const OWNED: bool> Default for PointIsOwned<OWNED> {
    fn default() -> Self {
        PointIsOwned::new_with_int(-1, -1)
    }
}
// wxDefaultSize
impl<const OWNED: bool> Default for SizeIsOwned<OWNED> {
    fn default() -> Self {
        SizeIsOwned::new_with_int(-1, -1)
    }
}
// wxDefaultValidator
impl<const OWNED: bool> Default for ValidatorIsOwned<OWNED> {
    fn default() -> Self {
        ValidatorIsOwned::new()
    }
}

wx_class! { SizerItemList =
    SizerItemListIsOwned<true>(wxSizerItemList) impl
    SizerItemListMethods
}
impl<const OWNED: bool> SizerItemListIsOwned<OWNED> {
    pub fn new() -> Self {
        unsafe { SizerItemListIsOwned(ffi::wxSizerItemList_new()) }
    }
}
impl<const OWNED: bool> Drop for SizerItemListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSizerItemList_delete(self.0) }
        }
    }
}

wx_class! { WindowList =
    WindowListIsOwned<true>(wxWindowList) impl
        WindowListMethods
}
impl<const OWNED: bool> WindowListIsOwned<OWNED> {
    pub fn new() -> Self {
        unsafe { WindowListIsOwned(ffi::wxWindowList_new()) }
    }
}
impl<const OWNED: bool> Drop for WindowListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxWindowList_delete(self.0) }
        }
    }
}

pub fn message_box<T: WindowMethods>(
    message: &str,
    caption: &str,
    style: c_int,
    parent: Option<&T>,
) {
    unsafe {
        let message = wx_base::wx_string_from(message);
        let caption = wx_base::wx_string_from(caption);
        let parent = match parent {
            Some(r) => r.as_ptr(),
            None => ptr::null_mut(),
        };
        ffi::wxRustMessageBox(message, caption, style, parent, -1, -1)
    }
}
