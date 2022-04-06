use std::fs::*;
use std::io::prelude::*;
use std::process::*;
use windows_bindgen::*;

fn main() {
    std::fs::create_dir_all(".windows/winmd").unwrap();

    // I would have expected Cargo to wait until the component crate (which is a dependency) is built
    // before starting to run this dependent crate but Cargo seems to be a little too eager to get started.
    // I'm not sure why...
    while copy("../component/.windows/winmd/component.winmd", ".windows/winmd/component.winmd").is_err() {
        std::thread::yield_now();
    }

    let gen = Gen { namespace: "test_component", component: true, ..Default::default() };
    let mut bindings = File::create("src/bindings.rs").unwrap();
    bindings.write_all(gen_namespace(&gen).as_bytes()).unwrap();
    bindings.write_all(gen_namespace_impl(&gen).as_bytes()).unwrap();
    drop(bindings);

    Command::new("rustfmt").arg("src/bindings.rs").status().unwrap();
}
