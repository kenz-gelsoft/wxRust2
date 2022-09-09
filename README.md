# wxRust2

This is my 2nd exploration of binding [the wxWidgets cross-platform toolkit](https://www.wxwidgets.org/).

**This project is in the very-early stage of the development.** I'm playing with this in my very limited spare time. So don't expect this project will be usable (or useful) state in the near future.

---

## How to use

1. [Install prerequisite](#prerequisite) or [Specify feature to use vendored wx binary crate](#use-vendored-wx-binary-crate)
2. Specify wxrust dependency
    ```TOML
    [dependencies]
    wx = { version = "0.0.1-alpha", package = "wxrust" }
    ```

See documentation (after crate released) or [wx-core/README.md](wx-core/README.md) for more details.

### Prerequisite

Install the wxWidgets library. Currently (not fully) supported library versions:

|wx   |Windows |macOS   |Linux   |
|-----|--------|--------|--------|
|3.2.0 Installed|[Official MSVC DLL](#windows)|[Homebrew](#macos)|[Codelite repo](#linux)|
|3.2.0 [Vendored](#use-vendored-wx-binary-crate)|MinGW64 Static|Universal Static|N/A|
|3.0.5|N/A|N/A|[Ubuntu Package](#linux)|

Following installation method are (somewhat) tested with:

#### Windows

- Install prebuilt binary and set `%wxwin` environment variable to installed path.
    - See official instruction: https://docs.wxwidgets.org/3.2.0/plat_msw_binaries.html

#### macOS

- Install from Homebrew like `brew install wxwidgets`

#### Linux

- Use codelite repo package
    - see: https://docs.codelite.org/wxWidgets/repo320/
- Or distro package

### Use vendored wx binary crate

You can use prebuilt wx binaries without installing wxWidgets system-wide.

1. Specify `--features vendored` to cargo.
2. Override `wxrust-config` crate dependency with the following git respositry crate (or your fork of it):
    ```toml
    [patch.crates-io]
    wxrust-config = { git = "https://github.com/kenz-gelsoft/wxrust-vendored-config" }
    ```
    - You need to specify git or local separate repo's crate, as `crates.io` won't host crates with (large) binary crates such as this.

This configuration links to following per-build-target crates by default. You should be able to [override this by crate name](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html):

|Build target|Crate name|Git repository|Build configuration|
|------------|----------|--------------|-------------------|
|`cfg(target_os = "macos")`|`wx-universal-apple-darwin`|https://github.com/ancwrd1/wx-universal-apple-darwin|Lean and mean config, see repo.| 
|`x86_64-pc-windows-gnu`|`wx-x86_64-pc-windows-gnu`|https://github.com/ancwrd1/wx-x86_64-pc-windows-gnu|Lean and mean config, see repo.|
|`x86_64-pc-windows-msvc`|`wx-x86_64-pc-windows-msvc`|https://github.com/kenz-gelsoft/wx-x86_64-pc-windows-msvc|Bundled wx3.2.0 official build|

[@ancwrd1](https://github.com/ancwrd1) suggested this feature and kindly helped to support this. Thank you!

---

## License

[MIT License](https://opensource.org/licenses/mit-license.php). but you can (and shoudld) treat this library as [wxWindows Library Licence](https://www.wxwidgets.org/about/licence/) (same with required wxWidgets library dependency).

Large part of this project is the binding generator in Python (doxybindgen). This part may be usable and want to be used for another traditional C++ APIs with Doxygen documented (c.f. like Haiku OS APIs(Kits)) in future. So permissive license is desirable.

### Samples ported from wxWidgets

[samples/](./samples/) subdirectory contains sample codes ported from the wxWidgets Library. These sample codes are licensed under the [wxWindows Library Licence](https://www.wxwidgets.org/about/licence/).
