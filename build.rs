use std::process::Command;

fn main() {
    let source_files = vec![
        // "src/lib.rs",
        "src/generated.rs",
    ];
    wx_config_cflags(&mut cxx_build::bridges(source_files))
        .file("src/wxrust.cc")
        .file("src/wxrust2.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxx-demo");

    print_wx_config_libs_for_cargo();
}

fn wx_config_cflags(cc_build: &mut cc::Build) -> &mut cc::Build {
    // from `wx-config --cflags`
    let cflags = wx_config(&["--cflags"]);
    for arg in cflags.split_whitespace() {
        if arg.starts_with("-I") {
            cc_build.include(&arg[2..]);
        } else if arg.starts_with("-D") {
            let split = &mut arg[2..].split('=');
            cc_build.define(split.next().unwrap(), split.next().unwrap_or(""));
        } else {
            panic!("unsupported argument '{}'. please file a bug.", arg)
        }
    }
    cc_build
}

fn print_wx_config_libs_for_cargo() {
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
    String::from_utf8_lossy(&output.stdout).to_string()
}
