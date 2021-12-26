use wx;
use wx::{App, Button, Frame};

fn main() {
    App::on_init(|| {
        let mut frame = Frame::new("Hello, 世界");
        let button = Button::new(&mut frame, "Greet");
        frame.centre();
        frame.show();
    });

    wx::entry();
}
