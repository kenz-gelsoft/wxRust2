use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

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
    // the checkboxes
    chk_allow_new: wx::CheckBox,
    chk_allow_edit: wx::CheckBox,
    chk_allow_delete: wx::CheckBox,
    chk_allow_no_reorder: wx::CheckBox,

    sizer_lbox: wx::BoxSizer,
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
            chk_allow_new,
            chk_allow_edit,
            chk_allow_delete,
            chk_allow_no_reorder,

            sizer_lbox: sizer_right, // save it to modify it later
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
            match m {
                EditableListboxPage::Reset => self.on_button_reset(config_ui),
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
        // Do nothing.
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
        config_ui.chk_allow_new.set_value(false);
        config_ui.chk_allow_edit.set_value(false);
        config_ui.chk_allow_delete.set_value(false);
        config_ui.chk_allow_no_reorder.set_value(false);
    }

    fn create_lbox(&self, config_ui: &ConfigUI) {
        let mut flags = wx::BORDER_DEFAULT;

        if config_ui.chk_allow_new.get_value() {
            flags |= wx::EL_ALLOW_NEW as c_long;
        }
        if config_ui.chk_allow_edit.get_value() {
            flags |= wx::EL_ALLOW_EDIT as c_long;
        }
        if config_ui.chk_allow_delete.get_value() {
            flags |= wx::EL_ALLOW_DELETE as c_long;
        }
        if config_ui.chk_allow_no_reorder.get_value() {
            flags |= wx::EL_NO_REORDER as c_long;
        }

        let items = wx::ArrayString::new();
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.get_strings(&items);
            config_ui.sizer_lbox.detach_window(Some(lbox));
            lbox.destroy();
        }

        let lbox = wx::EditableListBox::builder(Some(&self.base))
            .id(EditableListboxPage::Listbox.into())
            .label("Match these wildcards:")
            .style(flags)
            .build();
        lbox.set_strings(&items);

        config_ui.sizer_lbox.add_window_int(
            Some(&lbox),
            1,
            wx::GROW | wx::ALL,
            5,
            wx::Object::none(),
        );
        *self.lbox.borrow_mut() = Some(lbox);

        // relayout the sizer
        config_ui.sizer_lbox.layout();
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_lbox(config_ui);
    }

    fn on_check_box(&self, config_ui: &ConfigUI) {
        self.create_lbox(config_ui);
    }
}
