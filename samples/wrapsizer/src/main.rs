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
        WrapSizerFrame { base: frame }
    }
}
