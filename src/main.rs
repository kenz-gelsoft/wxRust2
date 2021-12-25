use wx;

fn main() {
    wx::App::on_init(|| {
        let mut frame = wx::Frame::new("Hello, 世界");
        frame.centre();
        frame.show();
    });

    wx::entry();
}
