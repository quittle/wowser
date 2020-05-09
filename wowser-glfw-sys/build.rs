use bindgen;
use cmake;
use git2::{build::CheckoutBuilder, Repository};
use std::env;
use std::path::Path;

const URL: &str = "https://github.com/glfw/glfw.git";
const TAG: &str = "3.3.2";

fn checkout_glfw(dir: &Path) {
    let repo: Repository;
    if !dir.exists() {
        repo = Repository::clone(URL, dir).expect("Failed to clone");
    } else {
        repo = Repository::init(dir).expect("Invalid repository");
    }

    let object = repo.revparse_single(TAG).expect("Failed to find tag");
    repo.checkout_tree(&object, Some(CheckoutBuilder::new().force()))
        .expect("Failed to checkout tag");
}

fn build_glfw(dir: &Path) {
    cmake::Config::new(dir)
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
}

fn generate_c_bindings(out_dir: &Path) {
    let bindings = bindgen::Builder::default()
        .header(
            out_dir
                .join("include/GLFW/glfw3.h")
                .to_str()
                .expect("Invalid header path"),
        )
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

    let glfw_dir = out_dir.join("glfw");
    let built_lib = out_dir
        .join("lib")
        .to_str()
        .expect("Invalid path")
        .to_owned();

    checkout_glfw(&glfw_dir);
    build_glfw(&glfw_dir);
    generate_c_bindings(out_dir);

    // Link against the compiled GLFW
    println!("cargo:rustc-link-lib=static=glfw3");
    println!("cargo:rustc-link-search=native={}", built_lib);

    // Link against the system X11 library. This needs to come after glfw3
    // for some, unknown reason.
    println!("cargo:rustc-link-lib=dylib=X11");
}
