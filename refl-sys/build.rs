use {
    std::{env, path::Path},
    bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks},
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
        ].contains(&name) {
            true => MacroParsingBehavior::Ignore,
            false => MacroParsingBehavior::Default,
        }
    }
}

fn main() {
    let mut cflags = Vec::new();
    let library = "elementary";
    println!("cargo:rustc-link-lib=dylib={library}");
    match pkg_config::probe_library(library) {
        Ok(lib) => for dir in lib.include_paths {
            cflags.push(format!("-I{}", dir.display()));
        }
        Err(e) => {
            eprintln!("Failed to find {library}: {e}");
            std::process::exit(1);
        }
    }
    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&cflags)
        .parse_callbacks(Box::new(MacroCallback::default()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
