use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

#[cfg(target_os = "windows")]
const FILE_SELECTOR_DEFAULT_WILDCARD_STR: &str = "*.*";

#[cfg(not(target_os = "windows"))]
const FILE_SELECTOR_DEFAULT_WILDCARD_STR: &str = "*";

// control ids
#[derive(Clone, Copy)]
enum DirCtrlPage {
    Reset = wx::ID_HIGHEST as isize,
    SetPath,
    Ctrl,
}
impl DirCtrlPage {
    fn from(v: c_int) -> Option<Self> {
        use DirCtrlPage::*;
        for e in [Reset, SetPath, Ctrl] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<DirCtrlPage> for c_int {
    fn from(w: DirCtrlPage) -> Self {
        w as c_int
    }
}

#[derive(Clone, Copy)]
enum StdPath {
    Unknown = 0,
    Config,
    Data,
    Documents,
    LocalData,
    Plugins,
    Resources,
    UserConfig,
    UserData,
    UserLocalData,
    Max,
}
impl StdPath {
    fn from(v: c_int) -> Option<Self> {
        use StdPath::*;
        for e in [
            Unknown,
            Config,
            Data,
            Documents,
            LocalData,
            Plugins,
            Resources,
            UserConfig,
            UserData,
            UserLocalData,
            Max,
        ] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<StdPath> for c_int {
    fn from(p: StdPath) -> Self {
        p as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // other controls
// --------------
// chk_dir_text_ctrl: wx::CheckBox,
// chk_dir_change_dir: wx::CheckBox,
// chk_dir_must_exist: wx::CheckBox,
// chk_small: wx::CheckBox,
// // text_initial_dir: wx::TextCtrl,
// sizer: wx::BoxSizer,
}

#[derive(Clone)]
pub struct DirCtrlWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    dir_ctrl: Rc<RefCell<Option<wx::GenericDirCtrl>>>,
}
impl WidgetsPage for DirCtrlWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "DirCtrl";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let box_left = wx::StaticBox::builder(Some(&self.base))
            .label("Dir control details")
            .build();

        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&box_left), wx::VERTICAL);

        let (sizer, path) = self.create_sizer_with_text_and_button(
            DirCtrlPage::SetPath.into(),
            "Set &path",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_int(
            Some(&sizer),
            0,
            wx::ALL | wx::ALIGN_RIGHT,
            5,
            wx::Object::none(),
        );

        let sizer_use_flags =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Flags");
        let chk_dir_only = self.create_check_box_and_add_to_sizer(
            &sizer_use_flags,
            "wxDIRCTRL_DIR_ONLY",
            wx::ID_ANY,
        );
        let chk_3d = self.create_check_box_and_add_to_sizer(
            &sizer_use_flags,
            "wxDIRCTRL_3D_INTERNAL",
            wx::ID_ANY,
        );
        let chk_first = self.create_check_box_and_add_to_sizer(
            &sizer_use_flags,
            "wxDIRCTRL_SELECT_FIRST",
            wx::ID_ANY,
        );
        let chk_filters = self.create_check_box_and_add_to_sizer(
            &sizer_use_flags,
            "wxDIRCTRL_SHOW_FILTERS",
            wx::ID_ANY,
        );
        let chk_labels = self.create_check_box_and_add_to_sizer(
            &sizer_use_flags,
            "wxDIRCTRL_EDIT_LABELS",
            wx::ID_ANY,
        );
        let chk_multi = self.create_check_box_and_add_to_sizer(
            &sizer_use_flags,
            "wxDIRCTRL_MULTIPLE",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_sizerflags(
            Some(&sizer_use_flags),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let sizer_filters =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Filters");
        let filters = [
            self.create_check_box_and_add_to_sizer(
                &sizer_filters,
                &format!(
                    "all files ({})|{}",
                    FILE_SELECTOR_DEFAULT_WILDCARD_STR, FILE_SELECTOR_DEFAULT_WILDCARD_STR
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
            .id(DirCtrlPage::Reset.into())
            .label("&Reset")
            .build();
        sizer_left.add_window_int(
            Some(&btn),
            0,
            wx::ALIGN_CENTRE_HORIZONTAL | wx::ALL,
            15,
            wx::Object::none(),
        );

        let std_paths = wx::ArrayString::new();
        for path in [
            "&none",
            "&config",
            "&data",
            "&documents",
            "&local data",
            "&plugins",
            "&resources",
            "&user config",
            "&user data",
            "&user local data",
        ] {
            std_paths.add(path);
        }

        // middle pane
        let radio_std_path = wx::RadioBox::builder(Some(&self.base))
            .label("Standard path")
            .choices(std_paths)
            .major_dimension(1)
            .build();

        // right pane
        let dir_ctrl = wx::GenericDirCtrl::builder(Some(&self.base))
            .id(DirCtrlPage::Ctrl.into())
            // wxDirDialogDefaultFolderStr
            .dir("/")
            .style(0)
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
            Some(&radio_std_path),
            0,
            wx::GROW | wx::ALL,
            10,
            wx::Object::none(),
        );
        sizer_top.add_window_int(
            Some(&dir_ctrl),
            1,
            wx::GROW | (wx::ALL & !wx::RIGHT),
            10,
            wx::Object::none(),
        );

        self.base.set_sizer(Some(&sizer_top), true);

        // final initializations
        let config_ui = ConfigUI {
            // chk_dir_text_ctrl,
            // chk_dir_change_dir,
            // chk_dir_must_exist,
            // chk_small,
            // // text_initial_dir,
            // sizer,
        };
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            DirCtrlPage::from(event.get_id()),
        ) {
            // match m {
            //     DirCtrlPage::Reset => self.on_button_reset(config_ui),
            //     DirCtrlPage::SetDir => self.on_button_set_dir(config_ui),
            //     _ => (),
            // };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        self.on_check_box();
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        // Do nothing
    }
}
impl DirCtrlWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        DirCtrlWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            dir_ctrl: Rc::new(RefCell::new(None)),
        }
    }

    fn recreate_widget(&self) {
        // if let Some(config_ui) = self.config_ui.borrow().as_ref() {
        //     config_ui.sizer.remove_int(1);
        //     self.create_dir_ctrl(config_ui);

        //     if let Some(dir_ctrl) = self.dir_ctrl.borrow().as_ref() {
        //         config_ui.sizer.insert_window_int(
        //             1,
        //             Some(dir_ctrl),
        //             0,
        //             wx::EXPAND | wx::ALL,
        //             5,
        //             wx::Object::none(),
        //         );
        //     }

        //     config_ui.sizer.layout();
        // }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        // config_ui
        //     .chk_dir_text_ctrl
        //     .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_USE_TEXTCTRL) != 0);
        // config_ui
        //     .chk_dir_must_exist
        //     .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_DIR_MUST_EXIST) != 0);
        // config_ui
        //     .chk_dir_change_dir
        //     .set_value((wx::DIRP_DEFAULT_STYLE & wx::DIRP_CHANGE_DIR) != 0);
        // config_ui
        //     .chk_small
        //     .set_value((wx::FLP_DEFAULT_STYLE & wx::DIRP_SMALL) != 0);
    }

    fn create_dir_ctrl(&self, config_ui: &ConfigUI) {
        // if let Some(dir_ctrl) = self.dir_ctrl.borrow().as_ref() {
        //     dir_ctrl.destroy();
        // }

        // let mut style = wx::BORDER_DEFAULT;

        // if config_ui.chk_dir_text_ctrl.get_value() {
        //     style |= wx::DIRP_USE_TEXTCTRL as c_long;
        // }

        // if config_ui.chk_dir_must_exist.get_value() {
        //     style |= wx::DIRP_DIR_MUST_EXIST as c_long;
        // }

        // if config_ui.chk_dir_change_dir.get_value() {
        //     style |= wx::DIRP_CHANGE_DIR as c_long;
        // }

        // if config_ui.chk_small.get_value() {
        //     style |= wx::DIRP_SMALL as c_long;
        // }

        // FIXME: wxGetHomeDir() is needed?
        // let dir_ctrl = wx::GenericDirCtrl::builder(Some(&self.base))
        //     .id(DirCtrlPage::Dir.into())
        //     .message("Hello!".into())
        //     .style(style)
        //     .build();

        // *self.dir_ctrl.borrow_mut() = Some(dir_ctrl);
    }

    fn on_button_set_dir(&self, config_ui: &ConfigUI) {
        if let Some(dir_ctrl) = self.dir_ctrl.borrow().as_ref() {
            // dir_ctrl.set_initial_directory(&config_ui.text_initial_dir.get_value());
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
