use std::env;
use std::path::Path;

fn generate_c_bindings(out_dir: &Path) {
    let bindings = bindgen::Builder::default()
        .header("include/main.h")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Unable to write bindings");
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not provided");
    let out_dir = Path::new(&out_dir);

    generate_c_bindings(out_dir);

    // Link against the system GL and X11 libraries.
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=GL");
}
