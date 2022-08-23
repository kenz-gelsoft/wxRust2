// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Name:        minimal.cpp
// Purpose:     Minimal wxWidgets sample
// Author:      Julian Smart
// Modified by:
// Created:     04/01/98
// Copyright:   (c) Julian Smart
/////////////////////////////////////////////////////////////////////////////

#![windows_subsystem = "windows"]

use std::os::raw::{c_int, c_long};
use wx;
use wx::methods::*;

const MINIMAL_QUIT: c_int = wx::ID_EXIT;
const MINIMAL_ABOUT: c_int = wx::ID_ABOUT;

fn main() {
    wx::App::run(|_| {
        let frame = MyFrame::new("Minimal wxRust App");
        let weak_frame = frame.clone();
        if let Some(frame) = frame.base.get() {
            frame.bind(wx::RustEvent::Menu, move |event: &wx::CommandEvent| {
                weak_frame.handle_menu(event)
            });
            frame.show(true);
        }
    });
}

#[derive(Clone)]
struct MyFrame {
    base: wx::WeakRef<wx::FrameIsOwned<false>>,
}
impl MyFrame {
    fn new(title: &str) -> Self {
        let frame = wx::Frame::builder(wx::Window::none()).title(title).build();
        let file_menu = wx::Menu::new().item_h(MINIMAL_QUIT, "E&xit\tAlt-X", "Quit this program");
        let help_menu = wx::Menu::new().item_h(MINIMAL_ABOUT, "&About\tF1", "Show about dialog");

        let menu_bar = wx::MenuBar::new(0);
        menu_bar.append(Some(&file_menu), "&File");
        menu_bar.append(Some(&help_menu), "&Help");

        frame.set_menu_bar(Some(&menu_bar));

        frame.create_status_bar(2, wx::STB_DEFAULT_STYLE, 0, "statusBar");
        frame.set_status_text("Welcome to wxRust!", 0);
        MyFrame {
            base: frame.to_weak_ref(),
        }
    }

    fn handle_menu(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let Some(frame) = self.base.get() {
            match event.get_id() {
                MINIMAL_QUIT => self.on_quit(&frame),
                MINIMAL_ABOUT => self.on_about(&frame),
                _ => (),
            };
        }
    }

    fn on_quit<F: FrameMethods>(&self, frame: &F) {
        frame.close(true);
    }

    fn on_about<F: FrameMethods>(&self, frame: &F) {
        wx::message_box(
            "Message",
            "About wxRust minimal sample",
            (wx::OK | wx::ICON_INFORMATION as c_long)
                .try_into()
                .unwrap(),
            Some(frame),
        )
    }
}
