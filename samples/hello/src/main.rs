#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let frame = wx::Frame::builder(wx::Window::none())
            .title("Hello, 世界")
            .build();
        let button = wx::Button::builder(Some(&frame)).label("Greet").build();
        let i = 3;
        println!("i={}", i);
        let button_copy = button.clone();
        button.bind(wx::RUST_EVT_BUTTON, move |_: &wx::CommandEvent| {
            println!("i={}", i);
            button_copy.set_label("clicked");
            println!("s={}", button_copy.get_label())
        });
        // frame.centre(wx::BOTH);
        // frame.show(true);
        unsafe {
            let weak_frame = wx::WeakRef::<wx::Frame>::from(frame.as_ptr());
            if let Some(f) = weak_frame.get() {
                f.centre(wx::BOTH);
                f.show(true);
            }
        }
    });
}
