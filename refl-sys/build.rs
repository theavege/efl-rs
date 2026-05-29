use {
    bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks},
    std::{env, path::Path},
};

#[derive(Debug, Default)]
struct MacroCallback();

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        match [
            "FP_NORMAL",
            "FP_NAN",
            "FP_INFINITE",
            "FP_ZERO",
            "FP_SUBNORMAL",
            "IPPORT_RESERVED",
        ]
        .contains(&name)
        {
            true => MacroParsingBehavior::Ignore,
            false => MacroParsingBehavior::Default,
        }
    }
}

#[cfg(target_os = "windows")]
fn compile() -> Vec<String> {
    use std::process::Command;
    let out = env::var("OUT_DIR").unwrap();
    let home = env::var("HOMEPATH").unwrap();
    let mut run = Command::new("git")
        .args([
            "submodule",
            "update",
            "--init",
            "--recursive",
            "--force",
            "--remote",
            "--depth",
            "1",
        ])
        .output()
        .expect("\x1b[31mFailed to execute 'git submodule update'!\x1b[0m");
    if !run.status.success() {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
    };
    run = Command::new("gcc")
        .current_dir("use/ewpi")
        .args([
            "-O2",
            "-std=c99",
            &format!("-o={home}/ewpi.exe"),
            "ewpi.c",
            "ewpi_map.c",
            "ewpi_spawn.c",
        ])
        .output()
        .expect("\x1b[31mFailed to execute gcc!\x1b[0m");
    if !run.status.success() {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
    };
    run = Command::new(&format!("{home}/ewpi.exe"))
        .current_dir(&format!("{out}"))
        .arg("-–jobs=8")
        .output()
        .expect("\x1b[31mFailed to execute 'ewpi -–jobs=8'!\x1b[0m");
    if !run.status.success() {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
    };
    run = Command::new("meson")
        .env("EWPI_PATH", format!("{out}\\ewpi_64"))
        .env("PKG_CONFIG_PATH", format!("{out}/ewpi_64/lib/pkgconfig"))
        .env("CPPFLAGS", format!("-I{out}/ewpi_64/include"))
        .env("LDFLAGS", format!("-L{out}/ewpi_64/lib"))
        .args([
            "setup",
            &format!("--prefix={out}/efl_64"),
            "--libdir=lib",
            "--buildtype=release",
            "--strip",
            "--default-library shared",
            "-Dsystemd=false",
            "-Dpulseaudio=false",
            "-Dv4l2=false",
            "-Dlibmount=false",
            "-Deeze=false",
            "-Dx11=false",
            "-Dxinput2=false",
            "-Devas-loaders-disabler='pdf','ps','rsvg','json'",
            "-Dopengl=none",
            "-Dpixman=true",
            "-Dembedded-lz4=false",
            "-Dfribidi=true",
            "-Dinput=false",
            "-Dbuild-examples=false",
            "-Dbuild-tests=false",
            "-Dbindings='cxx'",
            "-Dlua-interpreter=luajit",
            "-Delua=true",
            &format!("{out}/build"),
        ])
        .output()
        .expect("\x1b[31mFailed to execute meson!\x1b[0m");
    if !run.status.success() {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
    };
    run = Command::new("ninja")
        .args(["-C", &format!("{out}/build")])
        .output()
        .expect("\x1b[31mFailed to execute ninja!\x1b[0m");
    match run.status.success() {
        true => eprintln!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
        false => panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
    };
    println!("cargo:rustc-link-search=native={out}\\build");
    println!("cargo:rustc-link-lib=static=efl");
    Vec::from([format!("-I{out}/ewpi_64/include")])
}

#[cfg(target_os = "linux")]
fn compile() -> Vec<String> {
    let library = "elementary";
    println!("cargo:rustc-link-lib=dylib={library}");
    let mut includes = Vec::new();
    match pkg_config::probe_library(library) {
        Ok(lib) => {
            for dir in lib.include_paths {
                includes.push(format!("-I{}", dir.display()));
            }
        }
        Err(e) => {
            eprintln!("Failed to find {library}: {e}");
            std::process::exit(1);
        }
    }
    includes
}

fn main() {
    bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_args(compile())
        .parse_callbacks(Box::new(MacroCallback::default()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
