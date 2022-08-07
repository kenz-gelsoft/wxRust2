/////////////////////////////////////////////////////////////////////////////
// Program:     wxWidgets Widgets Sample
// Name:        slider.cpp
// Purpose:     Part of the widgets sample showing wxSlider
// Author:      Vadim Zeitlin
// Created:     16.04.01
// Copyright:   (c) 2001 Vadim Zeitlin
// Licence:     wxWindows licence
/////////////////////////////////////////////////////////////////////////////

use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum SliderPage {
    Reset = wx::ID_HIGHEST as isize,
    Clear,
    SetValue,
    SetMinAndMax,
    SetRange,
    SetLineSize,
    SetPageSize,
    SetTickFreq,
    SetThumbLen,
    CurValueText,
    ValueText,
    MinText,
    MaxText,
    RangeMinText,
    RangeMaxText,
    LineSizeText,
    PageSizeText,
    TickFreqText,
    ThumbLenText,
    RadioSides,
    BothSides,
    SelectRange,
    Slider,
}
impl SliderPage {
    fn from(v: c_int) -> Option<Self> {
        use SliderPage::*;
        for e in [
            Reset,
            Clear,
            SetValue,
            SetMinAndMax,
            SetRange,
            SetLineSize,
            SetPageSize,
            SetTickFreq,
            SetThumbLen,
            CurValueText,
            ValueText,
            MinText,
            MaxText,
            RangeMinText,
            RangeMaxText,
            LineSizeText,
            PageSizeText,
            TickFreqText,
            ThumbLenText,
            RadioSides,
            BothSides,
            SelectRange,
            Slider,
        ] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<SliderPage> for c_int {
    fn from(w: SliderPage) -> Self {
        w as c_int
    }
}

// the selection mode
#[derive(Clone, Copy)]
enum SliderTicks {
    _None,
    Top,
    Bottom,
    Left,
    Right,
}
impl SliderTicks {
    fn from(v: c_int) -> Option<Self> {
        use SliderTicks::*;
        for e in [_None, Top, Bottom, Left, Right] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<SliderTicks> for c_int {
    fn from(w: SliderTicks) -> Self {
        w as c_int
    }
}

#[derive(Clone)]
pub struct ConfigUI {
    // the controls
    // ------------

    // the check/radio boxes for styles
    chk_min_max_labels: wx::CheckBox,
    chk_value_label: wx::CheckBox,
    chk_inverse: wx::CheckBox,
    chk_ticks: wx::CheckBox,
    chk_both_sides: wx::CheckBox,
    chk_select_range: wx::CheckBox,

    radio_sides: wx::RadioBox,

    // sizer
    sizer_slider: wx::BoxSizer,

    // the text entries for set value/range
    text_value: wx::TextCtrl,
    text_min: wx::TextCtrl,
    text_max: wx::TextCtrl,
    text_range_min: wx::TextCtrl,
    text_range_max: wx::TextCtrl,
    text_line_size: wx::TextCtrl,
    text_page_size: wx::TextCtrl,
    text_tick_freq: wx::TextCtrl,
    text_thumb_len: wx::TextCtrl,
}

#[derive(Clone)]
pub struct SliderWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the slider itself
    slider: Rc<RefCell<Option<wx::Slider>>>,

    // the slider range
    min: RefCell<c_int>,
    max: RefCell<c_int>,

    // the slider selection range
    range_min: RefCell<c_int>,
    range_max: RefCell<c_int>,
}
impl WidgetsPage for SliderWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "Slider";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let s_box = wx::StaticBox::builder(Some(&self.base))
            .label("&Set style")
            .build();
        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box), wx::VERTICAL);

        let chk_inverse =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Inverse", wx::ID_ANY);
        let chk_ticks =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Show &ticks", wx::ID_ANY);
        let chk_min_max_labels =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Show min/max &labels", wx::ID_ANY);
        let chk_value_label =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Show &value label", wx::ID_ANY);

        let sides = wx::ArrayString::new();
        sides.add("default");
        sides.add("top");
        sides.add("bottom");
        sides.add("left");
        sides.add("right");
        let radio_sides = wx::RadioBox::builder(Some(&self.base))
            .id(SliderPage::RadioSides.into())
            .label("&Label position")
            .choices(sides)
            .major_dimension(1)
            .style(wx::RA_SPECIFY_COLS.into())
            .build();
        sizer_left.add_window_sizerflags(
            Some(&radio_sides),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let chk_both_sides = self.create_check_box_and_add_to_sizer(
            &sizer_left,
            "&Both sides",
            SliderPage::BothSides.into(),
        );
        let chk_select_range = self.create_check_box_and_add_to_sizer(
            &sizer_left,
            "&Selection range",
            SliderPage::SelectRange.into(),
        );
        chk_both_sides.set_tool_tip_str("\"Both sides\" is only supported \nin Universal");
        chk_select_range.set_tool_tip_str("\"Select range\" is only supported \nin wxMSW");

        sizer_left.add_spacer(5);

        let btn = wx::Button::builder(Some(&self.base))
            .id(SliderPage::Reset.into())
            .label("&Reset")
            .build();
        sizer_left.add_window_sizerflags(
            Some(&btn),
            wx::SizerFlags::new(0)
                // .centre_horizontal()
                .centre()
                .border_int(wx::ALL, 15),
        );

        // middle pane
        let s_box2 = wx::StaticBox::builder(Some(&self.base))
            .label("&Change slider value")
            .build();
        let sizer_middle = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box2), wx::VERTICAL);

        let (sizer_row, text) =
            self.create_sizer_with_text_and_label("Current value", SliderPage::CurValueText.into());
        text.set_editable(false);
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let (sizer_row, text_value) = self.create_sizer_with_text_and_button(
            SliderPage::SetValue.into(),
            "Set &value",
            SliderPage::ValueText.into(),
        );
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let (sizer_row, text_min) = self.create_sizer_with_text_and_button(
            SliderPage::SetMinAndMax.into(),
            "&Min and max",
            SliderPage::MinText.into(),
        );

        let text_max = wx::TextCtrl::builder(Some(&self.base))
            .id(SliderPage::MaxText.into())
            .build();
        sizer_row.add_window_sizerflags(
            Some(&text_max),
            wx::SizerFlags::new(1)
                // .centre_vertical()
                .centre()
                .border(wx::LEFT),
        );

        text_min.set_value(&format!("{}", self.min.borrow()));
        text_max.set_value(&format!("{}", self.max.borrow()));

        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let (sizer_row, text_range_min) = self.create_sizer_with_text_and_button(
            SliderPage::SetRange.into(),
            "&Selection",
            SliderPage::RangeMinText.into(),
        );
        let text_range_max = wx::TextCtrl::builder(Some(&self.base))
            .id(SliderPage::RangeMaxText.into())
            .build();
        sizer_row.add_window_sizerflags(
            Some(&text_range_max),
            wx::SizerFlags::new(1)
                // .centre_vertical()
                .centre()
                .border(wx::LEFT),
        );

        text_range_min.set_value(&format!("{}", self.range_min.borrow()));
        text_range_max.set_value(&format!("{}", self.range_max.borrow()));

        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let (sizer_row, text_line_size) = self.create_sizer_with_text_and_button(
            SliderPage::SetLineSize.into(),
            "Li&ne size",
            SliderPage::LineSizeText.into(),
        );
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let (sizer_row, text_page_size) = self.create_sizer_with_text_and_button(
            SliderPage::SetPageSize.into(),
            "P&age size",
            SliderPage::PageSizeText.into(),
        );
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let (sizer_row, text_tick_freq) = self.create_sizer_with_text_and_button(
            SliderPage::SetTickFreq.into(),
            "Tick &frequency",
            SliderPage::TickFreqText.into(),
        );
        text_tick_freq.set_value("10");
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        let (sizer_row, text_thumb_len) = self.create_sizer_with_text_and_button(
            SliderPage::SetThumbLen.into(),
            "Thumb &length",
            SliderPage::ThumbLenText.into(),
        );
        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        // right pane
        let sizer_right = wx::BoxSizer::new(wx::HORIZONTAL);

        // the 3 panes panes compose the window
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_left),
            wx::SizerFlags::new(0)
                .expand()
                .border_int((wx::ALL & !wx::LEFT), 10),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_middle),
            wx::SizerFlags::new(1).expand().border_int(wx::ALL, 10),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_right),
            wx::SizerFlags::new(1)
                .expand()
                .border_int((wx::ALL & !wx::RIGHT), 10),
        );
        let config_ui = ConfigUI {
            chk_min_max_labels,
            chk_value_label,
            chk_inverse,
            chk_ticks,
            chk_both_sides,
            chk_select_range,

            radio_sides,

            sizer_slider: sizer_right,

            text_value,
            text_min,
            text_max,
            text_range_min,
            text_range_max,
            text_line_size,
            text_page_size,
            text_tick_freq,
            text_thumb_len,
        };

        self.reset(&config_ui);
        self.create_slider(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        // final initializations
        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(m), Some(config_ui)) = (
            SliderPage::from(event.get_id()),
            self.config_ui.borrow().as_ref(),
        ) {
            match m {
                SliderPage::Reset => self.on_button_reset(config_ui),
                SliderPage::SetValue => self.on_button_set_value(config_ui),
                SliderPage::SetMinAndMax => self.on_button_set_min_and_max(config_ui),
                SliderPage::SetRange => self.on_button_set_range(config_ui),
                SliderPage::SetLineSize => self.on_button_set_line_size(config_ui),
                SliderPage::SetPageSize => self.on_button_set_page_size(config_ui),
                SliderPage::SetTickFreq => self.on_button_set_tick_freq(config_ui),
                SliderPage::SetThumbLen => self.on_button_set_thumb_len(config_ui),
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
impl SliderWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        SliderWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            slider: Rc::new(RefCell::new(None)),
            min: RefCell::new(0),
            max: RefCell::new(100),
            range_min: RefCell::new(20),
            range_max: RefCell::new(80),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        config_ui.chk_inverse.set_value(false);
        config_ui.chk_ticks.set_value(true);
        config_ui.chk_value_label.set_value(true);
        config_ui.chk_min_max_labels.set_value(true);
        config_ui.chk_both_sides.set_value(false);
        config_ui.chk_select_range.set_value(false);

        config_ui
            .radio_sides
            .set_selection(SliderTicks::_None.into());
    }

    fn create_slider(&self, config_ui: &ConfigUI) {
        let mut flags = wx::BORDER_DEFAULT;

        if config_ui.chk_inverse.get_value() {
            flags |= wx::SL_INVERSE as c_long;
        }
        if config_ui.chk_min_max_labels.get_value() {
            flags |= wx::SL_MIN_MAX_LABELS as c_long;
        }
        if config_ui.chk_value_label.get_value() {
            flags |= wx::SL_VALUE_LABEL as c_long;
        }
        if config_ui.chk_ticks.get_value() {
            flags |= wx::SL_AUTOTICKS as c_long;
        }

        // notice that the style names refer to the _ticks_ positions while we want
        // to allow the user to select the label(s) positions and the labels are on
        // the opposite side from the ticks, hence the apparent reversal below
        if let Some(m) = SliderTicks::from(config_ui.radio_sides.get_selection()) {
            flags |= match m {
                SliderTicks::Top => wx::SL_BOTTOM,
                SliderTicks::Left => wx::SL_RIGHT | wx::SL_VERTICAL,
                SliderTicks::Bottom => wx::SL_TOP,
                SliderTicks::Right => wx::SL_LEFT | wx::SL_VERTICAL,
                _ => 0,
            } as c_long;
        }

        if config_ui.chk_both_sides.get_value() {
            flags |= wx::SL_BOTH as c_long;
        }
        if config_ui.chk_select_range.get_value() {
            flags |= wx::SL_SELRANGE as c_long;
        }

        let min = *self.min.borrow();
        let max = *self.max.borrow();
        let mut val = min;
        if let Some(slider) = self.slider.borrow().as_ref() {
            let val_old = slider.get_value();
            if self.is_valid_value(val_old) {
                val = val_old;
            }

            config_ui.sizer_slider.detach_window(Some(slider));

            if config_ui.sizer_slider.get_children().get_count() > 0 {
                // we have 2 spacers, remove them too
                config_ui.sizer_slider.remove_int(0);
                config_ui.sizer_slider.remove_int(0);
            }

            slider.destroy();
        }

        let slider = wx::Slider::builder(Some(&self.base))
            .id(SliderPage::Slider.into())
            .value(val)
            .min_value(min)
            .max_value(max)
            .style(flags)
            .build();

        if slider.has_flag(wx::SL_VERTICAL) {
            config_ui.sizer_slider.add_stretch_spacer(1);
            config_ui.sizer_slider.add_window_sizerflags(
                Some(&slider),
                wx::SizerFlags::new(0).expand().border(wx::ALL),
            );
            config_ui.sizer_slider.add_stretch_spacer(1);
        } else {
            config_ui.sizer_slider.add_window_sizerflags(
                Some(&slider),
                wx::SizerFlags::new(1).centre().border(wx::ALL),
            );
        }

        config_ui
            .text_line_size
            .set_value(&format!("{}", slider.get_line_size()));
        config_ui
            .text_page_size
            .set_value(&format!("{}", slider.get_page_size()));
        config_ui
            .text_thumb_len
            .set_value(&format!("{}", slider.get_thumb_length()));
        *self.slider.borrow_mut() = Some(slider);

        if config_ui.chk_ticks.get_value() {
            self.do_set_tick_freq(config_ui);
        }
        if config_ui.chk_select_range.get_value() {
            self.do_set_selection_range(config_ui);
        }

        self.base.layout();
    }

    // is this slider value in range?
    fn is_valid_value(&self, val: c_int) -> bool {
        let min = *self.min.borrow();
        let max = *self.max.borrow();
        return (val >= min) && (val <= max);
    }

    // set the tick frequency from the text field value
    fn do_set_tick_freq(&self, config_ui: &ConfigUI) {
        let freq = config_ui.text_tick_freq.get_value();
        if let (Ok(freq), Some(slider)) = (freq.parse(), self.slider.borrow().as_ref()) {
            slider.set_tick_freq(freq);
        }
    }

    // set the selection range from the text field values
    fn do_set_selection_range(&self, config_ui: &ConfigUI) {
        let min_new = config_ui.text_range_min.get_value();
        let max_new = config_ui.text_range_max.get_value();
        if let (Ok(min_new), Ok(max_new), Some(slider)) = (
            min_new.parse(),
            max_new.parse(),
            self.slider.borrow().as_ref(),
        ) {
            let min = *self.min.borrow();
            let max = *self.max.borrow();
            if min_new >= max_new || min_new < min || max_new > max {
                return;
            }
            *self.range_min.borrow_mut() = min_new;
            *self.range_max.borrow_mut() = max_new;

            slider.set_selection(min_new, max_new);
        }
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);
        self.create_slider(config_ui);
    }

    // set the line size from the text field value
    fn on_button_set_line_size(&self, config_ui: &ConfigUI) {
        let line_size = config_ui.text_line_size.get_value();
        if let (Ok(line_size), Some(slider)) = (line_size.parse(), self.slider.borrow().as_ref()) {
            slider.set_line_size(line_size);
        }
    }

    // set the page size from the text field value
    fn on_button_set_page_size(&self, config_ui: &ConfigUI) {
        let page_size = config_ui.text_page_size.get_value();
        if let (Ok(page_size), Some(slider)) = (page_size.parse(), self.slider.borrow().as_ref()) {
            slider.set_page_size(page_size);
        }
    }

    fn on_button_set_tick_freq(&self, config_ui: &ConfigUI) {
        self.do_set_tick_freq(config_ui);
    }

    // set the thumb len from the text field value
    fn on_button_set_thumb_len(&self, config_ui: &ConfigUI) {
        let len = config_ui.text_thumb_len.get_value();
        if let (Ok(len), Some(slider)) = (len.parse(), self.slider.borrow().as_ref()) {
            slider.set_thumb_length(len);
        }
        self.base.layout();
    }

    fn on_button_set_min_and_max(&self, config_ui: &ConfigUI) {
        let min_new = config_ui.text_min.get_value();
        let max_new = config_ui.text_max.get_value();
        if let (Ok(min_new), Ok(max_new), Some(slider)) = (
            min_new.parse(),
            max_new.parse(),
            self.slider.borrow().as_ref(),
        ) {
            if min_new >= max_new {
                return;
            }
            *self.min.borrow_mut() = min_new;
            *self.max.borrow_mut() = max_new;

            slider.set_range(min_new, max_new);
        }
    }

    fn on_button_set_range(&self, config_ui: &ConfigUI) {
        self.do_set_selection_range(config_ui);
    }

    fn on_button_set_value(&self, config_ui: &ConfigUI) {
        let val = config_ui.text_value.get_value();
        if let (Ok(val), Some(slider)) = (val.parse(), self.slider.borrow().as_ref()) {
            slider.set_value(val);
        }
    }

    fn on_check_or_radio_box(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.create_slider(config_ui);
        }
    }
}
