use std::env;

pub fn wx_config_cflags(cc_build: &mut cc::Build) -> &mut cc::Build {
    // from `wx-config --cflags`
    let cflags = wx_config(&["--cflags"]);
    // ignore too many warnings with wx3.0
    cc_build
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-ignored-qualifiers")
        .flag_if_supported("-Wno-unused-parameter");
    for arg in cflags.split_whitespace() {
        if arg.starts_with("-I") {
            cc_build.include(&arg[2..]);
        } else if arg.starts_with("-D") {
            let split = &mut arg[2..].split('=');
            cc_build.define(split.next().unwrap(), split.next().unwrap_or(""));
        } else if arg.starts_with("-pthread") {
            cc_build.flag(arg);
        } else {
            panic!("unsupported argument '{}'. please file a bug.", arg)
        }
    }
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    if target_env.eq("msvc") {
        cc_build.flag("/EHsc");
    }
    cc_build
}

pub fn print_wx_config_libs_for_cargo() {
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
        } else if arg.starts_with("-pthread") {
            // ignore
        } else {
            panic!("unsupported argument '{}'. please file a bug.", arg)
        }
    }
}

fn dep_links() -> String {
    let target = env::var("TARGET").unwrap().replace('-', "_").to_uppercase();
    if target.contains("APPLE") {
        "UNIVERSAL_APPLE_DARWIN".to_owned()
    } else {
        target
    }
}

fn wx_config(args: &[&str]) -> String {
    let flags: Vec<_> = env::var(format!("DEP_WX_{}_CFLAGS", dep_links()))
        .unwrap()
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect();
    let (ldflags, cflags): (Vec<_>, Vec<_>) = flags
        .into_iter()
        .partition(|f| f.starts_with("-l") || f.starts_with("-L"));
    return if args.contains(&"--cflags") {
        cflags.join(" ")
    } else {
        ldflags.join(" ")
    };
}
