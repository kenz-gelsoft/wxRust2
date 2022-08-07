/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        checkbox.cpp
// Purpose:     Part of the widgets sample showing wxCheckBox
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
enum CheckboxPage {
    Reset = wx::ID_HIGHEST as isize,
    ChangeLabel,
    Check,
    Uncheck,
    PartCheck,
    ChkRight,
    Checkbox,
}
impl CheckboxPage {
    fn from(v: c_int) -> Option<Self> {
        use CheckboxPage::*;
        for e in [
            Reset,
            ChangeLabel,
            Check,
            Uncheck,
            PartCheck,
            ChkRight,
            Checkbox,
        ] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<CheckboxPage> for c_int {
    fn from(w: CheckboxPage) -> Self {
        w as c_int
    }
}

const CHECKBOX_KIND_2STATE: c_int = 0;
const CHECKBOX_KIND_3STATE: c_int = 1;
const CHECKBOX_KIND_3STATE_USER: c_int = 2;

#[derive(Clone)]
pub struct ConfigUI {
    // the controls to choose the checkbox style
    chk_right: wx::CheckBox,
    radio_kind: wx::RadioBox,

    // the text entries for command parameters
    text_label: wx::TextCtrl,

    // sizer
    sizer_checkbox: wx::BoxSizer,
}

#[derive(Clone)]
pub struct CheckBoxWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the checkbox itself
    checkbox: Rc<RefCell<Option<wx::CheckBox>>>,
}
impl WidgetsPage for CheckBoxWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "CheckBox";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let s_box = wx::StaticBox::builder(Some(&self.base))
            .label("&Set style")
            .build();

        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box), wx::VERTICAL);

        let chk_right = self.create_check_box_and_add_to_sizer(
            &sizer_left,
            "&Right aligned",
            CheckboxPage::ChkRight.into(),
        );

        sizer_left.add_spacer(10);

        let kinds = wx::ArrayString::new();
        kinds.add("usual &2-state checkbox");
        kinds.add("&3rd state settable by program");
        kinds.add("&user-settable 3rd state");
        let radio_kind = wx::RadioBox::builder(Some(&self.base))
            .label("&Kind")
            .choices(kinds)
            .major_dimension(1)
            .build();
        sizer_left.add_window_sizerflags(
            Some(&radio_kind),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        sizer_left.add_spacer(5);

        let btn = wx::Button::builder(Some(&self.base))
            .id(CheckboxPage::Reset.into())
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
            CheckboxPage::ChangeLabel.into(),
            "Change label",
            wx::ID_ANY,
        );
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        sizer_middle.add_window_int(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(CheckboxPage::Check.into())
                    .label("&Check it")
                    .build(),
            ),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );
        sizer_middle.add_window_int(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(CheckboxPage::Uncheck.into())
                    .label("&Uncheck it")
                    .build(),
            ),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );
        sizer_middle.add_window_int(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(CheckboxPage::PartCheck.into())
                    .label("Put in &3rd state")
                    .build(),
            ),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        // right pane
        let sizer_right = wx::BoxSizer::new(wx::HORIZONTAL);
        let checkbox = wx::CheckBox::builder(Some(&self.base))
            .id(CheckboxPage::Checkbox.into())
            .label("&Check me!")
            .build();
        sizer_right.add_int_int(0, 0, 1, wx::CENTRE as i32, wx::ALL, wx::Object::none());
        sizer_right.add_window_int(
            Some(&checkbox),
            1,
            wx::CENTRE as i32,
            wx::ALL,
            wx::Object::none(),
        );
        *self.checkbox.borrow_mut() = Some(checkbox);
        sizer_right.add_int_int(0, 0, 1, wx::CENTRE as i32, wx::ALL, wx::Object::none());
        sizer_right.set_min_size_int(150, 0);

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
            Some(&sizer_right),
            wx::SizerFlags::new(1)
                .expand()
                .double_border(wx::ALL & !wx::RIGHT),
        );
        *self.config_ui.borrow_mut() = Some(ConfigUI {
            chk_right,
            radio_kind,
            text_label,
            sizer_checkbox: sizer_right, // save it to modify it later
        });

        // do create the main control
        self.reset();
        self.create_check_box();

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let Some(m) = CheckboxPage::from(event.get_id()) {
            match m {
                CheckboxPage::Reset => self.on_button_reset(),
                CheckboxPage::ChangeLabel => self.on_button_change_label(),
                CheckboxPage::Check => self.on_button_check(),
                CheckboxPage::Uncheck => self.on_button_uncheck(),
                // TODO: Support update ui event to disable this when not 3state
                CheckboxPage::PartCheck => self.on_button_part_check(),
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, event: &wx::CommandEvent) {
        if let Some(m) = CheckboxPage::from(event.get_id()) {
            match m {
                CheckboxPage::Checkbox => self.on_check_box(),
                _ => self.on_style_change(),
            };
        }
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        self.on_style_change();
    }
}
impl CheckBoxWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        CheckBoxWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            checkbox: Rc::new(RefCell::new(None)),
        }
    }

    fn reset(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            config_ui.chk_right.set_value(false);
            config_ui.radio_kind.set_selection(CHECKBOX_KIND_2STATE);
        }
    }

    fn create_check_box(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            let mut label = "".to_string();
            if let Some(checkbox) = self.checkbox.borrow().as_ref() {
                label = checkbox.get_label();

                // TODO: remove (and delete) all checkboxes
                let count = config_ui.sizer_checkbox.get_children().get_count();
                for _ in 0..count {
                    config_ui.sizer_checkbox.remove_int(0);
                }
                checkbox.destroy();
            }

            //     if label.is_empty() {
            //         label = config_ui.text_label.get_value();
            //     }

            let mut flags = wx::BORDER_DEFAULT;

            if config_ui.chk_right.is_checked() {
                flags |= wx::ALIGN_RIGHT as c_long;
            }

            flags |= match config_ui.radio_kind.get_selection() {
                CHECKBOX_KIND_3STATE_USER => wx::CHK_ALLOW_3RD_STATE_FOR_USER | wx::CHK_3STATE,
                CHECKBOX_KIND_3STATE => wx::CHK_3STATE,
                _ => wx::CHK_2STATE,
            } as c_long;

            let new_checkbox = wx::CheckBox::builder(Some(&self.base))
                .id(CheckboxPage::Checkbox.into())
                .label(&label)
                .style(flags)
                .build();

            let sizer_checkbox = &config_ui.sizer_checkbox;
            sizer_checkbox.add_stretch_spacer(1);
            sizer_checkbox.add_window_sizerflags(
                Some(&new_checkbox),
                wx::SizerFlags::new(0).centre().border(wx::ALL),
            );
            sizer_checkbox.add_stretch_spacer(1);

            sizer_checkbox.layout();

            *self.checkbox.borrow_mut() = Some(new_checkbox);
        }
    }

    fn on_button_reset(&self) {
        self.reset();
        self.create_check_box();
    }

    fn on_button_check(&self) {
        self.checkbox.borrow().as_ref().unwrap().set_value(true);
    }
    fn on_button_uncheck(&self) {
        self.checkbox.borrow().as_ref().unwrap().set_value(false);
    }
    fn on_button_part_check(&self) {
        self.checkbox
            .borrow()
            .as_ref()
            .unwrap()
            .set3_state_value(wx::CHK_UNDETERMINED);
    }
    fn on_style_change(&self) {
        self.create_check_box();
    }

    fn on_button_change_label(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.checkbox
                .borrow()
                .as_ref()
                .unwrap()
                .set_label(&config_ui.text_label.get_value());
        }
    }

    fn on_check_box(&self) {}
}
