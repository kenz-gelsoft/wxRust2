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
    m_button: Option<wx::Button>,
    m_text_label: Option<wx::TextCtrl>,
    m_sizer_button: Option<wx::BoxSizer>,
}
impl ButtonWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        ButtonWidgetsPage {
            base: panel,
            m_button: None,
            m_text_label: None,
            m_sizer_button: None,
        }
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
        sizer_left.add_spacer(15);

        let halign = wx::ArrayString::new();
        halign.add("left");
        halign.add("centre");
        halign.add("right");
        let radio_halign = wx::RadioBox::builder(Some(&self.base))
            .label("&Horz alignment")
            .choices(halign)
            .build();

        let valign = wx::ArrayString::new();
        valign.add("left");
        valign.add("centre");
        valign.add("right");
        let radio_valign = wx::RadioBox::builder(Some(&self.base))
            .label("&Vert alignment")
            .choices(valign)
            .build();

        sizer_left.add_window_sizerflags(
            Some(&radio_halign),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        sizer_left.add_window_sizerflags(
            Some(&radio_valign),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        sizer_left.add_spacer(5);

        let btn = wx::Button::builder(Some(&self.base))
            .id(ButtonPage::Reset.into())
            .label("&Reset")
            .build();
        sizer_left.add_window_int(
            Some(&btn),
            0,
            wx::ALIGN_CENTRE_HORIZONTAL | wx::ALL,
            15,
            wx::Object::none(),
        );

        // middle pane
        let s_box2 = wx::StaticBox::builder(Some(&self.base))
            .label("&Operations")
            .build();
        let sizer_middle = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box2), wx::VERTICAL);

        let (sizer_row, text_label) = self.create_sizer_with_text_and_button(
            ButtonPage::ChangeLabel.into(),
            "Change label",
            wx::ID_ANY,
        );
        text_label.set_value("&Press me!");
        self.m_text_label = Some(text_label);
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        // right pane
        let sizer_button = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_button.set_min_size_int(150, 0);

        // the 3 panes panes compose the window
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_left),
            wx::SizerFlags::new(0)
                .expand()
                .double_border(wx::ALL & !wx::LEFT),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_middle),
            wx::SizerFlags::new(1).expand().double_border(wx::ALL),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_button),
            wx::SizerFlags::new(1)
                .expand()
                .double_border(wx::ALL & !wx::RIGHT),
        );
        self.m_sizer_button = Some(sizer_button);

        // do create the main control
        self.reset();
        // self.create_button();

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn recreate_widget(&mut self) {
        self.create_button();
    }

    fn reset(&self) {
        // TODO reset checkboxes to initial values
    }

    fn create_button(&mut self) {
        let mut label = "".to_string();
        if let Some(button) = &self.m_button {
            label = button.get_label();

            // TODO: remove (and delete) all buttons
            // let count = self.m_sizer_button.get_children().get_count();
        }

        if label.is_empty() {
            label = self.m_text_label.as_ref().unwrap().get_value();
        }

        self.m_button = Some(
            wx::Button::builder(Some(&self.base))
                .id(ButtonPage::Button.into())
                .label(&label)
                .build(),
        );

        if let Some(sizer_button) = &self.m_sizer_button {
            sizer_button.add_stretch_spacer(1);
            sizer_button.add_window_sizerflags(
                self.m_button.as_ref(),
                wx::SizerFlags::new(0).centre().border(wx::ALL),
            );
            sizer_button.add_stretch_spacer(1);

            sizer_button.layout();
        }
    }

    // Utility methods from (and to be placed to) the base WidgetPage class

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
}
