use std::fs::*;
use std::io::prelude::*;
use std::process::*;
use windows_bindgen::*;

fn main() -> std::io::Result<()> {
    create_dir_all(".windows/winmd")?;
    copy("../nightly_component/.windows/winmd/component.winmd", ".windows/winmd/component.winmd")?;

    std::fs::remove_file("src/bindings.rs").ok();
    let mut bindings = File::create("src/bindings.rs")?;
    let gen = Gen { namespace: "test_nightly_component", component: true, ..Default::default() };
    bindings.write_all(gen_namespace(&gen).as_bytes())?;
    drop(bindings);

    Command::new("rustfmt").arg("src/bindings.rs").status()?;
    Ok(())
}
