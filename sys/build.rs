use std::env;
use std::path::PathBuf;

use bindgen::Builder;

fn main() {
    // Link shared library
    println!("cargo:rustc-link-lib=bladerf");

    let bindings = Builder::default()
        .header("wrapper.h")
        .allowlist_item("(bladerf|BLADERF).*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
