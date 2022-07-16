use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::{methods::*, ArrayString};

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

#[derive(Clone, Copy)]
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
    dir: wx::TextCtrl,
    path: wx::TextCtrl,
    filename: wx::TextCtrl,

    // flags
    chk_multiple: wx::CheckBox,
    chk_no_show_hidden: wx::CheckBox,

    radio_file_ctrl_mode: wx::RadioBox,

    // filters
    fltr: [wx::CheckBox; 3],

    // sizer
    sizer: wx::BoxSizer,
}

#[derive(Clone)]
pub struct FileCtrlWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    file_ctrl: Rc<RefCell<Option<wx::FileCtrl>>>,
}
impl WidgetsPage for FileCtrlWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "FileCtrl";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let sizer_left = wx::BoxSizer::new(wx::VERTICAL);

        let mut mode = wx::ArrayString::new();
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
            ),
            self.create_check_box_and_add_to_sizer(
                &sizer_filters,
                "C++ files (*.cpp; *.h)|*.cpp;*.h",
                wx::ID_ANY,
            ),
            self.create_check_box_and_add_to_sizer(
                &sizer_filters,
                "PNG images (*.png)|*.png",
                wx::ID_ANY,
            ),
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
            .style(wx::FC_OPEN)
            .build();

        // the 3 panes panes compose the window
        sizer_top.add_sizer_int(
            Some(&sizer_left),
            0,
            (wx::ALL & !wx::LEFT),
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
            dir,
            path,
            filename,
            chk_multiple,
            chk_no_show_hidden,
            radio_file_ctrl_mode,
            fltr,
            sizer: sizer_top,
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
                FileCtrlPage::Reset => self.on_button_reset(&config_ui),
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            // self.on_check_box(config_ui);
        }
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        // Do nothing.
    }
}
impl FileCtrlWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        FileCtrlWidgetsPage {
            base: panel,
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
        let mut flags = wx::BORDER_DEFAULT;

        // if config_ui.chk_allow_new.get_value() {
        //     flags |= wx::EL_ALLOW_NEW as c_long;
        // }
        // if config_ui.chk_allow_edit.get_value() {
        //     flags |= wx::EL_ALLOW_EDIT as c_long;
        // }
        // if config_ui.chk_allow_delete.get_value() {
        //     flags |= wx::EL_ALLOW_DELETE as c_long;
        // }
        // if config_ui.chk_allow_no_reorder.get_value() {
        //     flags |= wx::EL_NO_REORDER as c_long;
        // }

        // let items = wx::ArrayString::new();
        // if let Some(file_ctrl) = self.file_ctrl.borrow().as_ref() {
        //     // TODO: provide safe solution
        //     unsafe {
        //         file_ctrl.get_strings(items.as_ptr());
        //     }
        //     config_ui.sizer_file_ctrl.detach_window(Some(file_ctrl));
        //     file_ctrl.destroy();
        // }

        // let file_ctrl = wx::FileCtrl::builder(Some(&self.base))
        //     .id(FileCtrlPage::Listbox.into())
        //     .label("Match these wildcards:")
        //     .style(flags)
        //     .build();
        // file_ctrl.set_strings(&items);

        // config_ui.sizer_file_ctrl.add_window_int(
        //     Some(&file_ctrl),
        //     1,
        //     wx::GROW | wx::ALL,
        //     5,
        //     wx::Object::none(),
        // );
        // *self.file_ctrl.borrow_mut() = Some(file_ctrl);

        // // relayout the sizer
        // config_ui.sizer_file_ctrl.layout();
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_file_ctrl(config_ui);
    }

    fn on_check_box(&self, config_ui: &ConfigUI) {
        self.create_file_ctrl(config_ui);
    }
}
