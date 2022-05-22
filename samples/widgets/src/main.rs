#![windows_subsystem = "windows"]

use std::os::raw::c_int;
use wx;
use wx::methods::*;

// mod activityindicator;
// use activityindicator::*;
mod button;
use button::*;

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
    m_panel: wx::Panel,
    m_book: wx::Notebook,
    current_page: Option<ButtonWidgetsPage>,
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

        let mut frame = WidgetsFrame {
            base: base,
            m_panel: panel,
            m_book: book,
            current_page: None,
        };
        frame.on_create();
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
        let btn = wx::Button::builder(Some(&self.m_panel))
            .id(Widgets::Quit.into())
            .label("E&xit")
            .build();
        sizer_btns.add_window_int(Some(&btn), 0, 0, 0, wx::Object::none());
        sizer_down.add_sizer_sizerflags(
            Some(&sizer_btns),
            wx::SizerFlags::new(0).border(wx::ALL).right(),
        );

        sizer_top.add_window_sizerflags(
            Some(&self.m_book),
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

        self.m_panel.set_sizer(Some(&sizer_top), true);

        // wxPersistentRegisterAndRestore
        let size_min = self.m_panel.get_best_size();

        self.base.set_client_size_size(&size_min);
        self.base.set_min_client_size(&size_min);
    }

    fn current_page(&self) -> wx::Panel {
        // FIXME: figure out a way to avoid cloning wx::Window structs
        self.current_page.as_ref().unwrap().base.clone()
    }

    fn connect_to_widget_events(&self) {
        // TODO
    }

    fn init_book(&mut self) {
        self.current_page = Some(ButtonWidgetsPage::new(&self.m_book));

        self.m_book.add_page(
            Some(&self.current_page()),
            "Button",
            false,
            wx::BookCtrlBase::NO_IMAGE,
        );

        let self_copy = self.clone();
        self.base.bind(
            wx::RustEvent::BookctrlPageChanged,
            move |event: &wx::BookCtrlEvent| {
                self_copy.on_page_changed(event);
            },
        );
    }

    fn on_page_changed(&self, event: &wx::BookCtrlEvent) {
        let sel = event.get_selection();

        let menu_bar = self.base.get_menu_bar().get().unwrap();
        if let Some(item) = menu_bar.find_item((Widgets::GoToPage as c_int) + sel, wx::Menu::none())
        {
            item.check(true);
        }

        menu_bar.check(Widgets::BusyCursor.into(), false);

        let current_page = self.current_page();
        if current_page.get_children().is_empty() {
            // FIXME: figure out a way to avoid cloning wx::Window structs
            let mut mutable_copy = self.current_page.as_ref().unwrap().clone();
            mutable_copy.create_content();
            current_page.layout();

            self.connect_to_widget_events();
        }
    }
}
