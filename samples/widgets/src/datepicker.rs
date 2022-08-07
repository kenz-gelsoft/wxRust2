// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        datepick.cpp
// Purpose:     Part of the widgets sample showing date picker
// Author:      Dimitri Schoolwerth, Vadim Zeitlin
// Created:     27 Sep 2003
// Copyright:   (c) 2003 wxWindows team
// Licence:     wxWindows licence
/////////////////////////////////////////////////////////////////////////////

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
    // SetNullText,
    Picker,
}
impl DatePickerPage {
    fn from(v: c_int) -> Option<Self> {
        use DatePickerPage::*;
        for e in [
            Reset, Set, SetRange, // SetNullText,
            Picker,
        ] {
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
    sizer_date_picker: wx::BoxSizer,

    text_cur: wx::TextCtrl,
    text_min: wx::TextCtrl,
    text_max: wx::TextCtrl,
    // text_null: wx::TextCtrl,
    radio_kind: wx::RadioBox,
    chk_style_century: wx::CheckBox,
    chk_style_allow_none: wx::CheckBox,
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

        // let (sizer_row, text_null) =
        //     self.create_sizer_with_text_and_label("&Null text", wx::ID_ANY);
        // sizer_middle.add_sizer_sizerflags(
        //     Some(&sizer_row),
        //     wx::SizerFlags::new(0).expand().border(wx::ALL),
        // );

        // sizer_middle.add_window_sizerflags(
        //     Some(
        //         &wx::Button::builder(Some(&self.base))
        //             .id(DatePickerPage::SetNullText.into())
        //             .label("Set &null text")
        //             .build(),
        //     ),
        //     wx::SizerFlags::new(0).centre().border(wx::ALL),
        // );

        // right pane: control itself
        let sizer_right = wx::BoxSizer::new(wx::HORIZONTAL);

        let date_picker = wx::DatePickerCtrl::builder(Some(&self.base))
            .id(DatePickerPage::Picker.into())
            .build();

        let centre = wx::CENTRE.try_into().unwrap();
        sizer_right.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sizer_right.add_window_int(Some(&date_picker), 1, centre, 0, wx::Object::none());
        sizer_right.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        *self.date_picker.borrow_mut() = Some(date_picker);

        // the 3 panes panes compose the window
        sizer_top.add_sizer_int(
            Some(&sizer_left),
            0,
            wx::ALL & !wx::LEFT,
            10,
            wx::Object::none(),
        );
        sizer_top.add_sizer_int(
            Some(&sizer_middle),
            0,
            wx::TOP | wx::BOTTOM,
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

        // final initializations
        chk_style_century.set_value(true);

        let config_ui = ConfigUI {
            sizer_date_picker: sizer_right, // save it to modify it later

            text_cur,
            text_min,
            text_max,
            // text_null,
            radio_kind,
            chk_style_century,
            chk_style_allow_none,
        };
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            DatePickerPage::from(event.get_id()),
        ) {
            match m {
                DatePickerPage::Reset => self.on_button_reset(config_ui),
                DatePickerPage::Set => self.on_button_set(config_ui),
                DatePickerPage::SetRange => self.on_button_set_range(config_ui),
                // DatePickerPage::SetNullText => self.on_button_set_null_text(config_ui),
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        // Do nothing
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

    fn reset(&self, config_ui: &ConfigUI) {
        let today = wx::DateTime::today();

        if let Some(date_picker) = self.date_picker.borrow().as_ref() {
            date_picker.set_value(&today);
        }
        config_ui.text_cur.set_value(&today.format_iso_date());
    }

    fn create_date_picker(&self, config_ui: &ConfigUI) {
        let mut value: Option<wx::DateTime> = None;
        if let Some(date_picker) = self.date_picker.borrow().as_ref() {
            value = Some(date_picker.get_value());

            // TODO: remove (and delete) all buttons
            let count = config_ui.sizer_date_picker.get_children().get_count();
            for _ in 0..count {
                config_ui.sizer_date_picker.remove_int(0);
            }
            date_picker.destroy();
        }

        let mut style = wx::BORDER_DEFAULT;
        style |= match config_ui.radio_kind.get_selection() {
            0 => wx::DP_DEFAULT,
            1 => wx::DP_SPIN,
            2 => wx::DP_DROPDOWN,
            _ => 0,
        } as c_long;

        if config_ui.chk_style_century.get_value() {
            style |= wx::DP_SHOWCENTURY as c_long;
        }
        if config_ui.chk_style_allow_none.get_value() {
            style |= wx::DP_ALLOWNONE as c_long;
        }

        let date_picker = wx::DatePickerCtrl::builder(Some(&self.base))
            .id(DatePickerPage::Picker.into())
            .style(style)
            .dt(value)
            .build();

        let centre = wx::CENTRE.try_into().unwrap();
        config_ui
            .sizer_date_picker
            .add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        config_ui.sizer_date_picker.add_window_int(
            Some(&date_picker),
            1,
            centre,
            0,
            wx::Object::none(),
        );
        config_ui
            .sizer_date_picker
            .add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        config_ui.sizer_date_picker.layout();
        *self.date_picker.borrow_mut() = Some(date_picker);
    }

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);
        self.create_date_picker(config_ui);
    }

    fn get_date_from_text_control(&self, text: &wx::TextCtrl) -> Option<wx::DateTime> {
        let value = text.get_value();
        if !value.is_empty() {
            let dt = wx::DateTime::new();
            if let Some(len) = dt.parse_date(&value) {
                if len == value.len() {
                    return Some(dt);
                }
            }
            println!("Invalid date \"{}\"", value);
        }
        return None;
    }

    fn on_button_set(&self, config_ui: &ConfigUI) {
        if let (Some(dt), Some(date_picker)) = (
            self.get_date_from_text_control(&config_ui.text_cur),
            self.date_picker.borrow().as_ref(),
        ) {
            date_picker.set_value(&dt);
        }
    }

    fn on_button_set_range(&self, config_ui: &ConfigUI) {
        if let (Some(dt1), Some(dt2), Some(date_picker)) = (
            self.get_date_from_text_control(&config_ui.text_min),
            self.get_date_from_text_control(&config_ui.text_max),
            self.date_picker.borrow().as_ref(),
        ) {
            date_picker.set_range(&dt1, &dt2);

            if !date_picker.get_range(Some(&dt1), Some(&dt2)) {
                println!("No range set");
            } else {
                let dt1 = if dt1.is_valid() {
                    dt1.format_iso_date()
                } else {
                    String::new()
                };
                config_ui.text_min.set_value(&dt1);
                let dt2 = if dt2.is_valid() {
                    dt2.format_iso_date()
                } else {
                    String::new()
                };
                config_ui.text_max.set_value(&dt2);

                println!("Date picker range updated");
            }
        }
    }

    // fn on_button_set_null_text(&self, config_ui: &ConfigUI) {
    //     if let Some(date_picker) = self.date_picker.borrow().as_ref() {
    //         date_picker.set_null_text(&config_ui.text_null.get_value());
    //     }
    // }
}
