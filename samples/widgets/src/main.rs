#![windows_subsystem = "windows"]

use std::os::raw::c_int;
use std::rc::Rc;
use wx;
use wx::methods::*;

// mod activityindicator;
// use activityindicator::*;
mod button;
use button::*;

mod checkbox;
use checkbox::*;

mod choice;
use choice::*;

mod clrpicker;
use clrpicker::*;

mod combobox;
use combobox::*;

mod datepicker;
use datepicker::*;

enum Widgets {
    ClearLog = 100,
    Quit,

    BookCtrl,

    SetTooltip,
    SetFgColour,
    SetBgColour,
    SetPageBg,
    SetFont,
    Enable,
    Show,

    BorderNone,
    BorderStatic,
    BorderSimple,
    BorderRaised,
    BorderSunken,
    BorderDouble,
    BorderDefault,

    VariantNormal,
    VariantSmall,
    VariantMini,
    VariantLarge,

    LayoutDirection,

    GlobalBusyCursor,
    BusyCursor,

    GoToPage,
    GoToPageLast = Widgets::GoToPage as isize + 100,

    End,
}
impl From<Widgets> for c_int {
    fn from(w: Widgets) -> Self {
        w as c_int
    }
}

enum TextEntry {
    DisableAutoComplete = Widgets::End as isize,
    AutoCompleteFixed,
    AutoCompleteFilenames,
    AutoCompleteDirectories,
    AutoCompleteCustom,
    AutoCompleteKeyLength,

    SetHint,
    End,
}
const TEXT_ENTRY_BEGIN: c_int = TextEntry::DisableAutoComplete as c_int;
impl From<TextEntry> for c_int {
    fn from(te: TextEntry) -> Self {
        te as c_int
    }
}

fn main() {
    wx::App::run(|_| {
        // TODO
        // SetVendorName("wxWidgets_Samples");

        let title = if cfg!(windows) {
            "wxMSW"
        } else if cfg!(target_os = "macos") {
            "wxMAC"
        } else {
            "wxGTK"
        };

        let frame = WidgetsFrame::new(&format!("{} widgets demo", title));
        frame.base.show(true);
    });
}

#[derive(Clone)]
struct WidgetsFrame {
    base: wx::Frame,
    panel: wx::Panel,
    book: wx::Notebook,
    pages: Vec<Rc<dyn WidgetsPage>>,
}
impl WidgetsFrame {
    fn new(title: &str) -> Self {
        let base = wx::Frame::builder(wx::Window::none()).title(title).build();
        let panel = wx::Panel::builder(Some(&base)).build();

        let style = wx::BK_DEFAULT;
        let book = wx::Notebook::builder(Some(&panel))
            .id(Widgets::BookCtrl.into())
            .style(style.into())
            .build();

        let button_page = Rc::new(ButtonWidgetsPage::new(&book));
        let check_box_page = Rc::new(CheckBoxWidgetsPage::new(&book));
        let choice_page = Rc::new(ChoiceWidgetsPage::new(&book));
        let clrpicker_page = Rc::new(ColourPickerWidgetsPage::new(&book));
        let combobox_page = Rc::new(ComboboxWidgetsPage::new(&book));
        let datepicker_page = Rc::new(DatePickerWidgetsPage::new(&book));
        let mut frame = WidgetsFrame {
            base,
            panel,
            book,
            pages: vec![
                button_page,
                check_box_page,
                choice_page,
                clrpicker_page,
                combobox_page,
                datepicker_page,
            ],
        };
        frame.on_create();

        let frame_copy = frame.clone();
        frame
            .base
            .bind(wx::RustEvent::Button, move |event: &wx::CommandEvent| {
                frame_copy.current_page().handle_button(event);
            });
        let frame_copy = frame.clone();
        frame
            .base
            .bind(wx::RustEvent::CheckBox, move |event: &wx::CommandEvent| {
                frame_copy.current_page().handle_checkbox(event);
            });
        let frame_copy = frame.clone();
        frame
            .base
            .bind(wx::RustEvent::RadioBox, move |event: &wx::CommandEvent| {
                frame_copy.current_page().handle_radiobox(event);
            });

        frame
    }
    fn on_create(&mut self) {
        let mbar = wx::MenuBar::new(0);

        let menu_widget = wx::Menu::new()
            .item(Widgets::SetTooltip, "Set &tooltip...\tCtrl-T")
            .separator()
            .item(Widgets::SetFgColour, "Set &foreground...\tCtrl-F")
            .item(Widgets::SetBgColour, "Set &background...\tCtrl-B")
            .item(Widgets::SetPageBg, "Set &page background...\tShift-Ctrl-B")
            .item(Widgets::SetFont, "Set f&ont...\tCtrl-O")
            .check_item(Widgets::Enable, "&Enable/disable\tCtrl-E")
            .check_item(Widgets::Show, "Show/Hide")
            .sub_menu(
                "Set &border",
                &wx::Menu::new()
                    .radio_item(Widgets::BorderDefault, "De&fault\tCtrl-Shift-9")
                    .radio_item(Widgets::BorderNone, "&None\tCtrl-Shift-0")
                    .radio_item(Widgets::BorderSimple, "&Simple\tCtrl-Shift-1")
                    .radio_item(Widgets::BorderDouble, "&Double\tCtrl-Shift-2")
                    .radio_item(Widgets::BorderStatic, "Stati&c\tCtrl-Shift-3")
                    .radio_item(Widgets::BorderRaised, "&Raised\tCtrl-Shift-4")
                    .radio_item(Widgets::BorderSunken, "S&unken\tCtrl-Shift-5"),
            )
            .sub_menu(
                "Set &variant",
                &wx::Menu::new()
                    .radio_item(Widgets::VariantMini, "&Mini\tCtrl-Shift-6")
                    .radio_item(Widgets::VariantSmall, "&Small\tCtrl-Shift-7")
                    .radio_item(Widgets::VariantNormal, "&Normal\tCtrl-Shift-8")
                    .radio_item(Widgets::VariantLarge, "&Large\tCtrl-Shift-9"),
            )
            .separator()
            .check_item(Widgets::LayoutDirection, "Toggle &layout direction\tCtrl-L")
            .separator()
            .check_item(
                Widgets::GlobalBusyCursor,
                "Toggle &global busy cursor\tCtrl-Shift-U",
            )
            .check_item(Widgets::BusyCursor, "Toggle b&usy cursor\tCtrl-U")
            .separator()
            .item(wx::ID_EXIT, "&Quit\tCtrl-Q");

        menu_widget.check(
            Widgets::LayoutDirection.into(),
            self.base.get_layout_direction() == wx::Layout_RightToLeft,
        );
        mbar.append(Some(&menu_widget), "&Widget");

        let menu_text_entry = wx::Menu::new()
            .radio_item(TextEntry::DisableAutoComplete, "&Disable auto-completion")
            .radio_item(TextEntry::AutoCompleteFixed, "Fixed-&list auto-completion")
            .radio_item(
                TextEntry::AutoCompleteFilenames,
                "&Files names auto-completion",
            )
            .radio_item(
                TextEntry::AutoCompleteDirectories,
                "&Directories names auto-completion",
            )
            .radio_item(TextEntry::AutoCompleteCustom, "&Custom auto-completion")
            .radio_item(TextEntry::AutoCompleteKeyLength, "Custom with &min length")
            .separator()
            .item(TextEntry::SetHint, "Set help &hint");
        mbar.append(Some(&menu_text_entry), "&Text");

        self.base.set_menu_bar(Some(&mbar));

        mbar.check(Widgets::Enable.into(), true);
        mbar.check(Widgets::Show.into(), true);

        mbar.check(Widgets::VariantNormal.into(), true);

        // create controls
        let sizer_top = wx::BoxSizer::new(wx::VERTICAL);

        self.init_book();

        let sizer_down = wx::BoxSizer::new(wx::VERTICAL);

        let sizer_btns = wx::BoxSizer::new(wx::HORIZONTAL);
        let btn = wx::Button::builder(Some(&self.panel))
            .id(Widgets::Quit.into())
            .label("E&xit")
            .build();
        sizer_btns.add_window_int(Some(&btn), 0, 0, 0, wx::Object::none());
        sizer_down.add_sizer_sizerflags(
            Some(&sizer_btns),
            wx::SizerFlags::new(0).border(wx::ALL).right(),
        );

        sizer_top.add_window_sizerflags(
            Some(&self.book),
            wx::SizerFlags::new(1)
                .expand()
                .double_border(wx::ALL & !(wx::TOP | wx::BOTTOM)),
        );
        sizer_top.add_spacer(5);
        sizer_top.add_sizer_sizerflags(
            Some(&sizer_down),
            wx::SizerFlags::new(0)
                .expand()
                .double_border(wx::ALL & !wx::TOP),
        );

        self.panel.set_sizer(Some(&sizer_top), true);

        // wxPersistentRegisterAndRestore
        let size_min = self.panel.get_best_size();

        self.base.set_client_size_size(&size_min);
        self.base.set_min_client_size(&size_min);
    }

    fn init_book(&mut self) {
        for page in self.pages.iter() {
            self.book.add_page(
                Some(page.base()),
                page.label(),
                false,
                wx::BookCtrlBase::NO_IMAGE,
            );
        }

        let self_copy = self.clone();
        self.base.bind(
            wx::RustEvent::BookctrlPageChanged,
            move |event: &wx::BookCtrlEvent| {
                let sel = event.get_selection();
                self_copy.on_page_changed(sel);
            },
        );

        // for other books set selection twice to force connected event handler
        // to force lazy creation of initial visible content
        self.book.set_selection(1);
        self.book.set_selection(0);
    }

    fn current_page<'a>(&'a self) -> &'a Rc<dyn WidgetsPage> {
        let selection = self.book.get_selection();
        let index = if selection < 0 { 0 } else { selection } as usize;
        &self.pages[index]
    }

    fn connect_to_widget_events(&self) {
        // TODO
    }

    fn on_page_changed(&self, sel: c_int) {
        let menu_bar = self.base.get_menu_bar().get().unwrap();
        if let Some(item) = menu_bar.find_item((Widgets::GoToPage as c_int) + sel, wx::Menu::none())
        {
            item.check(true);
        }

        menu_bar.check(Widgets::BusyCursor.into(), false);

        // create the pages on demand, otherwise the sample startup is too slow as
        // it creates hundreds of controls
        let cur_page = self.current_page();
        if cur_page.base().get_children().is_empty() {
            cur_page.create_content();
            cur_page.base().layout();
        }
    }
}

trait WidgetsPage {
    fn base(&self) -> &wx::Panel;
    fn label(&self) -> &str;
    fn handle_button(&self, event: &wx::CommandEvent);
    fn handle_checkbox(&self, event: &wx::CommandEvent);
    fn handle_radiobox(&self, event: &wx::CommandEvent);
    fn create_content(&self);

    // Utility methods

    fn create_sizer_with_text(
        &self,
        control: &wx::Control,
        id: c_int,
    ) -> (wx::BoxSizer, wx::TextCtrl) {
        let sizer_row = wx::BoxSizer::new(wx::HORIZONTAL);
        let text = wx::TextCtrl::builder(Some(self.base()))
            .id(id)
            .style(wx::TE_PROCESS_ENTER.into())
            .build();

        sizer_row.add_window_int(
            Some(control),
            0,
            wx::RIGHT | wx::ALIGN_CENTRE_VERTICAL,
            5,
            wx::Object::none(),
        );
        sizer_row.add_window_int(
            Some(&text),
            1,
            wx::LEFT | wx::ALIGN_CENTRE_VERTICAL,
            5,
            wx::Object::none(),
        );

        (sizer_row, text)
    }

    // create a sizer containing a label and a text ctrl
    fn create_sizer_with_text_and_label(
        &self,
        label: &str,
        id: c_int,
    ) -> (wx::BoxSizer, wx::TextCtrl) {
        let static_text = wx::StaticText::builder(Some(self.base()))
            .label(label)
            .build();
        let as_control = unsafe {
            // FIXME: Support this safe upcast as safecall
            wx::Control::from_ptr(static_text.as_ptr())
        };
        self.create_sizer_with_text(&as_control, id)
    }

    // create a sizer containing a button and a text ctrl
    fn create_sizer_with_text_and_button(
        &self,
        id_btn: c_int,
        label: &str,
        id: c_int,
    ) -> (wx::BoxSizer, wx::TextCtrl) {
        let btn = wx::Button::builder(Some(self.base()))
            .id(id_btn)
            .label(label)
            .build();
        let as_control = unsafe {
            // FIXME: Support this safe upcast as safecall
            wx::Control::from_ptr(btn.as_ptr())
        };
        self.create_sizer_with_text(&as_control, id)
    }

    fn create_check_box_and_add_to_sizer(
        &self,
        sizer: &wx::StaticBoxSizer,
        label: &str,
        id: c_int,
    ) -> wx::CheckBox {
        let checkbox = wx::CheckBox::builder(Some(self.base()))
            .id(id)
            .label(label)
            .build();
        sizer.add_window_sizerflags(Some(&checkbox), wx::SizerFlags::new(0).double_horz_border());
        sizer.add_spacer(2);

        return checkbox;
    }
}
