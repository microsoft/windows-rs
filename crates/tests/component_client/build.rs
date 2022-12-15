use std::fs::*;
use std::process::*;

fn main() -> std::io::Result<()> {
    create_dir_all(".windows/winmd")?;
    copy("../component/.windows/winmd/component.winmd", ".windows/winmd/component.winmd")?;

    let files = metadata::reader::File::with_default(&[".windows/winmd/component.winmd"])?;
    write("src/bindings.rs", bindgen::component("test_component", &files))?;
    Command::new("rustfmt").arg("src/bindings.rs").status()?;
    Ok(())
}
