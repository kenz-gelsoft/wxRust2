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
enum EditableListboxPage {
    Reset = wx::ID_HIGHEST as isize,
    Listbox,
    ContainerTests,
}
impl EditableListboxPage {
    fn from(v: c_int) -> Option<Self> {
        use EditableListboxPage::*;
        for e in [Reset, Listbox, ContainerTests] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<EditableListboxPage> for c_int {
    fn from(w: EditableListboxPage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // // the text entries for command parameters
    // path: wx::TextCtrl,

    // radio_std_path: wx::RadioBox,

    // // flags
    // chk_dir_only: wx::CheckBox,
    // chk_3d: wx::CheckBox,
    // chk_first: wx::CheckBox,
    // chk_filters: wx::CheckBox,
    // chk_labels: wx::CheckBox,
    // chk_multi: wx::CheckBox,

    // // filters
    // fltr: [wx::CheckBox; 3],

    // // sizer
    // sizer: wx::BoxSizer,
}

#[derive(Clone)]
pub struct EditableListboxWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    lbox: Rc<RefCell<Option<wx::EditableListBox>>>,
}
impl WidgetsPage for EditableListboxWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "EditableListbox";
    }
    fn create_content(&self) {
        /*
        What we create here is a frame having 2 panes: style pane is the
        leftmost one and the pane containing the listbox itself to the right
        */
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let box_left = wx::StaticBox::builder(Some(&self.base))
            .label("&Set listbox parameters")
            .build();
        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&box_left), wx::VERTICAL);

        let chk_allow_new =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Allow new items", wx::ID_ANY);
        let chk_allow_edit =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Allow editing items", wx::ID_ANY);
        let chk_allow_delete =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Allow deleting items", wx::ID_ANY);
        let chk_allow_no_reorder = self.create_check_box_and_add_to_sizer(
            &sizer_left,
            "Block user reordering",
            wx::ID_ANY,
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(EditableListboxPage::Reset.into())
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
        let sizer_right = wx::BoxSizer::new(wx::VERTICAL);
        let lbox = wx::EditableListBox::builder(Some(&self.base))
            .id(EditableListboxPage::Listbox.into())
            .style(0)
            .build();
        sizer_right.add_window_int(Some(&lbox), 1, wx::GROW | wx::ALL, 5, wx::Object::none());
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
            Some(&sizer_right),
            1,
            wx::GROW | (wx::ALL & !wx::RIGHT),
            10,
            wx::Object::none(),
        );

        // final initializations
        let config_ui = ConfigUI {
            // path,
            // radio_std_path,
            // chk_dir_only,
            // chk_3d,
            // chk_first,
            // chk_filters,
            // chk_labels,
            // chk_multi,
            // fltr,
            // sizer: sizer_top,
        };
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            EditableListboxPage::from(event.get_id()),
        ) {
            // match m {
            //     EditableListboxPage::Reset => self.on_button_reset(&config_ui),
            //     EditableListboxPage::SetPath => self.on_button_set_path(&config_ui),
            //     _ => (),
            // };
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
impl EditableListboxWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        EditableListboxWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            lbox: Rc::new(RefCell::new(None)),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        // config_ui.path.clear();

        // config_ui.chk_dir_only.set_value(false);
        // config_ui.chk_3d.set_value(false);
        // config_ui.chk_first.set_value(false);
        // config_ui.chk_filters.set_value(false);
        // config_ui.chk_labels.set_value(false);
        // config_ui.chk_multi.set_value(false);

        // config_ui.radio_std_path.set_selection(0);

        // self.create_lbox(config_ui, true);
    }

    fn create_lbox(&self, config_ui: &ConfigUI, default_path: bool) {
        let no_updates = wx::WindowUpdateLocker::new_with_window(Some(&self.base));

        let mut style = wx::BORDER_DEFAULT;
        // if config_ui.chk_dir_only.is_checked() {
        //     style |= wx::DIRCTRL_DIR_ONLY as c_long;
        // }
        // if config_ui.chk_3d.is_checked() {
        //     style |= wx::DIRCTRL_3D_INTERNAL as c_long;
        // }
        // if config_ui.chk_first.is_checked() {
        //     style |= wx::DIRCTRL_SELECT_FIRST as c_long;
        // }
        // if config_ui.chk_filters.is_checked() {
        //     style |= wx::DIRCTRL_SHOW_FILTERS as c_long;
        // }
        // if config_ui.chk_labels.is_checked() {
        //     style |= wx::DIRCTRL_EDIT_LABELS as c_long;
        // }
        // if config_ui.chk_multi.is_checked() {
        //     style |= wx::DIRCTRL_MULTIPLE as c_long;
        // }
        let lbox = wx::EditableListBox::builder(Some(&self.base))
            .id(EditableListboxPage::Listbox.into())
            .style(style)
            .build();

        // update sizer's child window
        if let Some(old_lbox) = self.lbox.borrow().as_ref() {
            // config_ui
            //     .sizer
            //     .replace_window(Some(old_lbox), Some(&lbox), false);

            old_lbox.destroy();
        }
        // update our pointer
        *self.lbox.borrow_mut() = Some(lbox);

        // relayout the sizer
        // config_ui.sizer.layout();
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_set_path(&self, config_ui: &ConfigUI) {
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            // lbox.set_path(&config_ui.path.get_value());
        }
    }

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_lbox(config_ui, false);
    }

    fn on_check_box(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.create_lbox(config_ui, false);
        }
    }

    fn on_radio_box(&self, config_ui: &ConfigUI) {
        // do nothing
    }
}
