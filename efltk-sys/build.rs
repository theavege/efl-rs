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

    // Detect and report EFL version
    match pkg_config::probe_library(library) {
        Ok(probe) => {
            let version = probe.version;
            println!("cargo:warning=Detected EFL version: {}", version);
            // EFL 1.25+ is required for full widget support
            if let Ok(version) = semver::Version::parse(&version) {
                if version < semver::Version::new(1, 25, 0) {
                    println!(
                        "cargo:warning=EFL version >= 1.25.0 is recommended for full widget support"
                    );
                }
            } else {
                println!("cargo:warning=Could not parse detected EFL version");
            }
        }
        Err(_) => {
            println!("cargo:warning=Could not detect EFL version");
        }
    }

    let mut includes = Vec::new();
    for dir in pkg_config::probe_library(library)?.include_paths {
        includes.push(format!("-I{}", dir.display()));
    }
    Ok(includes)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    // MSVC-style targets (e.g. the `x86_64-pc-windows-gnullvm` triple used by
    // the clang64 MINGW/MSYS2 toolchain) make Clang parse its own AMX headers
    // with Microsoft extensions enabled, which emits anonymous `_str` wrapper
    // structs (`__tile1024i_str`, ...) around 1024-byte vector types. Bindgen
    // then generates a layout test that underflows at compile time and adds a
    // `packed` attribute that conflicts with the type's `align(64)` hint, so
    // compiling `efltk-sys` fails with E0587/E0080. Disable the Microsoft
    // extensions and blocklist the intrinsic wrapper types below.
    let target = env::var("TARGET").unwrap_or_default();
    let mut clang_args = get_include_paths()?;
    if target.contains("windows") {
        clang_args.push("-fno-ms-extensions".to_string());
    }
    bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_args(clang_args)
        .blocklist_type("__tile.*")
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
        // Additional libc internals that may leak through
        .blocklist_function("abort")
        .blocklist_function("abs")
        .blocklist_function("exit")
        .blocklist_function("free")
        .blocklist_function("labs")
        .blocklist_function("llabs")
        .blocklist_function("rand")
        .blocklist_function("srand")
        .blocklist_function("time")
        // EFL-specific: blocklist internal types that shouldn't be exposed
        .blocklist_type("_Efl_Object")
        .blocklist_type("_Evas_Object")
        .blocklist_type("_Elm_Widget")
        .parse_callbacks(Box::new(MacroCallback::default()))
        .generate()?
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))?;
    Ok(())
}
