use std::os::raw::{c_int, c_long};
use std::ptr;

mod generated;
pub use generated::*;

// re-export wx_base
pub use wx_base::*;

#[doc(hidden)]
pub mod methods {
    pub use super::generated::methods::*;

    // re-export wx_base::methods
    pub use wx_base::methods::*;

    pub trait Buildable<'a, P, B> {
        fn builder(parent: Option<&'a P>) -> B;
    }
}
use methods::*;

mod ffi {
    use std::os::raw::{c_int, c_void};
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);
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

pub struct FrameBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    title: String,
}
impl<'a, P: WindowMethods> Buildable<'a, P, FrameBuilder<'a, P>> for Frame {
    fn builder(parent: Option<&'a P>) -> FrameBuilder<'a, P> {
        FrameBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            title: "".to_string(),
        }
    }
}
impl<'a, P: WindowMethods> FrameBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn title(&mut self, s: &str) -> &mut Self {
        self.title = s.to_string();
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
    pub fn build(&mut self) -> Frame {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Frame::new(
            self.parent,
            self.id,
            &self.title,
            &pos,
            &size,
            DEFAULT_FRAME_STYLE,
            "",
        )
    }
}

pub struct PanelBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    pos: Option<Point>,
    size: Option<Size>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, PanelBuilder<'a, P>> for Panel {
    fn builder(parent: Option<&'a P>) -> PanelBuilder<'a, P> {
        PanelBuilder {
            parent: parent,
            pos: None,
            size: None,
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
    pub fn build(&mut self) -> Panel {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Panel::new(self.parent, ID_ANY, &pos, &size, 0, "")
    }
}

pub struct ButtonBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    title: String,
    pos: Option<Point>,
    size: Option<Size>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ButtonBuilder<'a, P>> for Button {
    fn builder(parent: Option<&'a P>) -> ButtonBuilder<'a, P> {
        ButtonBuilder {
            parent: parent,
            id: ID_ANY,
            title: "".to_string(),
            pos: None,
            size: None,
        }
    }
}
impl<'a, P: WindowMethods> ButtonBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn title(&mut self, s: &str) -> &mut Self {
        self.title = s.to_string();
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
    pub fn build(&mut self) -> Button {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Button::new(
            self.parent,
            self.id,
            &self.title,
            &pos,
            &size,
            0,
            &Validator::default(),
            "",
        )
    }
}

pub struct CheckBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    title: String,
    pos: Option<Point>,
    size: Option<Size>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, CheckBoxBuilder<'a, P>> for CheckBox {
    fn builder(parent: Option<&'a P>) -> CheckBoxBuilder<'a, P> {
        CheckBoxBuilder {
            parent: parent,
            id: ID_ANY,
            title: "".to_string(),
            pos: None,
            size: None,
        }
    }
}
impl<'a, P: WindowMethods> CheckBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn title(&mut self, s: &str) -> &mut Self {
        self.title = s.to_string();
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
    pub fn build(&mut self) -> CheckBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        CheckBox::new(
            self.parent,
            self.id,
            &self.title,
            &pos,
            &size,
            0,
            &Validator::default(),
            "",
        )
    }
}

pub struct ListBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ListBoxBuilder<'a, P>> for ListBox {
    fn builder(parent: Option<&'a P>) -> ListBoxBuilder<'a, P> {
        ListBoxBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
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
    pub fn build(&mut self) -> ListBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        ListBox::new(
            self.parent,
            self.id,
            &pos,
            &size,
            &ArrayString::new(),
            0,
            &Validator::default(),
            "",
        )
    }
}

pub struct ToolBarBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    style: c_long,
    pos: Option<Point>,
    size: Option<Size>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ToolBarBuilder<'a, P>> for ToolBar {
    fn builder(parent: Option<&'a P>) -> ToolBarBuilder<'a, P> {
        ToolBarBuilder {
            parent: parent,
            id: ID_ANY,
            style: 0,
            pos: None,
            size: None,
        }
    }
}
impl<'a, P: WindowMethods> ToolBarBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
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
    pub fn build(&mut self) -> ToolBar {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        ToolBar::new(self.parent, self.id, &pos, &size, self.style, "")
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
