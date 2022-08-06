use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum TimePickerPage {
    Reset = wx::ID_HIGHEST as isize,
    Set,
    Picker,
}
impl TimePickerPage {
    fn from(v: c_int) -> Option<Self> {
        use TimePickerPage::*;
        for e in [Reset, Set, Picker] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<TimePickerPage> for c_int {
    fn from(w: TimePickerPage) -> Self {
        w as c_int
    }
}

#[derive(Clone, Copy)]
enum Align {
    Left,
    Centre,
    Right,
}
impl Align {
    fn from(v: c_int) -> Option<Self> {
        use Align::*;
        for e in [Left, Centre, Right] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<Align> for c_int {
    fn from(v: Align) -> Self {
        v as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // // the check/radio boxes for styles
    // chk_vert: wx::CheckBox,
    // chk_arrow_keys: wx::CheckBox,
    // chk_wrap: wx::CheckBox,
    // chk_process_enter: wx::CheckBox,

    // radio_align: wx::RadioBox,

    // // the text entries for set value/range
    // text_value: wx::TextCtrl,
    // text_min: wx::TextCtrl,
    // text_max: wx::TextCtrl,
    // text_base: wx::TextCtrl,
    // text_increment: wx::TextCtrl,
    sizer_time_picker: wx::BoxSizer,
}

#[derive(Clone)]
pub struct TimePickerWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,

    // the spinbtn and the spinctrl
    spinbtn: Rc<RefCell<Option<wx::SpinButton>>>,
    spinctrl: Rc<RefCell<Option<wx::SpinCtrl>>>,
    spinctrldbl: Rc<RefCell<Option<wx::SpinCtrlDouble>>>,

    // the spinbtn range
    min: RefCell<c_int>,
    max: RefCell<c_int>,

    // and numeric base
    base_val: RefCell<c_int>,

    // the increment
    increment: RefCell<c_int>,
}
impl WidgetsPage for TimePickerWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "TimePicker";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let sizer_left = wx::BoxSizer::new(wx::VERTICAL);
        sizer_left.add_window_sizerflags(
            Some(
                &wx::Button::builder(Some(&self.base))
                    .id(TimePickerPage::Reset.into())
                    .label("&Reset")
                    .build(),
            ),
            wx::SizerFlags::new(0).centre().border(wx::ALL),
        );

        // middle pane
        let sizer_middle = wx::BoxSizer::new(wx::VERTICAL);
        let (sizer_row, text_cur) = self.create_sizer_with_text_and_button(
            TimePickerPage::Set.into(),
            "&Set time",
            wx::ID_ANY,
        );
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        text_cur.set_min_size(&wx::Size::new_with_int(
            self.base.get_text_extent("  99:99:99  ").get_width(),
            -1,
        ));

        // right pane: control itself
        let sizer_right = wx::BoxSizer::new(wx::HORIZONTAL);

        let time_picker = wx::TimePickerCtrl::builder(Some(&self.base))
            .id(TimePickerPage::Picker.into())
            .build();

        sizer_right.add_int_int(0, 0, 1, wx::CENTRE as c_int, 0, wx::Object::none());
        sizer_right.add_window_int(
            Some(&time_picker),
            1,
            wx::CENTRE as c_int,
            0,
            wx::Object::none(),
        );
        sizer_right.add_int_int(0, 0, 1, wx::CENTRE as c_int, 0, wx::Object::none());

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
            0,
            wx::TOP | wx::BOTTOM,
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
            // chk_vert,
            // chk_arrow_keys,
            // chk_wrap,
            // chk_process_enter,

            // radio_align,

            // text_value,
            // text_min,
            // text_max,
            // text_base,
            // text_increment,
            sizer_time_picker: sizer_right, // save it to modify it later
        };

        // final initializations
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            TimePickerPage::from(event.get_id()),
        ) {
            match m {
                TimePickerPage::Reset => self.on_button_reset(config_ui),
                // TimePickerPage::SetValue => self.on_button_set_value(config_ui),
                // TimePickerPage::SetMinAndMax => self.on_button_set_min_and_max(config_ui),
                // TimePickerPage::SetBase => self.on_button_set_base(config_ui),
                // TimePickerPage::SetIncrement => self.on_button_set_increment(config_ui),
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
impl TimePickerWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        TimePickerWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            spinbtn: Rc::new(RefCell::new(None)),
            spinctrl: Rc::new(RefCell::new(None)),
            spinctrldbl: Rc::new(RefCell::new(None)),

            min: RefCell::new(0),
            max: RefCell::new(10),

            base_val: RefCell::new(10),
            increment: RefCell::new(1),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        // config_ui.chk_vert.set_value(true);
        // config_ui.chk_arrow_keys.set_value(true);
        // config_ui.chk_wrap.set_value(false);
        // config_ui.chk_process_enter.set_value(false);
        // config_ui.radio_align.set_selection(Align::Right.into());
    }

    fn create_spin(&self, config_ui: &ConfigUI) {
        let mut flags = wx::BORDER_DEFAULT;

        // flags |= if config_ui.chk_vert.get_value() {
        //     wx::SP_VERTICAL
        // } else {
        //     wx::SP_HORIZONTAL
        // } as c_long;

        // if config_ui.chk_arrow_keys.get_value() {
        //     flags |= wx::SP_ARROW_KEYS as c_long;
        // }
        // if config_ui.chk_wrap.get_value() {
        //     flags |= wx::SP_WRAP as c_long;
        // }
        // if config_ui.chk_process_enter.get_value() {
        //     flags |= wx::TE_PROCESS_ENTER as c_long;
        // }

        let mut text_flags = 0;
        // if let Some(align) = Align::from(config_ui.radio_align.get_selection()) {
        //     text_flags = match align {
        //         Align::Centre => wx::ALIGN_CENTRE_HORIZONTAL,
        //         Align::Right => wx::ALIGN_RIGHT,
        //         _ => wx::ALIGN_LEFT, // no-op
        //     } as c_long;
        // }

        let mut val = *self.min.borrow();
        if let Some(spinbtn) = self.spinbtn.borrow().as_ref() {
            let val_old = spinbtn.get_value();
            if self.is_valid_value(val_old) {
                val = val_old;
            }
            config_ui.sizer_time_picker.clear(true);
        }

        let spinbtn = wx::SpinButton::builder(Some(&self.base))
            // .id(TimePickerPage::SpinBtn.into())
            .style(flags | text_flags)
            .build();
        spinbtn.set_value(val);
        let (min, max) = (*self.min.borrow(), *self.max.borrow());
        spinbtn.set_range(min, max);

        let spinctrl = wx::SpinCtrl::builder(Some(&self.base))
            // .id(TimePickerPage::SpinCtrl.into())
            .value(&format!("{}", val))
            .style(flags | text_flags)
            .min(min)
            .max(max)
            .initial(val)
            .build();

        let spinctrldbl = wx::SpinCtrlDouble::builder(Some(&self.base))
            // .id(TimePickerPage::SpinCtrlDouble.into())
            .value(&format!("{}", val))
            .style(flags | text_flags)
            .min(min.into())
            .max(max.into())
            .initial(val.into())
            .inc(0.1)
            .build();

        // Add spacers, labels and spin controls to the sizer.
        let sizer_time_picker = &config_ui.sizer_time_picker;

        sizer_time_picker.add_int_int(0, 0, 1, 0, 0, wx::Object::none());
        sizer_time_picker.add_window_int(
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
        sizer_time_picker.add_window_int(
            Some(&spinbtn),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        *self.spinbtn.borrow_mut() = Some(spinbtn);

        sizer_time_picker.add_int_int(0, 0, 1, 0, 0, wx::Object::none());
        sizer_time_picker.add_window_int(
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
        sizer_time_picker.add_window_int(
            Some(&spinctrl),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        *self.spinctrl.borrow_mut() = Some(spinctrl);

        sizer_time_picker.add_int_int(0, 0, 1, 0, 0, wx::Object::none());
        sizer_time_picker.add_window_int(
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
        sizer_time_picker.add_window_int(
            Some(&spinctrldbl),
            0,
            wx::ALIGN_CENTRE | wx::ALL,
            5,
            wx::Object::none(),
        );
        *self.spinctrldbl.borrow_mut() = Some(spinctrldbl);

        sizer_time_picker.add_int_int(0, 0, 1, 0, 0, wx::Object::none());

        config_ui.sizer_time_picker.layout();
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

    fn on_button_set_min_and_max(&self, config_ui: &ConfigUI) {
        // let min_new = config_ui.text_min.get_value();
        // let max_new = config_ui.text_max.get_value();
        // if let (Ok(min_new), Ok(max_new), Some(spinbtn), Some(spinctrl), Some(spinctrldbl)) = (
        //     min_new.parse(),
        //     max_new.parse(),
        //     self.spinbtn.borrow().as_ref(),
        //     self.spinctrl.borrow().as_ref(),
        //     self.spinctrldbl.borrow().as_ref(),
        // ) {
        //     if min_new > max_new {
        //         return;
        //     }

        //     *self.min.borrow_mut() = min_new;
        //     *self.max.borrow_mut() = max_new;

        //     spinbtn.set_range(min_new, max_new);
        //     spinctrl.set_range(min_new, max_new);
        //     spinctrldbl.set_range(min_new.into(), max_new.into());

        //     config_ui.sizer_time_picker.layout();
        // }
    }

    fn on_button_set_base(&self, config_ui: &ConfigUI) {
        // let base = config_ui.text_base.get_value();
        // if let (Ok(base), Some(spinctrl)) = (base.parse(), self.spinctrl.borrow().as_ref()) {
        //     *self.base_val.borrow_mut() = base;

        //     spinctrl.set_base(base);

        //     config_ui.sizer_time_picker.layout();
        // }
    }

    fn on_button_set_increment(&self, config_ui: &ConfigUI) {
        // let increment = config_ui.text_increment.get_value();
        // if let (Ok(increment), Some(spinctrl)) =
        //     (increment.parse(), self.spinctrl.borrow().as_ref())
        // {
        //     *self.increment.borrow_mut() = increment;

        //     spinctrl.set_increment(increment);

        //     config_ui.sizer_time_picker.layout();
        // }
    }

    fn on_button_set_value(&self, config_ui: &ConfigUI) {
        if let (Some(spinbtn), Some(spinctrl), Some(spinctrldbl)) = (
            self.spinbtn.borrow().as_ref(),
            self.spinctrl.borrow().as_ref(),
            self.spinctrldbl.borrow().as_ref(),
        ) {
            // if config_ui.text_value.is_empty() {
            //     spinctrl.set_value_str("");
            //     spinctrldbl.set_value_str("");
            //     return;
            // }

            // let val = config_ui.text_value.get_value();
            // if let Ok(val) = val.parse() {
            //     if !self.is_valid_value(val) {
            //         return;
            //     }
            //     spinbtn.set_value(val);
            //     spinctrl.set_value_int(val);
            //     spinctrldbl.set_value_double(val.into());
            // }
        }
    }

    fn on_check_box(&self, config_ui: &ConfigUI) {
        self.create_spin(config_ui);
    }
}
