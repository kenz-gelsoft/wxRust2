use wx;
use wx::*;

fn main() {
    App::on_init(|| {
        let frame = Frame::new("Hello, 世界");
        let button = Button::new(&frame, "Greet");
        button.set_label("clicked");
        let i = 3;
        println!("i={}", i);
        button.bind(wx::EventType::Button, move || {
            println!("i={}", i);
        });
        frame.centre();
        frame.show();
    });

    wx::entry();
}
