use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=terraform.toml");

    let bindings = tf_bindgen::Builder::default()
        .config("terraform.toml")
        .generate()
        .unwrap();

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_dir, "terraform.rs").unwrap();
}
