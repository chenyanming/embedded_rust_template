//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
extern crate bindgen;
extern crate cc;

fn main() {
    // // Tell cargo to tell rustc to link the system bzip2
    // // shared library.
    // // println!("cargo:rustc-link-lib=atom");
    // // Tell cargo to invalidate the built crate whenever the wrapper changes
    // // println!("cargo:rerun-if-changed=atom.h");
    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    // // The input header we would like to generate
    // // bindings for.
    //     .header("src/c/bindings.h")
    //     .ctypes_prefix("cty")
    //     .use_core()
    // // Tell cargo to invalidate the built crate whenever any of the
    // // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    // // Finish the builder and generate the bindings.
    //     .generate()
    // // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // cc::Build::new()
    //     .file("src/c/atom.c")
    //     .compile("atom");
    // println!("cargo:rerun-if-changed=src/c/atom.c");

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // bindings
    //     // .write_to_file(out.join("bindings.rs"))
    //     .write_to_file("src/bindings.rs")
    //     .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
}
