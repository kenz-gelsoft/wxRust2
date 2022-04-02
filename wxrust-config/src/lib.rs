use std::env;
use std::process::Command;

pub fn wx_config_cflags(cc_build: &mut cc::Build) -> &mut cc::Build {
    // from `wx-config --cflags`
    let cflags = wx_config(&["--cflags"]);
    // ignore too many warnings with wx3.0
    cc_build.flag_if_supported("-Wno-deprecated-copy")
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
    // required to use DLLs
    if cfg!(windows) {
        cc_build.define("WXUSINGDLL", "");
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

fn wx_config(args: &[&str]) -> String {
    if cfg!(windows) {
        let wxwin = env::var("wxwin")
            .expect("Set 'wxwin' environment variable to point the wxMSW binaries dir.");
        if args.contains(&"--cflags") {
            let includes = vec![
                format!("-I{}\\include", wxwin),
                // TODO: determine this name automatically
                format!("-I{}\\lib\\vc14x_x64_dll\\mswud", wxwin),
            ];
            includes.join(" ")
        } else {
            let libs = vec![
                // TODO: determine this name automatically
                format!("-L{}\\lib\\vc14x_x64_dll", wxwin),
                // TODO: determine libraries list automatically
                "-lwxbase31ud".to_string(),
                "-lwxmsw31ud_core".to_string(),
            ];
            libs.join(" ")
        }
    } else {
        let output = Command::new("wx-config")
            .args(args)
            .output()
            .expect("failed execute wx-config command.");
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
