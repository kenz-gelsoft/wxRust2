// SPDX-License-Identifier: MIT
//
// wxRust2 Hello World Sample.
// Created by:  KENZ<KENZ.gelsoft@gmail.com>

#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let first_arg = wx::App::args().nth(1).unwrap_or("世界".to_owned());

        let frame = wx::Frame::builder(wx::Window::none())
            .title(&format!("Hello, {}", first_arg))
            .build();

        let button = wx::Button::builder(Some(&frame)).label("Greet").build();
        let weak_button = button.to_weak_ref();
        button.bind(wx::RustEvent::Button, move |_: &wx::CommandEvent| {
            let Some(button) = weak_button.get() else {return};
            button.set_label("clicked");
            println!("{}", button.get_label())
        });

        frame.centre(wx::BOTH);
        frame.show(true);
    });
}
