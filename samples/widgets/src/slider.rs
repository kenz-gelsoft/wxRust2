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

// the list type
#[derive(Clone, Copy)]
enum LboxType {
    Slider,
    CheckSlider,
    RearrangeList,
}
impl LboxType {
    fn from(v: c_int) -> Option<Self> {
        use LboxType::*;
        for e in [Slider, CheckSlider, RearrangeList] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<LboxType> for c_int {
    fn from(w: LboxType) -> Self {
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

    s_item: RefCell<c_int>,
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
                .centre_horizontal()
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
            wx::SizerFlags::new(1).centre_vertical().border(wx::LEFT),
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
            wx::SizerFlags::new(1).centre_vertical().border(wx::LEFT),
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
                //     SliderPage::Change => self.on_button_change(config_ui),
                //     SliderPage::Delete => self.on_button_delete(config_ui),
                //     SliderPage::DeleteSel => self.on_button_delete_sel(config_ui),
                //     SliderPage::EnsureVisible => self.on_button_ensure_visible(config_ui),
                //     SliderPage::Clear => self.on_button_clear(),
                //     SliderPage::Add => self.on_button_add(config_ui),
                //     SliderPage::AddSeveral => self.on_button_add_several(),
                //     SliderPage::AddMany => self.on_button_add_many(),
                //     // wx3.0 unsupported
                //     // SliderPage::GetTopItem => self.on_button_top_item(),
                //     // SliderPage::GetCountPerPage => self.on_button_page_count(),
                //     SliderPage::MoveUp => self.on_button_move_up(),
                //     SliderPage::MoveDown => self.on_button_move_down(),
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
            s_item: RefCell::new(1),
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
            self.do_set_tick_freq();
        }
        if config_ui.chk_select_range.get_value() {
            self.do_set_selection_range();
        }

        self.base.layout();
    }

    // is this slider value in range?
    fn is_valid_value(&self, val: c_int) -> bool {
        let min = *self.min.borrow();
        let max = *self.max.borrow();
        return (val >= min) && (val <= max);
    }

    fn get_valid_index_from_text(&self, text: &wx::TextCtrl) -> Option<u32> {
        let idx = text.get_value();
        // if let (Ok(idx), Some(slider)) = (idx.parse(), self.slider.borrow().as_ref()) {
        //     if idx < slider.get_count() {
        //         return Some(idx);
        //     }
        // }
        return None;
    }

    fn do_set_tick_freq(&self) {}

    fn do_set_selection_range(&self) {}

    fn on_button_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);
        self.create_slider(config_ui);
    }

    fn on_button_change(&self, config_ui: &ConfigUI) {
        if let Some(slider) = self.slider.borrow().as_ref() {
            let selections = wx::ArrayInt::new();
            // let count = slider.get_selections(&selections);
            // let s = config_ui.text_change.get_value();
            // for n in 0..count {
            //     let i = selections.item(n.try_into().unwrap()).try_into().unwrap();
            //     slider.set_string(i, &s);
            // }
        }
    }

    fn on_button_ensure_visible(&self, config_ui: &ConfigUI) {
        // if let (Some(n), Some(slider)) = (
        //     self.get_valid_index_from_text(&config_ui.text_ensure_visible),
        //     self.slider.borrow().as_ref(),
        // ) {
        //     slider.ensure_visible(n.try_into().unwrap());
        // }
    }

    fn on_button_delete(&self, config_ui: &ConfigUI) {
        // if let (Some(n), Some(slider)) = (
        //     self.get_valid_index_from_text(&config_ui.text_delete),
        //     self.slider.borrow().as_ref(),
        // ) {
        //     slider.delete(n.try_into().unwrap());
        // }
    }

    fn on_button_delete_sel(&self, config_ui: &ConfigUI) {
        if let Some(slider) = self.slider.borrow().as_ref() {
            let selections = wx::ArrayInt::new();
            // let mut n = slider.get_selections(&selections) - 1;
            // while n >= 0 {
            //     let i = selections.item(n.try_into().unwrap()).try_into().unwrap();
            //     slider.delete(i);
            //     n -= 1;
            // }
        }
    }

    fn on_button_clear(&self) {
        // if let Some(slider) = self.slider.borrow().as_ref() {
        //     slider.clear();
        // }
    }

    // fn on_button_top_item(&self) {
    //     if let Some(slider) = self.slider.borrow().as_ref() {
    //         let item = slider.get_top_item();
    //         println!("Topmost visible item is: {}", item);
    //     }
    // }

    // fn on_button_page_count(&self) {
    //     if let Some(slider) = self.slider.borrow().as_ref() {
    //         let count = slider.get_count_per_page();
    //         println!("{} items fit into this listbox.", count);
    //     }
    // }

    fn on_button_add(&self, config_ui: &ConfigUI) {
        // let s = config_ui.text_add.get_value();
        // if !config_ui.text_add.is_modified() {
        //     // update the default string
        //     let s_item = *self.s_item.borrow();
        //     config_ui
        //         .text_add
        //         .set_value(&format!("test item \t{}", s_item));
        //     *self.s_item.borrow_mut() = s_item + 1;
        // }

        // if let Some(slider) = self.slider.borrow().as_ref() {
        //     slider.append_str(&s);
        // }
    }

    fn on_button_add_many(&self) {
        // "many" means 1000 here
        let strings = wx::ArrayString::new();
        for n in 0..1000 {
            strings.add(&format!("item #{}", n));
        }
        // if let Some(slider) = self.slider.borrow().as_ref() {
        //     slider.append_arraystring(&strings);
        // }
    }

    fn on_button_add_several(&self) {
        let items = wx::ArrayString::new();
        items.add("First");
        items.add("another one");
        items.add("and the last (very very very very very very very very very very long) one");
        // if let Some(slider) = self.slider.borrow().as_ref() {
        //     slider.insert_arraystring(&items, 0);
        // }
    }

    fn on_button_move_up(&self) {
        // TODO: RearrangeList
    }

    fn on_button_move_down(&self) {
        // TODO: RearrangeList
    }

    fn on_check_or_radio_box(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.create_slider(config_ui);
        }
    }
}
