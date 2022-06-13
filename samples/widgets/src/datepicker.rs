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
    chk_colour_text_ctrl: wx::CheckBox,
    chk_colour_show_label: wx::CheckBox,
    chk_colour_show_alpha: wx::CheckBox,
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
        // left pane
        let boxleft = wx::BoxSizer::new(wx::VERTICAL);

        let clrbox =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&DatePicker style");
        let chk_colour_text_ctrl =
            self.create_check_box_and_add_to_sizer(&clrbox, "With label", wx::ID_ANY);
        let chk_colour_show_label =
            self.create_check_box_and_add_to_sizer(&clrbox, "With textctrl", wx::ID_ANY);
        let chk_colour_show_alpha =
            self.create_check_box_and_add_to_sizer(&clrbox, "With opacity", wx::ID_ANY);
        boxleft.add_sizer_int(Some(&clrbox), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        boxleft.add_window_int(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(DatePickerPage::Reset.into())
                    .label("&Reset")
                    .build(),
            ),
            0,
            wx::ALIGN_CENTRE_HORIZONTAL | wx::ALL,
            15,
            wx::Object::none(),
        );

        self.reset(); // set checkboxes state

        // create pickers
        self.create_picker();

        // right pane
        let sizer = wx::BoxSizer::new(wx::VERTICAL);
        sizer.add_int_int(1, 1, 1, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer
        if let Some(date_picker) = self.date_picker.borrow().as_ref() {
            sizer.add_window_int(
                Some(date_picker),
                0,
                wx::ALIGN_CENTER | wx::ALL,
                5,
                wx::Object::none(),
            );
        }
        sizer.add_int_int(1, 1, 1, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer

        // global pane
        let sz = wx::BoxSizer::new(wx::HORIZONTAL);
        sz.add_sizer_int(Some(&boxleft), 0, wx::GROW | wx::ALL, 5, wx::Object::none());
        sz.add_sizer_int(Some(&sizer), 1, wx::GROW | wx::ALL, 5, wx::Object::none());

        *self.config_ui.borrow_mut() = Some(ConfigUI {
            chk_colour_text_ctrl,
            chk_colour_show_label,
            chk_colour_show_alpha,

            sizer,
        });

        self.base.set_sizer(Some(&sz), true);
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
            config_ui
                .chk_colour_text_ctrl
                .set_value((wx::CLRP_DEFAULT_STYLE & wx::CLRP_USE_TEXTCTRL) != 0);
            config_ui
                .chk_colour_show_label
                .set_value((wx::CLRP_DEFAULT_STYLE & wx::CLRP_SHOW_LABEL) != 0);
            config_ui
                .chk_colour_show_alpha
                .set_value((wx::CLRP_DEFAULT_STYLE & wx::CLRP_SHOW_ALPHA) != 0);
        }
    }

    fn create_picker(&self) {
        if let Some(date_picker) = self.date_picker.borrow().as_ref() {
            date_picker.destroy();
        }

        let mut style = wx::BORDER_DEFAULT;
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            if config_ui.chk_colour_text_ctrl.get_value() {
                style |= wx::CLRP_USE_TEXTCTRL as c_long;
            }
            if config_ui.chk_colour_show_label.get_value() {
                style |= wx::CLRP_SHOW_LABEL as c_long;
            }
            if config_ui.chk_colour_show_alpha.get_value() {
                style |= wx::CLRP_SHOW_ALPHA as c_long;
            }
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
