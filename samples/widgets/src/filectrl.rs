// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        filectrl.cpp
// Purpose:     Part of the widgets sample showing wxFileCtrl
// Author:      Diaa M. Sami
// Created:     28 Jul 2007
// Copyright:   (c) 2007 Diaa M. Sami
/////////////////////////////////////////////////////////////////////////////

use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum FileCtrlPage {
    Reset = wx::ID_HIGHEST as isize,
    SetDirectory,
    SetPath,
    SetFilename,
    Ctrl,
}
impl FileCtrlPage {
    fn from(v: c_int) -> Option<Self> {
        use FileCtrlPage::*;
        for e in [Reset, SetDirectory, SetPath, SetFilename, Ctrl] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<FileCtrlPage> for c_int {
    fn from(w: FileCtrlPage) -> Self {
        w as c_int
    }
}

#[derive(Clone, Copy, PartialEq)]
enum FileCtrlMode {
    Open = 0,
    Save,
}
impl FileCtrlMode {
    fn from(v: c_int) -> Option<Self> {
        use FileCtrlMode::*;
        for e in [Open, Save] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<FileCtrlMode> for c_int {
    fn from(m: FileCtrlMode) -> Self {
        m as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // the text entries for command parameters
    dir: wx::TextCtrlIsOwned<false>,
    path: wx::TextCtrlIsOwned<false>,
    filename: wx::TextCtrlIsOwned<false>,

    // flags
    chk_multiple: wx::CheckBoxIsOwned<false>,
    chk_no_show_hidden: wx::CheckBoxIsOwned<false>,

    radio_file_ctrl_mode: wx::RadioBoxIsOwned<false>,

    // filters
    fltr: [wx::CheckBoxIsOwned<false>; 3],

    // sizer
    sizer: wx::BoxSizerIsOwned<false>,
}

#[derive(Clone)]
pub struct FileCtrlWidgetsPage {
    pub base: wx::PanelIsOwned<false>,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    file_ctrl: Rc<RefCell<Option<wx::FileCtrl>>>,
}
impl WidgetsPage for FileCtrlWidgetsPage {
    fn base(&self) -> &wx::PanelIsOwned<false> {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "FileCtrl";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let sizer_left = wx::BoxSizer::new(wx::VERTICAL);

        let mode = wx::ArrayString::new();
        mode.add("open");
        mode.add("save");
        let radio_file_ctrl_mode = wx::RadioBox::builder(Some(&self.base))
            .label("wxFileCtrl mode")
            .choices(mode)
            .build();

        sizer_left.add_window_int(
            Some(&radio_file_ctrl_mode),
            0,
            wx::ALL | wx::EXPAND,
            5,
            wx::Object::none(),
        );

        let (sizer, dir) = self.create_sizer_with_text_and_button(
            FileCtrlPage::SetDirectory.into(),
            "Set &directory",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_int(Some(&sizer), 0, wx::ALL | wx::EXPAND, 5, wx::Object::none());
        let (sizer, path) = self.create_sizer_with_text_and_button(
            FileCtrlPage::SetPath.into(),
            "Set &path",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_int(Some(&sizer), 0, wx::ALL | wx::EXPAND, 5, wx::Object::none());
        let (sizer, filename) = self.create_sizer_with_text_and_button(
            FileCtrlPage::SetFilename.into(),
            "Set &filename",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_int(Some(&sizer), 0, wx::ALL | wx::EXPAND, 5, wx::Object::none());

        let sizer_use_flags =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Flags");

        let chk_multiple =
            self.create_check_box_and_add_to_sizer(&sizer_use_flags, "wxFC_MULTIPLE", wx::ID_ANY);
        let chk_no_show_hidden = self.create_check_box_and_add_to_sizer(
            &sizer_use_flags,
            "wxFC_NOSHOWHIDDEN",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_sizerflags(
            Some(&sizer_use_flags),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let sizer_filters =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Filters");
        let fltr = [
            self.create_check_box_and_add_to_sizer(
                &sizer_filters,
                &format!(
                    "all files ({})|{}",
                    wx::FILE_SELECTOR_DEFAULT_WILDCARD_STR,
                    wx::FILE_SELECTOR_DEFAULT_WILDCARD_STR
                ),
                wx::ID_ANY,
            )
            .to_unowned(),
            self.create_check_box_and_add_to_sizer(
                &sizer_filters,
                "C++ files (*.cpp; *.h)|*.cpp;*.h",
                wx::ID_ANY,
            )
            .to_unowned(),
            self.create_check_box_and_add_to_sizer(
                &sizer_filters,
                "PNG images (*.png)|*.png",
                wx::ID_ANY,
            )
            .to_unowned(),
        ];
        sizer_left.add_sizer_sizerflags(
            Some(&sizer_filters),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(FileCtrlPage::Reset.into())
            .label("&Reset")
            .build();
        sizer_left.add_window_int(
            Some(&btn),
            0,
            wx::ALIGN_CENTRE_HORIZONTAL | wx::ALL,
            15,
            wx::Object::none(),
        );

        // right pane
        let file_ctrl = wx::FileCtrl::builder(Some(&self.base))
            .id(FileCtrlPage::Ctrl.into())
            .style(wx::FC_OPEN.into())
            .build();

        // the 3 panes panes compose the window
        sizer_top.add_sizer_int(
            Some(&sizer_left),
            0,
            wx::ALL & !wx::LEFT,
            10,
            wx::Object::none(),
        );
        sizer_top.add_window_int(
            Some(&file_ctrl),
            1,
            wx::GROW | (wx::ALL & !wx::RIGHT),
            10,
            wx::Object::none(),
        );
        *self.file_ctrl.borrow_mut() = Some(file_ctrl);

        self.base.set_sizer(Some(&sizer_top), true);

        // final initializations
        let config_ui = ConfigUI {
            dir: dir.to_unowned(),
            path: path.to_unowned(),
            filename: filename.to_unowned(),
            chk_multiple: chk_multiple.to_unowned(),
            chk_no_show_hidden: chk_no_show_hidden.to_unowned(),
            radio_file_ctrl_mode: radio_file_ctrl_mode.to_unowned(),
            fltr,
            sizer: sizer_top.to_unowned(),
        };
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            FileCtrlPage::from(event.get_id()),
        ) {
            match m {
                FileCtrlPage::Reset => self.on_button_reset(config_ui),
                FileCtrlPage::SetDirectory => self.on_button_set_directory(config_ui),
                FileCtrlPage::SetPath => self.on_button_set_path(config_ui),
                FileCtrlPage::SetFilename => self.on_button_set_filename(config_ui),
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.on_check_box(config_ui);
        }
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.on_switch_mode(config_ui);
        }
    }
}
impl FileCtrlWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        FileCtrlWidgetsPage {
            base: panel.to_unowned(),
            config_ui: RefCell::new(None),
            file_ctrl: Rc::new(RefCell::new(None)),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        if let Some(file_ctrl) = self.file_ctrl.borrow().as_ref() {
            config_ui.dir.set_value(&file_ctrl.get_directory());
        }
        config_ui.radio_file_ctrl_mode.set_selection(
            if (wx::FC_DEFAULT_STYLE & wx::FC_OPEN) != 0 {
                FileCtrlMode::Open.into()
            } else {
                FileCtrlMode::Save.into()
            },
        );
    }

    fn create_file_ctrl(&self, config_ui: &ConfigUI) {
        let _no_updates = wx::WindowUpdateLocker::new_with_window(Some(&self.base));

        let mut style = wx::BORDER_DEFAULT;

        if FileCtrlMode::from(config_ui.radio_file_ctrl_mode.get_selection())
            == Some(FileCtrlMode::Open)
        {
            style |= wx::FC_OPEN as c_long;
            config_ui.chk_multiple.enable(true);
            if config_ui.chk_multiple.is_checked() {
                style |= wx::FC_MULTIPLE as c_long;
            }
        } else {
            style |= wx::FC_SAVE as c_long;
            // wxFC_SAVE is incompatible with wxFC_MULTIPLE
            config_ui.chk_multiple.set_value(false);
            config_ui.chk_multiple.enable(false);
        }

        if config_ui.chk_no_show_hidden.is_checked() {
            style |= wx::FC_NOSHOWHIDDEN as c_long;
        }

        let file_ctrl = wx::FileCtrl::builder(Some(&self.base))
            .id(FileCtrlPage::Ctrl.into())
            .default_directory("")
            .default_filename("")
            .wild_card("")
            .style(style)
            .build();

        let mut wildcard = String::new();
        for fltr in config_ui.fltr.iter() {
            if fltr.is_checked() {
                if !wildcard.is_empty() {
                    wildcard.push_str("|");
                }
                wildcard.push_str(&fltr.get_label());
            }
        }
        file_ctrl.set_wildcard(&wildcard);

        // update sizer's child window
        if let Some(old_file_ctrl) = self.file_ctrl.borrow().as_ref() {
            config_ui
                .sizer
                .replace_window(Some(old_file_ctrl), Some(&file_ctrl), false);

            old_file_ctrl.destroy();
        }
        // update our pointer
        *self.file_ctrl.borrow_mut() = Some(file_ctrl);

        // relayout the sizer
        config_ui.sizer.layout();
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_set_directory(&self, config_ui: &ConfigUI) {
        if let Some(file_ctrl) = self.file_ctrl.borrow().as_ref() {
            file_ctrl.set_directory(&config_ui.dir.get_value());
        }
    }

    fn on_button_set_path(&self, config_ui: &ConfigUI) {
        if let Some(file_ctrl) = self.file_ctrl.borrow().as_ref() {
            file_ctrl.set_path(&config_ui.path.get_value());
        }
    }

    fn on_button_set_filename(&self, config_ui: &ConfigUI) {
        if let Some(file_ctrl) = self.file_ctrl.borrow().as_ref() {
            file_ctrl.set_filename(&config_ui.filename.get_value());
        }
    }

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_file_ctrl(config_ui);
    }

    fn on_check_box(&self, config_ui: &ConfigUI) {
        self.create_file_ctrl(config_ui);
    }

    fn on_switch_mode(&self, config_ui: &ConfigUI) {
        self.create_file_ctrl(config_ui);
    }
}
