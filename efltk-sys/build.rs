use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use std::{env, error::Error, path::Path};

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

fn get_include_paths() -> Result<Vec<String>, Box<dyn Error>> {
    let library = "elementary";
    println!("cargo:rustc-link-lib=dylib={library}");
    let mut includes = Vec::new();
    for dir in pkg_config::probe_library(library)?.include_paths {
        includes.push(format!("-I{}", dir.display()));
    }
    Ok(includes)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_args(get_include_paths()?)
        // Do not emit bindings for libc symbols (memcmp, strlen, ...). Generated
        // declarations for these trip the `library_definitions` lint and can
        // shadow the real runtime symbols.
        .blocklist_function("bcmp")
        .blocklist_function("bcopy")
        .blocklist_function("bzero")
        .blocklist_function("ffs")
        .blocklist_function("index")
        .blocklist_function("malloc")
        .blocklist_function("memcmp")
        .blocklist_function("memcpy")
        .blocklist_function("memmove")
        .blocklist_function("mempcpy")
        .blocklist_function("memrchr")
        .blocklist_function("memset")
        .blocklist_function("realloc")
        .blocklist_function("rindex")
        .blocklist_function("stpcpy")
        .blocklist_function("stpncpy")
        .blocklist_function("strcasecmp")
        .blocklist_function("strcasestr")
        .blocklist_function("strcat")
        .blocklist_function("strchr")
        .blocklist_function("strchrnul")
        .blocklist_function("strcmp")
        .blocklist_function("strcoll")
        .blocklist_function("strcpy")
        .blocklist_function("strcspn")
        .blocklist_function("strlen")
        .blocklist_function("strncasecmp")
        .blocklist_function("strncat")
        .blocklist_function("strncmp")
        .blocklist_function("strncpy")
        .blocklist_function("strnlen")
        .blocklist_function("strpbrk")
        .blocklist_function("strrchr")
        .blocklist_function("strsep")
        .blocklist_function("strspn")
        .blocklist_function("strstr")
        .blocklist_function("strtok_r")
        .blocklist_function("strtold")
        .blocklist_function("usleep")
        .parse_callbacks(Box::new(MacroCallback::default()))
        .generate()?
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))?;
    Ok(())
}
