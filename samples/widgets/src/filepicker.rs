// SPDX-License-Identifier: LGPL-2.0-or-later WITH WxWindows-exception-3.1
//
// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        filepicker.cpp
// Purpose:     Part of the widgets sample showing wx*PickerCtrl
// Author:      Francesco Montorsi
// Created:     20/6/2006
// Copyright:   (c) 2006 Francesco Montorsi
/////////////////////////////////////////////////////////////////////////////

use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

#[derive(Clone, Copy)]
enum FilePickerMode {
    Open = 0,
    Save,
}
impl FilePickerMode {
    fn from(v: c_int) -> Option<Self> {
        use FilePickerMode::*;
        for e in [Open, Save] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<FilePickerMode> for c_int {
    fn from(e: FilePickerMode) -> Self {
        e as c_int
    }
}

// control ids
#[derive(Clone, Copy)]
enum PickerPage {
    Reset = wx::ID_HIGHEST as isize,
    File,
    SetDir,
    CurrentPath,
}
impl PickerPage {
    fn from(v: c_int) -> Option<Self> {
        use PickerPage::*;
        for e in [Reset, File, SetDir, CurrentPath] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
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
    chk_file_text_ctrl: wx::CheckBox,
    chk_file_overwrite_prompt: wx::CheckBox,
    chk_file_must_exist: wx::CheckBox,
    chk_file_change_dir: wx::CheckBox,
    chk_small: wx::CheckBox,
    radio_file_picker_mode: wx::RadioBox,
    text_initial_dir: wx::TextCtrl,

    sizer: wx::BoxSizer,
}

#[derive(Clone)]
pub struct FilePickerWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the picker
    file_picker: Rc<RefCell<Option<wx::FilePickerCtrl>>>,
}
impl WidgetsPage for FilePickerWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "FilePicker";
    }
    fn create_content(&self) {
        // left pane
        let boxleft = wx::BoxSizer::new(wx::VERTICAL);

        let mode = wx::ArrayString::new();
        mode.add("open");
        mode.add("save");
        let radio_file_picker_mode = wx::RadioBox::builder(Some(&self.base))
            .label("wxFilePicker mode")
            .choices(mode)
            .build();
        boxleft.add_window_int(
            Some(&radio_file_picker_mode),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let filebox =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&FilePicker style");
        let chk_file_text_ctrl =
            self.create_check_box_and_add_to_sizer(&filebox, "With textctrl", wx::ID_ANY);
        let chk_file_overwrite_prompt =
            self.create_check_box_and_add_to_sizer(&filebox, "Overwrite prompt", wx::ID_ANY);
        let chk_file_must_exist =
            self.create_check_box_and_add_to_sizer(&filebox, "Dir must exist", wx::ID_ANY);
        let chk_file_change_dir =
            self.create_check_box_and_add_to_sizer(&filebox, "Change working dir", wx::ID_ANY);
        let chk_small =
            self.create_check_box_and_add_to_sizer(&filebox, "&Small version", wx::ID_ANY);

        boxleft.add_sizer_int(Some(&filebox), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let (sizer_initial_dir, text_initial_dir) = self.create_sizer_with_text_and_button(
            PickerPage::SetDir.into(),
            "&Initial directory",
            wx::ID_ANY,
        );
        boxleft.add_sizer_sizerflags(
            Some(&sizer_initial_dir),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        boxleft.add_spacer(10);

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

        let sizer = wx::BoxSizer::new(wx::VERTICAL);
        let config_ui = ConfigUI {
            chk_file_text_ctrl,
            chk_file_overwrite_prompt,
            chk_file_must_exist,
            chk_file_change_dir,
            chk_small,
            radio_file_picker_mode,
            text_initial_dir,

            sizer,
        };
        self.reset(&config_ui); // set checkboxes state

        let label_path = wx::StaticText::builder(Some(&self.base))
            .id(PickerPage::CurrentPath.into())
            .label("")
            .build();

        // create pickers
        self.create_picker(&config_ui);

        // right pane
        let sizer = &config_ui.sizer;
        sizer.add_stretch_spacer(1);
        if let Some(file_picker) = self.file_picker.borrow().as_ref() {
            sizer.add_window_sizerflags(
                Some(file_picker),
                wx::SizerFlags::new(0).expand().border(wx::ALL),
            );
        }
        sizer.add_stretch_spacer(1);
        sizer.add_window_sizerflags(
            Some(&label_path),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        sizer.add_stretch_spacer(1);

        // global pane
        let sz = wx::BoxSizer::new(wx::HORIZONTAL);
        sz.add_sizer_int(Some(&boxleft), 0, wx::GROW | wx::ALL, 5, wx::Object::none());
        sz.add_sizer_int(Some(sizer), 1, wx::GROW | wx::ALL, 5, wx::Object::none());
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sz), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            PickerPage::from(event.get_id()),
        ) {
            match m {
                PickerPage::Reset => self.on_button_reset(config_ui),
                PickerPage::SetDir => self.on_button_set_dir(config_ui),
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        self.on_check_box();
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        self.on_radio_box();
    }
}
impl FilePickerWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        FilePickerWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            file_picker: Rc::new(RefCell::new(None)),
        }
    }

    fn create_picker(&self, config_ui: &ConfigUI) {
        if let Some(file_picker) = self.file_picker.borrow().as_ref() {
            file_picker.destroy();
        }

        let mut style = wx::BORDER_DEFAULT;

        if config_ui.chk_file_text_ctrl.get_value() {
            style |= wx::FLP_USE_TEXTCTRL as c_long;
        }

        if config_ui.chk_file_overwrite_prompt.get_value() {
            style |= wx::FLP_OVERWRITE_PROMPT as c_long;
        }

        if config_ui.chk_file_must_exist.get_value() {
            style |= wx::FLP_FILE_MUST_EXIST as c_long;
        }

        if config_ui.chk_file_change_dir.get_value() {
            style |= wx::FLP_CHANGE_DIR as c_long;
        }

        if config_ui.chk_small.get_value() {
            style |= wx::FLP_SMALL as c_long;
        }

        if config_ui.radio_file_picker_mode.get_selection() == FilePickerMode::Open.into() {
            style |= wx::FLP_OPEN as c_long;
        } else {
            style |= wx::FLP_SAVE as c_long;
        }

        // FIXME: wxGetHomeDir() is needed?
        let file_picker = wx::FilePickerCtrl::builder(Some(&self.base))
            .id(PickerPage::File.into())
            .message("Hello!".into())
            .wildcard("*".into())
            .style(style)
            .build();

        *self.file_picker.borrow_mut() = Some(file_picker);
    }

    fn recreate_widget(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            config_ui.sizer.remove_int(1);
            self.create_picker(config_ui);

            if let Some(file_picker) = self.file_picker.borrow().as_ref() {
                config_ui.sizer.insert_window_int(
                    1,
                    Some(file_picker),
                    0,
                    wx::EXPAND | wx::ALL,
                    5,
                    wx::Object::none(),
                );
            }

            config_ui.sizer.layout();
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        config_ui.radio_file_picker_mode.set_selection(
            if (wx::FLP_DEFAULT_STYLE & wx::FLP_OPEN) != 0 {
                FilePickerMode::Open
            } else {
                FilePickerMode::Save
            }
            .into(),
        );

        config_ui
            .chk_file_text_ctrl
            .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_USE_TEXTCTRL) != 0);
        config_ui
            .chk_file_overwrite_prompt
            .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_USE_TEXTCTRL) != 0);
        config_ui
            .chk_file_must_exist
            .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_DIR_MUST_EXIST) != 0);
        config_ui
            .chk_file_change_dir
            .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_CHANGE_DIR) != 0);
        config_ui
            .chk_small
            .set_value((wx::FLP_DEFAULT_STYLE & wx::DIRP_SMALL) != 0);

        self.update_file_picker_mode(config_ui);
    }

    fn update_file_picker_mode(&self, config_ui: &ConfigUI) {
        if let Some(mode) = FilePickerMode::from(config_ui.radio_file_picker_mode.get_selection()) {
            match mode {
                FilePickerMode::Open => {
                    config_ui.chk_file_overwrite_prompt.set_value(false);
                    config_ui.chk_file_overwrite_prompt.disable();
                    config_ui.chk_file_must_exist.enable(true);
                }
                FilePickerMode::Save => {
                    config_ui.chk_file_must_exist.set_value(false);
                    config_ui.chk_file_must_exist.disable();
                    config_ui.chk_file_overwrite_prompt.enable(true);
                }
            }
        }
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_set_dir(&self, config_ui: &ConfigUI) {
        if let Some(file_picker) = self.file_picker.borrow().as_ref() {
            let dir = config_ui.text_initial_dir.get_value();
            file_picker.set_initial_directory(&dir);
        }
    }

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.recreate_widget();
    }

    fn on_check_box(&self) {
        self.recreate_widget();
    }

    fn on_radio_box(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.update_file_picker_mode(config_ui);
            self.recreate_widget();
        }
    }
}
