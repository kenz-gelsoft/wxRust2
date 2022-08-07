// wxWidgets Sample (partially/incompletely) ported in Rust.
// Ported by:   KENZ<KENZ.gelsoft@gmail.com>
// Original C++ Version's Copyright is:
/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        dirpicker.cpp
// Purpose:     Shows wxDirPickerCtrl
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
    Dir,
    SetDir,
}
impl PickerPage {
    fn from(v: c_int) -> Option<Self> {
        use PickerPage::*;
        for e in [Reset, Dir, SetDir] {
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
    chk_dir_text_ctrl: wx::CheckBox,
    chk_dir_change_dir: wx::CheckBox,
    chk_dir_must_exist: wx::CheckBox,
    chk_small: wx::CheckBox,
    text_initial_dir: wx::TextCtrl,

    sizer: wx::BoxSizer,
}

#[derive(Clone)]
pub struct DirPickerWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the picker
    dir_picker: Rc<RefCell<Option<wx::DirPickerCtrl>>>,
}
impl WidgetsPage for DirPickerWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "DirPicker";
    }
    fn create_content(&self) {
        // left pane
        let boxleft = wx::BoxSizer::new(wx::VERTICAL);

        let dirbox =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&DirPicker style");
        let chk_dir_text_ctrl =
            self.create_check_box_and_add_to_sizer(&dirbox, "With textctrl", wx::ID_ANY);
        let chk_dir_must_exist =
            self.create_check_box_and_add_to_sizer(&dirbox, "Dir must exist", wx::ID_ANY);
        let chk_dir_change_dir =
            self.create_check_box_and_add_to_sizer(&dirbox, "Change working dir", wx::ID_ANY);
        let chk_small =
            self.create_check_box_and_add_to_sizer(&dirbox, "&Small version", wx::ID_ANY);
        boxleft.add_sizer_int(Some(&dirbox), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

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
            chk_dir_text_ctrl,
            chk_dir_change_dir,
            chk_dir_must_exist,
            chk_small,
            text_initial_dir,

            sizer,
        };
        self.reset(&config_ui); // set checkboxes state

        // create pickers
        self.create_picker(&config_ui);

        // right pane
        config_ui
            .sizer
            .add_int_int(1, 1, 1, wx::GROW | wx::ALL, 5, wx::Object::none());
        // TODO: insert picker in create_picker()
        if let Some(dir_picker) = self.dir_picker.borrow().as_ref() {
            config_ui.sizer.add_window_int(
                Some(dir_picker),
                0,
                wx::EXPAND | wx::ALL,
                5,
                wx::Object::none(),
            );
        }
        config_ui
            .sizer
            .add_int_int(1, 1, 1, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer

        // global pane
        let sz = wx::BoxSizer::new(wx::HORIZONTAL);
        sz.add_sizer_int(Some(&boxleft), 0, wx::GROW | wx::ALL, 5, wx::Object::none());
        sz.add_sizer_int(
            Some(&config_ui.sizer),
            1,
            wx::GROW | wx::ALL,
            5,
            wx::Object::none(),
        );
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
        // Do nothing
    }
}
impl DirPickerWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        DirPickerWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            dir_picker: Rc::new(RefCell::new(None)),
        }
    }

    fn recreate_widget(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            config_ui.sizer.remove_int(1);
            self.create_picker(config_ui);

            if let Some(dir_picker) = self.dir_picker.borrow().as_ref() {
                config_ui.sizer.insert_window_int(
                    1,
                    Some(dir_picker),
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
        config_ui
            .chk_dir_text_ctrl
            .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_USE_TEXTCTRL) != 0);
        config_ui
            .chk_dir_must_exist
            .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_DIR_MUST_EXIST) != 0);
        config_ui
            .chk_dir_change_dir
            .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_CHANGE_DIR) != 0);
        config_ui
            .chk_small
            .set_value((wx::FLP_DEFAULT_STYLE & wx::DIRP_SMALL) != 0);
    }

    fn create_picker(&self, config_ui: &ConfigUI) {
        if let Some(dir_picker) = self.dir_picker.borrow().as_ref() {
            dir_picker.destroy();
        }

        let mut style = wx::BORDER_DEFAULT;

        if config_ui.chk_dir_text_ctrl.get_value() {
            style |= wx::DIRP_USE_TEXTCTRL as c_long;
        }

        if config_ui.chk_dir_must_exist.get_value() {
            style |= wx::DIRP_DIR_MUST_EXIST as c_long;
        }

        if config_ui.chk_dir_change_dir.get_value() {
            style |= wx::DIRP_CHANGE_DIR as c_long;
        }

        if config_ui.chk_small.get_value() {
            style |= wx::DIRP_SMALL as c_long;
        }

        // FIXME: wxGetHomeDir() is needed?
        let dir_picker = wx::DirPickerCtrl::builder(Some(&self.base))
            .id(PickerPage::Dir.into())
            .message("Hello!".into())
            .style(style)
            .build();

        *self.dir_picker.borrow_mut() = Some(dir_picker);
    }

    fn on_button_set_dir(&self, config_ui: &ConfigUI) {
        if let Some(dir_picker) = self.dir_picker.borrow().as_ref() {
            dir_picker.set_initial_directory(&config_ui.text_initial_dir.get_value());
        }
    }

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);
        self.recreate_widget();
    }

    fn on_check_box(&self) {
        self.recreate_widget();
    }
}
