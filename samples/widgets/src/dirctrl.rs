use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

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
    // the text entries for command parameters
    path: wx::TextCtrl,

    radio_std_path: wx::RadioBox,

    // flags
    chk_dir_only: wx::CheckBox,
    chk_3d: wx::CheckBox,
    chk_first: wx::CheckBox,
    chk_filters: wx::CheckBox,
    chk_labels: wx::CheckBox,
    chk_multi: wx::CheckBox,

    // filters
    fltr: [wx::CheckBox; 3],

    // sizer
    sizer: wx::BoxSizer,
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
        *self.dir_ctrl.borrow_mut() = Some(dir_ctrl);

        self.base.set_sizer(Some(&sizer_top), true);

        // final initializations
        let config_ui = ConfigUI {
            path,
            radio_std_path,
            chk_dir_only,
            chk_3d,
            chk_first,
            chk_filters,
            chk_labels,
            chk_multi,
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
            DirCtrlPage::from(event.get_id()),
        ) {
            match m {
                DirCtrlPage::Reset => self.on_button_reset(config_ui),
                DirCtrlPage::SetPath => self.on_button_set_path(config_ui),
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        self.on_check_box();
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.on_radio_box(config_ui);
        }
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

    fn reset(&self, config_ui: &ConfigUI) {
        config_ui.path.clear();

        config_ui.chk_dir_only.set_value(false);
        config_ui.chk_3d.set_value(false);
        config_ui.chk_first.set_value(false);
        config_ui.chk_filters.set_value(false);
        config_ui.chk_labels.set_value(false);
        config_ui.chk_multi.set_value(false);

        config_ui.radio_std_path.set_selection(0);

        for fltr in config_ui.fltr.iter() {
            fltr.set_value(false);
        }

        self.create_dir_ctrl(config_ui, true);
    }

    fn create_dir_ctrl(&self, config_ui: &ConfigUI, default_path: bool) {
        let no_updates = wx::WindowUpdateLocker::new_with_window(Some(&self.base));

        let mut style = wx::BORDER_DEFAULT;
        if config_ui.chk_dir_only.is_checked() {
            style |= wx::DIRCTRL_DIR_ONLY as c_long;
        }
        if config_ui.chk_3d.is_checked() {
            style |= wx::DIRCTRL_3D_INTERNAL as c_long;
        }
        if config_ui.chk_first.is_checked() {
            style |= wx::DIRCTRL_SELECT_FIRST as c_long;
        }
        if config_ui.chk_filters.is_checked() {
            style |= wx::DIRCTRL_SHOW_FILTERS as c_long;
        }
        if config_ui.chk_labels.is_checked() {
            style |= wx::DIRCTRL_EDIT_LABELS as c_long;
        }
        if config_ui.chk_multi.is_checked() {
            style |= wx::DIRCTRL_MULTIPLE as c_long;
        }

        let mut path = "/".to_owned(); // wxDirDialogDefaultFolderStr
        if !default_path {
            if let Some(old_dir_ctrl) = self.dir_ctrl.borrow().as_ref() {
                path = old_dir_ctrl.get_path();
            }
        }
        let dir_ctrl = wx::GenericDirCtrl::builder(Some(&self.base))
            .id(DirCtrlPage::Ctrl.into())
            .dir(&path)
            .style(style)
            .build();

        let mut filter = String::new();
        for fltr in config_ui.fltr.iter() {
            if fltr.is_checked() {
                if !filter.is_empty() {
                    filter.push_str("|");
                }
                filter.push_str(&fltr.get_label());
            }
        }
        dir_ctrl.set_filter(&filter);

        // update sizer's child window
        if let Some(old_dir_ctrl) = self.dir_ctrl.borrow().as_ref() {
            config_ui
                .sizer
                .replace_window(Some(old_dir_ctrl), Some(&dir_ctrl), false);

            old_dir_ctrl.destroy();
        }
        // update our pointer
        *self.dir_ctrl.borrow_mut() = Some(dir_ctrl);

        // relayout the sizer
        config_ui.sizer.layout();
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_set_path(&self, config_ui: &ConfigUI) {
        if let Some(dir_ctrl) = self.dir_ctrl.borrow().as_ref() {
            dir_ctrl.set_path(&config_ui.path.get_value());
        }
    }

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_dir_ctrl(config_ui, false);
    }

    fn on_check_box(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.create_dir_ctrl(config_ui, false);
        }
    }

    fn on_radio_box(&self, config_ui: &ConfigUI) {
        if let (Some(std_path), Some(dir_ctrl)) = (
            StdPath::from(config_ui.radio_std_path.get_selection()),
            self.dir_ctrl.borrow().as_ref(),
        ) {
            let stdp = wx::StandardPaths::get();

            let path = match std_path {
                StdPath::Config => stdp.get_config_dir(),
                StdPath::Data => stdp.get_data_dir(),
                StdPath::Documents => stdp.get_documents_dir(),
                StdPath::LocalData => stdp.get_local_data_dir(),
                StdPath::Plugins => stdp.get_plugins_dir(),
                StdPath::Resources => stdp.get_resources_dir(),
                StdPath::UserConfig => stdp.get_user_config_dir(),
                StdPath::UserData => stdp.get_user_data_dir(),
                StdPath::UserLocalData => stdp.get_user_local_data_dir(),
                _ => "".to_owned(),
            };
            dir_ctrl.set_path(&path);
        }
    }
}
