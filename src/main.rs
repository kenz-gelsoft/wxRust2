use wx;
use wx::*;

fn main() {
    App::on_init(|| {
        let frame = Frame::new("Hello, 世界");
        let button = Button::new(&frame, "Greet");
        frame.centre();
        frame.show();
    });

    wx::entry();
}
