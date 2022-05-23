#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let frame = WrapSizerFrame::new();
    });
}

#[derive(Clone)]
struct WrapSizerFrame {
    base: wx::Frame,
    m_panel: wx::Panel,
    m_ok_button: wx::Button,
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
            base: frame,
            m_panel: panel,
            m_ok_button: ok_button,
        };
        new_frame.on_create();
        new_frame
    }

    fn on_create(&self) {
        // Root sizer, vertical
        let sizer_root = wx::BoxSizer::new(wx::VERTICAL);

        // Some toolbars in a wrap sizer
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_top.add_window_int(Some(&self.make_tool_bar()), 0, 0, 0, wx::Object::none());
        sizer_top.add_int_int(20, 1, 0, 0, 0, wx::Object::none());
        sizer_top.add_window_int(Some(&self.make_tool_bar()), 0, 0, 0, wx::Object::none());
        sizer_top.add_int_int(20, 1, 0, 0, 0, wx::Object::none());
        sizer_top.add_window_int(Some(&self.make_tool_bar()), 0, 0, 0, wx::Object::none());
        sizer_root.add_sizer_sizerflags(
            Some(&sizer_top),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        // A number of checkboxes inside a wrap sizer
        let sizer_mid =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.m_panel), "With check-boxes");
        let sizer_mid_wrap = wx::WrapSizer::new(wx::HORIZONTAL, wx::WRAPSIZER_DEFAULT_FLAGS);
        for n_check in 0..6 {
            let chk = wx::CheckBox::builder(Some(&self.m_panel))
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
        let sizer_bottom = wx::StaticBoxSizer::new_with_int(
            wx::VERTICAL,
            Some(&self.m_panel),
            "With wxSHAPED item",
        );
        let sizer_bottom_box = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_bottom
            .add_sizer_sizerflags(Some(&sizer_bottom_box), wx::SizerFlags::new(100).expand());
        sizer_bottom_box.add_window_sizerflags(
            Some(
                &wx::ListBox::builder(Some(&self.m_panel))
                    .pos(wx::Point::new_with_int(0, 0))
                    .size(wx::Size::new_with_int(70, 70))
                    .build(),
            ),
            wx::SizerFlags::new(0).expand().shaped(),
        );
        sizer_bottom_box.add_spacer(10);
        sizer_bottom_box.add_window_sizerflags(
            Some(
                &wx::CheckBox::builder(Some(&self.m_panel))
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
        sizer_root.add_window_sizerflags(
            Some(&self.m_ok_button),
            wx::SizerFlags::new(0).centre().double_border(wx::ALL),
        );
        let copy_self = self.clone();
        self.m_ok_button
            .bind(wx::RustEvent::Button, move |_: &wx::CommandEvent| {
                copy_self.on_button()
            });

        // Set sizer for the panel
        self.m_panel.set_sizer(Some(&sizer_root), true);

        self.base
            .set_client_size_size(&self.m_panel.get_best_size());

        self.base.show(true);
    }

    fn on_button(&self) {
        self.base.close(false);
    }

    fn add_tool_bar_button(&self, tb: &wx::ToolBar, label: &str, artid: &str) {
        let bm =
            wx::ArtProvider::get_bitmap(artid, "wxART_OTHER_C", &wx::Size::new_with_int(16, 16));
        tb.add_tool_int_str(wx::ID_ANY, label, &bm, "", wx::ITEM_NORMAL);
    }

    fn make_tool_bar(&self) -> wx::ToolBar {
        let tb = wx::ToolBar::builder(Some(&self.m_panel))
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
