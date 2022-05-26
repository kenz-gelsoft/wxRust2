# wxRust2

This is my 2nd exploration of binding [the wxWidgets cross-platform toolkit](https://www.wxwidgets.org/).

**This project is in the very-early stage of the development.** I'm playing with this in my very limited spare time. So don't expect this project will be usable (or useful) state in the near future.

## Goals (not to resurrect [the predecessor](https://github.com/kenz-gelsoft/wxRust) for):

* Smaller footprint by supporting [the wx's multilib configuration](https://docs.wxwidgets.org/trunk/page_libs.html) (in code size), and avoiding Box-ing everything (in calling overhead).
    * I'm not convinced forcing generic programming rather than trait object flexibility, but gtk-rs looks like usable with this choice AFAIK
* Simpler integration with the Rust ecosystem by utilizing [the cc crate](https://crates.io/crates/cc) rather than CMake and of course, the Cargo.
* Easier to support new APIs by depending wx directly and generating both Rust and C(++) parts of the binding from the single richer metadata ([Doxygen XML output](https://www.doxygen.nl/manual/customize.html#xmlgenerator), yes, [the proven way by the wxPython Phoenix project](https://wiki.wxpython.org/ProjectPhoenix/DevelopmentProcess)).

## Current status

Implementing the binding generator to be usable state with a small set of wx classes.

* Working on
    * [ ] https://github.com/kenz-gelsoft/wxRust2/issues/61 Core library coverage
        * generate for all classes in core library
* Next
    * [ ] Other libraries
        * after that, more libraries would be generated
* Done
    * [x] Investigating modern binding approaches
        * rust-bindgen's C++ support: expected to be the easiest way to binding exising C++ APIs, but it doesn't support some edge cases. especially methods inlined and absent in binary.
        * autocxx: this is for newly(specially) designed API. it supports limited but sane subset of the C++ construct. not suitable for binding an existing API (when I investigated).
        * cxx.rs: this is also for newly designed API, but it allows calling C++ methods handy by generating only the rust API definition and ctor manually.
    * [x] Prototyping handy-binding with cxx.rs
        * minimal effort to generate first some methods bindings, [it looks promising then](https://github.com/kenz-gelsoft/wxRust2/blob/99051ba57160f76aa999c4f8d15ed0b6c08188c2/src/lib.rs). small part of the API could be generated with this approach.
        * but found I need some hacks and codegen in C++ header/source code to work around cxx.rs limitations to support wider range of API.
    * [x] Codegen without cxx.rs (or rust-bindgen)
        * already generated almost the Rust code/C++ header/C++ source triple then. [so no need to rust-bindgen/cxx.rs like solution anymore.](https://github.com/kenz-gelsoft/wxRust2/pull/17)
    * [x] Lifetime management
        * owned non-wxWindow wxObjects has value semantics (impl Drop to call native dtor)
        * https://github.com/kenz-gelsoft/wxRust2/issues/51 planning to use wxTrackable for WeakRef-like wx managed (non-owned) object
        * doesn't manage wxWindow object for now (almost all of them will be managed by wx's windowing system)
    * [x] Small set of API working
        * compiles and bare simple example runs on macOS, Linux
        * working to compile on Window (fighting with linking error)

## License

[MIT License](https://opensource.org/licenses/mit-license.php). but you can (and shoudld) treat this library as [wxWindows Library Licence](https://www.wxwidgets.org/about/licence/) (same with required wxWidgets library dependency).

Large part of this project is the binding generator in Python (doxybindgen). This part may be usable and want to be used for another traditional C++ APIs with Doxygen documented (c.f. like Haiku OS APIs(Kits)) in future. So permissive license is desirable.
