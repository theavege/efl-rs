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
    Command::new("git")
        .args([
            "submodule",
            "update",
            "--init",
            "--depth",
            "1",
            "--recursive",
            "--force",
            "--remote",
        ])
        .status()
        .unwrap();
    Command::new("gcc")
        .args(["-std=c99", "-o", "ewpi", "ewpi.c", "ewpi_map.c"])
        .status()
        .unwrap();
    Command::new("meson").args(["setup"]).status().unwrap();
    println!("cargo:rustc-link-search=native={}/build", out);
    println!("cargo:rustc-link-lib=static=efl");
    Vec::new()
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
