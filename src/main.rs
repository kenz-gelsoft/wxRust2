use wx;
use wx::*;

fn main() {
    wx::App::run(|| {
        let default_pos = Point::new1(-1, -1);
        let default_size = Size::new1(-1, -1);
        let window_none: Option<&Window> = None;
        let frame = Frame::new1(window_none, wxID_ANY, "Hello, 世界", 
                &default_pos, &default_size,
                wxDEFAULT_FRAME_STYLE as i32, "");
        let default_validator = Validator::new();
        let button = Button::new1(Some(&frame), wxID_ANY, "Greet",
                &default_pos, &default_size, 0, &default_validator, "");
        // let i = 3;
        // println!("i={}", i);
        // let button_copy = button.clone();
        // button.bind(wx::EventType::Button, move || {
        //     println!("i={}", i);
        //     button_copy.set_label("clicked");
        // });
        // MEMO: we must choose a non-virtual overriden method
        wx::FrameMethods::centre(&frame, wxBOTH);
        frame.show(true);
    });
}
