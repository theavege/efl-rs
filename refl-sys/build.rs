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
    use std::{
        ffi::OsString,
        path::{Path, PathBuf},
        process::Command,
    };

    fn run_command(command: &mut Command, failure: &str) {
        let output = command
            .output()
            .unwrap_or_else(|e| panic!("\x1b[31m{failure}: {e}\x1b[0m"));
        if !output.status.success() {
            panic!(
                "\x1b[31m{failure}\nstdout:\n{}\nstderr:\n{}\x1b[0m",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
        }
    }

    fn windows_home() -> PathBuf {
        env::var_os("USERPROFILE")
            .map(PathBuf::from)
            .or_else(|| {
                let drive = env::var_os("HOMEDRIVE")?;
                let path = env::var_os("HOMEPATH")?;
                let mut home = OsString::from(drive);
                home.push(path);
                Some(PathBuf::from(home))
            })
            .expect("\x1b[31mNeither USERPROFILE nor HOMEDRIVE/HOMEPATH is set\x1b[0m")
    }

    fn prepend_path(path: &Path) -> OsString {
        let old_path = env::var_os("PATH").unwrap_or_default();
        let mut paths = env::split_paths(&old_path).collect::<Vec<_>>();
        paths.insert(0, path.to_path_buf());
        env::join_paths(paths).expect("failed to build PATH")
    }

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out.join("efl-build");
    let ewpi_dir = Path::new("use").join("ewpi");
    let efl_dir = Path::new("use").join("efl");
    let home = windows_home();
    let ewpi_prefix = home.join("ewpi_64");
    let efl_prefix = home.join("efl_64");
    let ewpi_bin = ewpi_prefix.join("bin");
    let pkg_config_path = ewpi_prefix.join("lib").join("pkgconfig");
    let command_path = prepend_path(&ewpi_bin);

    run_command(
        Command::new("git").args([
            "submodule",
            "update",
            "--init",
            "--recursive",
            "--force",
            "--remote",
            "--depth",
            "1",
        ]),
        "Failed to execute 'git submodule update'!",
    );

    run_command(
        Command::new("gcc").current_dir(&ewpi_dir).args([
            "-O2",
            "-std=c99",
            "-o",
            "ewpi.exe",
            "ewpi.c",
            "ewpi_map.c",
            "ewpi_spawn.c",
        ]),
        "Failed to execute gcc!",
    );

    run_command(
        Command::new(ewpi_dir.join("ewpi.exe")).arg("--jobs=8"),
        "Failed to execute ewpi!",
    );

    run_command(
        Command::new("meson")
            .env("EWPI_PATH", &ewpi_prefix)
            .env("PKG_CONFIG_PATH", &pkg_config_path)
            .env(
                "CPPFLAGS",
                format!("-I{}", ewpi_prefix.join("include").display()),
            )
            .env(
                "LDFLAGS",
                format!("-L{}", ewpi_prefix.join("lib").display()),
            )
            .env("PATH", &command_path)
            .args([
                "setup",
                &build_dir.display().to_string(),
                &efl_dir.display().to_string(),
                &format!("--prefix={}", efl_prefix.display()),
                "--libdir=lib",
                "--buildtype=release",
                "--strip",
                "--default-library=shared",
                "-Dsystemd=false",
                "-Dpulseaudio=false",
                "-Dv4l2=false",
                "-Dlibmount=false",
                "-Deeze=false",
                "-Dx11=false",
                "-Dxinput2=false",
                "-Devas-loaders-disabler=pdf,ps,rsvg,json",
                "-Dopengl=none",
                "-Dpixman=true",
                "-Dembedded-lz4=false",
                "-Dfribidi=true",
                "-Dinput=false",
                "-Dbuild-examples=false",
                "-Dbuild-tests=false",
                "-Dbindings=cxx",
                "-Dlua-interpreter=luajit",
                "-Delua=true",
            ]),
        "Failed to execute meson!",
    );

    run_command(
        Command::new("ninja").env("PATH", &command_path).args([
            "-C",
            &build_dir.display().to_string(),
            "install",
        ]),
        "Failed to execute ninja!",
    );

    env::set_var("PATH", &command_path);
    env::set_var(
        "PKG_CONFIG_PATH",
        env::join_paths([efl_prefix.join("lib").join("pkgconfig"), pkg_config_path])
            .expect("failed to build PKG_CONFIG_PATH"),
    );
    let mut includes = Vec::new();
    match pkg_config::probe_library("elementary") {
        Ok(lib) => {
            for dir in lib.include_paths {
                includes.push(format!("-I{}", dir.display()));
            }
        }
        Err(e) => {
            eprintln!("Failed to find elementary: {e}");
            std::process::exit(1);
        }
    }
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
