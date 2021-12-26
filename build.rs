use std::process::Command;

fn main() {
    cxx_build::bridge("src/lib.rs")
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

    // from `wx-config --libs`
    let libs = wx_config(&["--libs"]);
    let mut next_is_framework_name = false;
    for arg in libs.split_whitespace() {
        if next_is_framework_name {
            println!("cargo:rustc-link-lib=framework={}", arg);
            next_is_framework_name = false;
        } else if arg == "-framework" {
            next_is_framework_name = true;
        } else if arg.starts_with("-L") {
            println!("cargo:rustc-link-search=native={}", &arg[2..]);
        } else if arg.starts_with("-l") {
            println!("cargo:rustc-link-lib={}", &arg[2..]);
        } else {
            panic!("unsupported argument '{}'. please file a bug.", arg)
        }
    }
}

fn wx_config(args: &[&str]) -> String {
	let output = Command::new("wx-config")
        .args(args)
        .output()
        .expect("failed execute wx-config command.");
	String::from_utf8_lossy(&output.stdout)
        .to_string()
}
