// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        fontpicker.cpp
// Purpose:     Shows wxFontPickerCtrl
// Author:      Francesco Montorsi
// Created:     20/6/2006
// Copyright:   (c) 2006 Francesco Montorsi
// Licence:     wxWindows licence
/////////////////////////////////////////////////////////////////////////////

use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum PickerPage {
    Reset = wx::ID_HIGHEST as isize,
    Font,
}
impl From<PickerPage> for c_int {
    fn from(w: PickerPage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // other controls
    // --------------
    chk_font_text_ctrl: wx::CheckBox,
    chk_font_desc_as_label: wx::CheckBox,
    chk_font_use_font_for_label: wx::CheckBox,
    sizer: wx::BoxSizer,
}

#[derive(Clone)]
pub struct FontPickerWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the picker
    font_picker: Rc<RefCell<Option<wx::FontPickerCtrl>>>,
}
impl WidgetsPage for FontPickerWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "FontPicker";
    }
    fn create_content(&self) {
        // left pane
        let boxleft = wx::BoxSizer::new(wx::VERTICAL);

        let fontbox =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&FontPicker style");
        let chk_font_text_ctrl =
            self.create_check_box_and_add_to_sizer(&fontbox, "With textctrl", wx::ID_ANY);
        let chk_font_desc_as_label =
            self.create_check_box_and_add_to_sizer(&fontbox, "Font desc as btn label", wx::ID_ANY);
        let chk_font_use_font_for_label =
            self.create_check_box_and_add_to_sizer(&fontbox, "Use font for label", wx::ID_ANY);
        boxleft.add_sizer_int(Some(&fontbox), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        boxleft.add_window_int(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(PickerPage::Reset.into())
                    .label("&Reset")
                    .build(),
            ),
            0,
            wx::ALIGN_CENTRE_HORIZONTAL | wx::ALL,
            15,
            wx::Object::none(),
        );

        // create pickers
        self.create_picker();

        // right pane
        let sizer = wx::BoxSizer::new(wx::VERTICAL);
        sizer.add_int_int(1, 1, 1, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer
        if let Some(font_picker) = self.font_picker.borrow().as_ref() {
            sizer.add_window_int(
                Some(font_picker),
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

        let config_ui = ConfigUI {
            chk_font_text_ctrl,
            chk_font_desc_as_label,
            chk_font_use_font_for_label,

            sizer,
        };
        self.reset(&config_ui); // set checkboxes state
        *self.config_ui.borrow_mut() = Some(config_ui);

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
impl FontPickerWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        FontPickerWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            font_picker: Rc::new(RefCell::new(None)),
        }
    }

    fn create_picker(&self) {
        if let Some(font_picker) = self.font_picker.borrow().as_ref() {
            font_picker.destroy();
        }

        let mut style = wx::BORDER_DEFAULT;
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            if config_ui.chk_font_text_ctrl.get_value() {
                style |= wx::FNTP_USE_TEXTCTRL as c_long;
            }
            if config_ui.chk_font_use_font_for_label.get_value() {
                style |= wx::FNTP_USEFONT_FOR_LABEL as c_long;
            }
            if config_ui.chk_font_desc_as_label.get_value() {
                style |= wx::FNTP_FONTDESC_AS_LABEL as c_long;
            }
        }

        let font_picker = wx::FontPickerCtrl::builder(Some(&self.base))
            .id(PickerPage::Font.into())
            // TODO: wxSWISS_FONT
            .style(style)
            .build();
        *self.font_picker.borrow_mut() = Some(font_picker);
    }

    fn recreate_widget(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            config_ui.sizer.remove_int(1);
            self.create_picker();

            if let Some(font_pickr) = self.font_picker.borrow().as_ref() {
                config_ui.sizer.insert_window_int(
                    1,
                    Some(font_pickr),
                    0,
                    wx::ALIGN_CENTER | wx::ALL,
                    5,
                    wx::Object::none(),
                );
            }
            config_ui.sizer.layout();
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        config_ui
            .chk_font_text_ctrl
            .set_value((wx::FNTP_DEFAULT_STYLE & wx::FNTP_USE_TEXTCTRL) != 0);
        config_ui
            .chk_font_use_font_for_label
            .set_value((wx::FNTP_DEFAULT_STYLE & wx::FNTP_USEFONT_FOR_LABEL) != 0);
        config_ui
            .chk_font_desc_as_label
            .set_value((wx::FNTP_DEFAULT_STYLE & wx::FNTP_FONTDESC_AS_LABEL) != 0);
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.reset(config_ui);
        }
        self.recreate_widget();
    }

    fn on_check_box(&self) {
        self.recreate_widget();
    }
}
