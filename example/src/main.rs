#![windows_subsystem = "windows"]

use wx_base::{wxBOTH, wxDEFAULT_FRAME_STYLE, wxID_ANY, wxRUST_EVT_BUTTON};
use wx_base::methods::*;
use wx;
use wx::methods::*;

fn main() {
    wx_base::App::run(|_| {
        let frame = wx::Frame::new(wx::Window::none(), wxID_ANY, "Hello, 世界", 
                &wx::Point::default(), &wx::Size::default(),
                wxDEFAULT_FRAME_STYLE, "");
        let button = wx::Button::new(Some(&frame), wxID_ANY, "Greet",
                &wx::Point::default(), &wx::Size::default(), 0,
                &wx::Validator::default(), "");
        let i = 3;
        println!("i={}", i);
        let button_copy = button.clone();
        button.bind(wxRUST_EVT_BUTTON, move |_: &wx::CommandEvent| {
            println!("i={}", i);
            button_copy.set_label("clicked");
            println!("s={}", button_copy.get_label())
        });
        frame.centre(wxBOTH);
        frame.show(true);
    });
}
