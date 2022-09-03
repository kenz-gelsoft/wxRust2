use wxrust_config::{wx_config, wx_config_cflags};

fn main() {
    wx_config_cflags(&mut cc::Build::new())
        .cpp(true)
        .file("src/manual.cpp")
        .file("src/generated.cpp")
        .include("include")
        .flag_if_supported("-std=c++14")
        .compile("wx");

    // from `wx-config --libs`
    let libs = wx_config(&["--libs"]);
    println!("cargo:rustc-flags={}", libs.join(" "));
}
