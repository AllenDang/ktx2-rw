use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the ktx2 library
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let lib_name = match (target_os.as_str(), target_arch.as_str()) {
        ("linux", "x86_64") => "ktx2-linux-x64-musl",
        ("macos", "x86_64") => "ktx2-macos-x64",
        ("macos", "aarch64") => "ktx2-macos-arm64",
        ("windows", "x86_64") => "ktx2-windows-x64",
        _ => panic!("Unsupported platform: {target_os}-{target_arch}"),
    };

    // Add the library search path
    println!("cargo:rustc-link-search=native=libktx2-sys/lib");

    // Link the platform-specific static library
    println!("cargo:rustc-link-lib=static={lib_name}");

    // Link required system libraries for C++
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=c++");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=stdc++");
        }
        "windows" => {
            // Windows uses MSVC runtime by default
        }
        _ => {}
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=libktx2-sys/include/ktx.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("libktx2-sys/include/ktx.h")
        // Add include path
        .clang_arg("-Ilibktx2-sys/include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
