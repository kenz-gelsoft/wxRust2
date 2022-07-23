use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum ListboxPage {
    Reset = wx::ID_HIGHEST as isize,
    Add,
    AddText,
    AddSeveral,
    AddMany,
    Clear,
    Change,
    ChangeText,
    Delete,
    DeleteText,
    DeleteSel,
    Listbox,
    EnsureVisible,
    EnsureVisibleText,
    ContainerTests,
    GetTopItem,
    GetCountPerPage,
    MoveUp,
    MoveDown,
}
impl ListboxPage {
    fn from(v: c_int) -> Option<Self> {
        use ListboxPage::*;
        for e in [
            Reset,
            Add,
            AddText,
            AddSeveral,
            AddMany,
            Clear,
            Change,
            ChangeText,
            Delete,
            DeleteText,
            DeleteSel,
            Listbox,
            EnsureVisible,
            EnsureVisibleText,
            ContainerTests,
            GetTopItem,
            GetCountPerPage,
            MoveUp,
            MoveDown,
        ] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<ListboxPage> for c_int {
    fn from(w: ListboxPage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // the controls
    // ------------
    // the checkboxes
    chk_sort: wx::CheckBox,

    // sizer
    sizer_lbox: wx::BoxSizer,

    // the text entries for "Add/change string" and "Delete" buttons
    text_add: wx::TextCtrl,
    text_change: wx::TextCtrl,
    text_delete: wx::TextCtrl,
}

#[derive(Clone)]
pub struct ListboxWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the lbox itself
    lbox: Rc<RefCell<Option<wx::ListBox>>>,

    s_item: RefCell<c_int>,
}
impl WidgetsPage for ListboxWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "ListBox";
    }
    fn create_content(&self) {
        /*
            What we create here is a frame having 3 panes: style pane is the
            leftmost one, in the middle the pane with buttons allowing to perform
            miscellaneous listbox operations and the pane containing the listbox
            itself to the right
        */
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let s_box = wx::StaticBox::builder(Some(&self.base))
            .label("&Set lbox parameters")
            .build();

        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box), wx::VERTICAL);

        let chk_sort =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Sort items", wx::ID_ANY);

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Reset.into())
            .label("&Reset")
            .build();
        sizer_left.add_window_int(
            Some(&btn),
            0,
            wx::ALIGN_CENTRE_HORIZONTAL | wx::ALL,
            15,
            wx::Object::none(),
        );

        // middle pane
        let s_box2 = wx::StaticBox::builder(Some(&self.base))
            .label("&Change lbox contents")
            .build();
        let sizer_middle = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box2), wx::VERTICAL);

        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Add.into())
            .label("&Add this string")
            .build();
        let text_add = wx::TextCtrl::builder(Some(&self.base))
            .id(ListboxPage::AddText.into())
            .value("test item 0")
            .build();
        sizer_row.add_window_int(Some(&btn), 0, wx::RIGHT, 5, wx::Object::none());
        sizer_row.add_window_int(Some(&text_add), 1, wx::LEFT, 5, wx::Object::none());
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::AddSeveral.into())
            .label("&Insert a few strings")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::AddMany.into())
            .label("Add &many strings")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Change.into())
            .label("C&hange current")
            .build();
        let text_change = wx::TextCtrl::builder(Some(&self.base))
            .id(ListboxPage::ChangeText.into())
            .build();
        sizer_row.add_window_int(Some(&btn), 0, wx::RIGHT, 5, wx::Object::none());
        sizer_row.add_window_int(Some(&text_change), 1, wx::LEFT, 5, wx::Object::none());
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Delete.into())
            .label("&Delete this item")
            .build();
        let text_delete = wx::TextCtrl::builder(Some(&self.base))
            .id(ListboxPage::DeleteText.into())
            .build();
        sizer_row.add_window_int(Some(&btn), 0, wx::RIGHT, 5, wx::Object::none());
        sizer_row.add_window_int(Some(&text_delete), 1, wx::LEFT, 5, wx::Object::none());
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::DeleteSel.into())
            .label("Delete &selection")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::Clear.into())
            .label("&Clear")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(ListboxPage::ContainerTests.into())
            .label("Run &tests")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        // right pane
        let sizer_right = wx::BoxSizer::new(wx::VERTICAL);
        let lbox = wx::ListBox::builder(Some(&self.base))
            .id(ListboxPage::Listbox.into())
            .build();
        sizer_right.add_window_int(Some(&lbox), 0, wx::ALL | wx::GROW, 5, wx::Object::none());
        sizer_right.set_min_size_int(150, 0);
        *self.lbox.borrow_mut() = Some(lbox);

        // the 3 panes panes compose the window
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_left),
            wx::SizerFlags::new(0)
                .expand()
                .double_border(wx::ALL & !wx::LEFT),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_middle),
            wx::SizerFlags::new(1).expand().double_border(wx::ALL),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_right),
            wx::SizerFlags::new(1)
                .expand()
                .double_border(wx::ALL & !wx::RIGHT),
        );
        *self.config_ui.borrow_mut() = Some(ConfigUI {
            chk_sort,

            sizer_lbox: sizer_right, // save it to modify it later

            text_add,
            text_change,
            text_delete,
        });

        // do create the main control
        self.reset();
        self.create_lbox();

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(m), Some(config_ui)) = (
            ListboxPage::from(event.get_id()),
            self.config_ui.borrow().as_ref(),
        ) {
            match m {
                ListboxPage::Reset => self.on_button_reset(),
                ListboxPage::Change => self.on_button_change(config_ui),
                ListboxPage::Delete => self.on_button_delete(config_ui),
                ListboxPage::DeleteSel => self.on_button_delete_sel(),
                ListboxPage::Clear => self.on_button_clear(),
                ListboxPage::Add => self.on_button_add(config_ui),
                ListboxPage::AddSeveral => self.on_button_add_several(),
                ListboxPage::AddMany => self.on_button_add_many(),
                // TODO: Support update ui event to disable this when not 3state
                _ => (),
            };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        self.on_check_or_radio_box();
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        self.on_check_or_radio_box();
    }
}
impl ListboxWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        ListboxWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            lbox: Rc::new(RefCell::new(None)),
            s_item: RefCell::new(1),
        }
    }

    fn reset(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            config_ui.chk_sort.set_value(false);
        }
    }

    fn create_lbox(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            let mut flags = wx::BORDER_DEFAULT;

            if config_ui.chk_sort.is_checked() {
                flags |= wx::CB_SORT as c_long;
            }

            let items = wx::ArrayString::new();
            if let Some(lbox) = self.lbox.borrow().as_ref() {
                // TODO: remove (and delete) all lboxes
                let count = lbox.get_count();
                for n in 0..count {
                    items.add(&lbox.get_string(n));
                }

                config_ui.sizer_lbox.detach_window(Some(lbox));
                lbox.destroy();
            }

            let new_lbox = wx::ListBox::builder(Some(&self.base))
                .id(ListboxPage::Listbox.into())
                .style(flags)
                .build();
            new_lbox.set_arraystring(&items);

            let sizer_lbox = &config_ui.sizer_lbox;
            sizer_lbox.add_window_int(
                Some(&new_lbox),
                0,
                wx::GROW | wx::ALL,
                5,
                wx::Object::none(),
            );
            sizer_lbox.layout();

            *self.lbox.borrow_mut() = Some(new_lbox);
        }
    }

    fn on_button_reset(&self) {
        self.reset();
        self.create_lbox();
    }

    fn on_button_change(&self, config_ui: &ConfigUI) {
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            let selection = lbox.get_selection();
            if selection != wx::NOT_FOUND {
                lbox.set_string(
                    selection.try_into().unwrap(),
                    &config_ui.text_change.get_value(),
                );
            }
        }
    }

    fn on_button_delete(&self, config_ui: &ConfigUI) {
        let n = config_ui.text_delete.get_value();
        if let (Ok(n), Some(lbox)) = (n.parse(), self.lbox.borrow().as_ref()) {
            if n < lbox.get_count() {
                lbox.delete(n);
            }
        }
    }

    fn on_button_delete_sel(&self) {
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            let selection = lbox.get_selection();
            if selection != wx::NOT_FOUND {
                lbox.delete(selection.try_into().unwrap());
            }
        }
    }

    fn on_button_clear(&self) {
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.clear();
        }
    }

    fn on_button_add(&self, config_ui: &ConfigUI) {
        let s = config_ui.text_add.get_value();
        if !config_ui.text_add.is_modified() {
            // update the default string
            let s_item = *self.s_item.borrow();
            config_ui
                .text_add
                .set_value(&format!("test item {}", s_item));
            *self.s_item.borrow_mut() = s_item + 1;
        }

        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.append_str(&s);
        }
    }

    fn on_button_add_many(&self) {
        // "many" means 1000 here
        let strings = wx::ArrayString::new();
        for n in 0..1000 {
            strings.add(&format!("item #{}", n));
        }
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.append_arraystring(&strings);
        }
    }

    fn on_button_add_several(&self) {
        let items = wx::ArrayString::new();
        items.add("First");
        items.add("another one");
        items.add("and the last (very very very very very very very very very very long) one");
        if let Some(lbox) = self.lbox.borrow().as_ref() {
            lbox.insert_arraystring(&items, 0);
        }
    }

    fn on_check_or_radio_box(&self) {
        self.create_lbox();
    }
}
