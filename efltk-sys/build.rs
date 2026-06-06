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
    let out_dir = env::var("OUT_DIR").unwrap();
    let app_data = env::var("LocalAppData").unwrap();
    for exe in ["ewpi-x86_64-win10-1.1.exe", "efl-x86_64-win10-1.26.exe"] {
        let path = Path::new(&out_dir).join(exe);
        if !path.exists() {
            let url = format!("https://download.enlightenment.org/rel/win/efl/{exe}");
            let response = reqwest::blocking::Client::new()
                .get(url)
                .timeout(std::time::Duration::from_secs(120))
                .send()
                .expect(&format!("Failed to download {exe}"))
                .bytes()
                .expect(&format!("Failed to read {exe}"));
            std::fs::write(&path, response).expect(&format!("Failed to write {exe}"));
        }
        let dir = Path::new(&out_dir).join(exe.split("-").next().unwrap());
        if !dir.exists() {
            let run = Command::new(path)
                .args(["/NCRC", "/S", &format!("/D={}", dir.display())])
                .output()
                .expect(&format!("\x1b[31mFailed to execute {exe}!\x1b[0m"));
            if !run.status.success() {
                panic!(
                    "\x1b[31mMESON\nstdout:\n{}\nstderr:\n{}\x1b[0m",
                    String::from_utf8_lossy(&run.stdout),
                    String::from_utf8_lossy(&run.stderr),
                );
            }
        }
    }
    println!("cargo:rustc-link-search=native={app_data}\\efl_64\\lib");
    println!("cargo:rustc-link-lib=static=efl");
    let mut includes = Vec::new();
    for source in ["ewpi", "efl"] {
        for entry in glob::glob(&format!("{app_data}\\{source}_64\\**\\*.h"))
            .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => includes.push(format!("-I{}", path.parent().unwrap().display())),
                Err(e) => panic!("Failed to find: {e}"),
            }
        }
    }
    includes.dedup();
    includes
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
