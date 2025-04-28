// Build Scripts in Rust
// This example demonstrates how to use build scripts to customize the build process

// First, create a build.rs file in your project root:
/*
// build.rs
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Tell cargo to rerun this if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();

    // Example: Generate a version file
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let version_file = Path::new(&out_dir).join("version.rs");
    std::fs::write(
        version_file,
        format!("pub const VERSION: &str = \"{}\";", version),
    ).unwrap();

    // Example: Run a C program and link it
    let status = Command::new("gcc")
        .args(&["-c", "src/helper.c", "-o"])
        .arg(&format!("{}/helper.o", out_dir))
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to compile helper.c");
    }

    // Tell cargo to link the object file
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=helper");

    // Example: Check for system libraries
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dl");
    }

    // Example: Set custom cfg flags
    if cfg!(target_arch = "x86_64") {
        println!("cargo:rustc-cfg=target_arch_x86_64");
    }

    // Example: Set environment variables for the build
    println!("cargo:rustc-env=CUSTOM_VAR=value");
}
*/

// Now, let's create a Rust file that uses the build script output:
pub mod build_info {
    // Include the generated version file
    include!(concat!(env!("OUT_DIR"), "/version.rs"));

    // Function that uses build-time information
    pub fn get_build_info() -> String {
        format!("Version: {}", VERSION)
    }
}

// Example of using system-specific code
#[cfg(target_arch_x86_64)]
pub fn x86_64_specific_function() {
    println!("This function is only available on x86_64");
}

// Example of using environment variables set by build script
pub fn get_custom_var() -> String {
    env!("CUSTOM_VAR").to_string()
}

// Example of using external C code
extern "C" {
    fn helper_function() -> i32;
}

pub fn call_helper() -> i32 {
    unsafe { helper_function() }
}

// Example of conditional compilation
#[cfg(target_os = "linux")]
pub fn linux_specific_function() {
    println!("This function is only available on Linux");
}

#[cfg(target_os = "windows")]
pub fn windows_specific_function() {
    println!("This function is only available on Windows");
}

// Example of using build script to generate code
pub fn main() {
    // Print build information
    println!("Build Info: {}", build_info::get_build_info());

    // Use system-specific code
    #[cfg(target_arch_x86_64)]
    x86_64_specific_function();

    // Use environment variables
    println!("Custom Var: {}", get_custom_var());

    // Call external C function
    let result = call_helper();
    println!("Helper function result: {}", result);

    // Use OS-specific code
    #[cfg(target_os = "linux")]
    linux_specific_function();

    #[cfg(target_os = "windows")]
    windows_specific_function();
} 