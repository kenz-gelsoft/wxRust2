// SPDX-License-Identifier: MIT
//
// wxRust2 Hello World Sample.
// Created by:  KENZ<KENZ.gelsoft@gmail.com>

#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let mut first_arg = "世界".to_owned();
        if let Some(arg) = wx::App::args().nth(1) {
            first_arg = arg;
        }
        let frame = wx::Frame::builder(wx::Window::none())
            .title(&format!("Hello, {}", first_arg))
            .build();
        let button = wx::Button::builder(Some(&frame)).label("Greet").build();
        let i = 3;
        println!("i={}", i);
        let weak_button = button.to_weak_ref();
        button.bind(wx::RustEvent::Button, move |_: &wx::CommandEvent| {
            if let Some(button) = weak_button.get() {
                println!("i={}", i);
                button.set_label("clicked");
                println!("s={}", button.get_label())
            }
        });
        frame.centre(wx::BOTH);
        frame.show(true);
    });
}
