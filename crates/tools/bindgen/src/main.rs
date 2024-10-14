//! Regenerates bindings sources for `crates/tests/bindgen`

use std::path::Path;
use windows_bindgen2::bindgen;

fn split(args: &str) {
    bindgen(args.split_whitespace());
}

fn main() {
    if !Path::new("crates/tests/bindgen/Cargo.toml").exists() {
        println!("This tool must be run from the root of the repo.");
        std::process::exit(1);
    }

    std::fs::remove_dir_all("crates/tests/bindgen/src").unwrap();
    std::fs::create_dir_all("crates/tests/bindgen/src").unwrap();
    std::env::set_current_dir("crates/tests/bindgen/src").unwrap();
    std::fs::write("lib.rs", "").unwrap();

    split("--out smoke.rs --filter GetTickCount --sys --flat --no-comment --no-allow");

    // TODO: test with path using white space

    // TODO: test failure/panics
}
