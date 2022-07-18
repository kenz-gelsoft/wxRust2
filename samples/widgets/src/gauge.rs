use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum GaugePage {
    Reset = wx::ID_HIGHEST as isize,
    Progress,
    IndeterminateProgress,
    Clear,
    SetValue,
    SetRange,
    CurValueText,
    ValueText,
    RangeText,
    Timer,
    IndeterminateTimer,
    Gauge,
}
impl GaugePage {
    fn from(v: c_int) -> Option<Self> {
        use GaugePage::*;
        for e in [
            Reset,
            Progress,
            IndeterminateProgress,
            Clear,
            SetValue,
            SetRange,
            CurValueText,
            ValueText,
            RangeText,
            Timer,
            IndeterminateTimer,
            Gauge,
        ] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<GaugePage> for c_int {
    fn from(w: GaugePage) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // the checkboxes for styles
    chk_vert: wx::CheckBox,
    chk_smooth: wx::CheckBox,
    chk_progress: wx::CheckBox,

    sizer_gauge: wx::BoxSizer,
}

#[derive(Clone)]
pub struct GaugeWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    gauge: Rc<RefCell<Option<wx::Gauge>>>,
    range: c_int,
}
impl WidgetsPage for GaugeWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "Gauge";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let box_left = wx::StaticBox::builder(Some(&self.base))
            .label("&Set style")
            .build();

        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&box_left), wx::VERTICAL);

        let chk_vert = self.create_check_box_and_add_to_sizer(&sizer_left, "&Vertical", wx::ID_ANY);
        let chk_smooth = self.create_check_box_and_add_to_sizer(&sizer_left, "&Smooth", wx::ID_ANY);
        let chk_progress =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Progress", wx::ID_ANY);

        sizer_left.add_int_int(5, 5, 0, wx::GROW | wx::ALL, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(GaugePage::Reset.into())
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
        let box2 = wx::StaticBox::builder(Some(&self.base))
            .label("&Set style")
            .build();

        let sizer_middle = wx::StaticBoxSizer::new_with_staticbox(Some(&box2), wx::VERTICAL);

        let (sizer_row, text) = self.create_sizer_with_text_and_button(
            GaugePage::CurValueText.into(),
            "Current value",
            wx::ID_ANY,
        );
        text.set_editable(false);

        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_value) = self.create_sizer_with_text_and_button(
            GaugePage::SetValue.into(),
            "Set &value",
            GaugePage::ValueText.into(),
        );
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let (sizer_row, text_range) = self.create_sizer_with_text_and_button(
            GaugePage::SetRange.into(),
            "Set &range",
            GaugePage::RangeText.into(),
        );
        text_range.set_value(&format!("{}", self.range));
        sizer_middle.add_sizer_int(
            Some(&sizer_row),
            0,
            wx::ALL | wx::GROW,
            5,
            wx::Object::none(),
        );

        let btn = wx::Button::builder(Some(&self.base))
            .id(GaugePage::Progress.into())
            .label("Simulate &progress")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(GaugePage::IndeterminateProgress.into())
            .label("Simulate &indeterminate job")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        let btn = wx::Button::builder(Some(&self.base))
            .id(GaugePage::Clear.into())
            .label("&Clear")
            .build();
        sizer_middle.add_window_int(Some(&btn), 0, wx::ALL | wx::GROW, 5, wx::Object::none());

        // right pane
        let sizer_right = wx::BoxSizer::new(wx::HORIZONTAL);
        let gauge = wx::Gauge::builder(Some(&self.base))
            .id(GaugePage::Gauge.into())
            .range(self.range)
            .build();
        sizer_right.add_window_int(
            Some(&gauge),
            1,
            wx::CENTRE as i32 | wx::ALL,
            5,
            wx::Object::none(),
        );
        sizer_right.set_min_size_int(150, 0);
        *self.gauge.borrow_mut() = Some(gauge);

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

        // final initializations
        let config_ui = ConfigUI {
            chk_vert,
            chk_smooth,
            chk_progress,

            sizer_gauge: sizer_right, // save it to modify it later
        };
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(config_ui), Some(m)) = (
            self.config_ui.borrow().as_ref(),
            GaugePage::from(event.get_id()),
        ) {
            match m {
                GaugePage::Reset => self.on_button_reset(config_ui),
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
impl GaugeWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        GaugeWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            gauge: Rc::new(RefCell::new(None)),
            range: 100,
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        config_ui.chk_vert.set_value(false);
        config_ui.chk_smooth.set_value(false);
        config_ui.chk_progress.set_value(false);
    }

    fn create_gauge(&self, config_ui: &ConfigUI) {
        let mut flags = wx::BORDER_DEFAULT;

        flags |= if config_ui.chk_vert.get_value() {
            wx::GA_VERTICAL
        } else {
            wx::GA_HORIZONTAL
        } as c_long;

        if config_ui.chk_smooth.get_value() {
            flags |= wx::GA_SMOOTH as c_long;
        }
        if config_ui.chk_progress.get_value() {
            flags |= wx::GA_PROGRESS as c_long;
        }

        let mut val = 0;
        if let Some(gauge) = self.gauge.borrow().as_ref() {
            val = gauge.get_value();

            config_ui.sizer_gauge.detach_window(Some(gauge));
            gauge.destroy();
        }

        let gauge = wx::Gauge::builder(Some(&self.base))
            .id(GaugePage::Gauge.into())
            .range(self.range)
            .style(flags)
            .build();
        gauge.set_value(val);

        if (flags & wx::GA_VERTICAL as c_long) != 0 {
            config_ui.sizer_gauge.add_window_int(
                Some(&gauge),
                0,
                wx::GROW | wx::ALL,
                5,
                wx::Object::none(),
            );
        } else {
            config_ui.sizer_gauge.add_window_int(
                Some(&gauge),
                1,
                wx::CENTRE as i32 | wx::ALL,
                5,
                wx::Object::none(),
            );
        }
        *self.gauge.borrow_mut() = Some(gauge);

        // relayout the sizer
        config_ui.sizer_gauge.layout();
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);

        self.create_gauge(config_ui);
    }

    fn on_check_box(&self, config_ui: &ConfigUI) {
        self.create_gauge(config_ui);
    }
}
