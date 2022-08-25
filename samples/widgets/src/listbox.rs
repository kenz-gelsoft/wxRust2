// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        listbox.cpp
// Purpose:     Part of the widgets sample showing wxListbox
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
enum ListboxPage {
    Reset = wx::ID_HIGHEST as isize,
    Add,
    AddText,
    AddSeveral,
    AddMany,
    Clear,
    Change,
    ChangeText,
    Delete,
    DeleteText,
    DeleteSel,
    Listbox,
    EnsureVisible,
    EnsureVisibleText,
    ContainerTests,
    GetTopItem,
    GetCountPerPage,
    MoveUp,
    MoveDown,
}
impl ListboxPage {
    fn from(v: c_int) -> Option<Self> {
        use ListboxPage::*;
        for e in [
            Reset,
            Add,
            AddText,
            AddSeveral,
            AddMany,
            Clear,
            Change,
            ChangeText,
            Delete,
            DeleteText,
            DeleteSel,
            Listbox,
            EnsureVisible,
            EnsureVisibleText,
            ContainerTests,
            GetTopItem,
            GetCountPerPage,
            MoveUp,
            MoveDown,
        ] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<ListboxPage> for c_int {
    fn from(w: ListboxPage) -> Self {
        w as c_int
    }
}

// the selection mode
#[derive(Clone, Copy)]
enum LboxSel {
    Single,
    Extended,
    Multiple,
}
impl LboxSel {
    fn from(v: c_int) -> Option<Self> {
        use LboxSel::*;
        for e in [Single, Extended, Multiple] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<LboxSel> for c_int {
    fn from(w: LboxSel) -> Self {
        w as c_int
    }
}

// the list type
#[derive(Clone, Copy)]
enum LboxType {
    ListBox,
    CheckListBox,
    RearrangeList,
}
impl LboxType {
    fn from(v: c_int) -> Option<Self> {
        use LboxType::*;
        for e in [ListBox, CheckListBox, RearrangeList] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<LboxType> for c_int {
    fn from(w: LboxType) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // the controls
    // ------------
    // the sel mode radiobox
    radio_sel_mode: wx::RadioBoxIsOwned<false>,

    // List type selection radiobox
    radio_list_type: wx::RadioBoxIsOwned<false>,

    // the checkboxes
    chk_v_scroll: wx::CheckBoxIsOwned<false>,
    chk_h_scroll: wx::CheckBoxIsOwned<false>,
    chk_sort: wx::CheckBoxIsOwned<false>,
    chk_owner_draw: wx::CheckBoxIsOwned<false>,

    // sizer
    sizer_lbox: wx::BoxSizerIsOwned<false>,

    // the text entries for "Add/change string" and "Delete" buttons
    text_add: wx::TextCtrlIsOwned<false>,
    text_change: wx::TextCtrlIsOwned<false>,
    text_ensure_visible: wx::TextCtrlIsOwned<false>,
    text_delete: wx::TextCtrlIsOwned<false>,
}

#[derive(Clone)]
pub struct ListboxWidgetsPage {
    pub base: wx::PanelIsOwned<false>,
    config_ui: RefCell<Option<ConfigUI>>,
    // the lbox itself
    lbox: Rc<RefCell<Option<wx::ListBox>>>,

    s_item: RefCell<c_int>,
}
impl WidgetsPage for ListboxWidgetsPage {
    fn base(&self) -> &wx::PanelIsOwned<false> {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "ListBox";
    }
    fn create_content(&self) {
        /*
            What we create here is a frame having 3 panes: style pane is the
            leftmost one, in the middle the pane with buttons allowing to perform
            miscellaneous listbox operations and the pane containing the listbox
            itself to the right
        */
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let s_box = wx::StaticBox::builder(Some(&self.base))
            .label("&Set listbox parameters")
            .build();
        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box), wx::VERTICAL);

        let modes = wx::ArrayString::new();
        modes.add("single");
        modes.add("extended");
        modes.add("multiple");

        let radio_sel_mode = wx::RadioBox::builder(Some(&self.base))
            .label("Selection &mode:")
            .choices(modes)
            .major_dimension(1)
            .style(wx::RA_SPECIFY_COLS.into())
            .build();

        let list_types = wx::ArrayString::new();
        list_types.add("list box");
        list_types.add("check list box");
        list_types.add("rearrange list");

        let radio_list_type = wx::RadioBox::builder(Some(&self.base))
            .label("&List type:")
            .choices(list_types)
            .major_dimension(1)
            .style(wx::RA_SPECIFY_COLS.into())
            .build();

        let chk_v_scroll = self.create_check_box_and_add_to_sizer(
            &sizer_left,
            "Always show &vertical scrollbar",
            wx::ID_ANY,
        );
        let chk_h_scroll = self.create_check_box_and_add_to_sizer(
            &sizer_left,
            "Show &horizontal scrollbar",
            wx::ID_ANY,
        );

        let chk_sort =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Sort items", wx::ID_ANY);
        let chk_owner_draw =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Owner drawn", wx::ID_ANY);

        sizer_left.add_int_int(5, 5, 0, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer
        sizer_left.add_window_int(
            Some(&radio_sel_mode),
            0,
            wx::GROW | wx::ALL,
            5,
            wx::Object::none(),
        );

        sizer_left.add_int_int(5, 5, 0, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer
        sizer_left.add_window_int(
            Some(&radio_list_type),
            0,
            wx::GROW | wx::ALL,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Reset.into())
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
            .label("&Change listbox contents")
            .build();
        let sizer_middle = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box2), wx::VERTICAL);

        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Add.into())
            .label("&Add this string")
            .build();
        let text_add = wx::TextCtrl::builder(Some(&self.base))
            .id(ListboxPage::AddText.into())
            .value("test item \t0")
            .build();
        sizer_row.add_window_int(Some(&btn), 0, wx::RIGHT, 5, wx::Object::none());
        sizer_row.add_window_int(Some(&text_add), 1, wx::LEFT, 5, wx::Object::none());
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::AddSeveral.into())
            .label("&Insert a few strings")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::AddMany.into())
            .label("Add &many strings")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Change.into())
            .label("C&hange current")
            .build();
        let text_change = wx::TextCtrl::builder(Some(&self.base))
            .id(ListboxPage::ChangeText.into())
            .build();
        sizer_row.add_window_int(Some(&btn), 0, wx::RIGHT, 5, wx::Object::none());
        sizer_row.add_window_int(Some(&text_change), 1, wx::LEFT, 5, wx::Object::none());
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::EnsureVisible.into())
            .label("Make item &visible")
            .build();
        let text_ensure_visible = wx::TextCtrl::builder(Some(&self.base))
            .id(ListboxPage::EnsureVisibleText.into())
            .build();
        sizer_row.add_window_int(Some(&btn), 0, wx::RIGHT, 5, wx::Object::none());
        sizer_row.add_window_int(
            Some(&text_ensure_visible),
            1,
            wx::LEFT,
            5,
            wx::Object::none(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Delete.into())
            .label("&Delete this item")
            .build();
        let text_delete = wx::TextCtrl::builder(Some(&self.base))
            .id(ListboxPage::DeleteText.into())
            .build();
        sizer_row.add_window_int(Some(&btn), 0, wx::RIGHT, 5, wx::Object::none());
        sizer_row.add_window_int(Some(&text_delete), 1, wx::LEFT, 5, wx::Object::none());
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::DeleteSel.into())
            .label("Delete &selection")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Clear.into())
            .label("&Clear")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::MoveUp.into())
            .label("Move item &up")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::MoveDown.into())
            .label("Move item &down")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::GetTopItem.into())
            .label("Get top item")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::GetCountPerPage.into())
            .label("Get count per page")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::ContainerTests.into())
            .label("Run &tests")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        // right pane
        let sizer_right = wx::BoxSizer::new(wx::VERTICAL);
        let lbox = wx::ListBox::builder(Some(&self.base))
            .id(ListboxPage::Listbox.into())
            .style(wx::LB_HSCROLL.into())
            .build();
        sizer_right.add_window_int(Some(&lbox), 1, wx::ALL | wx::GROW, 5, wx::Object::none());
        sizer_right.set_min_size_int(150, 0);
        *self.lbox.borrow_mut() = Some(lbox);

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
            1,
            wx::GROW | wx::ALL,
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
            radio_sel_mode: radio_sel_mode.to_unowned(),

            // List type selection radiobox
            radio_list_type: radio_list_type.to_unowned(),

            // the checkboxes
            chk_v_scroll: chk_v_scroll.to_unowned(),
            chk_h_scroll: chk_h_scroll.to_unowned(),
            chk_sort: chk_sort.to_unowned(),
            chk_owner_draw: chk_owner_draw.to_unowned(),

            sizer_lbox: sizer_right.to_unowned(), // save it to modify it later

            text_add: text_add.to_unowned(),
            text_change: text_change.to_unowned(),
            text_ensure_visible: text_ensure_visible.to_unowned(),
            text_delete: text_delete.to_unowned(),
        };

        // do create the main control
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(m), Some(config_ui)) = (
            ListboxPage::from(event.get_id()),
            self.config_ui.borrow().as_ref(),
        ) {
            match m {
                ListboxPage::Reset => self.on_button_reset(config_ui),
                ListboxPage::Change => self.on_button_change(config_ui),
                ListboxPage::Delete => self.on_button_delete(config_ui),
                ListboxPage::DeleteSel => self.on_button_delete_sel(config_ui),
                ListboxPage::EnsureVisible => self.on_button_ensure_visible(config_ui),
                ListboxPage::Clear => self.on_button_clear(),
                ListboxPage::Add => self.on_button_add(config_ui),
                ListboxPage::AddSeveral => self.on_button_add_several(),
                ListboxPage::AddMany => self.on_button_add_many(),
                // wx3.0 unsupported
                // ListboxPage::GetTopItem => self.on_button_top_item(),
                // ListboxPage::GetCountPerPage => self.on_button_page_count(),
                ListboxPage::MoveUp => self.on_button_move_up(),
                ListboxPage::MoveDown => self.on_button_move_down(),
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
impl ListboxWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        ListboxWidgetsPage {
            base: panel.to_unowned(),
            config_ui: RefCell::new(None),
            lbox: Rc::new(RefCell::new(None)),
            s_item: RefCell::new(1),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        config_ui
            .radio_sel_mode
            .set_selection(LboxSel::Single.into());
        config_ui
            .radio_list_type
            .set_selection(LboxType::ListBox.into());
        config_ui.chk_v_scroll.set_value(false);
        config_ui.chk_h_scroll.set_value(true);
        config_ui.chk_sort.set_value(false);

        config_ui.chk_owner_draw.set_value(false);
    }

    fn create_lbox(&self, config_ui: &ConfigUI) {
        let mut flags = wx::BORDER_DEFAULT;

        if let Some(m) = LboxSel::from(config_ui.radio_sel_mode.get_selection()) {
            flags |= match m {
                LboxSel::Extended => wx::LB_EXTENDED,
                LboxSel::Multiple => wx::LB_MULTIPLE,
                _ => wx::LB_SINGLE,
            } as c_long;
        }

        if config_ui.chk_v_scroll.is_checked() {
            flags |= wx::LB_ALWAYS_SB as c_long;
        }
        if config_ui.chk_h_scroll.is_checked() {
            flags |= wx::LB_HSCROLL as c_long;
        }
        if config_ui.chk_sort.is_checked() {
            flags |= wx::LB_SORT as c_long;
        }
        if config_ui.chk_owner_draw.is_checked() {
            flags |= wx::LB_OWNERDRAW as c_long;
        }

        let items = wx::ArrayString::new();
        let mut order: Vec<bool> = vec![];
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            // TODO: remove (and delete) all lboxes
            let count = lbox.get_count();
            for n in 0..count {
                items.add(&lbox.get_string(n));
            }

            if let Some(check_lbox) = lbox.as_unowned::<wx::CheckListBox>() {
                for n in 0..count {
                    order.push(check_lbox.is_checked(n));
                }
            }

            config_ui.sizer_lbox.detach_window(Some(lbox));
            lbox.destroy();
        }

        if let Some(m) = LboxType::from(config_ui.radio_list_type.get_selection()) {
            let new_lbox = match m {
                LboxType::CheckListBox => {
                    let check_lbox = wx::CheckListBox::builder(Some(&self.base))
                        .id(ListboxPage::Listbox.into())
                        .choices(items)
                        .style(flags)
                        .build();
                    for n in 0..order.len() {
                        check_lbox.check(n as u32, order[n]);
                    }
                    check_lbox.into()
                }
                // LboxType::RearrangeList => {
                //     // TODO
                // }
                _ => wx::ListBox::builder(Some(&self.base))
                    .id(ListboxPage::Listbox.into())
                    .choices(items)
                    .style(flags)
                    .build(),
            };

            let sizer_lbox = &config_ui.sizer_lbox;
            sizer_lbox.add_window_int(
                Some(&new_lbox),
                1,
                wx::GROW | wx::ALL,
                5,
                wx::Object::none(),
            );
            sizer_lbox.layout();

            *self.lbox.borrow_mut() = Some(new_lbox);
        }
    }

    fn get_valid_index_from_text<T: wx::TextEntryMethods>(&self, text: &T) -> Option<u32> {
        let idx = text.get_value();
        if let (Ok(idx), Some(lbox)) = (idx.parse(), self.lbox.borrow().as_ref()) {
            if idx < lbox.get_count() {
                return Some(idx);
            }
        }
        return None;
    }

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);
        self.create_lbox(config_ui);
    }

    fn on_button_change(&self, config_ui: &ConfigUI) {
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            let selections = wx::ArrayInt::new();
            let count = lbox.get_selections(&selections);
            let s = config_ui.text_change.get_value();
            for n in 0..count {
                let i = selections.item(n.try_into().unwrap()).try_into().unwrap();
                lbox.set_string(i, &s);
            }
        }
    }

    fn on_button_ensure_visible(&self, config_ui: &ConfigUI) {
        if let (Some(n), Some(lbox)) = (
            self.get_valid_index_from_text(&config_ui.text_ensure_visible.to_unowned()),
            self.lbox.borrow().as_ref(),
        ) {
            lbox.ensure_visible(n.try_into().unwrap());
        }
    }

    fn on_button_delete(&self, config_ui: &ConfigUI) {
        if let (Some(n), Some(lbox)) = (
            self.get_valid_index_from_text(&config_ui.text_delete),
            self.lbox.borrow().as_ref(),
        ) {
            lbox.delete(n.try_into().unwrap());
        }
    }

    fn on_button_delete_sel(&self, _: &ConfigUI) {
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            let selections = wx::ArrayInt::new();
            let mut n = lbox.get_selections(&selections) - 1;
            while n >= 0 {
                let i = selections.item(n.try_into().unwrap()).try_into().unwrap();
                lbox.delete(i);
                n -= 1;
            }
        }
    }

    fn on_button_clear(&self) {
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.clear();
        }
    }

    // fn on_button_top_item(&self) {
    //     if let Some(lbox) = self.lbox.borrow().as_ref() {
    //         let item = lbox.get_top_item();
    //         println!("Topmost visible item is: {}", item);
    //     }
    // }

    // fn on_button_page_count(&self) {
    //     if let Some(lbox) = self.lbox.borrow().as_ref() {
    //         let count = lbox.get_count_per_page();
    //         println!("{} items fit into this listbox.", count);
    //     }
    // }

    fn on_button_add(&self, config_ui: &ConfigUI) {
        let s = config_ui.text_add.get_value();
        if !config_ui.text_add.is_modified() {
            // update the default string
            let s_item = *self.s_item.borrow();
            config_ui
                .text_add
                .set_value(&format!("test item \t{}", s_item));
            *self.s_item.borrow_mut() = s_item + 1;
        }

        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.append_str(&s);
        }
    }

    fn on_button_add_many(&self) {
        // "many" means 1000 here
        let strings = wx::ArrayString::new();
        for n in 0..1000 {
            strings.add(&format!("item #{}", n));
        }
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.append_arraystring(&strings);
        }
    }

    fn on_button_add_several(&self) {
        let items = wx::ArrayString::new();
        items.add("First");
        items.add("another one");
        items.add("and the last (very very very very very very very very very very long) one");
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.insert_arraystring(&items, 0);
        }
    }

    fn on_button_move_up(&self) {
        // TODO: RearrangeList
    }

    fn on_button_move_down(&self) {
        // TODO: RearrangeList
    }

    fn on_check_or_radio_box(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.create_lbox(config_ui);
        }
    }
}
