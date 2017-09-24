extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let nspr_include_dir = "-I".to_string() +
        &env::var("NSPR_INCLUDE_DIR").expect("Please set NSPR_INCLUDE_DIR");
    let nspr_lib_dir = env::var("NSPR_LIB_DIR").expect("Please set NSPR_LIB_DIR");

    println!("cargo:rustc-link-search=native={}", nspr_lib_dir);
    println!("cargo:rustc-link-lib=nspr4");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(nspr_include_dir)
        .generate()
        .expect("Unable to generate bindings");

    // $OUT_DIR is set by cargo.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
