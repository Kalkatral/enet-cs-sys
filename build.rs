extern crate bindgen;
extern crate cmake;

use std::{env, path::PathBuf};

use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let target = env::var("TARGET").unwrap();
    let is_debug = env::var("DEBUG").unwrap() == "true";

    // generate the bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("enet_.*")
        .generate()
        .expect("failed to generate bindings.");

    // write the generated bindings to ${OUT_DIR}/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("could not write bindings");

    // build ENet-CS to a static library
    let built = Config::new("vendor/ENet-CSharp/Source/Native")
        .define("ENET_STATIC", "1")
        .build_target("enet_static")
        .build();

    // add the newly compiled static library to the library search path of rustc
    if target.contains("windows") {
        if is_debug {
            println!(
                "cargo:rustc-link-search=native={}/build/Debug",
                built.display()
            );
        } else {
            println!(
                "cargo:rustc-link-search=native={}/build/Release",
                built.display()
            );
        }
        println!("cargo:rustc-link-lib=dylib=winmm");
    } else {
        println!("cargo:rustc-link-search=native={}/build", built.display());
    }
    println!("cargo:rustc-link-lib=static=enet_static");
}
