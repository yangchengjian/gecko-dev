extern crate bindgen;
extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=dylib=arcore_sdk_c");
    println!("cargo:rustc-link-lib=dylib=jnigraphics");
    println!("cargo:rustc-link-search=native=/Volumes/t/sft/arcore/arcore-jni/jni/arm64-v8a");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("include/arcore_c_api.h")
        .clang_arg("--sysroot=/Volumes/t/sft/android/android-ndk-r21d/sysroot")
        .trust_clang_mangling(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Generate .rs
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("arcore_bindings.rs"))
        .expect("Couldn't write bindings!");

    // Genrerate .h
    // Write the bindings to the $CARGO_MANIFEST_DIR/bindings.h file
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}
