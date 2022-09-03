use wxrust_config::{print_wx_config_libs_for_cargo, wx_config_cflags};

fn main() {
    wx_config_cflags(&mut cc::Build::new())
        .cpp(true)
        .file("src/manual.cpp")
        .file("src/generated.cpp")
        .include("include")
        .flag_if_supported("-std=c++14")
        // ignore too many warnings with wx3.0
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-ignored-qualifiers")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("wx");

    print_wx_config_libs_for_cargo();
}
