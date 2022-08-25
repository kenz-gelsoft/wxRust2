// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        combobox.cpp
// Purpose:     Part of the widgets sample showing wxComboBox
// Author:      Vadim Zeitlin
// Created:     27.03.01
// Copyright:   (c) 2001 Vadim Zeitlin
/////////////////////////////////////////////////////////////////////////////

use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum ComboPage {
    Reset = wx::ID_HIGHEST as isize,
    Popup,
    Dismiss,
    SetCurrent,
    CurText,
    InsertionPointText,
    Insert,
    InsertText,
    Add,
    AddText,
    SetFirst,
    SetFirstText,
    AddSeveral,
    AddMany,
    Clear,
    Change,
    ChangeText,
    Delete,
    DeleteText,
    DeleteSel,
    SetValue,
    SetValueText,
    Combo,
    ContainerTests,
    Dynamic,
}
impl ComboPage {
    fn from(v: c_int) -> Option<Self> {
        use ComboPage::*;
        for e in [
            Reset,
            Popup,
            Dismiss,
            SetCurrent,
            CurText,
            InsertionPointText,
            Insert,
            InsertText,
            Add,
            AddText,
            SetFirst,
            SetFirstText,
            AddSeveral,
            AddMany,
            Clear,
            Change,
            ChangeText,
            Delete,
            DeleteText,
            DeleteSel,
            SetValue,
            SetValueText,
            Combo,
            ContainerTests,
            Dynamic,
        ] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<ComboPage> for c_int {
    fn from(w: ComboPage) -> Self {
        w as c_int
    }
}

// const COMBO_KIND_DEFAULT: c_int = 0;
const COMBO_KIND_SIMPLE: c_int = 1;
const COMBO_KIND_DROP_DOWN: c_int = 2;

#[derive(Clone)]
pub struct ConfigUI {
    // the controls
    // ------------

    // the sel mode radiobox
    radio_kind: wx::RadioBoxIsOwned<false>,

    // the checkboxes for styles
    chk_sort: wx::CheckBoxIsOwned<false>,
    chk_readonly: wx::CheckBoxIsOwned<false>,
    chk_process_enter: wx::CheckBoxIsOwned<false>,

    // sizer
    sizer_combo: wx::BoxSizerIsOwned<false>,

    // the text entries for "Add/change string" and "Delete" buttons
    text_insert: wx::TextCtrlIsOwned<false>,
    text_add: wx::TextCtrlIsOwned<false>,
    text_set_first: wx::TextCtrlIsOwned<false>,
    text_change: wx::TextCtrlIsOwned<false>,
    text_set_value: wx::TextCtrlIsOwned<false>,
    text_delete: wx::TextCtrlIsOwned<false>,
    text_cur: wx::TextCtrlIsOwned<false>,
}

#[derive(Clone)]
pub struct ComboboxWidgetsPage {
    pub base: wx::PanelIsOwned<false>,
    config_ui: RefCell<Option<ConfigUI>>,
    // the combobox itself
    combobox: Rc<RefCell<Option<wx::ComboBox>>>,
    combobox1: Rc<RefCell<Option<wx::ComboBox>>>,

    s_item: RefCell<c_int>,
}
impl WidgetsPage for ComboboxWidgetsPage {
    fn base(&self) -> &wx::PanelIsOwned<false> {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "Combobox";
    }
    fn create_content(&self) {
        /*
           What we create here is a frame having 3 panes: style pane is the
           leftmost one, in the middle the pane with buttons allowing to perform
           miscellaneous combobox operations and the pane containing the combobox
           itself to the right
        */
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // upper left pane

        // should be in sync with ComboKind_XXX values
        let kinds = wx::ArrayString::new();
        kinds.add("default");
        kinds.add("simple");
        kinds.add("drop down");

        let radio_kind = wx::RadioBox::builder(Some(&self.base))
            .label("Combobox &kind:")
            .choices(kinds)
            .major_dimension(1)
            .style(wx::RA_SPECIFY_COLS.into())
            .build();

        let sizer_left_top =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Set style");

        let chk_sort =
            self.create_check_box_and_add_to_sizer(&sizer_left_top, "&Sort items", wx::ID_ANY);

        let chk_readonly =
            self.create_check_box_and_add_to_sizer(&sizer_left_top, "&Read only", wx::ID_ANY);

        let chk_process_enter =
            self.create_check_box_and_add_to_sizer(&sizer_left_top, "Process &Enter", wx::ID_ANY);

        sizer_left_top.add_int_int(5, 5, 0, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer
        sizer_left_top.add_window_int(
            Some(&radio_kind),
            0,
            wx::GROW | wx::ALL,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ComboPage::Reset.into())
            .label("&Reset")
            .build();
        sizer_left_top.add_window_int(
            Some(&btn),
            0,
            wx::ALIGN_CENTRE_HORIZONTAL | wx::ALL,
            15,
            wx::Object::none(),
        );

        // lower left pane
        let sizer_left_bottom =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Popup");
        sizer_left_bottom.add_window_sizerflags(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(ComboPage::Popup.into())
                    .label("&Show")
                    .build(),
            ),
            wx::SizerFlags::new(0).border(wx::ALL).centre(),
        );
        sizer_left_bottom.add_window_sizerflags(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(ComboPage::Dismiss.into())
                    .label("&Hide")
                    .build(),
            ),
            wx::SizerFlags::new(0).border(wx::ALL).centre(),
        );

        let sizer_left = wx::BoxSizer::new(wx::VERTICAL);
        sizer_left.add_sizer_int(Some(&sizer_left_top), 0, 0, 0, wx::Object::none());
        sizer_left.add_spacer(10);
        sizer_left.add_sizer_sizerflags(Some(&sizer_left_bottom), wx::SizerFlags::new(0).expand());

        // middle pane
        let s_box2 = wx::StaticBox::builder(Some(&self.base))
            .label("&Change combobox contents")
            .build();
        let sizer_middle = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box2), wx::VERTICAL);

        let (sizer_row, text_cur) = self.create_sizer_with_text_and_button(
            ComboPage::SetCurrent.into(),
            "Current &selection",
            ComboPage::CurText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text) = self.create_sizer_with_text_and_label(
            "Insertion Point",
            ComboPage::InsertionPointText.into(),
        );
        text.set_editable(false);
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_insert) = self.create_sizer_with_text_and_button(
            ComboPage::Insert.into(),
            "&Insert this string",
            ComboPage::InsertText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_add) = self.create_sizer_with_text_and_button(
            ComboPage::Add.into(),
            "&Add this string",
            ComboPage::AddText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_set_first) = self.create_sizer_with_text_and_button(
            ComboPage::SetFirst.into(),
            "Change &1st string",
            ComboPage::SetFirstText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ComboPage::AddSeveral.into())
            .label("&Append a few strings")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ComboPage::AddMany.into())
            .label("Append &many strings")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let (sizer_row, text_change) = self.create_sizer_with_text_and_button(
            ComboPage::Change.into(),
            "C&hange current",
            ComboPage::ChangeText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_delete) = self.create_sizer_with_text_and_button(
            ComboPage::Delete.into(),
            "&Delete this item",
            ComboPage::DeleteText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ComboPage::DeleteSel.into())
            .label("Delete &selection")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ComboPage::Clear.into())
            .label("&Clear")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let (sizer_row, text_set_value) = self.create_sizer_with_text_and_button(
            ComboPage::SetValue.into(),
            "SetValue",
            ComboPage::SetValueText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ComboPage::ContainerTests.into())
            .label("Run &tests")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        // right pane
        let sizer_right = wx::BoxSizer::new(wx::VERTICAL);
        let combobox = wx::ComboBox::builder(Some(&self.base))
            .id(ComboPage::Combo.into())
            .build();
        sizer_right.add_window_int(
            Some(&combobox),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );
        *self.combobox.borrow_mut() = Some(combobox);

        let combobox1 = wx::ComboBox::builder(Some(&self.base))
            .id(ComboPage::Combo.into())
            .build();
        combobox1.append_str("Dynamic ComboBox Test - Click me!");
        combobox1.set_selection(0);
        sizer_right.add_int_int(20, 20, 0, wx::EXPAND, 0, wx::Object::none());
        sizer_right.add_window_int(
            Some(&combobox1),
            0,
            wx::GROW | wx::ALL,
            5,
            wx::Object::none(),
        );
        *self.combobox1.borrow_mut() = Some(combobox1);

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
            radio_kind: radio_kind.to_unowned(),

            chk_sort: chk_sort.to_unowned(),
            chk_readonly: chk_readonly.to_unowned(),
            chk_process_enter: chk_process_enter.to_unowned(),

            sizer_combo: sizer_right.to_unowned(), // save it to modify it later

            text_insert: text_insert.to_unowned(),
            text_add: text_add.to_unowned(),
            text_set_first: text_set_first.to_unowned(),
            text_change: text_change.to_unowned(),
            text_set_value: text_set_value.to_unowned(),
            text_delete: text_delete.to_unowned(),
            text_cur: text_cur.to_unowned(),
        });

        // final initializations
        self.reset();

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(m), Some(config_ui)) = (
            ComboPage::from(event.get_id()),
            self.config_ui.borrow().as_ref(),
        ) {
            match m {
                ComboPage::Reset => self.on_button_reset(),
                ComboPage::Change => self.on_button_change(config_ui),
                ComboPage::Delete => self.on_button_delete(config_ui),
                ComboPage::DeleteSel => self.on_button_delete_sel(),
                ComboPage::Clear => self.on_button_clear(),
                ComboPage::Insert => self.on_button_insert(config_ui),
                ComboPage::Add => self.on_button_add(config_ui),
                ComboPage::SetFirst => self.on_button_set_first(config_ui),
                ComboPage::AddSeveral => self.on_button_add_several(),
                ComboPage::AddMany => self.on_button_add_many(),
                ComboPage::SetValue => self.on_button_set_value(config_ui),
                ComboPage::SetCurrent => self.on_button_set_current(config_ui),
                // TODO: Support update ui event to disable this when not 3state
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        self.on_check_or_radio_box();
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        self.on_check_or_radio_box();
    }
}
impl ComboboxWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        ComboboxWidgetsPage {
            base: panel.to_unowned(),
            config_ui: RefCell::new(None),
            combobox: Rc::new(RefCell::new(None)),
            combobox1: Rc::new(RefCell::new(None)),
            s_item: RefCell::new(1),
        }
    }

    // ----------------------------------------------------------------------------
    // operations
    // ----------------------------------------------------------------------------

    fn reset(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            config_ui.chk_sort.set_value(false);
            config_ui.chk_readonly.set_value(false);
            config_ui.chk_process_enter.set_value(false);
        }
    }

    fn create_combo(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            let mut flags = wx::BORDER_DEFAULT;

            if config_ui.chk_sort.get_value() {
                flags |= wx::CB_SORT as c_long;
            }
            if config_ui.chk_readonly.get_value() {
                flags |= wx::CB_READONLY as c_long;
            }
            if config_ui.chk_process_enter.get_value() {
                flags |= wx::TE_PROCESS_ENTER as c_long;
            }

            flags |= match config_ui.radio_kind.get_selection() {
                COMBO_KIND_SIMPLE => wx::CB_SIMPLE,
                COMBO_KIND_DROP_DOWN => wx::CB_DROPDOWN,
                _ => 0,
            } as c_long;

            let items = wx::ArrayString::new();
            let mut sel_item = wx::NOT_FOUND;
            if let Some(combobox) = self.combobox.borrow().as_ref() {
                let count = combobox.get_count();
                for n in 0..count {
                    items.add(&combobox.get_string(n));
                }

                sel_item = combobox.get_selection();
            }

            let new_cb = wx::ComboBox::builder(Some(&self.base))
                .id(ComboPage::Combo.into())
                .choices(items)
                .style(flags)
                .build();
            if sel_item != wx::NOT_FOUND {
                new_cb.set_selection(sel_item);
            }

            if let Some(combobox) = self.combobox.borrow().as_ref() {
                config_ui
                    .sizer_combo
                    .replace_window(Some(combobox), Some(&new_cb), false);
                config_ui.sizer_combo.layout();

                combobox.destroy();
            }

            *self.combobox.borrow_mut() = Some(new_cb);
        }
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self) {
        self.reset();
        self.create_combo();
    }

    fn on_button_change(&self, config_ui: &ConfigUI) {
        if let Some(combobox) = self.combobox.borrow().as_ref() {
            let sel = combobox.get_selection();
            if sel != wx::NOT_FOUND {
                combobox.set_string(sel.try_into().unwrap(), &config_ui.text_change.get_value());
            }
        }
    }

    fn on_button_delete(&self, config_ui: &ConfigUI) {
        let n = config_ui.text_delete.get_value();
        if let (Ok(n), Some(combobox)) = (n.parse(), self.combobox.borrow().as_ref()) {
            if n >= combobox.get_count() {
                return;
            }
            combobox.delete(n);
        }
    }

    fn on_button_delete_sel(&self) {
        if let Some(combobox) = self.combobox.borrow().as_ref() {
            let sel = combobox.get_selection();
            if sel != wx::NOT_FOUND {
                combobox.delete(sel.try_into().unwrap());
            }
        }
    }

    fn on_button_set_value(&self, config_ui: &ConfigUI) {
        let value = config_ui.text_set_value.get_value();
        if let Some(combobox) = self.combobox.borrow().as_ref() {
            combobox.set_value(&value);
        }
    }

    fn on_button_clear(&self) {
        if let Some(combobox) = self.combobox.borrow().as_ref() {
            ItemContainerMethods::clear(combobox);
        }
    }

    fn on_button_insert(&self, config_ui: &ConfigUI) {
        let s = config_ui.text_insert.get_value();
        if !config_ui.text_insert.is_modified() {
            // update the default string
            let s_item = *self.s_item.borrow();
            config_ui
                .text_insert
                .set_value(&format!("test item {}", s_item));
            *self.s_item.borrow_mut() = s_item + 1;
        }

        if let Some(combobox) = self.combobox.borrow().as_ref() {
            let sel = combobox.get_selection();
            if sel >= 0 {
                combobox.insert_str_uint(&s, sel.try_into().unwrap());
            }
        }
    }

    fn on_button_add(&self, config_ui: &ConfigUI) {
        let s = config_ui.text_add.get_value();
        if !config_ui.text_add.is_modified() {
            // update the default string
            let s_item = *self.s_item.borrow();
            config_ui
                .text_add
                .set_value(&format!("test item {}", s_item));
            *self.s_item.borrow_mut() = s_item + 1;
        }

        if let Some(combobox) = self.combobox.borrow().as_ref() {
            combobox.append_str(&s);
        }
    }

    fn on_button_set_first(&self, config_ui: &ConfigUI) {
        if let Some(combobox) = self.combobox.borrow().as_ref() {
            if combobox.is_list_empty() {
                println!("No string to change.");
                return;
            }
            combobox.set_string(0, &config_ui.text_set_first.get_value());
        }
    }

    fn on_button_add_many(&self) {
        // "many" means 1000 here
        if let Some(combobox) = self.combobox.borrow().as_ref() {
            for n in 0..1000 {
                combobox.append_str(&format!("item #{}", n));
            }
        }
    }

    fn on_button_set_current(&self, config_ui: &ConfigUI) {
        let n = config_ui.text_cur.get_value();
        if let (Ok(n), Some(combobox)) = (n.parse(), self.combobox.borrow().as_ref()) {
            combobox.set_selection(n);
        }
    }

    fn on_button_add_several(&self) {
        let items = wx::ArrayString::new();
        items.add("First");
        items.add("another one");
        items.add("and the last (very very very very very very very very very very long) one");
        if let Some(combobox) = self.combobox.borrow().as_ref() {
            combobox.insert_arraystring(&items, 0);
        }
    }

    fn on_check_or_radio_box(&self) {
        self.create_combo();
    }
}
