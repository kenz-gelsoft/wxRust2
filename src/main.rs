use std::ptr;
use wx;
use wx::*;

fn main() {
    wx::App::run(|| {
        let s = ffi_manual::NewString("Hello, 世界");
        let s2 = ffi_manual::NewString("");
        let default_pos = Point::new1(-1, -1);
        let default_size = Size::new1(-1, -1);
        let frame = Frame::new1(ptr::null_mut(), wxID_ANY, &s, 
                &default_pos, &default_size,
                wxDEFAULT_FRAME_STYLE as i32, &s2);
        frame.show(true);
        // let frame = Frame::new("Hello, 世界");
        // let button = Button::new(&frame, "Greet");
        // let i = 3;
        // println!("i={}", i);
        // let button_copy = button.clone();
        // button.bind(wx::EventType::Button, move || {
        //     println!("i={}", i);
        //     button_copy.set_label("clicked");
        // });
        // frame.centre();
        // frame.show();
    });
}
