// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Name:        wrapsizer.cpp
// Purpose:     wxWidgets sample demonstrating wxWrapSizer use
// Author:      Arne Steinarson
// Created:     21.01.2008
// Copyright:   (c) Arne Steinarson
/////////////////////////////////////////////////////////////////////////////

#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let _frame = WrapSizerFrame::new();
    });
}

#[derive(Clone)]
struct WrapSizerFrame {
    base: wx::WeakRef<wx::Frame>,
    m_panel: wx::WeakRef<wx::Panel>,
    m_ok_button: wx::WeakRef<wx::Button>,
}
impl WrapSizerFrame {
    fn new() -> Self {
        let frame = wx::Frame::builder(wx::Window::none())
            .title("wxWrapSizer Sample")
            .build();
        let panel = wx::Panel::builder(Some(&frame)).build();
        let ok_button = wx::Button::builder(Some(&panel))
            .id(wx::ID_OK)
            .label("")
            .build();
        let new_frame = WrapSizerFrame {
            base: frame.to_weak_ref(),
            m_panel: panel.to_weak_ref(),
            m_ok_button: ok_button.to_weak_ref(),
        };
        new_frame.on_create(&frame, &panel);
        new_frame
    }

    fn on_create<F: FrameMethods, P: PanelMethods>(&self, frame: &F, panel: &P) {
        // Root sizer, vertical
        let sizer_root = wx::BoxSizer::new(wx::VERTICAL);

        // Some toolbars in a wrap sizer
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_top.add_window_int(
            Some(&self.make_tool_bar(panel)),
            0,
            0,
            0,
            wx::Object::none(),
        );
        sizer_top.add_int_int(20, 1, 0, 0, 0, wx::Object::none());
        sizer_top.add_window_int(
            Some(&self.make_tool_bar(panel)),
            0,
            0,
            0,
            wx::Object::none(),
        );
        sizer_top.add_int_int(20, 1, 0, 0, 0, wx::Object::none());
        sizer_top.add_window_int(
            Some(&self.make_tool_bar(panel)),
            0,
            0,
            0,
            wx::Object::none(),
        );
        sizer_root.add_sizer_sizerflags(
            Some(&sizer_top),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        // A number of checkboxes inside a wrap sizer
        let sizer_mid =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(panel), "With check-boxes");
        let sizer_mid_wrap = wx::WrapSizer::new(wx::HORIZONTAL, wx::WRAPSIZER_DEFAULT_FLAGS);
        for n_check in 0..6 {
            let chk = wx::CheckBox::builder(Some(panel))
                .label(&format!("Option {}", n_check))
                .build();
            sizer_mid_wrap
                .add_window_sizerflags(Some(&chk), wx::SizerFlags::new(0).centre().border(wx::ALL));
        }

        sizer_mid.add_sizer_sizerflags(Some(&sizer_mid_wrap), wx::SizerFlags::new(100).expand());
        sizer_root.add_sizer_sizerflags(
            Some(&sizer_mid),
            wx::SizerFlags::new(100).expand().border(wx::ALL),
        );

        // A shaped item inside a box sizer
        let sizer_bottom =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(panel), "With wxSHAPED item");
        let sizer_bottom_box = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_bottom
            .add_sizer_sizerflags(Some(&sizer_bottom_box), wx::SizerFlags::new(100).expand());
        sizer_bottom_box.add_window_sizerflags(
            Some(
                &wx::ListBox::builder(Some(panel))
                    .pos(wx::Point::new_with_int(0, 0))
                    .size(wx::Size::new_with_int(70, 70))
                    .build(),
            ),
            wx::SizerFlags::new(0).expand().shaped(),
        );
        sizer_bottom_box.add_spacer(10);
        sizer_bottom_box.add_window_sizerflags(
            Some(
                &wx::CheckBox::builder(Some(panel))
                    .label("A much longer option...")
                    .build(),
            ),
            wx::SizerFlags::new(0).border(wx::ALL),
        );
        sizer_root.add_sizer_sizerflags(
            Some(&sizer_bottom),
            wx::SizerFlags::new(100).expand().border(wx::ALL),
        );

        // OK Button
        if let Some(ok_button) = self.m_ok_button.get() {
            sizer_root.add_window_sizerflags(
                Some(&ok_button),
                wx::SizerFlags::new(0).centre().double_border(wx::ALL),
            );
            let copy_self = self.clone();
            ok_button.bind(wx::RustEvent::Button, move |_: &wx::CommandEvent| {
                if let Some(frame) = copy_self.base.get() {
                    copy_self.on_button(&frame);
                }
            });
        }

        // Set sizer for the panel
        panel.set_sizer(Some(&sizer_root), true);

        frame.set_client_size_size(&panel.get_best_size());

        frame.show(true);
    }

    fn on_button<F: FrameMethods>(&self, frame: &F) {
        frame.close(false);
    }

    fn add_tool_bar_button(&self, tb: &wx::ToolBar, label: &str, artid: &str) {
        let bm =
            wx::ArtProvider::get_bitmap(artid, "wxART_OTHER_C", &wx::Size::new_with_int(16, 16));
        let bm = wx::BitmapBundle::from(bm);
        tb.add_tool_int_str(wx::ID_ANY, label, &bm, "", wx::ITEM_NORMAL);
    }

    fn make_tool_bar<P: wx::PanelMethods>(&self, panel: &P) -> wx::ToolBar {
        let tb = wx::ToolBar::builder(Some(panel))
            .style(wx::TB_NODIVIDER.into())
            .build();
        self.add_tool_bar_button(&tb, "Help", "wxART_HELP_BOOK");
        tb.add_separator();
        self.add_tool_bar_button(&tb, "Open", "wxART_FILE_OPEN");
        tb.add_separator();
        self.add_tool_bar_button(&tb, "Up", "wxART_GO_DIR_UP");
        self.add_tool_bar_button(&tb, "Execute", "wxART_EXECUTABLE_FILE");
        tb
    }
}
