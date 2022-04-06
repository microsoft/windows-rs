use std::fs::*;
use std::io::prelude::*;
use std::process::*;
use windows_bindgen::*;

fn main() {
    println!("cargo:rerun-if-changed=../component/.windows/winmd/component.winmd");
    std::fs::create_dir_all(".windows/winmd").unwrap();
    copy("../component/.windows/winmd/component.winmd", ".windows/winmd/component.winmd").unwrap();

    let gen = Gen { namespace: "test_component", component: true, ..Default::default() };
    let mut bindings = File::create("src/bindings.rs").unwrap();
    bindings.write_all(gen_namespace(&gen).as_bytes()).unwrap();
    bindings.write_all(gen_namespace_impl(&gen).as_bytes()).unwrap();
    drop(bindings);

    Command::new("rustfmt").arg("src/bindings.rs").status().unwrap();
}
