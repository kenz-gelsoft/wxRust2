// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        searchctrl.cpp
// Purpose:     Shows wxSearchCtrl
// Author:      Robin Dunn
// Created:     9-Dec-2006
// Copyright:   (c) 2006
/////////////////////////////////////////////////////////////////////////////

use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::c_int;
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum ID {
    SearchCB = wx::ID_HIGHEST as isize,
    CancelCB,
    MenuCB,

    SearchMenu,
    SearchMenuLast = ID::SearchMenu as isize + 5,
}
impl ID {
    fn from(v: c_int) -> Option<Self> {
        use ID::*;
        for e in [SearchCB, CancelCB, MenuCB, SearchMenu, SearchMenuLast] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<ID> for c_int {
    fn from(w: ID) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    search_btn_check: wx::CheckBox,
    cancel_btn_check: wx::CheckBox,
    menu_btn_check: wx::CheckBox,
}

#[derive(Clone)]
pub struct SearchCtrlWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    srch_ctrl: Rc<RefCell<Option<wx::SearchCtrl>>>,
}
impl WidgetsPage for SearchCtrlWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "SearchCtrl";
    }
    fn create_content(&self) {
        self.create_control();

        let s_box = wx::StaticBoxSizer::new_with_staticbox(
            Some(
                &wx::StaticBox::builder(Some(&self.base))
                    .label("Options")
                    .build(),
            ),
            wx::VERTICAL,
        );

        let search_btn_check = wx::CheckBox::builder(Some(&self.base))
            .id(ID::SearchCB.into())
            .label("Search button")
            .build();
        let cancel_btn_check = wx::CheckBox::builder(Some(&self.base))
            .id(ID::CancelCB.into())
            .label("Cancel button")
            .build();
        let menu_btn_check = wx::CheckBox::builder(Some(&self.base))
            .id(ID::MenuCB.into())
            .label("Search menu")
            .build();

        search_btn_check.set_value(true);

        s_box.add_window_sizerflags(
            Some(&search_btn_check),
            wx::SizerFlags::new(0).border(wx::ALL),
        );
        s_box.add_window_sizerflags(
            Some(&cancel_btn_check),
            wx::SizerFlags::new(0).border(wx::ALL),
        );
        s_box.add_window_sizerflags(
            Some(&menu_btn_check),
            wx::SizerFlags::new(0).border(wx::ALL),
        );

        let sizer = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer.add_sizer_sizerflags(
            Some(&s_box),
            wx::SizerFlags::new(0).expand().triple_border(wx::ALL),
        );
        if let Some(srch_ctrl) = self.srch_ctrl.borrow().as_ref() {
            sizer.add_window_sizerflags(
                Some(srch_ctrl),
                wx::SizerFlags::new(0).expand().triple_border(wx::ALL),
            );
        }

        self.base.set_sizer(Some(&sizer), true);
        let config_ui = ConfigUI {
            search_btn_check,
            cancel_btn_check,
            menu_btn_check,
        };
        *self.config_ui.borrow_mut() = Some(config_ui);
    }

    fn handle_button(&self, _: &wx::CommandEvent) {
        // Do nothing
    }
    fn handle_checkbox(&self, event: &wx::CommandEvent) {
        if let (Some(m), Some(config_ui)) =
            (ID::from(event.get_id()), self.config_ui.borrow().as_ref())
        {
            match m {
                ID::SearchCB => self.on_toggle_search_button(config_ui),
                ID::CancelCB => self.on_toggle_cancel_button(config_ui),
                ID::MenuCB => self.on_toggle_search_menu(config_ui),
                _ => (),
            };
        }
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        // Do nothing
    }
}
impl SearchCtrlWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        SearchCtrlWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            srch_ctrl: Rc::new(RefCell::new(None)),
        }
    }

    fn create_control(&self) {
        if let Some(srch_ctrl) = self.srch_ctrl.borrow().as_ref() {
            srch_ctrl.destroy();
        }

        let style = wx::BORDER_DEFAULT;

        let srch_ctrl = wx::SearchCtrl::builder(Some(&self.base))
            .size(
                // self.base.from_dip_size(&wx::Size::new_with_int(150, -1)) // requires wx3.1+
                wx::Size::new_with_int(150, -1),
            )
            .style(style)
            .build();
        *self.srch_ctrl.borrow_mut() = Some(srch_ctrl);
    }

    fn create_test_menu(&self) -> wx::Menu {
        let menu = wx::Menu::new();
        if let Some(menu_item) =
            menu.append_int_str(wx::ID_ANY, "Recent Searches", "", wx::ITEM_NORMAL)
        {
            menu_item.enable(false);
            for i in 0..(ID::SearchMenuLast as c_int - ID::SearchMenu as c_int) {
                let item_text = format!("item {}", i);
                let tip_text = format!("tip {}", i);
                menu.append_int_str(
                    (ID::SearchMenu as c_int) + i,
                    &item_text,
                    &tip_text,
                    wx::ITEM_CHECK,
                );
            }
        }
        menu
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_toggle_search_button(&self, config_ui: &ConfigUI) {
        if let Some(srch_ctrl) = self.srch_ctrl.borrow().as_ref() {
            srch_ctrl.show_search_button(config_ui.search_btn_check.get_value());
        }
    }
    fn on_toggle_cancel_button(&self, config_ui: &ConfigUI) {
        if let Some(srch_ctrl) = self.srch_ctrl.borrow().as_ref() {
            srch_ctrl.show_cancel_button(config_ui.cancel_btn_check.get_value());
        }
    }
    fn on_toggle_search_menu(&self, config_ui: &ConfigUI) {
        if let Some(srch_ctrl) = self.srch_ctrl.borrow().as_ref() {
            if config_ui.menu_btn_check.get_value() {
                srch_ctrl.set_menu(Some(&self.create_test_menu()));
            } else {
                srch_ctrl.set_menu(wx::Menu::none());
            }
        }
    }
}
