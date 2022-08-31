# wxRust2

This is my 2nd exploration of binding [the wxWidgets cross-platform toolkit](https://www.wxwidgets.org/).

**This project is in the very-early stage of the development.** I'm playing with this in my very limited spare time. So don't expect this project will be usable (or useful) state in the near future.

## How to use

(T.B.D.)

### Use vendored wx binary crate

Specify `--features vendored` to cargo, to use vendored prebuilt wx binary crate. This configuration links to following per-build-target crates by default. You should be able to [override this by crate name](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html):

|Build target|Crate name|Git repository|Build configuration|
|------------|----------|--------------|-------------------|
|`cfg(target_os = "macos")`|`wx-universal-apple-darwin`|https://github.com/ancwrd1/wx-universal-apple-darwin|Lean and mean config, see repo.| 
|`x86_64-pc-windows-gnu`|`wx-x86_64-pc-windows-gnu`|https://github.com/ancwrd1/wx-x86_64-pc-windows-gnu|Lean and mean config, see repo.|
|`x86_64-pc-windows-msvc`|`wx-x86_64-pc-windows-msvc`|https://github.com/kenz-gelsoft/wx-x86_64-pc-windows-msvc|Bundled wx3.1.7 official build|

[@ancwrd1](https://github.com/ancwrd1) suggested this feature and kindly helped to support this. Thank you!

## License

[MIT License](https://opensource.org/licenses/mit-license.php). but you can (and shoudld) treat this library as [wxWindows Library Licence](https://www.wxwidgets.org/about/licence/) (same with required wxWidgets library dependency).

Large part of this project is the binding generator in Python (doxybindgen). This part may be usable and want to be used for another traditional C++ APIs with Doxygen documented (c.f. like Haiku OS APIs(Kits)) in future. So permissive license is desirable.

### Samples ported from wxWidgets

[samples/](./samples/) subdirectory contains sample codes ported from the wxWidgets Library. These sample codes are licensed under the [wxWindows Library Licence](https://www.wxwidgets.org/about/licence/).
