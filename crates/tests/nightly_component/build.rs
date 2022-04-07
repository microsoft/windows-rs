use std::fs::*;
use std::io::prelude::*;
use std::process::*;
use windows_bindgen::*;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/component.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    std::fs::create_dir_all(".windows/winmd")?;

    // Getting midlrt.exe to run on the CI build is very difficulty. Since this is a temporary solution
    // until windows-rs has its own winmd generator, we can just build this locally and keep the winmd
    // files checked in for now.
    let _ = Command::new("midlrt.exe").arg("/winrt").arg("/nomidl").arg("/h").arg("nul").arg("/metadata_dir").arg(&metadata_dir).arg("/reference").arg(format!("{}\\Windows.Foundation.winmd", metadata_dir)).arg("/winmd").arg(".windows/winmd/component.winmd").arg("src/component.idl").status();

    let gen = Gen { namespace: "test_nightly_component", component: true, ..Default::default() };
    let mut bindings = File::create("src/bindings.rs")?;
    bindings.write_all(gen_namespace(&gen).as_bytes())?;
    bindings.write_all(gen_namespace_impl(&gen).as_bytes())?;
    drop(bindings);

    Command::new("rustfmt").arg("src/bindings.rs").status()?;
    Ok(())
}
