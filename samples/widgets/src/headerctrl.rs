use crate::WidgetsPage;
use std::cell::RefCell;
use std::os::raw::{c_int, c_long};
use std::rc::Rc;
use wx::methods::*;

const NUMBER_OF_COLUMNS: usize = 4;
const COL_WITH_BITMAP_DEFAULT: bool = false;
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
    header: Rc<RefCell<Option<wx::Gauge>>>,
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

        let btn = wx::Button::builder(Some(&self.base))
            .label("&Reset")
            .build();
        sizer_header.add_window_sizerflags(
            Some(&btn),
            wx::SizerFlags::new(0).center_horizontal().border(wx::ALL),
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
                    .style(wx::RA_SPECIFY_COLS)
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
        self.recreate_widget();

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
        self.reset_header_style(&config_ui);
        *self.config_ui.borrow_mut() = Some(config_ui);
    }

    fn handle_button(&self, event: &wx::CommandEvent) {
        // Do nothing.
    }
    fn handle_checkbox(&self, _: &wx::CommandEvent) {
        // Do nothing.
    }
    fn handle_radiobox(&self, _: &wx::CommandEvent) {
        // Do nothing.
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

    fn recreate_widget(&self) {
        // TODO
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

    fn create_header(&self, config_ui: &ConfigUI) {
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
        if let Some(header) = self.header.borrow().as_ref() {
            val = header.get_value();

            config_ui.sizer_header.detach_window(Some(header));
            header.destroy();
        }

        let header = wx::Gauge::builder(Some(&self.base)).style(flags).build();
        header.set_value(val);

        if (flags & wx::GA_VERTICAL as c_long) != 0 {
            config_ui.sizer_header.add_window_int(
                Some(&header),
                0,
                wx::GROW | wx::ALL,
                5,
                wx::Object::none(),
            );
        } else {
            config_ui.sizer_header.add_window_int(
                Some(&header),
                1,
                wx::CENTRE as i32 | wx::ALL,
                5,
                wx::Object::none(),
            );
        }
        *self.header.borrow_mut() = Some(header);

        // relayout the sizer
        config_ui.sizer_header.layout();
    }
}
