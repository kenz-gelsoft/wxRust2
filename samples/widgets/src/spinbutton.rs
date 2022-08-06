use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum SpinBtnPage {
    Reset = wx::ID_HIGHEST as isize,
    Clear,
    SetValue,
    SetMinAndMax,
    SetBase,
    SetIncrement,
    CurValueText,
    ValueText,
    MinText,
    MaxText,
    BaseText,
    SetIncrementText,
    SpinBtn,
    SpinCtrl,
    SpinCtrlDouble,
}
impl SpinBtnPage {
    fn from(v: c_int) -> Option<Self> {
        use SpinBtnPage::*;
        for e in [
            Reset,
            Clear,
            SetValue,
            SetMinAndMax,
            SetBase,
            SetIncrement,
            CurValueText,
            ValueText,
            MinText,
            MaxText,
            BaseText,
            SetIncrementText,
            SpinBtn,
            SpinCtrl,
            SpinCtrlDouble,
        ] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<SpinBtnPage> for c_int {
    fn from(w: SpinBtnPage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // the checkboxes
    chk_vert: wx::CheckBox,
    chk_arrow_keys: wx::CheckBox,
    chk_wrap: wx::CheckBox,
    chk_process_enter: wx::CheckBox,

    sizer_spin: wx::BoxSizer,
}

#[derive(Clone)]
pub struct SpinButtonWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    spinbtn: Rc<RefCell<Option<wx::SpinButton>>>,
    spinctrl: Rc<RefCell<Option<wx::SpinCtrl>>>,
    spinctrldbl: Rc<RefCell<Option<wx::SpinCtrlDouble>>>,

    min: RefCell<c_int>,
    max: RefCell<c_int>,
}
impl WidgetsPage for SpinButtonWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "Spin";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let s_box = wx::StaticBox::builder(Some(&self.base))
            .label("&Set style")
            .build();
        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box), wx::VERTICAL);

        let chk_vert = self.create_check_box_and_add_to_sizer(&sizer_left, "&Vertical", wx::ID_ANY);
        let chk_arrow_keys =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Arrow Keys", wx::ID_ANY);
        let chk_wrap = self.create_check_box_and_add_to_sizer(&sizer_left, "&Wrap", wx::ID_ANY);
        let chk_process_enter =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Process &Enter", wx::ID_ANY);

        sizer_left.add_int_int(5, 5, 0, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer

        let halign = wx::ArrayString::new();
        halign.add("left");
        halign.add("centre");
        halign.add("right");

        let radio_align = wx::RadioBox::builder(Some(&self.base))
            .label("&Text alignment")
            .choices(halign)
            .major_dimension(1)
            .build();
        sizer_left.add_window_int(
            Some(&radio_align),
            0,
            wx::GROW | wx::ALL,
            5,
            wx::Object::none(),
        );

        sizer_left.add_int_int(5, 5, 0, wx::GROW | wx::ALL, 5, wx::Object::none()); // spacer

        let btn = wx::Button::builder(Some(&self.base))
            .id(SpinBtnPage::Reset.into())
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
            .label("&Change spinbtn value")
            .build();
        let sizer_middle = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box2), wx::VERTICAL);

        let (sizer_row, text) = self
            .create_sizer_with_text_and_label("Current value", SpinBtnPage::CurValueText.into());
        text.set_editable(false);
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_value) = self.create_sizer_with_text_and_button(
            SpinBtnPage::SetValue.into(),
            "Set &value",
            SpinBtnPage::ValueText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_min) = self.create_sizer_with_text_and_button(
            SpinBtnPage::SetMinAndMax.into(),
            "&Min and max",
            SpinBtnPage::MinText.into(),
        );
        let text_max = wx::TextCtrl::builder(Some(&self.base))
            .id(SpinBtnPage::MaxText.into())
            .build();
        sizer_row.add_window_int(
            Some(&text_max),
            1,
            wx::LEFT | wx::ALIGN_CENTRE_VERTICAL,
            5,
            wx::Object::none(),
        );
        let (min, max) = (*self.min.borrow(), *self.max.borrow());
        text_min.set_value(&format!("{}", min));
        text_max.set_value(&format!("{}", max));
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_base) = self.create_sizer_with_text_and_button(
            SpinBtnPage::SetBase.into(),
            "Set &base",
            SpinBtnPage::BaseText.into(),
        );
        text_base.set_value("10");
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_increment) = self.create_sizer_with_text_and_button(
            SpinBtnPage::SetIncrement.into(),
            "Set Increment",
            SpinBtnPage::SetIncrementText.into(),
        );
        text_increment.set_value("1");
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        // right pane
        let sizer_right = wx::BoxSizer::new(wx::VERTICAL);
        sizer_right.set_min_size_int(150, 0);

        // the 3 panes panes compose the window
        sizer_top.add_sizer_int(
            Some(&sizer_left),
            0,
            wx::GROW | (wx::ALL & !wx::LEFT),
            10,
            wx::Object::none(),
        );
        sizer_top.add_sizer_int(
            Some(&sizer_middle),
            1,
            wx::GROW | wx::ALL,
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

        let config_ui = ConfigUI {
            chk_vert,
            chk_arrow_keys,
            chk_wrap,
            chk_process_enter,

            sizer_spin: sizer_right, // save it to modify it later
        };
        self.create_spin(&config_ui);

        // final initializations
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            SpinBtnPage::from(event.get_id()),
        ) {
            match m {
                SpinBtnPage::Reset => self.on_button_reset(config_ui),
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
impl SpinButtonWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        SpinButtonWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            spinbtn: Rc::new(RefCell::new(None)),
            spinctrl: Rc::new(RefCell::new(None)),
            spinctrldbl: Rc::new(RefCell::new(None)),
            min: RefCell::new(0),
            max: RefCell::new(0),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        config_ui.chk_vert.set_value(false);
        config_ui.chk_arrow_keys.set_value(false);
        config_ui.chk_wrap.set_value(false);
        config_ui.chk_process_enter.set_value(false);
    }

    fn create_spin(&self, config_ui: &ConfigUI) {
        let mut flags = wx::BORDER_DEFAULT;

        flags |= if config_ui.chk_vert.get_value() {
            wx::SP_VERTICAL
        } else {
            wx::SP_HORIZONTAL
        } as c_long;

        if config_ui.chk_arrow_keys.get_value() {
            flags |= wx::SP_ARROW_KEYS as c_long;
        }
        if config_ui.chk_wrap.get_value() {
            flags |= wx::SP_WRAP as c_long;
        }
        if config_ui.chk_process_enter.get_value() {
            flags |= wx::TE_PROCESS_ENTER as c_long;
        }

        // TODO: textFlags
        let mut val = *self.min.borrow();
        if let Some(spinbtn) = self.spinbtn.borrow().as_ref() {
            let val_old = spinbtn.get_value();
            if self.is_valid_value(val_old) {
                val = val_old;
            }
            config_ui.sizer_spin.clear(true);
        }

        let spinbtn = wx::SpinButton::builder(Some(&self.base))
            .id(SpinBtnPage::SpinBtn.into())
            .style(flags)
            .build();
        spinbtn.set_value(val);
        let (min, max) = (*self.min.borrow(), *self.max.borrow());
        spinbtn.set_range(min, max);

        let spinctrl = wx::SpinCtrl::builder(Some(&self.base))
            .id(SpinBtnPage::SpinCtrl.into())
            .value(&format!("{}", val))
            .style(flags)
            .min(min)
            .max(max)
            .initial(val)
            .build();

        let spinctrldbl = wx::SpinCtrlDouble::builder(Some(&self.base))
            .id(SpinBtnPage::SpinCtrlDouble.into())
            .value(&format!("{}", val))
            .style(flags)
            .min(min.into())
            .max(max.into())
            .initial(val.into())
            .inc(0.1)
            .build();

        // Add spacers, labels and spin controls to the sizer.
        let sizer_spin = &config_ui.sizer_spin;

        sizer_spin.add_int_int(0, 0, 1, 0, 0, wx::Object::none());
        sizer_spin.add_window_int(
            Some(
                &wx::StaticText::builder(Some(&self.base))
                    .label("wxSpinButton")
                    .build(),
            ),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        sizer_spin.add_window_int(
            Some(&spinbtn),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        *self.spinbtn.borrow_mut() = Some(spinbtn);

        sizer_spin.add_int_int(0, 0, 1, 0, 0, wx::Object::none());
        sizer_spin.add_window_int(
            Some(
                &wx::StaticText::builder(Some(&self.base))
                    .label("wxSpinCtrl")
                    .build(),
            ),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        sizer_spin.add_window_int(
            Some(&spinctrl),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        *self.spinctrl.borrow_mut() = Some(spinctrl);

        sizer_spin.add_int_int(0, 0, 1, 0, 0, wx::Object::none());
        sizer_spin.add_window_int(
            Some(
                &wx::StaticText::builder(Some(&self.base))
                    .label("wxSpinCtrlDouble")
                    .build(),
            ),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        sizer_spin.add_window_int(
            Some(&spinctrldbl),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        *self.spinctrldbl.borrow_mut() = Some(spinctrldbl);

        sizer_spin.add_int_int(0, 0, 1, 0, 0, wx::Object::none());

        config_ui.sizer_spin.layout();
    }

    // is this spinbtn value in range?
    fn is_valid_value(&self, val: c_int) -> bool {
        let min = *self.min.borrow();
        let max = *self.max.borrow();
        return (val >= min) && (val <= max);
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_spin(config_ui);
    }

    fn on_check_box(&self, config_ui: &ConfigUI) {
        self.create_spin(config_ui);
    }
}
