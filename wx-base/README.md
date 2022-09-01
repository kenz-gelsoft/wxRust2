# wx-base(wxrust-base)

This crate is a binding to [the wxBase library of the wxWidgets toolkit](https://docs.wxwidgets.org/3.2/page_libs.html#page_libs_wxbase).

This crate supports only small part of the library which is used by the `wx`(`wxrust`) crate.

Original C++ library supports basic non-GUI cross-platform programming, but this crate doesn't bacause there is many option already for that purpose in the Rust ecosystem.

Members of this crate are re-exported by the `wx`(`wxrust`) crate, so you don't need specify dependency to this crate.
