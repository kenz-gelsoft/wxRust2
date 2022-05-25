use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum ButtonPage {
    Reset = wx::ID_HIGHEST as isize,
    ChangeLabel,
    ChangeNote,
    ChangeImageMargins,
    Button,
}
impl ButtonPage {
    fn from(v: c_int) -> Option<Self> {
        use ButtonPage::*;
        for e in [Reset, ChangeLabel, ChangeNote, ChangeImageMargins, Button] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<ButtonPage> for c_int {
    fn from(w: ButtonPage) -> Self {
        w as c_int
    }
}

const BUTTON_HALIGN_LEFT: c_int = 0;
const BUTTON_HALIGN_CENTRE: c_int = 1;
const BUTTON_HALIGN_RIGHT: c_int = 2;

const BUTTON_VALIGN_TOP: c_int = 0;
const BUTTON_VALIGN_CENTRE: c_int = 1;
const BUTTON_VALIGN_BOTTOM: c_int = 2;

#[derive(Clone)]
pub struct ButtonWidgetsPage {
    pub base: wx::Panel,
    // the check/radio boxes for styles
    m_chk_fit: RefCell<Option<wx::CheckBox>>,
    m_chk_auth_needed: RefCell<Option<wx::CheckBox>>,
    m_chk_default: RefCell<Option<wx::CheckBox>>,
    m_chk_disable: RefCell<Option<wx::CheckBox>>,
    m_radio_halign: RefCell<Option<wx::RadioBox>>,
    m_radio_valign: RefCell<Option<wx::RadioBox>>,
    // the button itself and the sizer it is in
    m_button: RefCell<Option<wx::Button>>,
    m_sizer_button: RefCell<Option<wx::BoxSizer>>,
    // the text entries for command parameters
    m_text_label: RefCell<Option<wx::TextCtrl>>,
}
impl ButtonWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        ButtonWidgetsPage {
            base: panel,
            m_chk_fit: RefCell::new(None),
            m_chk_auth_needed: RefCell::new(None),
            m_chk_default: RefCell::new(None),
            m_chk_disable: RefCell::new(None),
            m_radio_halign: RefCell::new(None),
            m_radio_valign: RefCell::new(None),
            m_button: RefCell::new(None),
            m_sizer_button: RefCell::new(None),
            m_text_label: RefCell::new(None),
        }
    }

    pub fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let Some(m) = ButtonPage::from(event.get_id()) {
            match m {
                ButtonPage::Reset => self.on_button_reset(),
                ButtonPage::ChangeLabel => self.on_button_change_label(),
                _ => (),
            };
        }
    }

    pub fn create_content(&self) {
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
        *self.m_chk_fit.borrow_mut() =
            Some(self.create_check_box_and_add_to_sizer(&sizer_left, "&Fit exactly", wx::ID_ANY));
        *self.m_chk_auth_needed.borrow_mut() =
            Some(self.create_check_box_and_add_to_sizer(&sizer_left, "Require a&uth", wx::ID_ANY));
        *self.m_chk_default.borrow_mut() =
            Some(self.create_check_box_and_add_to_sizer(&sizer_left, "&Default", wx::ID_ANY));

        let chk_use_bitmap_class =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Use wxBitmapButton", wx::ID_ANY);
        chk_use_bitmap_class.set_value(true);

        *self.m_chk_disable.borrow_mut() =
            Some(self.create_check_box_and_add_to_sizer(&sizer_left, "Disable", wx::ID_ANY));

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
        let radio_halign = Some(
            wx::RadioBox::builder(Some(&self.base))
                .label("&Horz alignment")
                .choices(halign)
                .build(),
        );

        let valign = wx::ArrayString::new();
        valign.add("top");
        valign.add("centre");
        valign.add("bottom");
        let radio_valign = Some(
            wx::RadioBox::builder(Some(&self.base))
                .label("&Vert alignment")
                .choices(valign)
                .build(),
        );

        sizer_left.add_window_sizerflags(
            radio_halign.as_ref(),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        sizer_left.add_window_sizerflags(
            radio_valign.as_ref(),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        *self.m_radio_halign.borrow_mut() = radio_halign;
        *self.m_radio_valign.borrow_mut() = radio_valign;

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
        *self.m_text_label.borrow_mut() = Some(text_label);
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
        *self.m_sizer_button.borrow_mut() = Some(sizer_button);

        // do create the main control
        self.reset();
        self.create_button();

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn recreate_widget(&self) {
        self.create_button();
    }

    fn reset(&self) {
        // TODO reset checkboxes to initial values
    }

    fn create_button(&self) {
        let mut label = "".to_string();
        if let Some(button) = self.m_button.borrow().as_ref() {
            label = button.get_label();

            // TODO: remove (and delete) all buttons
            if let Some(sizer_button) = self.m_sizer_button.borrow().as_ref() {
                let count = sizer_button.get_children().get_count();
                for _ in 0..count {
                    sizer_button.remove_int(0);
                }
                button.destroy();
            }
        }

        if label.is_empty() {
            label = self.m_text_label.borrow().as_ref().unwrap().get_value();
        }

        let mut flags = wx::BORDER_DEFAULT;
        flags |= match self
            .m_radio_halign
            .borrow()
            .as_ref()
            .unwrap()
            .get_selection()
        {
            BUTTON_HALIGN_LEFT => wx::BU_LEFT,
            BUTTON_HALIGN_RIGHT => wx::BU_RIGHT,
            _ => 0,
        } as c_long;

        flags |= match self
            .m_radio_valign
            .borrow()
            .as_ref()
            .unwrap()
            .get_selection()
        {
            BUTTON_VALIGN_TOP => wx::BU_TOP,
            BUTTON_VALIGN_BOTTOM => wx::BU_BOTTOM,
            // centre vertical alignment is the default (no style)
            _ => 0,
        } as c_long;

        if self.m_chk_fit.borrow().as_ref().unwrap().get_value() {
            flags |= wx::BU_EXACTFIT as c_long;
        }

        let new_button = wx::Button::builder(Some(&self.base))
            .id(ButtonPage::Button.into())
            .label(&label)
            .style(flags)
            .build();

        if let Some(sizer_button) = self.m_sizer_button.borrow().as_ref() {
            sizer_button.add_stretch_spacer(1);
            sizer_button.add_window_sizerflags(
                Some(&new_button),
                wx::SizerFlags::new(0).centre().border(wx::ALL),
            );
            sizer_button.add_stretch_spacer(1);

            sizer_button.layout();
        }

        if self
            .m_chk_auth_needed
            .borrow()
            .as_ref()
            .unwrap()
            .get_value()
        {
            new_button.set_auth_needed(true);
        }

        if self.m_chk_default.borrow().as_ref().unwrap().get_value() {
            new_button.set_default();
        }

        new_button.enable(!self.m_chk_disable.borrow().as_ref().unwrap().is_checked());

        *self.m_button.borrow_mut() = Some(new_button);
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

    fn on_button_reset(&self) {
        self.reset();
        // TODO: make mut self callable here, or
        // make create_button() not to require mut self.
        self.create_button();
    }

    fn on_button_change_label(&self) {
        let label_text = self.m_text_label.borrow().as_ref().unwrap().get_value();
        println!("{}", label_text);

        self.m_button
            .borrow()
            .as_ref()
            .unwrap()
            .set_label(&label_text);

        self.m_sizer_button.borrow().as_ref().unwrap().layout();
    }
}
