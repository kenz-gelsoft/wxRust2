# wx(wxrust)

This crate is a binding to [the wxCore library of the wxWidgets toolkit](https://docs.wxwidgets.org/3.2/page_libs.html#page_libs_wxcore).

This crate is `wxrust` not `wx` in crates.io as the name is already in use.

It is recommended to specify dependency to this library [with renaming](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml) like this:

```TOML
[dependencies]
wx = { version = "0.0.*", package = "wxrust" }
```

## Example

```compile_fail
#![windows_subsystem = "windows"]

use wx;
use wx::methods::*;

fn main() {
    wx::App::run(|_| {
        let frame = wx::Frame::builder(wx::Window::none())
            .title("Hello, 世界")
            .build();
        let button = wx::Button::builder(Some(&frame)).label("Greet").build();
        let i = 3;
        println!("i={}", i);
        let weak_button = button.to_weak_ref();
        button.bind(wx::RustEvent::Button, move |_: &wx::CommandEvent| {
            if let Some(button) = weak_button.get() {
                println!("i={}", i);
                button.set_label("clicked");
                println!("s={}", button.get_label())
            }
        });
        frame.centre(wx::BOTH);
        frame.show(true);
    });
}
```
