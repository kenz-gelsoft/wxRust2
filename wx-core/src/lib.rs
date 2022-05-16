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

    pub trait Builder<T: ?Sized> {
        fn build<W: WindowMethods>(&self, parent: Option<&W>) -> T;
    }
    pub trait Buildable<B: Builder<Self>> {
        fn builder() -> B
        where
            Self: Sized;
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

pub struct FrameBuilder {
    title: String,
}
impl FrameBuilder {
    pub fn title(&mut self, s: &str) -> &mut Self {
        self.title = s.to_string();
        self
    }
}
impl Builder<Frame> for FrameBuilder {
    fn build<W: WindowMethods>(&self, parent: Option<&W>) -> Frame {
        Frame::new(
            parent,
            ID_ANY,
            &self.title,
            &Point::default(),
            &Size::default(),
            DEFAULT_FRAME_STYLE,
            "",
        )
    }
}
impl Buildable<FrameBuilder> for Frame {
    fn builder() -> FrameBuilder {
        FrameBuilder {
            title: "".to_string(),
        }
    }
}

pub struct PanelBuilder;
impl Builder<Panel> for PanelBuilder {
    fn build<W: WindowMethods>(&self, parent: Option<&W>) -> Panel {
        Panel::new(parent, ID_ANY, &Point::default(), &Size::default(), 0, "")
    }
}
impl Buildable<PanelBuilder> for Panel {
    fn builder() -> PanelBuilder {
        PanelBuilder
    }
}

pub struct ButtonBuilder {
    id: c_int,
    title: String,
}
impl ButtonBuilder {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn title(&mut self, s: &str) -> &mut Self {
        self.title = s.to_string();
        self
    }
}
impl Builder<Button> for ButtonBuilder {
    fn build<W: WindowMethods>(&self, parent: Option<&W>) -> Button {
        Button::new(
            parent,
            self.id,
            &self.title,
            &Point::default(),
            &Size::default(),
            0,
            &Validator::default(),
            "",
        )
    }
}
impl Buildable<ButtonBuilder> for Button {
    fn builder() -> ButtonBuilder {
        ButtonBuilder {
            id: ID_ANY,
            title: "".to_string(),
        }
    }
}

pub struct CheckBoxBuilder {
    title: String,
}
impl CheckBoxBuilder {
    pub fn title(&mut self, s: &str) -> &mut Self {
        self.title = s.to_string();
        self
    }
}
impl Builder<CheckBox> for CheckBoxBuilder {
    fn build<W: WindowMethods>(&self, parent: Option<&W>) -> CheckBox {
        CheckBox::new(
            parent,
            ID_ANY,
            &self.title,
            &Point::default(),
            &Size::default(),
            0,
            &Validator::default(),
            "",
        )
    }
}
impl Buildable<CheckBoxBuilder> for CheckBox {
    fn builder() -> CheckBoxBuilder {
        CheckBoxBuilder {
            title: "".to_string(),
        }
    }
}

pub struct ListBoxBuilder;
impl Builder<ListBox> for ListBoxBuilder {
    fn build<W: WindowMethods>(&self, parent: Option<&W>) -> ListBox {
        ListBox::new(
            parent,
            ID_ANY,
            &Point::new_with_int(0, 0),
            &Size::new_with_int(70, 70),
            &ArrayString::new(),
            0,
            &Validator::default(),
            "",
        )
    }
}
impl Buildable<ListBoxBuilder> for ListBox {
    fn builder() -> ListBoxBuilder {
        ListBoxBuilder
    }
}

pub struct ToolBarBuilder {
    style: c_long,
}
impl ToolBarBuilder {
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
}
impl Builder<ToolBar> for ToolBarBuilder {
    fn build<W: WindowMethods>(&self, parent: Option<&W>) -> ToolBar {
        ToolBar::new(
            parent,
            ID_ANY,
            &Point::default(),
            &Size::default(),
            self.style,
            "",
        )
    }
}
impl Buildable<ToolBarBuilder> for ToolBar {
    fn builder() -> ToolBarBuilder {
        ToolBarBuilder {
            style: 0,
        }
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
