use std::fs::*;

fn main() -> std::io::Result<()> {
    create_dir_all(".windows/winmd")?;
    copy(
        "../component/.windows/winmd/component.winmd",
        ".windows/winmd/component.winmd",
    )?;

    let files = metadata::reader::File::with_default(&[".windows/winmd/component.winmd"])?;
    write(
        "src/bindings.rs",
        bindgen::component("test_component", &files),
    )?;
    Ok(())
}
