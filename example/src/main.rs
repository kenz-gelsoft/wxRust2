#![windows_subsystem = "windows"]

use wx_base::*;
use wx;
use wx::*;

fn main() {
    wx_base::App::run(|_| {
        let frame = Frame::new(Window::none(), wxID_ANY, "Hello, 世界", 
                &Point::default(), &Size::default(),
                wxDEFAULT_FRAME_STYLE, "");
        let button = Button::new(Some(&frame), wxID_ANY, "Greet",
                &Point::default(), &Size::default(), 0,
                &Validator::default(), "");
        let i = 3;
        println!("i={}", i);
        let button_copy = button.clone();
        button.bind(wxRUST_EVT_BUTTON, move |_: &CommandEvent| {
            println!("i={}", i);
            button_copy.set_label("clicked");
            println!("s={}", button_copy.get_label())
        });
        frame.centre(wxBOTH);
        frame.show(true);
    });
}
