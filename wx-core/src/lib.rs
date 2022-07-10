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
    }
}

pub struct ColourPickerCtrlBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    colour: Option<Colour>,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ColourPickerCtrlBuilder<'a, P>> for ColourPickerCtrl {
    fn builder(parent: Option<&'a P>) -> ColourPickerCtrlBuilder<'a, P> {
        ColourPickerCtrlBuilder {
            parent: parent,
            id: ID_ANY,
            colour: None,
            pos: None,
            size: None,
            style: CLRP_DEFAULT_STYLE.into(),
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> ColourPickerCtrlBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn colour(&mut self, colour: Colour) -> &mut Self {
        self.colour = Some(colour);
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
    pub fn build(&mut self) -> ColourPickerCtrl {
        let colour = self
            .colour
            .take()
            .unwrap_or_else(|| Colour::new_with_str("BLACK"));
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        ColourPickerCtrl::new(
            self.parent,
            self.id,
            &colour,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct DatePickerCtrlBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    dt: Option<DateTime>,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, DatePickerCtrlBuilder<'a, P>> for DatePickerCtrl {
    fn builder(parent: Option<&'a P>) -> DatePickerCtrlBuilder<'a, P> {
        DatePickerCtrlBuilder {
            parent: parent,
            id: ID_ANY,
            dt: None,
            pos: None,
            size: None,
            style: (DP_DEFAULT | DP_SHOWCENTURY).into(),
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> DatePickerCtrlBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn dt(&mut self, dt: Option<DateTime>) -> &mut Self {
        self.dt = dt;
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
    pub fn build(&mut self) -> DatePickerCtrl {
        let dt = self.dt.take().unwrap_or_else(|| DateTime::new());
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        DatePickerCtrl::new(
            self.parent,
            self.id,
            &dt,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}
pub struct FilePickerCtrlBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    path: String,
    message: String,
    wildcard: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
    name: String,
}
impl<'a, P: WindowMethods> Buildable<'a, P, FilePickerCtrlBuilder<'a, P>> for FilePickerCtrl {
    fn builder(parent: Option<&'a P>) -> FilePickerCtrlBuilder<'a, P> {
        FilePickerCtrlBuilder {
            parent: parent,
            id: ID_ANY,
            path: "".into(),
            // TODO: wxDirSelectorPromptStr should be handled as constant
            message: "Select a file".into(),
            wildcard: "*".into(),
            pos: None,
            size: None,
            style: (DP_DEFAULT | DP_SHOWCENTURY).into(),
            validator: None,
            name: "".into(),
        }
    }
}
impl<'a, P: WindowMethods> FilePickerCtrlBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }
    pub fn message(&mut self, message: String) -> &mut Self {
        self.message = message;
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
    pub fn build(&mut self) -> FilePickerCtrl {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        FilePickerCtrl::new(
            self.parent,
            self.id,
            &self.path,
            &self.message,
            &self.wildcard,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct DirPickerCtrlBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    path: String,
    message: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, DirPickerCtrlBuilder<'a, P>> for DirPickerCtrl {
    fn builder(parent: Option<&'a P>) -> DirPickerCtrlBuilder<'a, P> {
        DirPickerCtrlBuilder {
            parent: parent,
            id: ID_ANY,
            path: "".into(),
            // TODO: wxDirSelectorPromptStr should be handled as constant
            message: "Select a directory".into(),
            pos: None,
            size: None,
            style: (DP_DEFAULT | DP_SHOWCENTURY).into(),
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> DirPickerCtrlBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }
    pub fn message(&mut self, message: String) -> &mut Self {
        self.message = message;
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
    pub fn build(&mut self) -> DirPickerCtrl {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        DirPickerCtrl::new(
            self.parent,
            self.id,
            &self.path,
            &self.message,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

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

pub struct ComboBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    value: String,
    pos: Option<Point>,
    size: Option<Size>,
    choices: Option<ArrayString>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ComboBoxBuilder<'a, P>> for ComboBox {
    fn builder(parent: Option<&'a P>) -> ComboBoxBuilder<'a, P> {
        ComboBoxBuilder {
            parent: parent,
            id: ID_ANY,
            value: "".to_string(),
            pos: None,
            size: None,
            choices: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> ComboBoxBuilder<'a, P> {
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
    pub fn build(&mut self) -> ComboBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let choices = self.choices.take().unwrap_or_else(|| ArrayString::new());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        ComboBox::new(
            self.parent,
            self.id,
            &self.value,
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
        let message = WxString::from(message);
        let message = message.as_ptr();
        let caption = WxString::from(caption);
        let caption = caption.as_ptr();
        let parent = match parent {
            Some(r) => r.as_ptr(),
            None => ptr::null_mut(),
        };
        ffi::wxRustMessageBox(message, caption, style, parent, -1, -1)
    }
}
