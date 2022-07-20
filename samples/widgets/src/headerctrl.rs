use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

const NUMBER_OF_COLUMNS: usize = 4;

const COL_ALIGN_FLAGS: [c_int; 4] = [
    wx::ALIGN_NOT,
    wx::ALIGN_LEFT,
    wx::ALIGN_CENTRE,
    wx::ALIGN_RIGHT,
];

const COL_WITH_BITMAP_DEFAULT: bool = false;
const COL_ALIGNMENT_FLAG_DEFAULT: c_int = wx::ALIGN_NOT;
const COL_ALIGNMENT_INDEX_DEFAULT: c_int = 0;

#[derive(Clone)]
struct ColSetting {
    chk_allow_resize: wx::CheckBox,
    chk_allow_reorder: wx::CheckBox,
    chk_allow_sort: wx::CheckBox,
    chk_allow_hide: wx::CheckBox,
    chk_with_bitmap: wx::CheckBox,
    rb_alignments: wx::RadioBox,
}

#[derive(Clone)]
pub struct ConfigUI {
    // the checkboxes for styles
    chk_allow_reorder: wx::CheckBox,
    chk_allow_hide: wx::CheckBox,
    chk_bitmap_on_right: wx::CheckBox,

    col_settings: [ColSetting; NUMBER_OF_COLUMNS],

    sizer_header: wx::StaticBoxSizer,
}

#[derive(Clone)]
pub struct HeaderCtrlWidgetsPage {
    pub base: wx::Panel,
    config_ui: RefCell<Option<ConfigUI>>,
    // the control itself
    header: Rc<RefCell<Option<wx::HeaderCtrlSimple>>>,
}
impl WidgetsPage for HeaderCtrlWidgetsPage {
    fn base(&self) -> &wx::Panel {
        return &self.base;
    }
    fn label(&self) -> &str {
        return "Gauge";
    }
    fn create_content(&self) {
        let sizer_top = wx::BoxSizer::new(wx::HORIZONTAL);

        // header style
        let sizer_header =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "&Header style");

        let chk_allow_reorder =
            self.create_check_box_and_add_to_sizer(&sizer_header, "Allow &reorder", wx::ID_ANY);
        let chk_allow_hide =
            self.create_check_box_and_add_to_sizer(&sizer_header, "Alow &hide", wx::ID_ANY);
        let chk_bitmap_on_right =
            self.create_check_box_and_add_to_sizer(&sizer_header, "&Bitmap on right", wx::ID_ANY);

        sizer_header.add_stretch_spacer(1);

        let btn_reset = wx::Button::builder(Some(&self.base))
            .label("&Reset")
            .build();
        sizer_header.add_window_sizerflags(
            Some(&btn_reset),
            wx::SizerFlags::new(0).center().border(wx::ALL),
        );
        sizer_top.add_sizer_sizerflags(Some(&sizer_header), wx::SizerFlags::new(0).expand());

        // column flags
        let mut col_settings: Vec<ColSetting> = vec![];
        for i in 0..NUMBER_OF_COLUMNS {
            let col_alignments = wx::ArrayString::new();
            col_alignments.add("none");
            col_alignments.add("left");
            col_alignments.add("centre");
            col_alignments.add("right");

            let sizer_col = wx::StaticBoxSizer::new_with_int(
                wx::VERTICAL,
                Some(&self.base),
                &format!("Column {} style", i + 1),
            );
            let col_setting = ColSetting {
                chk_allow_resize: self.create_check_box_and_add_to_sizer(
                    &sizer_col,
                    "Allow resize",
                    wx::ID_ANY,
                ),
                chk_allow_reorder: self.create_check_box_and_add_to_sizer(
                    &sizer_col,
                    "Allow reorder",
                    wx::ID_ANY,
                ),
                chk_allow_sort: self.create_check_box_and_add_to_sizer(
                    &sizer_col,
                    "Allow sort",
                    wx::ID_ANY,
                ),
                chk_allow_hide: self.create_check_box_and_add_to_sizer(
                    &sizer_col,
                    "Hidden",
                    wx::ID_ANY,
                ),
                chk_with_bitmap: self.create_check_box_and_add_to_sizer(
                    &sizer_col,
                    "With bitmap",
                    wx::ID_ANY,
                ),
                rb_alignments: wx::RadioBox::builder(Some(&self.base))
                    .label("Alignment")
                    .choices(col_alignments)
                    .major_dimension(2)
                    .style(wx::RA_SPECIFY_COLS.into())
                    .build(),
            };
            sizer_col.add_window_sizerflags(
                Some(&col_setting.rb_alignments),
                wx::SizerFlags::new(0).expand().border(wx::ALL),
            );
            self.reset_column_style(&col_setting);
            col_settings.push(col_setting);
            sizer_top.add_spacer(15);
            sizer_top.add_sizer_sizerflags(Some(&sizer_col), wx::SizerFlags::new(0).expand());
        }

        // bottom pane
        let sizer_header =
            wx::StaticBoxSizer::new_with_int(wx::VERTICAL, Some(&self.base), "Header");

        // the 2 panes compose the window
        let sizer_all = wx::BoxSizer::new(wx::VERTICAL);
        sizer_all.add_sizer_sizerflags(
            Some(&sizer_top),
            wx::SizerFlags::new(0).expand().border(wx::ALL),
        );
        sizer_all.add_sizer_sizerflags(
            Some(&sizer_header),
            wx::SizerFlags::new(1).expand().border(wx::ALL),
        );

        self.base.set_sizer(Some(&sizer_all), true);

        // final initializations
        let col_settings: [ColSetting; NUMBER_OF_COLUMNS] = col_settings.try_into().ok().unwrap();
        let config_ui = ConfigUI {
            chk_allow_reorder,
            chk_allow_hide,
            chk_bitmap_on_right,

            col_settings: col_settings.try_into().unwrap(),

            sizer_header, // save it to modify it later
        };
        self.recreate_widget(&config_ui);
        self.reset_header_style(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);

        // Bind event handlers
        let copy_self = self.clone();
        btn_reset.bind(wx::RustEvent::Button, move |_: &wx::CommandEvent| {
            copy_self.on_reset_button();
        });
    }

    fn handle_button(&self, _: &wx::CommandEvent) {
        // Do nothing.
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        self.on_style_check_or_radio_box();
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        self.on_style_check_or_radio_box();
    }
}
impl HeaderCtrlWidgetsPage {
    pub fn new<P: WindowMethods>(book: &P) -> Self {
        let panel = wx::Panel::builder(Some(book))
            .style(wx::CLIP_CHILDREN | wx::TAB_TRAVERSAL)
            .build();

        HeaderCtrlWidgetsPage {
            base: panel,
            config_ui: RefCell::new(None),
            header: Rc::new(RefCell::new(None)),
        }
    }

    fn recreate_widget(&self, config_ui: &ConfigUI) {
        config_ui.sizer_header.clear(true /* delete windows */);

        let flags = wx::BORDER_DEFAULT | self.get_header_style_flags(config_ui);

        let header = wx::HeaderCtrlSimple::builder(Some(&self.base))
            .style(flags)
            .build();

        let col_setting1 = &config_ui.col_settings[0];
        let col1 = wx::HeaderColumnSimple::new_with_str(
            "First",
            100,
            self.get_column_alignment_flag(col_setting1),
            self.get_column_style_flags(col_setting1),
        );
        if col_setting1.chk_with_bitmap.is_checked() {
            let icon_bitmap = wx::Bitmap::new();
            icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                "wxART_ERROR",
                "wxART_BUTTON_C",
                &wx::Size::default(),
            ));
            col1.set_bitmap(&wx::BitmapBundle::from(icon_bitmap));
        }
        header.append_column(&col1);

        let col_setting2 = &config_ui.col_settings[1];
        let col2 = wx::HeaderColumnSimple::new_with_str(
            "Second",
            200,
            self.get_column_alignment_flag(col_setting2),
            self.get_column_style_flags(col_setting2),
        );
        if col_setting2.chk_with_bitmap.is_checked() {
            let icon_bitmap = wx::Bitmap::new();
            icon_bitmap.copy_from_icon(&wx::ArtProvider::get_icon(
                "wxART_QUESTION",
                "wxART_BUTTON_C",
                &wx::Size::default(),
            ));
            col2.set_bitmap(&wx::BitmapBundle::from(icon_bitmap));
        }
        header.append_column(&col2);

        config_ui.sizer_header.add_stretch_spacer(1);
        config_ui
            .sizer_header
            .add_window_sizerflags(Some(&header), wx::SizerFlags::new(0).expand());
        *self.header.borrow_mut() = Some(header);
        config_ui.sizer_header.add_stretch_spacer(1);
        config_ui.sizer_header.layout();
    }

    fn reset_header_style(&self, config_ui: &ConfigUI) {
        config_ui
            .chk_allow_reorder
            .set_value((wx::HD_DEFAULT_STYLE & wx::HD_ALLOW_REORDER) != 0);
        config_ui
            .chk_allow_hide
            .set_value((wx::HD_DEFAULT_STYLE & wx::HD_ALLOW_HIDE) != 0);
        config_ui
            .chk_bitmap_on_right
            .set_value((wx::HD_DEFAULT_STYLE & wx::HD_BITMAP_ON_RIGHT) != 0);
    }

    fn get_header_style_flags(&self, config_ui: &ConfigUI) -> c_long {
        let mut flags = 0;

        if config_ui.chk_allow_reorder.is_checked() {
            flags |= wx::HD_ALLOW_REORDER;
        }
        if config_ui.chk_allow_hide.is_checked() {
            flags |= wx::HD_ALLOW_HIDE;
        }
        if config_ui.chk_bitmap_on_right.is_checked() {
            flags |= wx::HD_BITMAP_ON_RIGHT;
        }

        flags
    }

    fn reset_column_style(&self, col_setting: &ColSetting) {
        col_setting
            .chk_allow_resize
            .set_value((wx::COL_DEFAULT_FLAGS & wx::COL_RESIZABLE) != 0);
        col_setting
            .chk_allow_reorder
            .set_value((wx::COL_DEFAULT_FLAGS & wx::COL_REORDERABLE) != 0);
        col_setting
            .chk_allow_sort
            .set_value((wx::COL_DEFAULT_FLAGS & wx::COL_SORTABLE) != 0);
        col_setting
            .chk_allow_hide
            .set_value((wx::COL_DEFAULT_FLAGS & wx::COL_HIDDEN) != 0);
        col_setting
            .chk_with_bitmap
            .set_value(COL_WITH_BITMAP_DEFAULT);
        col_setting
            .rb_alignments
            .set_selection(COL_ALIGNMENT_INDEX_DEFAULT);
    }

    fn get_column_style_flags(&self, col_setting: &ColSetting) -> c_long {
        let mut flags = 0;

        if col_setting.chk_allow_resize.is_checked() {
            flags |= wx::COL_RESIZABLE;
        }
        if col_setting.chk_allow_reorder.is_checked() {
            flags |= wx::COL_REORDERABLE;
        }
        if col_setting.chk_allow_sort.is_checked() {
            flags |= wx::COL_SORTABLE;
        }
        if col_setting.chk_allow_hide.is_checked() {
            flags |= wx::COL_HIDDEN;
        }

        flags
    }

    fn get_column_alignment_flag(&self, col_setting: &ColSetting) -> c_int {
        let sel = col_setting.rb_alignments.get_selection();
        if sel == wx::NOT_FOUND {
            COL_ALIGNMENT_FLAG_DEFAULT
        } else {
            COL_ALIGN_FLAGS[sel as usize]
        }
    }

    // ----------------------------------------------------------------------------
    // event handlers
    // ----------------------------------------------------------------------------

    fn on_style_check_or_radio_box(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.recreate_widget(config_ui);
        }
    }

    fn on_reset_button(&self) {
        if let Some(config_ui) = self.config_ui.borrow().as_ref() {
            self.reset_header_style(config_ui);
            for col_setting in config_ui.col_settings.iter() {
                self.reset_column_style(col_setting);
            }
            self.recreate_widget(config_ui);
        }
    }
}
