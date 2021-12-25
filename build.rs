fn main() {
    cxx_build::bridge("src/main.rs")
        // from `wx-config --cflags`
        .include("/opt/homebrew/lib/wx/include/osx_cocoa-unicode-3.0")
        .include("/opt/homebrew/include/wx-3.0")
        .define("_FILE_OFFSET_BITS", "64")
        .define("WXUSINGDLL", None)
        .define("__WXMAC__", None)
        .define("__WXOSX__", None)
        .define("__WXOSX_COCOA__", None)
        // to here.
        .file("src/wxrust.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxx-demo");

    // getopts 使って option parse すれば簡単に書けそう
    // from `wx-config --libs`
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=Carbon");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=wx_baseu-3.0");
    println!("cargo:rustc-link-lib=framework=System");
    println!("cargo:rustc-link-lib=framework=OpenGL");
    println!("cargo:rustc-link-lib=wx_osx_cocoau_xrc-3.0");
    println!("cargo:rustc-link-lib=wx_osx_cocoau_html-3.0");
    println!("cargo:rustc-link-lib=wx_osx_cocoau_qa-3.0");
    println!("cargo:rustc-link-lib=wx_osx_cocoau_adv-3.0");
    println!("cargo:rustc-link-lib=wx_osx_cocoau_core-3.0");
    println!("cargo:rustc-link-lib=wx_baseu_xml-3.0");
    println!("cargo:rustc-link-lib=wx_baseu_net-3.0");
    println!("cargo:rustc-link-lib=wx_baseu-3.0");
}
