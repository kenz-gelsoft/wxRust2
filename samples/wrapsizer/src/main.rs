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

        panel.set_sizer(Some(&sizer_root), true);

        WrapSizerFrame {
            base: frame,
            m_panel: panel,
        }
    }
}
