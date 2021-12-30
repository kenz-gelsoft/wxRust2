use wx;
use wx::*;

fn main() {
    App::on_init(|| {
        let frame = Frame::new("Hello, 世界");
        let button = Button::new(&frame, "Greet");
        button.set_label("clicked");
        button.bind(wx::EventType::Button, || {
            println!("hello");
        });
        frame.centre();
        frame.show();
    });

    wx::entry();
}
