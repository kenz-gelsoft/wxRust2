use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum HyperlinkPage {
    Reset = wx::ID_HIGHEST as isize,
    SetLabel,
    SetURL,
    Ctrl,
}
impl HyperlinkPage {
    fn from(v: c_int) -> Option<Self> {
        use HyperlinkPage::*;
        for e in [Reset, SetLabel, SetURL, Ctrl] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<HyperlinkPage> for c_int {
    fn from(w: HyperlinkPage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // the checkboxes for styles
    // chk_vert: wx::CheckBox,
    // chk_smooth: wx::CheckBox,
    // chk_progress: wx::CheckBox,

    // the text entries for set value/range
    label: wx::TextCtrl,
    url: wx::TextCtrl,

    sizer_hyperlink: wx::BoxSizer,
}

#[derive(Clone)]
pub struct HyperlinkWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    hyperlink: Rc<RefCell<Option<wx::HyperlinkCtrl>>>,
    range: Rc<RefCell<c_int>>,
    // the timer for simulating hyperlink progress
    // timer: Rc<RefCell<Option<wx::Timer>>>,
}
impl WidgetsPage for HyperlinkWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "Hyperlink";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let box_left = wx::StaticBox::builder(Some(&self.base))
            .label("Hyperlink details")
            .build();

        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&box_left), wx::VERTICAL);

        let (sizer_row, label) = self.create_sizer_with_text_and_button(
            HyperlinkPage::SetLabel.into(),
            "Set &Label",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::ALIGN_RIGHT,
            5,
            wx::Object::none(),
        );

        let (sizer_row, url) = self.create_sizer_with_text_and_button(
            HyperlinkPage::SetURL.into(),
            "Set &URL",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::ALIGN_RIGHT,
            5,
            wx::Object::none(),
        );

        let alignments = wx::ArrayString::new();
        alignments.add("&left");
        alignments.add("&centre");
        alignments.add("&right");

        let radio_align_mode = wx::RadioBox::builder(Some(&self.base))
            .label("alignment")
            .choices(alignments)
            .build();
        radio_align_mode.set_selection(1); // start with "centre" selected since
                                           // wxHL_DEFAULT_STYLE contains wxHL_ALIGN_CENTRE
        sizer_left.add_window_int(
            Some(&radio_align_mode),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let check_generic = wx::CheckBox::builder(Some(&self.base))
            .label("Use generic version")
            .build();
        sizer_left.add_window_int(
            Some(&check_generic),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        // right pane
        let sz_hyperlink_long = wx::BoxSizer::new(wx::VERTICAL);
        let sz_hyperlink = wx::BoxSizer::new(wx::HORIZONTAL);

        let visit = wx::StaticText::builder(Some(&self.base))
            .label("Visit ")
            .build();

        let hyperlink = wx::HyperlinkCtrl::builder(Some(&self.base))
            .id(HyperlinkPage::Ctrl.into())
            .label("wxWidgets website")
            .url("www.wxwidgets.org")
            .build();

        let fun = wx::StaticText::builder(Some(&self.base))
            .label(" for fun!")
            .build();

        let centre = wx::CENTRE as i32;
        sz_hyperlink.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sz_hyperlink.add_window_int(Some(&visit), 0, centre, 0, wx::Object::none());
        sz_hyperlink.add_window_int(Some(&hyperlink), 0, centre, 0, wx::Object::none());
        sz_hyperlink.add_window_int(Some(&fun), 0, centre, 0, wx::Object::none());
        sz_hyperlink.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sz_hyperlink.set_min_size_int(150, 0);

        let hyperlink_long = wx::HyperlinkCtrl::builder(Some(&self.base))
            .label("This is a long hyperlink")
            .url("www.wxwidgets.org")
            .build();

        sz_hyperlink_long.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sz_hyperlink_long.add_sizer_int(
            Some(&sz_hyperlink),
            0,
            centre | wx::GROW,
            0,
            wx::Object::none(),
        );
        sz_hyperlink_long.add_int_int(0, 0, 1, centre, 0, wx::Object::none());
        sz_hyperlink_long.add_window_int(Some(&hyperlink_long), 0, wx::GROW, 0, wx::Object::none());
        sz_hyperlink_long.add_int_int(0, 0, 1, centre, 0, wx::Object::none());

        // the 3 panes panes compose the window
        sizer_top.add_sizer_int(
            Some(&sizer_left),
            0,
            wx::GROW | (wx::ALL & !wx::LEFT),
            10,
            wx::Object::none(),
        );
        sizer_top.add_sizer_int(
            Some(&sz_hyperlink_long),
            1,
            wx::GROW | (wx::ALL & !wx::RIGHT),
            10,
            wx::Object::none(),
        );

        // final initializations
        let config_ui = ConfigUI {
            // chk_vert,
            // chk_smooth,
            // chk_progress,

            label,
            url,

            sizer_hyperlink: sz_hyperlink_long, // save it to modify it later
        };
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            HyperlinkPage::from(event.get_id()),
        ) {
            // match m {
            //     HyperlinkPage::Reset => self.on_button_reset(config_ui),
            //     HyperlinkPage::Progress => self.on_button_progress(),
            //     HyperlinkPage::IndeterminateProgress => self.on_button_indeterminate_progress(),
            //     HyperlinkPage::Clear => self.on_button_clear(),
            //     HyperlinkPage::SetValue => self.on_button_set_value(),
            //     HyperlinkPage::SetRange => self.on_button_set_range(),
            //     _ => (),
            // };
        }
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.on_check_or_radio_box(config_ui);
        }
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.on_check_or_radio_box(config_ui);
        }
    }
}
impl HyperlinkWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();

        let page = HyperlinkWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            hyperlink: Rc::new(RefCell::new(None)),
            range: Rc::new(RefCell::new(100)),
            // timer: Rc::new(RefCell::new(None)),
        };

        let page_copy = page.clone();
        page.base
            .bind(wx::RustEvent::Timer, move |event: &wx::TimerEvent| {
                page_copy.handle_timer(event);
            });

        page
    }

    fn reset(&self, config_ui: &ConfigUI) {
        if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
            config_ui.label.set_value(&hyperlink.get_label());
            config_ui.url.set_value(&hyperlink.get_url());
        }
    }

    fn create_hyperlink(&self, config_ui: &ConfigUI) {
        let mut flags = wx::BORDER_DEFAULT;

        // flags |= if config_ui.chk_vert.get_value() {
        //     wx::GA_VERTICAL
        // } else {
        //     wx::GA_HORIZONTAL
        // } as c_long;

        // if config_ui.chk_smooth.get_value() {
        //     flags |= wx::GA_SMOOTH as c_long;
        // }
        // if config_ui.chk_progress.get_value() {
        //     flags |= wx::GA_PROGRESS as c_long;
        // }

        let mut val = 0;
        if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
            // val = hyperlink.get_value();

            config_ui.sizer_hyperlink.detach_window(Some(hyperlink));
            hyperlink.destroy();
        }

        let range = *self.range.borrow();
        // let hyperlink = wx::HyperlinkCtrl::builder(Some(&self.base))
        //     .id(HyperlinkPage::HyperlinkCtrl.into())
        //     .range(range)
        //     .style(flags)
        //     .build();
        // hyperlink.set_value(val);

        // if (flags & wx::GA_VERTICAL as c_long) != 0 {
        //     config_ui.sizer_hyperlink.add_window_int(
        //         Some(&hyperlink),
        //         0,
        //         wx::GROW | wx::ALL,
        //         5,
        //         wx::Object::none(),
        //     );
        // } else {
        //     config_ui.sizer_hyperlink.add_window_int(
        //         Some(&hyperlink),
        //         1,
        //         wx::CENTRE as i32 | wx::ALL,
        //         5,
        //         wx::Object::none(),
        //     );
        // }
        // *self.hyperlink.borrow_mut() = Some(hyperlink);

        // relayout the sizer
        config_ui.sizer_hyperlink.layout();
    }

    fn start_timer<W: WindowMethods>(&self, clicked: &W) {
        let interval = 300;

        // let is_progress_button = clicked.get_id() == HyperlinkPage::Progress.into();
        // let timer = wx::Timer::new_with_evthandler(
        //     Some(&self.base),
        //     if is_progress_button {
        //         HyperlinkPage::Timer.into()
        //     } else {
        //         HyperlinkPage::IndeterminateTimer.into()
        //     },
        // );
        // timer.start(interval, wx::TIMER_CONTINUOUS);
        // *self.timer.borrow_mut() = Some(timer);

        clicked.set_label("&Stop timer");

        // if is_progress_button {
        //     if let Some(hyperlink) = self
        //         .base
        //         .find_window_long(HyperlinkPage::IndeterminateProgress as c_long)
        //         .get()
        //     {
        //         hyperlink.disable();
        //     }
        // } else {
        //     if let Some(hyperlink) = self
        //         .base
        //         .find_window_long(HyperlinkPage::Progress as c_long)
        //         .get()
        //     {
        //         hyperlink.disable();
        //     }
        // }
    }

    fn stop_timer<W: WindowMethods>(&self, clicked: &W) {
        // if let Some(timer) = self.timer.borrow().as_ref() {
        //     timer.stop();
        // }
        // *self.timer.borrow_mut() = None;

        // if clicked.get_id() == HyperlinkPage::Progress.into() {
        //     clicked.set_label("Simulate &progress");
        //     if let Some(hyperlink) = self
        //         .base
        //         .find_window_long(HyperlinkPage::IndeterminateProgress as c_long)
        //         .get()
        //     {
        //         hyperlink.enable(true);
        //     }
        // } else {
        //     clicked.set_label("Simulate indeterminate job");
        //     if let Some(hyperlink) = self
        //         .base
        //         .find_window_long(HyperlinkPage::Progress as c_long)
        //         .get()
        //     {
        //         hyperlink.enable(true);
        //     }
        // }
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_hyperlink(config_ui);
    }

    fn on_button_progress(&self) {
        // if let Some(b) = self
        //     .base
        //     .find_window_long(HyperlinkPage::Progress as c_long)
        //     .get()
        // {
        //     if self.timer.borrow().is_none() {
        //         self.start_timer(&b);
        //     } else {
        //         self.stop_timer(&b);
        //     }
        // }
    }

    fn on_button_indeterminate_progress(&self) {
        // if let Some(b) = self
        //     .base
        //     .find_window_long(HyperlinkPage::IndeterminateProgress as c_long)
        //     .get()
        // {
        //     if self.timer.borrow().is_none() {
        //         self.start_timer(&b);
        //     } else {
        //         self.stop_timer(&b);
        //         if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
        //             hyperlink.set_value(0);
        //         }
        //     }
        // }
    }

    fn on_button_clear(&self) {
        if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
            // hyperlink.set_value(0);
        }
    }

    fn on_button_set_range(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            // if let (Ok(val), Some(hyperlink)) = (
            //     config_ui.text_range.get_value().parse(),
            //     self.hyperlink.borrow().as_ref(),
            // ) {
            //     *self.range.borrow_mut() = val;
            //     hyperlink.set_range(val);
            // }
        }
    }

    fn on_button_set_value(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            // if let (Ok(val), Some(hyperlink)) = (
            //     config_ui.text_value.get_value().parse(),
            //     self.hyperlink.borrow().as_ref(),
            // ) {
            //     hyperlink.set_value(val);
            // }
        }
    }

    fn on_check_or_radio_box(&self, config_ui: &ConfigUI) {
        self.create_hyperlink(config_ui);
    }

    fn handle_timer(&self, event: &wx::TimerEvent) {
        if let Some(m) = HyperlinkPage::from(event.get_id()) {
            // match m {
            //     HyperlinkPage::Timer => self.on_progress_timer(),
            //     HyperlinkPage::IndeterminateTimer => self.on_intermediate_progress_timer(),
            //     _ => (),
            // }
        }
    }

    fn on_progress_timer(&self) {
        if let (Some(hyperlink), range) = (self.hyperlink.borrow().as_ref(), *self.range.borrow()) {
            // let val = hyperlink.get_value();
            // if val < range {
            //     hyperlink.set_value(val + 1);
            // } else {
            //     // reached the end
            //     if let Some(b) = self
            //         .base
            //         .find_window_long(HyperlinkPage::Progress as c_long)
            //         .get()
            //     {
            //         self.stop_timer(&b);
            //     }
            // }
        }
    }

    fn on_intermediate_progress_timer(&self) {
        if let Some(hyperlink) = self.hyperlink.borrow().as_ref() {
            // hyperlink.pulse();
        }
    }
}
