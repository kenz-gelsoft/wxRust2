#![windows_subsystem = "windows"]

use std::os::raw::{c_int, c_long};
use wx_base::*;
use wx;
use wx::*;

const MINIMAL_QUIT: c_int  = wxID_EXIT;
const MINIMAL_ABOUT: c_int = wxID_ABOUT;

fn main() {
    wx_base::App::run(|_| {
        let frame = MyFrame::new("Minimal wxRust App");
        let frame_copy = frame.clone();
        frame.base.bind(wxRUST_EVT_MENU, move |event: &CommandEvent| {
            frame_copy.handle_menu(event)
        });
        frame.base.show(true);
    });
}

#[derive(Clone)]
struct MyFrame {
    base: Frame,
}
impl MyFrame {
    fn new(title: &str) -> Self {
        let frame = Frame::new(Window::none(), wxID_ANY, title, 
                &Point::default(), &Size::default(),
                wxDEFAULT_FRAME_STYLE, "");
        let file_menu = Menu::new();
        let help_menu = Menu::new();
        help_menu.append_int_str(MINIMAL_ABOUT, "&About\tF1", "Show about dialog", wxITEM_NORMAL);
        file_menu.append_int_str(MINIMAL_QUIT, "E&xit\tAlt-X", "Quit this program", wxITEM_NORMAL);

        let menu_bar = MenuBar::new(0);
        menu_bar.append(Some(&file_menu), "&File");
        menu_bar.append(Some(&help_menu), "&Help");

        frame.set_menu_bar(Some(&menu_bar));

        frame.create_status_bar(2, wxSTB_DEFAULT_STYLE, 0, "statusBar");
        frame.set_status_text("Welcome to wxRust!", 0);
        MyFrame {
            base: frame,
        }
    }

    fn handle_menu(&self, event: &CommandEvent) {
        println!("event={}", event.get_id());
        match event.get_id() {
            MINIMAL_QUIT => self.on_quit(),
            MINIMAL_ABOUT => self.on_about(),
            _ => (),
        };
    }

    fn on_quit(&self) {
        self.base.close(true);
    }
    
    fn on_about(&self) {
        message_box(
            "Message",
            "About wxRust minimal sample",
            (wxOK | wxICON_INFORMATION as c_long).try_into().unwrap(),
            Some(&self.base),
        )
    }
}
