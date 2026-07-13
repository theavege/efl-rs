//! Build script for efltk-sys
//!
//! This script handles:
//! - On Linux: Uses pkg-config to find EFL libraries
//! - On Windows: Downloads and installs EFL binaries automatically

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use std::{env, path::Path};

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
use std::{fs::File, io::Write, path::PathBuf, process::Command};

#[cfg(target_os = "windows")]
fn get_efl_install_dir() -> PathBuf {
    // Check if EFL is already installed via environment variable
    if let Ok(efl_dir) = env::var("EFL_DIR") {
        return PathBuf::from(efl_dir);
    }

    // Default to LocalAppData/efl
    let app_data = env::var("LocalAppData").unwrap_or_else(|_| {
        // Fallback for CI environments
        env::var("APPDATA").unwrap_or_else(|_| {
            eprintln!("Cannot determine AppData directory");
            eprintln!("Please set EFL_DIR environment variable to the EFL installation directory");
            std::process::exit(1);
        })
    });
    PathBuf::from(app_data).join("efl")
}

#[cfg(target_os = "windows")]
fn download_file(url: &str, destination: &Path) {
    eprintln!("Downloading: {url}");
    eprintln!("  to: {}", destination.display());

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .expect("Failed to create HTTP client");

    let response = client
        .get(url)
        .send()
        .expect(&format!("Failed to download: {url}"));

    if !response.status().is_success() {
        eprintln!("HTTP error {}: {}", response.status(), url);
        eprintln!("\nPlease download the EFL Windows installer manually from:");
        eprintln!("  https://www.enlightenment.org/download");
        eprintln!("\nAnd extract it to:");
        eprintln!("  {}", destination.parent().unwrap().display());
        std::process::exit(1);
    }

    let bytes = response
        .bytes()
        .expect(&format!("Failed to read response: {url}"));

    let mut file = File::create(destination)
        .expect(&format!("Failed to create file: {}", destination.display()));

    file.write_all(&bytes)
        .expect(&format!("Failed to write file: {}", destination.display()));

    eprintln!("  Downloaded successfully");
}

#[cfg(target_os = "windows")]
fn execute_command(command: &mut Command) -> Vec<u8> {
    let output = command.output().expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Command failed:");
        eprintln!("  stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("  stderr: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    output.stdout
}

#[cfg(target_os = "windows")]
fn compile() -> Vec<String> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let efl_dir = get_efl_install_dir();

    // Create EFL directory if it doesn't exist
    if !efl_dir.exists() {
        std::fs::create_dir_all(&efl_dir).expect("Failed to create EFL directory");
    }

    // Check if EFL is already installed
    let efl_lib = efl_dir.join("efl_64").join("lib");
    let efl_include = efl_dir.join("efl_64").join("include");

    if !efl_lib.exists() || !efl_include.exists() {
        // Download and install EFL
        eprintln!("EFL not found, downloading...");

        // Try to download the latest EFL Windows installer
        // Note: These URLs may change. Check https://www.enlightenment.org/download for updates.
        let installers = [
            // Try EFL 1.26 first (most recent stable)
            (
                "https://download.enlightenment.org/rel/win/efl/efl-x86_64-win10-1.26.exe",
                "efl",
            ),
            // Then try EWPI (EFL Windows Package Installer)
            (
                "https://download.enlightenment.org/rel/win/efl/ewpi-x86_64-win10-1.1.exe",
                "ewpi",
            ),
        ];

        for (url, name) in &installers {
            let installer_path = Path::new(&out_dir).join(&format!("{}.exe", name));

            if !installer_path.exists() {
                download_file(url, &installer_path);
            }

            // Run the installer
            let install_dir = efl_dir.join(name).join("64");
            if !install_dir.exists() {
                eprintln!("Installing: {}", installer_path.display());
                execute_command(Command::new(&installer_path).args([
                    "/NCRC",
                    "/S",
                    &format!("/D={}", install_dir.display()),
                ]));
                eprintln!("  Installed to: {}", install_dir.display());
            }
        }

        // Verify installation
        if !efl_lib.exists() || !efl_include.exists() {
            eprintln!("\nEFL installation verification failed!");
            eprintln!("Expected library directory: {}", efl_lib.display());
            eprintln!("Expected include directory: {}", efl_include.display());
            eprintln!("\nPlease ensure the EFL installer ran successfully.");
            eprintln!(
                "You can also manually install EFL from: https://www.enlightenment.org/download"
            );
            std::process::exit(1);
        }
    }

    // Set up linker arguments
    let efl_lib_path = format!("{}", efl_lib.to_string_lossy());
    let efl_include_path = format!("{}", efl_include.to_string_lossy());

    println!("cargo:rustc-link-search=native={}", efl_lib_path);
    println!("cargo:rustc-link-lib=static=efl");
    println!("cargo:rustc-link-lib=static=eina");
    println!("cargo:rustc-link-lib=static=evas");
    println!("cargo:rustc-link-lib=static=ecore");
    println!("cargo:rustc-link-lib=static=ecore_evas");
    println!("cargo:rustc-link-lib=static=ecore_input");
    println!("cargo:rustc-link-lib=static=ecore_imf");
    println!("cargo:rustc-link-lib=static=eldbus");
    println!("cargo:rustc-link-lib=static=eet");
    println!("cargo:rustc-link-lib=static=emile");

    // Collect include paths
    let mut includes = Vec::new();
    includes.push(format!("-I{}", efl_include_path));

    // Also check for ewpi includes
    let ewpi_include = efl_dir.join("ewpi_64").join("include");
    if ewpi_include.exists() {
        includes.push(format!("-I{}", ewpi_include.to_string_lossy()));
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
            eprintln!("\nOn Debian/Ubuntu, install with:");
            eprintln!("  sudo apt-get install libefl-all-dev");
            eprintln!("\nOn Fedora, install with:");
            eprintln!("  sudo dnf install efl-devel");
            eprintln!("\nOn Arch Linux, install with:");
            eprintln!("  sudo pacman -S efl");
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
