extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("vendor")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join("include").join("basiclu.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // This is the path to the intermediate object file for our library.
    // let obj_path = libdir_path.join("hello.o");
    // This is the path to the static library file.
    // let lib_path = libdir_path.join("lib").join("libbasiclu.a");

    // Tell cargo to look for shared libraries in the specified directory
    // println!(
    //     "cargo:rustc-link-search={}",
    //     libdir_path.join("lib").to_str().unwrap()
    // );
    println!("cargo:rustc-link-search={}", out_dir);

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=basiclu");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", headers_path_str);

    // compile basiclu library: OS dependant
    // if cfg!(target_os = "linux") {
    // makefile is using a special env variable
    let out = Command::new("make")
        // .env("VENDOR_DIR", libdir_path.to_str().unwrap())
        .args(&[
            "-C",
            libdir_path.to_str().unwrap(),
            "static",
            "install",
            "purge",
        ])
        .output()
        .expect("could not spawn `make`");
    assert!(out.status.success(), "{:?}", out);
    // }

    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // .header("wrapper.h")
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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
