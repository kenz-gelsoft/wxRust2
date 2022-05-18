#![windows_subsystem = "windows"]

use std::os::raw::c_int;
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
}
impl WidgetsFrame {
    fn new(title: &str) -> Self {
        let frame = wx::Frame::builder(wx::Window::none()).title(title).build();

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
                &wx::Menu::new()
                    .radio_item(Widgets::BorderDefault, "De&fault\tCtrl-Shift-9")
                    .radio_item(Widgets::BorderNone, "&None\tCtrl-Shift-0")
                    .radio_item(Widgets::BorderSimple, "&Simple\tCtrl-Shift-1")
                    .radio_item(Widgets::BorderDouble, "&Double\tCtrl-Shift-2")
                    .radio_item(Widgets::BorderStatic, "Stati&c\tCtrl-Shift-3")
                    .radio_item(Widgets::BorderRaised, "&Raised\tCtrl-Shift-4")
                    .radio_item(Widgets::BorderSunken, "S&unken\tCtrl-Shift-5"),
                "Set &border",
            )
            .sub_menu(
                &wx::Menu::new()
                    .radio_item(Widgets::VariantMini, "&Mini\tCtrl-Shift-6")
                    .radio_item(Widgets::VariantSmall, "&Small\tCtrl-Shift-7")
                    .radio_item(Widgets::VariantNormal, "&Normal\tCtrl-Shift-8")
                    .radio_item(Widgets::VariantLarge, "&Large\tCtrl-Shift-9"),
                "Set &variant",
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
            frame.get_layout_direction() == wx::Layout_RightToLeft,
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

        frame.set_menu_bar(Some(&mbar));

        mbar.check(Widgets::Enable.into(), true);
        mbar.check(Widgets::Show.into(), true);

        mbar.check(Widgets::VariantNormal.into(), true);

        WidgetsFrame { base: frame }
    }
}
