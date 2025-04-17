//! This build script is only for testing purposes.
//!
//! It is not used in the actual library.
//! To make this library work, you either
//! - link `dftd4` and `mctc-lib` in `build.rs` of your own project
//! - try `dftd4-src` crate (which may also requires some configurations)

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=DFTD4_DEV");
    if std::env::var("DFTD4_DEV").is_ok() {
        std::env::var("LD_LIBRARY_PATH")
            .unwrap()
            .split(":")
            .filter(|path| !path.is_empty())
            .filter(|path| PathBuf::from(path).exists())
            .for_each(|path| {
                println!("cargo:rustc-link-search=native={}", path);
            });
        println!("cargo:rustc-link-lib=dftd4");
    }
}
