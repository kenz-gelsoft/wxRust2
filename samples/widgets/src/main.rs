#![windows_subsystem = "windows"]

use std::os::raw::{c_int, c_long};
use wx;
use wx::methods::*;

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
const TextEntry_Begin: c_int = TextEntry::DisableAutoComplete as c_int;
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
}
impl WidgetsFrame {
    fn new(title: &str) -> Self {
        let frame = wx::Frame::builder(wx::Window::none()).title(title).build();

        let mbar = wx::MenuBar::new(0);

        use Widgets::*;
        let menu_widget = wx::Menu::new();

        append_item(&menu_widget, SetTooltip, "Set &tooltip...\tCtrl-T");
        menu_widget.append_separator();

        append_item(&menu_widget, SetFgColour, "Set &foreground...\tCtrl-F");
        append_item(&menu_widget, SetBgColour, "Set &background...\tCtrl-B");
        append_item(
            &menu_widget,
            SetPageBg,
            "Set &page background...\tShift-Ctrl-B",
        );
        append_item(&menu_widget, SetFont, "Set f&ont...\tCtrl-O");
        append_check_item(&menu_widget, Enable, "&Enable/disable\tCtrl-E");
        append_check_item(&menu_widget, Show, "Show/Hide");

        let menu_borders = wx::Menu::new();
        append_radio_item(&menu_borders, BorderDefault, "De&fault\tCtrl-Shift-9");
        append_radio_item(&menu_borders, BorderNone, "&None\tCtrl-Shift-0");
        append_radio_item(&menu_borders, BorderSimple, "&Simple\tCtrl-Shift-1");
        append_radio_item(&menu_borders, BorderDouble, "&Double\tCtrl-Shift-2");
        append_radio_item(&menu_borders, BorderStatic, "Stati&c\tCtrl-Shift-3");
        append_radio_item(&menu_borders, BorderRaised, "&Raised\tCtrl-Shift-4");
        append_radio_item(&menu_borders, BorderSunken, "S&unken\tCtrl-Shift-5");
        menu_widget.append_sub_menu(Some(&menu_borders), "Set &border", "");

        let menu_variants = wx::Menu::new();
        append_radio_item(&menu_variants, VariantMini, "&Mini\tCtrl-Shift-6");
        append_radio_item(&menu_variants, VariantSmall, "&Small\tCtrl-Shift-7");
        append_radio_item(&menu_variants, VariantNormal, "&Normal\tCtrl-Shift-8");
        append_radio_item(&menu_variants, VariantLarge, "&Large\tCtrl-Shift-9");
        menu_widget.append_sub_menu(Some(&menu_variants), "Set &variant", "");

        menu_widget.append_separator();
        append_check_item(
            &menu_widget,
            LayoutDirection,
            "Toggle &layout direction\tCtrl-L",
        );
        menu_widget.check(
            LayoutDirection.into(),
            frame.get_layout_direction() == wx::Layout_RightToLeft,
        );

        menu_widget.append_separator();
        append_check_item(
            &menu_widget,
            GlobalBusyCursor,
            "Toggle &global busy cursor\tCtrl-Shift-U",
        );
        append_check_item(&menu_widget, BusyCursor, "Toggle b&usy cursor\tCtrl-U");

        menu_widget.append_separator();
        append_item(&menu_widget, wx::ID_EXIT, "&Quit\tCtrl-Q");
        mbar.append(Some(&menu_widget), "&Widget");

        use TextEntry::*;
        let menu_text_entry = wx::Menu::new();
        append_radio_item(
            &menu_text_entry,
            DisableAutoComplete,
            "&Disable auto-completion",
        );
        append_radio_item(
            &menu_text_entry,
            AutoCompleteFixed,
            "Fixed-&list auto-completion",
        );
        append_radio_item(
            &menu_text_entry,
            AutoCompleteFilenames,
            "&Files names auto-completion",
        );
        append_radio_item(
            &menu_text_entry,
            AutoCompleteDirectories,
            "&Directories names auto-completion",
        );
        append_radio_item(
            &menu_text_entry,
            AutoCompleteCustom,
            "&Custom auto-completion",
        );
        append_radio_item(
            &menu_text_entry,
            AutoCompleteKeyLength,
            "Custom with &min length",
        );
        menu_text_entry.append_separator();
        append_item(&menu_text_entry, SetHint, "Set help &hint");
        mbar.append(Some(&menu_text_entry), "&Text");

        frame.set_menu_bar(Some(&mbar));

        mbar.check(Widgets::Enable.into(), true);
        mbar.check(Widgets::Show.into(), true);

        mbar.check(Widgets::VariantNormal.into(), true);

        WidgetsFrame { base: frame }
    }
}

fn append_item<ID: Into<c_int>>(menu: &wx::Menu, id: ID, s: &str) {
    menu.append_int_str(id.into(), s, "", wx::ITEM_NORMAL);
}
fn append_check_item<ID: Into<c_int>>(menu: &wx::Menu, id: ID, s: &str) {
    menu.append_check_item(id.into(), s, "");
}
fn append_radio_item<ID: Into<c_int>>(menu: &wx::Menu, id: ID, s: &str) {
    menu.append_radio_item(id.into(), s, "");
}
