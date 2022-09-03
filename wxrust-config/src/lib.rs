use std::env;
use std::process::Command;

pub fn wx_config_cflags(cc_build: &mut cc::Build) -> &mut cc::Build {
    // from `wx-config --cflags`
    let cflags = wx_config(&["--cflags"]);
    // ignore too many warnings with wx3.0
    cc_build
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-ignored-qualifiers")
        .flag_if_supported("-Wno-unused-parameter");
    for arg in cflags.iter() {
        cc_build.flag(arg);
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
    for arg in libs.iter() {
        if next_is_framework_name {
            println!("cargo:rustc-link-lib=framework={}", arg);
            next_is_framework_name = false;
        } else if arg == "-framework" {
            next_is_framework_name = true;
        } else if arg.starts_with("-pthread") {
            // ignore
        } else {
            println!("cargo:rustc-flags={}", arg);
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

fn wx_config(args: &[&str]) -> Vec<String> {
    if cfg!(feature = "vendored") {
        let flags: Vec<_> = env::var(format!("DEP_WX_{}_CFLAGS", dep_links()))
            .unwrap()
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect();
        let (ldflags, cflags): (Vec<_>, Vec<_>) = flags
            .into_iter()
            .partition(|f| f.starts_with("-l") || f.starts_with("-L"));
        return if args.contains(&"--cflags") {
            cflags
        } else {
            ldflags
        };
    }

    if cfg!(windows) {
        wx_config_win(args)
    } else {
        let output = Command::new("wx-config")
            .args(args)
            .output()
            .expect("failed execute wx-config command.");
        String::from_utf8_lossy(&output.stdout)
            .to_string()
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect()
    }
}

fn wx_config_win(args: &[&str]) -> Vec<String> {
    let wxwin = env::var("wxwin")
        .expect("Set 'wxwin' environment variable to point the wxMSW binaries dir.");
    // TODO: support linking with the wx debug DLL
    let is_debug = false; //env::var("PROFILE").unwrap() == "debug";
    let d_or_not = if is_debug { "d" } else { "" };
    if args.contains(&"--cflags") {
        let mut cflags = vec![
            format!("-I{}\\include", wxwin),
            format!("-I{}\\lib\\vc14x_x64_dll\\mswu{}", wxwin, d_or_not),
            "-DWXUSINGDLL".to_string(),
        ];
        if is_debug {
            cflags.push("-D_DEBUG".to_string());
        } else {
            cflags.push("-D__NO_VC_CRTDBG__".to_string());
            cflags.push("-DwxDEBUG_LEVEL=0".to_string());
            cflags.push("-DNDEBUG".to_string());
        }
        cflags
    } else {
        let libs = vec![
            format!("-L{}\\lib\\vc14x_x64_dll", wxwin),
            format!("-lwxbase32u{}", d_or_not),
            format!("-lwxmsw32u{}_core", d_or_not),
        ];
        libs
    }
}
