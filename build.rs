use std::path::Path;
use std::env;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("glfw-3.3.5").display());
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("glew-2.1.0/lib/Release/x64").display());
}