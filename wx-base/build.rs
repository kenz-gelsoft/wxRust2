use wxrust_config::wx_config;

fn main() {
    let mut cc_build = cc::Build::new();
    // from `wx-config --cflags`
    let cflags = wx_config(&["--cflags"]);
    for arg in cflags.iter() {
        cc_build.flag(arg);
    }
    cc_build
        .cpp(true)
        .file("src/manual.cpp")
        .file("src/generated/events.cpp")
        .file("src/generated.cpp")
        .include("include")
        .flag_if_supported("-std=c++14")
        // ignore too many warnings with wx3.0
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-ignored-qualifiers")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("wx");

    // from `wx-config --libs`
    let libs = wx_config(&["--libs"]);
    println!("cargo:rustc-flags={}", libs.join(" "));
}
