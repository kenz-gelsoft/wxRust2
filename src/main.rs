use std::os::raw::c_long;

use wx;
use wx::*;

fn main() {
    wx::App::run(|| {
        let frame = Frame::new1(Window::none(), wxID_ANY, "Hello, 世界", 
                &Point::default(), &Size::default(),
                wxDEFAULT_FRAME_STYLE as c_long, "");
        let button = Button::new1(Some(&frame), wxID_ANY, "Greet",
                &Point::default(), &Size::default(), 0,
                &Validator::default(), "");
        let i = 3;
        println!("i={}", i);
        let button_copy = button.clone();
        button.bind(wxRUST_EVT_BUTTON, move || {
            println!("i={}", i);
            wx::ButtonMethods::set_label(&button_copy, "clicked");
            println!("s={}", wx::ButtonMethods::get_label(&button_copy))
        });
        // MEMO: we must choose a non-virtual overriden method
        wx::FrameMethods::centre(&frame, wxBOTH);
        frame.show(true);
    });
}
