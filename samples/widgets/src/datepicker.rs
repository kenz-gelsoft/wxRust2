use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum DatePickerPage {
    Reset = wx::ID_HIGHEST as isize,
    Set,
    SetRange,
    SetNullText,
    Picker,
}
impl DatePickerPage {
    fn from(v: c_int) -> Option<Self> {
        use DatePickerPage::*;
        for e in [Reset, Set, SetRange, SetNullText, Picker] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<DatePickerPage> for c_int {
    fn from(w: DatePickerPage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // other controls
    // --------------
    // chk_colour_text_ctrl: wx::CheckBox,
    // chk_colour_show_label: wx::CheckBox,
    // chk_colour_show_alpha: wx::CheckBox,
    sizer: wx::BoxSizer,
}

#[derive(Clone)]
pub struct DatePickerWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the picker
    date_picker: Rc<RefCell<Option<wx::DatePickerCtrl>>>,
}
impl WidgetsPage for DatePickerWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "DatePicker";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane: style
        let sizer_left = wx::BoxSizer::new(wx::VERTICAL);

        let kinds = wx::ArrayString::new();
        kinds.add("&Default");
        kinds.add("&Spin");
        kinds.add("Drop do&wn");
        let radio_kind = wx::RadioBox::builder(Some(&self.base))
            .choices(kinds)
            .major_dimension(1)
            .style(wx::RA_SPECIFY_COLS.into())
            .build();
        sizer_left.add_window_sizerflags(
            Some(&radio_kind),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let sizer_style =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Style");
        let chk_style_century =
            self.create_check_box_and_add_to_sizer(&sizer_style, "Show &century", wx::ID_ANY);
        let chk_style_allow_none =
            self.create_check_box_and_add_to_sizer(&sizer_style, "Allow &no value", wx::ID_ANY);
        sizer_left.add_sizer_sizerflags(
            Some(&sizer_style),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        sizer_left.add_window_sizerflags(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(DatePickerPage::Reset.into())
                    .label("&Recreate")
                    .build(),
            ),
            wx::SizerFlags::new(0).centre().border(wx::ALL),
        );

        // middle pane: operations
        let sizer_middle = wx::BoxSizer::new(wx::VERTICAL);
        let (sizer_row, text_cur) = self.create_sizer_with_text_and_button(
            DatePickerPage::Set.into(),
            "&Set date",
            wx::ID_ANY,
        );
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        text_cur.set_min_size(&wx::Size::new_with_int(
            self.base.get_text_extent("  9999-99-99  ").get_width(),
            -1,
        ));

        sizer_middle.add_spacer(10);

        let (sizer_row, text_min) = self.create_sizer_with_text_and_label("&Min date", wx::ID_ANY);
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let (sizer_row, text_max) = self.create_sizer_with_text_and_label("Ma&x date", wx::ID_ANY);
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        sizer_middle.add_window_sizerflags(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(DatePickerPage::SetRange.into())
                    .label("Set &range")
                    .build(),
            ),
            wx::SizerFlags::new(0).centre().border(wx::ALL),
        );

        sizer_middle.add_spacer(10);

        let (sizer_row, text_null) =
            self.create_sizer_with_text_and_label("&Null text", wx::ID_ANY);
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        sizer_middle.add_window_sizerflags(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(DatePickerPage::SetNullText.into())
                    .label("Set &null text")
                    .build(),
            ),
            wx::SizerFlags::new(0).centre().border(wx::ALL),
        );

        // right pane: control itself
        let sizer_right = wx::BoxSizer::new(wx::HORIZONTAL);

        let date_picker = wx::DatePickerCtrl::builder(Some(&self.base))
            .id(DatePickerPage::Picker.into())
            .build();

        let centre = wx::CENTRE.try_into().unwrap();
        sizer_right.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sizer_right.add_window_int(Some(&date_picker), 1, centre, 0, wx::Object::none());
        sizer_right.add_int_int(0, 0, 1, centre, 0, wx::Object::none());

        // the 3 panes panes compose the window
        sizer_top.add_sizer_int(
            Some(&sizer_left),
            0,
            (wx::ALL & !wx::LEFT),
            10,
            wx::Object::none(),
        );
        sizer_top.add_sizer_int(
            Some(&sizer_middle),
            0,
            (wx::TOP | wx::BOTTOM),
            10,
            wx::Object::none(),
        );
        sizer_top.add_sizer_int(
            Some(&sizer_right),
            1,
            wx::GROW | (wx::ALL & !wx::RIGHT),
            10,
            wx::Object::none(),
        );

        *self.config_ui.borrow_mut() = Some(ConfigUI {
            // chk_colour_text_ctrl,
            // chk_colour_show_label,
            // chk_colour_show_alpha,
            sizer: sizer_right,
        });

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, _: &wx::CommandEvent) {
        self.on_button_reset();
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        self.on_check_box();
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        // Do nothing
    }
}
impl DatePickerWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        DatePickerWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            date_picker: Rc::new(RefCell::new(None)),
        }
    }

    fn recreate_widget(&self) {
        self.create_picker();

        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            // MEMO: Destroy()ing in create_picker() removes from its sizer.
            // config_ui.sizer.remove_int(1);
            self.create_picker();
            if let Some(date_picker) = self.date_picker.borrow().as_ref() {
                config_ui.sizer.insert_window_int(
                    1,
                    Some(date_picker),
                    0,
                    wx::ALIGN_CENTER | wx::ALL,
                    5,
                    wx::Object::none(),
                );
            }
            config_ui.sizer.layout();
        }
    }

    fn reset(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            // config_ui
            //     .chk_colour_text_ctrl
            //     .set_value((wx::CLRP_DEFAULT_STYLE & wx::CLRP_USE_TEXTCTRL) != 0);
            // config_ui
            //     .chk_colour_show_label
            //     .set_value((wx::CLRP_DEFAULT_STYLE & wx::CLRP_SHOW_LABEL) != 0);
            // config_ui
            //     .chk_colour_show_alpha
            //     .set_value((wx::CLRP_DEFAULT_STYLE & wx::CLRP_SHOW_ALPHA) != 0);
        }
    }

    fn create_picker(&self) {
        if let Some(date_picker) = self.date_picker.borrow().as_ref() {
            date_picker.destroy();
        }

        let mut style = wx::BORDER_DEFAULT;
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            // if config_ui.chk_colour_text_ctrl.get_value() {
            //     style |= wx::CLRP_USE_TEXTCTRL as c_long;
            // }
            // if config_ui.chk_colour_show_label.get_value() {
            //     style |= wx::CLRP_SHOW_LABEL as c_long;
            // }
            // if config_ui.chk_colour_show_alpha.get_value() {
            //     style |= wx::CLRP_SHOW_ALPHA as c_long;
            // }
        }

        let date_picker = wx::DatePickerCtrl::builder(Some(&self.base))
            // .id(PickerPage::Colour.into())
            // .colour(wx::Colour::new_with_str("RED"))
            .style(style)
            .build();
        *self.date_picker.borrow_mut() = Some(date_picker);
    }

    fn on_button_reset(&self) {
        self.reset();
        self.recreate_widget();
    }

    fn on_check_box(&self) {
        self.recreate_widget();
    }
}
