// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        clrpicker.cpp
// Purpose:     Shows wxColourPickerCtrl
// Author:      Francesco Montorsi
// Created:     20/6/2006
// Copyright:   (c) 2006 Francesco Montorsi
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
    Colour,
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
    chk_colour_text_ctrl: wx::CheckBoxIsOwned<false>,
    chk_colour_show_label: wx::CheckBoxIsOwned<false>,
    chk_colour_show_alpha: wx::CheckBoxIsOwned<false>,
    sizer: wx::BoxSizerIsOwned<false>,
}

#[derive(Clone)]
pub struct ColourPickerWidgetsPage {
    pub base: wx::PanelIsOwned<false>,
    config_ui: RefCell<Option<ConfigUI>>,
    // the picker
    clr_picker: Rc<RefCell<Option<wx::ColourPickerCtrl>>>,
}
impl WidgetsPage for ColourPickerWidgetsPage {
    fn base(&self) -> &wx::PanelIsOwned<false> {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "ColourPicker";
    }
    fn create_content(&self) {
        // left pane
        let boxleft = wx::BoxSizer::new(wx::VERTICAL);

        let clrbox =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&ColourPicker style");
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
                    .id(PickerPage::Reset.into())
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
        if let Some(clr_picker) = self.clr_picker.borrow().as_ref() {
            sizer.add_window_int(
                Some(clr_picker),
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
            chk_colour_text_ctrl: chk_colour_text_ctrl.to_unowned(),
            chk_colour_show_label: chk_colour_show_label.to_unowned(),
            chk_colour_show_alpha: chk_colour_show_alpha.to_unowned(),

            sizer: sizer.to_unowned(),
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
impl ColourPickerWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        ColourPickerWidgetsPage {
            base: panel.to_unowned(),
            config_ui: RefCell::new(None),
            clr_picker: Rc::new(RefCell::new(None)),
        }
    }

    fn recreate_widget(&self) {
        self.create_picker();

        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            // MEMO: Destroy()ing in create_picker() removes from its sizer.
            // config_ui.sizer.remove_int(1);
            self.create_picker();
            if let Some(clr_pickr) = self.clr_picker.borrow().as_ref() {
                config_ui.sizer.insert_window_int(
                    1,
                    Some(clr_pickr),
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
        if let Some(clr_picker) = self.clr_picker.borrow().as_ref() {
            clr_picker.destroy();
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

        let clr_picker = wx::ColourPickerCtrl::builder(Some(&self.base))
            .id(PickerPage::Colour.into())
            .colour(wx::Colour::new_with_str("RED"))
            .style(style)
            .build();
        *self.clr_picker.borrow_mut() = Some(clr_picker);
    }

    fn on_button_reset(&self) {
        self.reset();
        self.recreate_widget();
    }

    fn on_check_box(&self) {
        self.recreate_widget();
    }
}
