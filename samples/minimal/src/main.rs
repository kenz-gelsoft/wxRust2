#![windows_subsystem = "windows"]

use std::os::raw::c_int;
use wx_base::*;
use wx;
use wx::*;

const MINIMAL_QUIT: c_int  = wxID_EXIT;
const MINIMAL_ABOUT: c_int = wxID_ABOUT;

fn main() {
    wx_base::App::run(|| {
        let frame = Frame::new1(Window::none(), wxID_ANY, "Minimal wxWidgets App", 
                &Point::default(), &Size::default(),
                wxDEFAULT_FRAME_STYLE, "");
        let file_menu = Menu::new();
        let help_menu = Menu::new();
        help_menu.append(MINIMAL_ABOUT, "&About\tF1", "Show about dialog", wxITEM_NORMAL);
        file_menu.append(MINIMAL_QUIT, "E&xit\tAlt-X", "Quit this program", wxITEM_NORMAL);

        let menu_bar = MenuBar::new(0);
        menu_bar.append(Some(&file_menu), "&File");
        menu_bar.append(Some(&help_menu), "&Help");

        frame.set_menu_bar(Some(&menu_bar));
        // let button = Button::new1(Some(&frame), wxID_ANY, "Greet",
        //         &Point::default(), &Size::default(), 0,
        //         &Validator::default(), "");
        // let i = 3;
        // println!("i={}", i);
        // let button_copy = button.clone();
        // button.bind(wxRUST_EVT_BUTTON, move || {
        //     println!("i={}", i);
        //     button_copy.set_label("clicked");
        //     println!("s={}", button_copy.get_label())
        // });
        // frame.centre(wxBOTH);
        frame.show(true);
    });
}
