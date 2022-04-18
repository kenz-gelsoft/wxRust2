#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx_base::App::run(|_| {
        let frame = wx::Frame::new(wx::Window::none(), wx_base::ID_ANY, "Hello, 世界", 
                &wx::Point::default(), &wx::Size::default(),
                wx_base::DEFAULT_FRAME_STYLE, "");
        let button = wx::Button::new(Some(&frame), wx_base::ID_ANY, "Greet",
                &wx::Point::default(), &wx::Size::default(), 0,
                &wx::Validator::default(), "");
        let i = 3;
        println!("i={}", i);
        let button_copy = button.clone();
        button.bind(wx_base::RUST_EVT_BUTTON, move |_: &wx::CommandEvent| {
            println!("i={}", i);
            button_copy.set_label("clicked");
            println!("s={}", button_copy.get_label())
        });
        frame.centre(wx_base::BOTH);
        frame.show(true);
    });
}
