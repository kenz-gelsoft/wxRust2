use std::os::raw::c_int;
use wx::methods::*;

// control ids
enum ButtonPage {
    Reset = wx::ID_HIGHEST as isize,
    ChangeLabel,
    ChangeNote,
    ChangeImageMargins,
    Button,
}
impl From<ButtonPage> for c_int {
    fn from(w: ButtonPage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ButtonWidgetsPage {
    pub base: wx::Panel,
}
impl ButtonWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        ButtonWidgetsPage { base: panel }
    }
    pub fn create_content(&mut self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let s_box = wx::StaticBox::builder(Some(&self.base))
            .label("&Set style")
            .build();

        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box), wx::VERTICAL);

        let chk_bitmap_only =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Bitmap only", wx::ID_ANY);
        let chk_text_and_bitmap =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Text &and bitmap", wx::ID_ANY);
        let chk_fit =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Fit exactly", wx::ID_ANY);
        let chk_auth_needed =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Require a&uth", wx::ID_ANY);
        let chk_default =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Default", wx::ID_ANY);

        let chk_use_bitmap_class =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Use wxBitmapButton", wx::ID_ANY);
        chk_use_bitmap_class.set_value(true);

        let chk_disable =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Disable", wx::ID_ANY);

        sizer_left.add_spacer(5);

        let sizer_use_labels = wx::StaticBoxSizer::new_with_int(
            wx::VERTICAL,
            Some(&self.base),
            "&Use the following bitmaps in addition to the normal one?",
        );
        let chk_use_pressed = self.create_check_box_and_add_to_sizer(
            &sizer_use_labels,
            "&Pressed (small help icon)",
            wx::ID_ANY,
        );
        let chk_use_focused = self.create_check_box_and_add_to_sizer(
            &sizer_use_labels,
            "&Focused (small error icon)",
            wx::ID_ANY,
        );
        let chk_use_current = self.create_check_box_and_add_to_sizer(
            &sizer_use_labels,
            "&Current (small warning icon)",
            wx::ID_ANY,
        );
        let chk_use_disabled = self.create_check_box_and_add_to_sizer(
            &sizer_use_labels,
            "&Disabled (broken image icon)",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_sizerflags(
            Some(&sizer_use_labels),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        sizer_left.add_spacer(10);

        let dirs = wx::ArrayString::new();
        dirs.add("left");
        dirs.add("right");
        dirs.add("top");
        dirs.add("bottom");
        let radio_image_pos = wx::RadioBox::builder(Some(&self.base))
            .label("Image &position")
            .choices(dirs)
            .build();
        sizer_left.add_window_sizerflags(
            Some(&radio_image_pos),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        // TODO Integer Validator
        let (sizer_image_margins_row, text_image_margin_h) = self
            .create_sizer_with_text_and_button(
                ButtonPage::ChangeImageMargins.into(),
                "Horizontal and vertical",
                wx::ID_ANY,
            );

        // let start_btn = wx::Button::builder(Some(&self.base))
        //     .id(Button::Start.into())
        //     .label("&Start")
        //     .build();
        // sizer_oper.add_window_sizerflags(
        //     Some(&start_btn),
        //     wx::SizerFlags::new(0).expand().border(wx::ALL),
        // );
        // let stop_btn = wx::Button::builder(Some(&self.base))
        //     .id(Button::Stop.into())
        //     .label("&Stop")
        //     .build();
        // sizer_oper.add_window_sizerflags(
        //     Some(&stop_btn),
        //     wx::SizerFlags::new(0).expand().border(wx::ALL),
        // );

        // sizer_oper.add_window_sizerflags(
        //     Some(
        //         &wx::StaticText::builder(Some(&self.base))
        //             .id(Button::IsRunning.into())
        //             .label("Indicator is initializing...")
        //             .build(),
        //     ),
        //     wx::SizerFlags::new(0).expand().border(wx::ALL),
        // );

        // self.recreate_widget();

        // let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);
        // sizer_top.add_sizer_sizerflags(
        //     Some(&sizer_oper),
        //     wx::SizerFlags::new(0).expand().double_border(wx::ALL),
        // );
        // sizer_top.add_sizer_sizerflags(
        //     Some(&self.m_sizer_indicator),
        //     wx::SizerFlags::new(1).expand().double_border(wx::ALL),
        // );

        // self.base.set_sizer(Some(&sizer_top), true);
    }

    fn create_sizer_with_text<C: ControlMethods>(
        &self,
        control: &C,
        id: c_int,
    ) -> (wx::BoxSizer, wx::TextCtrl) {
        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let text = wx::TextCtrl::builder(Some(&self.base))
            .id(id)
            .style(wx::TE_PROCESS_ENTER.into())
            .build();

        sizer_row.add_window_int(
            Some(control),
            0,
            wx::RIGHT | wx::ALIGN_CENTRE_VERTICAL,
            5,
            wx::Object::none(),
        );
        sizer_row.add_window_int(
            Some(&text),
            1,
            wx::LEFT | wx::ALIGN_CENTRE_VERTICAL,
            5,
            wx::Object::none(),
        );

        (sizer_row, text)
    }

    fn create_sizer_with_text_and_button(
        &self,
        id_btn: c_int,
        label: &str,
        id: c_int,
    ) -> (wx::BoxSizer, wx::TextCtrl) {
        let btn = wx::Button::builder(Some(&self.base))
            .id(id_btn)
            .label(label)
            .build();
        self.create_sizer_with_text(&btn, id)
    }

    fn create_check_box_and_add_to_sizer<S: SizerMethods>(
        &self,
        sizer: &S,
        label: &str,
        id: c_int,
    ) -> wx::CheckBox {
        let checkbox = wx::CheckBox::builder(Some(&self.base))
            .id(id)
            .label(label)
            .build();
        sizer.add_window_sizerflags(Some(&checkbox), wx::SizerFlags::new(0).double_horz_border());
        sizer.add_spacer(2);

        return checkbox;
    }

    fn recreate_widget(&mut self) {
        // self.m_sizer_indicator.clear(true /* delete windows */);

        // self.m_indicator = Some(
        //     wx::Button::builder(Some(&self.base))
        //         .id(wx::ID_ANY)
        //         .style(wx::BORDER_DEFAULT)
        //         .build(),
        // );

        // self.m_sizer_indicator.add_stretch_spacer(1);
        // self.m_sizer_indicator
        //     .add_window_sizerflags(self.m_indicator.as_ref(), wx::SizerFlags::new(0).centre());
        // self.m_sizer_indicator.add_stretch_spacer(1);
        // self.m_sizer_indicator.layout();
    }
}
