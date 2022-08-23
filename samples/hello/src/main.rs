// SPDX-License-Identifier: MIT
//
// wxRust2 Hello World Sample.
// Created by:  KENZ<KENZ.gelsoft@gmail.com>

#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let frame = wx::Frame::builder(wx::Window::none())
            .title("Hello, 世界")
            .build();
        let button = wx::Button::builder(Some(&frame)).label("Greet").build();
        let i = 3;
        println!("i={}", i);
        let weak_button = wx::WeakRef::from(&button);
        button.bind(wx::RustEvent::Button, move |_: &wx::CommandEvent| {
            println!("i={}", i);
            if let Some(button) = weak_button.get() {
                button.set_label("clicked");
                println!("s={}", button.get_label())
            }
        });
        frame.centre(wx::BOTH);
        frame.show(true);
    });
}
