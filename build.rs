extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let vendor_path = PathBuf::from("vendor")
        .canonicalize()
        .expect("cannot canonicalize path");

    if cfg!(feature = "dynamic") {
        println!("cargo:rustc-link-lib=basiclu");
    } else {
        let src_dir = vendor_path.join("src");

        let mut builder = cc::Build::new();

        builder.flag("-Wno-unused-parameter");

        builder.include(vendor_path.join("include"));

        let basiclu_objects = [
            "basiclu_factorize",
            "basiclu_get_factors",
            "basiclu_initialize",
            "basiclu_object",
            "basiclu_obj_maxvolume",
            "basiclu_solve_dense",
            "basiclu_solve_for_update",
            "basiclu_solve_sparse",
            "basiclu_update",
            "lu_build_factors",
            "lu_condest",
            // "lu_def.h",
            "lu_dfs",
            "lu_factorize_bump",
            "lu_file",
            // "lu_file.h",
            "lu_garbage_perm",
            "lu_initialize",
            "lu_internal",
            // "lu_internal.h",
            // "lu_list.h",
            "lu_markowitz",
            "lu_matrix_norm",
            "lu_pivot",
            "lu_residual_test",
            "lu_setup_bump",
            "lu_singletons",
            "lu_solve_dense",
            "lu_solve_for_update",
            "lu_solve_sparse",
            "lu_solve_symbolic",
            "lu_solve_triangular",
            "lu_timer",
            // "lu_timer.h",
            "lu_update",
        ];

        for obj in basiclu_objects.iter() {
            builder.file(&src_dir.join(format!("{obj}.c")));
        }

        builder.compile("basiclu");
    }

    let headers_path = vendor_path.join("include").join("basiclu.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
