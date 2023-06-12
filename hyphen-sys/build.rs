use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=hyphen");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("bindings.generate() failed");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("bindings.write_to_file() failed");
}
