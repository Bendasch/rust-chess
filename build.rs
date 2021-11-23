use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir)
            .join("src/library/gui/res/glfw-3.3.5/lib-vc2022/x64")
            .display()
    );
}
