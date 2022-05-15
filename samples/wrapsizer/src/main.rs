#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let frame = WrapSizerFrame::new();
        let frame_copy = frame.clone();
        frame
            .m_ok_button
            .bind(wx::RUST_EVT_BUTTON, move |_: &wx::CommandEvent| {
                frame_copy.on_button()
            });
        frame.base.show(true);
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
        let frame = wx::Frame::new(
            wx::Window::none(),
            wx::ID_ANY,
            "wxWrapSizer Sample",
            &wx::Point::default(),
            &wx::Size::default(),
            wx::DEFAULT_FRAME_STYLE,
            "",
        );

        let panel = wx::Panel::new(
            Some(&frame),
            wx::ID_ANY,
            &wx::Point::default(),
            &wx::Size::default(),
            0,
            "",
        );

        // Root sizer, vertical
        let sizer_root = wx::BoxSizer::new(wx::VERTICAL);

        // Some toolbars in a wrap sizer
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_top.add_window_int(
            Some(&Self::make_tool_bar(&panel)),
            0,
            0,
            0,
            wx::Object::none(),
        );
        sizer_top.add_int_int(20, 1, 0, 0, 0, wx::Object::none());
        sizer_top.add_window_int(
            Some(&Self::make_tool_bar(&panel)),
            0,
            0,
            0,
            wx::Object::none(),
        );
        sizer_top.add_int_int(20, 1, 0, 0, 0, wx::Object::none());
        sizer_top.add_window_int(
            Some(&Self::make_tool_bar(&panel)),
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
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&panel), "With check-boxes");
        let sizer_mid_wrap = wx::WrapSizer::new(wx::HORIZONTAL, wx::WRAPSIZER_DEFAULT_FLAGS);
        for n_check in 0..6 {
            let chk = wx::CheckBox::new(
                Some(&panel),
                wx::ID_ANY,
                &format!("Option {}", n_check),
                &wx::Point::default(),
                &wx::Size::default(),
                0,
                &wx::Validator::default(),
                "",
            );
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
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&panel), "With wxSHAPED item");
        let sizer_bottom_box = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_bottom
            .add_sizer_sizerflags(Some(&sizer_bottom_box), wx::SizerFlags::new(100).expand());
        sizer_bottom_box.add_window_sizerflags(
            Some(&wx::ListBox::new(
                Some(&panel),
                wx::ID_ANY,
                &wx::Point::new_with_int(0, 0),
                &wx::Size::new_with_int(70, 70),
                &wx::ArrayString::new(),
                0,
                &wx::Validator::default(),
                "",
            )),
            wx::SizerFlags::new(0).expand().shaped(),
        );
        sizer_bottom_box.add_spacer(10);
        sizer_bottom_box.add_window_sizerflags(
            Some(&wx::CheckBox::new(
                Some(&panel),
                wx::ID_ANY,
                "A much longer option...",
                &wx::Point::default(),
                &wx::Size::default(),
                0,
                &wx::Validator::default(),
                "",
            )),
            wx::SizerFlags::new(0).border(wx::ALL),
        );
        sizer_root.add_sizer_sizerflags(
            Some(&sizer_bottom),
            wx::SizerFlags::new(100).expand().border(wx::ALL),
        );

        // OK Button
        let ok_button = wx::Button::new(
            Some(&panel),
            wx::ID_OK,
            "",
            &wx::Point::default(),
            &wx::Size::default(),
            0,
            &wx::Validator::default(),
            "",
        );
        sizer_root.add_window_sizerflags(
            Some(&ok_button),
            wx::SizerFlags::new(0).centre().double_border(wx::ALL),
        );

        // Set sizer for the panel
        panel.set_sizer(Some(&sizer_root), true);

        WrapSizerFrame {
            base: frame,
            m_panel: panel,
            m_ok_button: ok_button,
        }
    }

    fn on_button(&self) {
        self.base.close(false);
    }

    fn add_tool_bar_button(tb: &wx::ToolBar, label: &str, artid: &str) {
        let bm =
            wx::ArtProvider::get_bitmap(artid, "wxART_OTHER_C", &wx::Size::new_with_int(16, 16));
        tb.add_tool_int_str(wx::ID_ANY, label, &bm, "", wx::ITEM_NORMAL);
    }

    fn make_tool_bar(panel: &wx::Panel) -> wx::ToolBar {
        let tb = wx::ToolBar::new(
            Some(panel),
            wx::ID_ANY,
            &wx::Point::default(),
            &wx::Size::default(),
            wx::TB_NODIVIDER.into(),
            "",
        );
        Self::add_tool_bar_button(&tb, "Help", "wxART_HELP_BOOK");
        tb.add_separator();
        Self::add_tool_bar_button(&tb, "Open", "wxART_FILE_OPEN");
        tb.add_separator();
        Self::add_tool_bar_button(&tb, "Up", "wxART_GO_DIR_UP");
        Self::add_tool_bar_button(&tb, "Execute", "wxART_EXECUTABLE_FILE");
        tb
    }
}
