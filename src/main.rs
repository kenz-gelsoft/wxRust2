use wx;
use wx::*;

fn main() {
    wx::App::run(|| {
        let window_none: Option<&Window> = None;
        let frame = Frame::new1(window_none, wxID_ANY, "Hello, 世界", 
                &Point::default(), &Size::default(),
                wxDEFAULT_FRAME_STYLE as i32, "");
        let button = Button::new1(Some(&frame), wxID_ANY, "Greet",
                &Point::default(), &Size::default(), 0,
                &Validator::default(), "");
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
