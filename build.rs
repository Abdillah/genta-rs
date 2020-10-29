use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=tracker-sparql-2.0");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=tracker_sparql/sparql.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindings = bindgen::Builder::default();

    let tracker_sparql_2 = pkg_config::Config::new().probe("tracker-sparql-2.0").unwrap();
    for path in tracker_sparql_2.include_paths {
        bindings = bindings.clang_arg(format!("-I{}", path.to_str().unwrap()));
    }

    let bindings = bindings
        // The input header we would like to generate
        // bindings for.
        .header("tracker_sparql/sparql.h")
        .blacklist_item("FP_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(format!("{}/tracker_sparql", env::current_dir().unwrap().to_str().unwrap()));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
