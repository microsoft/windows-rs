use std::fs::*;
use std::process::*;

fn main() -> std::io::Result<()> {
    create_dir_all(".windows/winmd")?;
    copy("../component/.windows/winmd/component.winmd", ".windows/winmd/component.winmd")?;

    let files = vec![metadata::reader::File::new("../../libs/metadata/default/Windows.winmd").unwrap(), metadata::reader::File::new(".windows/winmd/component.winmd").unwrap()];
    write("src/bindings.rs", bindgen::component("test_component", &files))?;
    Command::new("rustfmt").arg("src/bindings.rs").status()?;
    Ok(())
}
