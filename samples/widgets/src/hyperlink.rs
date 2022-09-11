// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        hyperlnk.cpp
// Purpose:     Part of the widgets sample showing wxHyperlinkCtrl
// Author:      Dimitri Schoolwerth, Vadim Zeitlin
// Created:     27 Sep 2003
// Copyright:   (c) 2003 wxWindows team
/////////////////////////////////////////////////////////////////////////////

use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum HyperlinkPage {
    Reset = wx::ID_HIGHEST as isize,
    SetLabel,
    SetURL,
    Ctrl,
}
impl HyperlinkPage {
    fn from(v: c_int) -> Option<Self> {
        use HyperlinkPage::*;
        for e in [Reset, SetLabel, SetURL, Ctrl] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<HyperlinkPage> for c_int {
    fn from(w: HyperlinkPage) -> Self {
        w as c_int
    }
}

#[derive(Clone, Copy)]
enum Align {
    Left,
    Centre,
    Right,
    Max,
}
impl Align {
    fn from(v: c_int) -> Option<Self> {
        use Align::*;
        for e in [Left, Centre, Right, Max] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<Align> for c_int {
    fn from(w: Align) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    label: wx::TextCtrl,
    url: wx::TextCtrl,
    radio_align_mode: wx::RadioBox,

    sizer: wx::BoxSizer,
}

#[derive(Clone)]
pub struct HyperlinkWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    hyperlink: Rc<RefCell<Option<wx::HyperlinkCtrl>>>,
    hyperlink_long: Rc<RefCell<Option<wx::HyperlinkCtrl>>>,
}
impl WidgetsPage for HyperlinkWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "Hyperlink";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let box_left = wx::StaticBox::builder(Some(&self.base))
            .label("Hyperlink details")
            .build();

        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&box_left), wx::VERTICAL);

        let (sizer_row, label) = self.create_sizer_with_text_and_button(
            HyperlinkPage::SetLabel.into(),
            "Set &Label",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::ALIGN_RIGHT,
            5,
            wx::Object::none(),
        );

        let (sizer_row, url) = self.create_sizer_with_text_and_button(
            HyperlinkPage::SetURL.into(),
            "Set &URL",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::ALIGN_RIGHT,
            5,
            wx::Object::none(),
        );

        let alignments = wx::ArrayString::new();
        alignments.add("&left");
        alignments.add("&centre");
        alignments.add("&right");

        let radio_align_mode = wx::RadioBox::builder(Some(&self.base))
            .label("alignment")
            .choices(alignments)
            .build();
        radio_align_mode.set_selection(1); // start with "centre" selected since
                                           // wxHL_DEFAULT_STYLE contains wxHL_ALIGN_CENTRE
        sizer_left.add_window_int(
            Some(&radio_align_mode),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let check_generic = wx::CheckBox::builder(Some(&self.base))
            .label("Use generic version")
            .build();
        sizer_left.add_window_int(
            Some(&check_generic),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        // right pane
        let sz_hyperlink_long = wx::BoxSizer::new(wx::VERTICAL);
        let sz_hyperlink = wx::BoxSizer::new(wx::HORIZONTAL);

        let visit = wx::StaticText::builder(Some(&self.base))
            .label("Visit ")
            .build();

        let hyperlink = wx::HyperlinkCtrl::builder(Some(&self.base))
            .id(HyperlinkPage::Ctrl.into())
            .label("wxWidgets website")
            .url("www.wxwidgets.org")
            .build();

        let fun = wx::StaticText::builder(Some(&self.base))
            .label(" for fun!")
            .build();

        let centre = wx::CENTRE as i32;
        sz_hyperlink.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sz_hyperlink.add_window_int(Some(&visit), 0, centre, 0, wx::Object::none());
        sz_hyperlink.add_window_int(Some(&hyperlink), 0, centre, 0, wx::Object::none());
        sz_hyperlink.add_window_int(Some(&fun), 0, centre, 0, wx::Object::none());
        sz_hyperlink.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sz_hyperlink.set_min_size_int(150, 0);
        *self.hyperlink.borrow_mut() = Some(hyperlink);

        let hyperlink_long = wx::HyperlinkCtrl::builder(Some(&self.base))
            .label("This is a long hyperlink")
            .url("www.wxwidgets.org")
            .build();

        sz_hyperlink_long.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sz_hyperlink_long.add_sizer_int(
            Some(&sz_hyperlink),
            0,
            centre | wx::GROW,
            0,
            wx::Object::none(),
        );
        sz_hyperlink_long.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sz_hyperlink_long.add_window_int(Some(&hyperlink_long), 0, wx::GROW, 0, wx::Object::none());
        sz_hyperlink_long.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        *self.hyperlink_long.borrow_mut() = Some(hyperlink_long);

        // the 3 panes panes compose the window
        sizer_top.add_sizer_int(
            Some(&sizer_left),
            0,
            wx::GROW | (wx::ALL & !wx::LEFT),
            10,
            wx::Object::none(),
        );
        sizer_top.add_sizer_int(
            Some(&sz_hyperlink_long),
            1,
            wx::GROW | (wx::ALL & !wx::RIGHT),
            10,
            wx::Object::none(),
        );

        self.base.set_sizer(Some(&sizer_top), true);

        // final initializations
        let config_ui = ConfigUI {
            label,
            url,
            radio_align_mode,

            sizer: sizer_top,
        };
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            HyperlinkPage::from(event.get_id()),
        ) {
            match m {
                HyperlinkPage::Reset => self.on_button_reset(config_ui),
                HyperlinkPage::SetLabel => self.on_button_set_label(config_ui),
                HyperlinkPage::SetURL => self.on_button_set_url(config_ui),
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        // Do nothing.
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.on_alignment(config_ui);
        }
    }
    fn handle_update_ui(&self, _: &wx::UpdateUIEvent) {
        // Update UI
    }
}
impl HyperlinkWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();

        HyperlinkWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            hyperlink: Rc::new(RefCell::new(None)),
            hyperlink_long: Rc::new(RefCell::new(None)),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
            config_ui.label.set_value(&hyperlink.get_label());
            config_ui.url.set_value(&hyperlink.get_url());
        }
    }

    fn create_hyperlink(&self, config_ui: &ConfigUI) {
        let (label, url) = if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
            (hyperlink.get_label(), hyperlink.get_url())
        } else {
            ("".to_owned(), "".to_owned())
        };

        let mut style = wx::BORDER_DEFAULT;

        style |= wx::HL_DEFAULT_STYLE & !wx::BORDER_MASK;

        let hyperlink = wx::HyperlinkCtrl::builder(Some(&self.base))
            .id(HyperlinkPage::Ctrl.into())
            .label(&label)
            .url(&url)
            .style(style)
            .build();

        // update sizer's child window
        if let Some(old_hyperlink) = self.hyperlink.borrow().as_ref() {
            config_ui
                .sizer
                .replace_window(Some(old_hyperlink), Some(&hyperlink), true);

            old_hyperlink.destroy();
        }
        // update our pointer
        *self.hyperlink.borrow_mut() = Some(hyperlink);

        // relayout the sizer
        config_ui.sizer.layout();
    }

    fn create_hyperlink_long(&self, config_ui: &ConfigUI, align: c_long) {
        let mut style = wx::BORDER_DEFAULT;
        style |= align;
        style |= wx::HL_DEFAULT_STYLE & !(wx::HL_ALIGN_CENTRE | wx::BORDER_MASK);

        let hyperlink_long = wx::HyperlinkCtrl::builder(Some(&self.base))
            .label("This is a long hyperlink")
            .url("www.wxwidgets.org")
            .style(style)
            .build();

        // update sizer's child window
        if let Some(old_hyperlink_long) = self.hyperlink_long.borrow().as_ref() {
            config_ui
                .sizer
                .replace_window(Some(old_hyperlink_long), Some(&hyperlink_long), true);

            old_hyperlink_long.destroy();
        }
        // update our pointer
        *self.hyperlink_long.borrow_mut() = Some(hyperlink_long);

        // relayout the sizer
        config_ui.sizer.layout();
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_hyperlink(config_ui);
    }

    fn on_button_set_label(&self, config_ui: &ConfigUI) {
        if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
            hyperlink.set_label(&config_ui.label.get_value());
        }
        self.create_hyperlink(config_ui);
    }

    fn on_button_set_url(&self, config_ui: &ConfigUI) {
        if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
            hyperlink.set_url(&config_ui.url.get_value());
        }
        self.create_hyperlink(config_ui);
    }

    fn on_alignment(&self, config_ui: &ConfigUI) {
        if let Some(a) = Align::from(config_ui.radio_align_mode.get_selection()) {
            let addstyle = match a {
                Align::Centre => wx::HL_ALIGN_CENTRE,
                Align::Right => wx::HL_ALIGN_RIGHT as c_long,
                _ => wx::HL_ALIGN_LEFT as c_long,
            };
            self.create_hyperlink_long(config_ui, addstyle);
        }
    }
}
