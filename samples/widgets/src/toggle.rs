use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

// control ids
#[derive(Clone, Copy)]
enum TogglePage {
    Reset = wx::ID_HIGHEST as isize,
    ChangeLabel,
    Picker,
}
impl TogglePage {
    fn from(v: c_int) -> Option<Self> {
        use TogglePage::*;
        for e in [Reset, ChangeLabel, Picker] {
            if v == e.into() {
                return Some(e);
            }
        }
        return None;
    }
}
impl From<TogglePage> for c_int {
    fn from(w: TogglePage) -> Self {
        w as c_int
    }
}

const TOGGLE_IMAGE_POS_LEFT: c_int = 0;
// const TOGGLE_IMAGE_POS_RIGHT: c_int = 1;
// const TOGGLE_IMAGE_POS_TOP: c_int = 2;
// const TOGGLE_IMAGE_POS_BOTTOM: c_int = 3;

const TOGGLE_HALIGN_LEFT: c_int = 0;
const TOGGLE_HALIGN_CENTRE: c_int = 1;
const TOGGLE_HALIGN_RIGHT: c_int = 2;

const TOGGLE_VALIGN_TOP: c_int = 0;
const TOGGLE_VALIGN_CENTRE: c_int = 1;
const TOGGLE_VALIGN_BOTTOM: c_int = 2;

#[derive(Clone)]
pub struct ConfigUI {
    // the check/radio boxes for styles
    chk_bitmap_only: wx::CheckBox,
    chk_text_and_bitmap: wx::CheckBox,
    chk_fit: wx::CheckBox,
    chk_use_bitmap_class: wx::CheckBox,
    chk_disable: wx::CheckBox,

    // more checkboxes for wxBitmapButton only
    chk_use_pressed: wx::CheckBox,
    chk_use_focused: wx::CheckBox,
    chk_use_current: wx::CheckBox,
    chk_use_disabled: wx::CheckBox,

    // and an image position choice used if m_chkTextAndBitmap is on
    radio_image_pos: wx::RadioBox,
    radio_halign: wx::RadioBox,
    radio_valign: wx::RadioBox,

    // sizer
    sizer_toggle: wx::BoxSizer,

    // the text entries for command parameters
    text_label: wx::TextCtrl,
}

#[derive(Clone)]
pub struct ToggleWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the toggle itself
    toggle: Rc<RefCell<Option<wx::ToggleButton>>>,
}
impl WidgetsPage for ToggleWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "Toggle";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // left pane
        let s_box = wx::StaticBox::builder(Some(&self.base))
            .label("Styles")
            .build();

        let sizer_left = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box), wx::VERTICAL);

        let chk_bitmap_only =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Bitmap only", wx::ID_ANY);
        let chk_text_and_bitmap =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Text &and bitmap", wx::ID_ANY);

        let chk_fit =
            self.create_check_box_and_add_to_sizer(&sizer_left, "&Fit exactly", wx::ID_ANY);
        let chk_disable =
            self.create_check_box_and_add_to_sizer(&sizer_left, "Disable", wx::ID_ANY);

        let chk_use_bitmap_class = self.create_check_box_and_add_to_sizer(
            &sizer_left,
            "Use wxBitmapToggleButton",
            wx::ID_ANY,
        );
        chk_use_bitmap_class.set_value(true);

        sizer_left.add_spacer(5);

        let sizer_use_labels = wx::StaticBoxSizer::new_with_int(
            wx::VERTICAL,
            Some(&self.base),
            "&Use the following bitmaps in addition to the normal one?",
        );
        let chk_use_pressed = self.create_check_box_and_add_to_sizer(
            &sizer_use_labels,
            "&Pressed (small help icon)",
            wx::ID_ANY,
        );
        let chk_use_focused = self.create_check_box_and_add_to_sizer(
            &sizer_use_labels,
            "&Focused (small error icon)",
            wx::ID_ANY,
        );
        let chk_use_current = self.create_check_box_and_add_to_sizer(
            &sizer_use_labels,
            "&Current (small warning icon)",
            wx::ID_ANY,
        );
        let chk_use_disabled = self.create_check_box_and_add_to_sizer(
            &sizer_use_labels,
            "&Disabled (broken image icon)",
            wx::ID_ANY,
        );
        sizer_left.add_sizer_sizerflags(
            Some(&sizer_use_labels),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        sizer_left.add_spacer(10);

        let dirs = wx::ArrayString::new();
        dirs.add("left");
        dirs.add("right");
        dirs.add("top");
        dirs.add("bottom");
        let radio_image_pos = wx::RadioBox::builder(Some(&self.base))
            .label("Image &position")
            .choices(dirs)
            .build();
        sizer_left.add_window_sizerflags(
            Some(&radio_image_pos),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        sizer_left.add_spacer(15);

        let halign = wx::ArrayString::new();
        halign.add("left");
        halign.add("centre");
        halign.add("right");
        let radio_halign = wx::RadioBox::builder(Some(&self.base))
            .label("&Horz alignment")
            .choices(halign)
            .build();

        let valign = wx::ArrayString::new();
        valign.add("top");
        valign.add("centre");
        valign.add("bottom");
        let radio_valign = wx::RadioBox::builder(Some(&self.base))
            .label("&Vert alignment")
            .choices(valign)
            .build();

        sizer_left.add_window_sizerflags(
            Some(&radio_halign),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        sizer_left.add_window_sizerflags(
            Some(&radio_valign),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        sizer_left.add_spacer(5);

        let btn = wx::Button::builder(Some(&self.base))
            .id(TogglePage::Reset.into())
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
            .label("&Operations")
            .build();
        let sizer_middle = wx::StaticBoxSizer::new_with_staticbox(Some(&s_box2), wx::VERTICAL);

        let (sizer_row, text_label) = self.create_sizer_with_text_and_button(
            TogglePage::ChangeLabel.into(),
            "Change label",
            wx::ID_ANY,
        );
        text_label.set_value("&Toggle me!");

        sizer_middle.add_sizer_sizerflags(
            Some(&sizer_row),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );

        // right pane
        let sizer_toggle = wx::BoxSizer::new(wx::HORIZONTAL);
        sizer_toggle.set_min_size_int(150, 0);

        // the 3 panes panes compose the window
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_left),
            wx::SizerFlags::new(0)
                .expand()
                .border_int(wx::ALL & !wx::LEFT, 10),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_middle),
            wx::SizerFlags::new(1).expand().border_int(wx::ALL, 10),
        );
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_toggle),
            wx::SizerFlags::new(1)
                .expand()
                .border_int(wx::ALL & !wx::RIGHT, 10),
        );

        let config_ui = ConfigUI {
            chk_bitmap_only,
            chk_text_and_bitmap,
            chk_fit,
            chk_use_bitmap_class,
            chk_disable,

            chk_use_pressed,
            chk_use_focused,
            chk_use_current,
            chk_use_disabled,

            radio_image_pos,
            radio_halign,
            radio_valign,

            sizer_toggle,

            text_label,
        };

        // do create the main control
        self.reset(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);
        self.create_toggle();

        self.base.set_sizer(Some(&sizer_top), true);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        println!("event={}", event.get_id());
        if let (Some(m), Some(config_ui)) = (
            TogglePage::from(event.get_id()),
            self.config_ui.borrow().as_ref(),
        ) {
            match m {
                TogglePage::Reset => self.on_toggle_reset(config_ui),
                TogglePage::ChangeLabel => self.on_toggle_change_label(),
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
impl ToggleWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();
        ToggleWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            toggle: Rc::new(RefCell::new(None)),
        }
    }

    fn reset(&self, config_ui: &ConfigUI) {
        config_ui.chk_fit.set_value(true);
        config_ui.chk_disable.set_value(false);

        config_ui.chk_bitmap_only.set_value(false);
        config_ui.chk_text_and_bitmap.set_value(false);
        config_ui.chk_use_bitmap_class.set_value(true);

        config_ui.chk_use_pressed.set_value(true);
        config_ui.chk_use_focused.set_value(true);
        config_ui.chk_use_current.set_value(true);
        config_ui.chk_use_disabled.set_value(true);

        config_ui
            .radio_image_pos
            .set_selection(TOGGLE_IMAGE_POS_LEFT);
        config_ui.radio_halign.set_selection(TOGGLE_HALIGN_CENTRE);
        config_ui.radio_valign.set_selection(TOGGLE_VALIGN_CENTRE);

        if let Some(toggle) = self.toggle.borrow().as_ref() {
            toggle.set_value(false);
        }
    }

    fn create_toggle(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            let mut label = "".to_string();
            if let Some(toggle) = self.toggle.borrow().as_ref() {
                label = toggle.get_label();

                // TODO: remove (and delete) all toggles
                let count = config_ui.sizer_toggle.get_children().get_count();
                for _ in 0..count {
                    config_ui.sizer_toggle.remove_int(0);
                }
                toggle.destroy();
            }

            if label.is_empty() {
                label = config_ui.text_label.get_value();
            }

            let mut flags = wx::BORDER_DEFAULT;
            flags |= match config_ui.radio_halign.get_selection() {
                TOGGLE_HALIGN_LEFT => wx::BU_LEFT,
                TOGGLE_HALIGN_RIGHT => wx::BU_RIGHT,
                _ => 0,
            } as c_long;

            flags |= match config_ui.radio_valign.get_selection() {
                TOGGLE_VALIGN_TOP => wx::BU_TOP,
                TOGGLE_VALIGN_BOTTOM => wx::BU_BOTTOM,
                // centre vertical alignment is the default (no style)
                _ => 0,
            } as c_long;

            if config_ui.chk_fit.get_value() {
                flags |= wx::BU_EXACTFIT as c_long;
            }

            let mut shows_bitmap = false;
            let new_toggle = if config_ui.chk_bitmap_only.get_value() {
                shows_bitmap = true;

                let bbtn: wx::ToggleButton;
                // TODO: Support downcasting from BitmapButton into Button
                //
                // let bbtn: wx::BitmapButton;
                // if config_ui.chk_use_bitmap_class.get_value() {
                //     // TODO: create bitmap
                //     let icon_bitmap = wx::Bitmap::new();
                //     icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                //         // wxRust TODO: generate these constants...
                //         "wxART_INFORMATION",
                //         "wxART_BUTTON_C",
                //         &wx::Size::default(),
                //     ));
                //     bbtn = wx::BitmapButton::builder(Some(&self.base))
                //         .style(flags)
                //         .build(&icon_bitmap);
                // } else {
                // TODO: create bitmap
                bbtn = wx::ToggleButton::builder(Some(&self.base))
                    .id(TogglePage::Picker.into())
                    .build();
                let icon_bitmap = wx::Bitmap::new();
                icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                    // wxRust TODO: generate these constants...
                    "wxART_INFORMATION",
                    "wxART_BUTTON_C",
                    &wx::Size::default(),
                ));
                let icon_bitmap = wx::BitmapBundle::from(icon_bitmap);
                bbtn.set_bitmap_label(&icon_bitmap);
                // }

                if config_ui.chk_use_pressed.get_value() {
                    // TODO: CreateBitmap("pushed", wxART_HELP)
                    bbtn.set_bitmap_pressed(&icon_bitmap);
                }
                if config_ui.chk_use_focused.get_value() {
                    // TODO: CreateBitmap("focused", wxART_ERROR)
                    bbtn.set_bitmap_focus(&icon_bitmap);
                }
                if config_ui.chk_use_current.get_value() {
                    // TODO: CreateBitmap("hover", wxART_WARNING)
                    bbtn.set_bitmap_current(&icon_bitmap);
                }
                if config_ui.chk_use_disabled.get_value() {
                    // TODO: CreateBitmap("disabled", wxART_MISSING_IMAGE)
                    bbtn.set_bitmap_disabled(&icon_bitmap);
                }

                bbtn
            } else {
                wx::ToggleButton::builder(Some(&self.base))
                    .id(TogglePage::Picker.into())
                    .label(&label)
                    .style(flags)
                    .build()
            };

            if !shows_bitmap && config_ui.chk_text_and_bitmap.get_value() {
                shows_bitmap = true;

                let positions = [wx::LEFT, wx::RIGHT, wx::TOP, wx::BOTTOM];
                // TODO: implement From<> trait
                let icon_bitmap = wx::Bitmap::new();
                icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                    // wxRust TODO: generate these constants...
                    "wxART_INFORMATION",
                    "wxART_BUTTON_C",
                    &wx::Size::default(),
                ));
                new_toggle.set_bitmap(
                    &wx::BitmapBundle::from(icon_bitmap),
                    positions[config_ui.radio_image_pos.get_selection() as usize],
                );

                if config_ui.chk_use_pressed.get_value() {
                    let icon_bitmap = wx::Bitmap::new();
                    icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                        "wxART_HELP",
                        "wxART_BUTTON_C",
                        &wx::Size::default(),
                    ));
                    new_toggle.set_bitmap_pressed(&wx::BitmapBundle::from(icon_bitmap));
                }
                if config_ui.chk_use_focused.get_value() {
                    let icon_bitmap = wx::Bitmap::new();
                    icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                        "wxART_ERROR",
                        "wxART_BUTTON_C",
                        &wx::Size::default(),
                    ));
                    new_toggle.set_bitmap_focus(&wx::BitmapBundle::from(icon_bitmap));
                }
                if config_ui.chk_use_current.get_value() {
                    let icon_bitmap = wx::Bitmap::new();
                    icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                        "wxART_WARNING",
                        "wxART_BUTTON_C",
                        &wx::Size::default(),
                    ));
                    new_toggle.set_bitmap_current(&wx::BitmapBundle::from(icon_bitmap));
                }
                if config_ui.chk_use_disabled.get_value() {
                    let icon_bitmap = wx::Bitmap::new();
                    icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                        "wxART_MISSING_IMAGE",
                        "wxART_BUTTON_C",
                        &wx::Size::default(),
                    ));
                    new_toggle.set_bitmap_disabled(&wx::BitmapBundle::from(icon_bitmap));
                }
            }

            config_ui.chk_use_bitmap_class.enable(shows_bitmap);

            config_ui.chk_use_pressed.enable(shows_bitmap);
            config_ui.chk_use_focused.enable(shows_bitmap);
            config_ui.chk_use_current.enable(shows_bitmap);
            config_ui.chk_use_disabled.enable(shows_bitmap);
            config_ui
                .radio_image_pos
                .enable(config_ui.chk_text_and_bitmap.is_checked());

            new_toggle.enable(!config_ui.chk_disable.is_checked());

            let sizer_toggle = &config_ui.sizer_toggle;
            sizer_toggle.add_stretch_spacer(1);
            sizer_toggle.add_window_sizerflags(
                Some(&new_toggle),
                wx::SizerFlags::new(0).centre().border(wx::ALL),
            );
            sizer_toggle.add_stretch_spacer(1);

            sizer_toggle.layout();

            *self.toggle.borrow_mut() = Some(new_toggle);
        }
    }

    fn on_toggle_reset(&self, config_ui: &ConfigUI) {
        self.reset(config_ui);
        self.create_toggle();
    }

    fn on_check_or_radio_box(&self) {
        self.create_toggle();
        self.base.layout(); // make sure the text field for changing note displays correctly.
    }

    fn on_toggle_change_label(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            let label_text = config_ui.text_label.get_value();
            println!("{}", label_text);

            self.toggle
                .borrow()
                .as_ref()
                .unwrap()
                .set_label(&label_text);

            config_ui.sizer_toggle.layout();
        }
    }
}
