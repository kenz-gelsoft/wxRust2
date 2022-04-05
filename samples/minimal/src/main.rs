#![windows_subsystem = "windows"]

use std::os::raw::{c_int, c_long};
use wx_base::*;
use wx;
use wx::*;

const MINIMAL_QUIT: c_int  = wxID_EXIT;
const MINIMAL_ABOUT: c_int = wxID_ABOUT;

fn main() {
    wx_base::App::run(|_| {
        let frame = my_frame("Minimal wxRust App");
        frame.show(true);
    });
}

fn my_frame_dispatch_event(frame: &Frame, event: &Event) {
    println!("event={}", event.get_id());
    match event.get_id() {
        MINIMAL_QUIT => my_frame_on_quit(frame),
        MINIMAL_ABOUT => my_frame_on_about(frame),
        _ => (),
    };
}

fn my_frame(title: &str) -> Frame {
    let frame = Frame::new1(Window::none(), wxID_ANY, title, 
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
    let frame_copy = frame.clone();
    frame.bind(wxRUST_EVT_MENU, move |event: &Event| {
        my_frame_dispatch_event(&frame_copy, event)
    });

    frame.create_status_bar(2, wxSTB_DEFAULT_STYLE, 0, "statusBar");
    frame.set_status_text("Welcome to wxRust!", 0);
    frame
}

fn my_frame_on_quit(frame: &Frame) {
    frame.close(true);
}

fn my_frame_on_about(frame: &Frame) {
    message_box(
        "Message",
        "About wxRust minmimal sample",
        (wxOK | wxICON_INFORMATION as c_long).try_into().unwrap(),
        Some(frame),
    )
}
