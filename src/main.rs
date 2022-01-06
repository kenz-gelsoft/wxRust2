use wx;
use wx::*;

fn main() {
    wx::App::run(|| {
        let frame = Frame::new("Hello, 世界");
        let button = Button::new(&frame, "Greet");
        let i = 3;
        println!("i={}", i);
        let button_copy = button.clone();
        button.bind(wx::EventType::Button, move || {
            println!("i={}", i);
            button_copy.set_label("clicked");
        });
        frame.centre();
        frame.show();
    });
}
