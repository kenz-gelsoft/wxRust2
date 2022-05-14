#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let frame = WrapSizerFrame::new();
        frame.base.show(true);
    });
}

#[derive(Clone)]
struct WrapSizerFrame {
    base: wx::Frame,
    m_panel: wx::Panel,
}
impl WrapSizerFrame {
    fn new() -> Self {
        let frame = wx::Frame::new(
            wx::Window::none(),
            wx::ID_ANY,
            "wxWrapSizer Sample",
            &wx::Point::default(),
            &wx::Size::default(),
            wx::DEFAULT_FRAME_STYLE,
            "",
        );

        let panel = wx::Panel::new(
            Some(&frame),
            wx::ID_ANY,
            &wx::Point::default(),
            &wx::Size::default(),
            0,
            "",
        );

        let sizer_root = wx::BoxSizer::new(wx::VERTICAL);

        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_top.add_window_int(
            Some(&Self::make_tool_bar(&panel)),
            0,
            0,
            0,
            wx::Object::none(),
        );
        sizer_top.add_int_int(20, 1, 0, 0, 0, wx::Object::none());
        sizer_top.add_window_int(
            Some(&Self::make_tool_bar(&panel)),
            0,
            0,
            0,
            wx::Object::none(),
        );
        sizer_top.add_int_int(20, 1, 0, 0, 0, wx::Object::none());
        sizer_top.add_window_int(
            Some(&Self::make_tool_bar(&panel)),
            0,
            0,
            0,
            wx::Object::none(),
        );
        sizer_root.add_sizer_sizerflags(
            Some(&sizer_top),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        panel.set_sizer(Some(&sizer_root), true);

        WrapSizerFrame {
            base: frame,
            m_panel: panel,
        }
    }

    fn add_tool_bar_button(tb: &wx::ToolBar, label: &str, artid: &str) {
        let bm = wx::ArtProvider::get_bitmap(artid, "wxART_OTHER_C", &wx::Size::new_with_int(16, 16));
        tb.add_tool_int_str(wx::ID_ANY, label, &bm, "", wx::ITEM_NORMAL);
    }

    fn make_tool_bar(panel: &wx::Panel) -> wx::ToolBar {
        let tb = wx::ToolBar::new(
            Some(panel),
            wx::ID_ANY,
            &wx::Point::default(),
            &wx::Size::default(),
            wx::TB_NODIVIDER.into(),
            "",
        );
        Self::add_tool_bar_button(&tb, "Help", "wxART_HELP_BOOK");
        tb.add_separator();
        Self::add_tool_bar_button(&tb, "Open", "wxART_FILE_OPEN");
        tb.add_separator();
        Self::add_tool_bar_button(&tb, "Up", "wxART_GO_DIR_UP");
        Self::add_tool_bar_button(&tb, "Execute", "wxART_EXECUTABLE_FILE");
        tb
    }
}
