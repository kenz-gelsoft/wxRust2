# `wx`

This crate is a binding to [the wxCore library of the wxWidgets toolkit](https://docs.wxwidgets.org/3.2/page_libs.html#page_libs_wxcore).

This crate is `wxrust` not `wx` in crates.io as the name is already in use.

It is recommended to specify dependency to this library [with renaming](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml) like this:

```TOML
[dependencies]
wx = { version = "0.0.*", package = "wxrust" }
```
