// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        timepick.cpp
// Purpose:     Part of the widgets sample showing time picker
// Author:      Vadim Zeitlin
// Created:     2011-12-20
// Copyright:   (c) 2011 wxWindows team
/////////////////////////////////////////////////////////////////////////////

extern crate regex;

use crate::WidgetsPage;
use regex::Regex;
use std::cell::RefCell;
use std::os::raw::c_int;
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum TimePickerPage {
    Reset = wx::ID_HIGHEST as isize,
    Set,
    Picker,
}
impl TimePickerPage {
    fn from(v: c_int) -> Option<Self> {
        use TimePickerPage::*;
        for e in [Reset, Set, Picker] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<TimePickerPage> for c_int {
    fn from(w: TimePickerPage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    text_cur: wx::TextCtrl,

    sizer_time_picker: wx::BoxSizer,
}

#[derive(Clone)]
pub struct TimePickerWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,

    // the time_picker
    time_picker: Rc<RefCell<Option<wx::TimePickerCtrl>>>,
}
impl WidgetsPage for TimePickerWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "TimePicker";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let sizer_left = wx::BoxSizer::new(wx::VERTICAL);
        sizer_left.add_window_sizerflags(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(TimePickerPage::Reset.into())
                    .label("&Reset")
                    .build(),
            ),
            wx::SizerFlags::new(0).centre().border(wx::ALL),
        );

        // middle pane
        let sizer_middle = wx::BoxSizer::new(wx::VERTICAL);
        let (sizer_row, text_cur) = self.create_sizer_with_text_and_button(
            TimePickerPage::Set.into(),
            "&Set time",
            wx::ID_ANY,
        );
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        text_cur.set_min_size(&wx::Size::new_with_int(
            self.base.get_text_extent("  99:99:99  ").get_width(),
            -1,
        ));

        // right pane: control itself
        let sizer_right = wx::BoxSizer::new(wx::HORIZONTAL);

        let time_picker = wx::TimePickerCtrl::builder(Some(&self.base))
            .id(TimePickerPage::Picker.into())
            .build();

        sizer_right.add_int_int(0, 0, 1, wx::CENTRE as c_int, 0, wx::Object::none());
        sizer_right.add_window_int(
            Some(&time_picker),
            1,
            wx::CENTRE as c_int,
            0,
            wx::Object::none(),
        );
        sizer_right.add_int_int(0, 0, 1, wx::CENTRE as c_int, 0, wx::Object::none());
        *self.time_picker.borrow_mut() = Some(time_picker);

        // the 3 panes panes compose the window
        sizer_top.add_sizer_int(
            Some(&sizer_left),
            0,
            wx::GROW | (wx::ALL & !wx::LEFT),
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

        let config_ui = ConfigUI {
            text_cur,

            sizer_time_picker: sizer_right, // save it to modify it later
        };

        // final initializations
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            TimePickerPage::from(event.get_id()),
        ) {
            match m {
                TimePickerPage::Reset => self.on_button_reset(config_ui),
                TimePickerPage::Set => self.on_button_set(config_ui),
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        // Do nothing.
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        // Do nothing.
    }
}
impl TimePickerWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        TimePickerWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            time_picker: Rc::new(RefCell::new(None)),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        if let Some(time_picker) = self.time_picker.borrow().as_ref() {
            // MEMO: original sample sets Today(), but Now() is better
            let now = wx::DateTime::now();

            time_picker.set_value(&now);
            config_ui.text_cur.set_value(&now.format_iso_time());
        }
    }

    fn create_time_picker(&self, config_ui: &ConfigUI) {
        let mut value: Option<wx::DateTime> = None;
        if let Some(time_picker) = self.time_picker.borrow().as_ref() {
            value = Some(time_picker.get_value());

            let count = config_ui.sizer_time_picker.get_children().get_count();
            for _n in 0..count {
                config_ui.sizer_time_picker.remove_int(0);
            }

            time_picker.destroy();
        }

        let style = wx::BORDER_DEFAULT;

        let time_picker = wx::TimePickerCtrl::builder(Some(&self.base))
            .id(TimePickerPage::Picker.into())
            .dt(value)
            .style(style)
            .build();

        let sizer_time_picker = &config_ui.sizer_time_picker;

        sizer_time_picker.add_int_int(0, 0, 1, wx::CENTRE as c_int, 0, wx::Object::none());
        sizer_time_picker.add_window_int(
            Some(&time_picker),
            1,
            wx::CENTRE as c_int,
            0,
            wx::Object::none(),
        );
        sizer_time_picker.add_int_int(0, 0, 1, wx::CENTRE as c_int, 0, wx::Object::none());
        *self.time_picker.borrow_mut() = Some(time_picker);

        config_ui.sizer_time_picker.layout();
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_time_picker(config_ui);
    }

    fn on_button_set(&self, config_ui: &ConfigUI) {
        let text = config_ui.text_cur.get_value();
        let re = Regex::new(r"^(\d+):(\d+):(\d+)$").unwrap();
        if let Some(caps) = re.captures(&text) {
            if let (Ok(h), Ok(m), Ok(s), Some(time_picker)) = (
                caps[1].parse(),
                caps[2].parse(),
                caps[3].parse(),
                self.time_picker.borrow().as_ref(),
            ) {
                time_picker.set_time(h, m, s);
            }
        }
    }
}
