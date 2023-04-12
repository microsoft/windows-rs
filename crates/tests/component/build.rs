use std::fs::*;
use std::process::*;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/component.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    std::fs::create_dir_all(".windows/winmd")?;

    let mut command = Command::new("midlrt.exe");
    command
        .arg("/winrt")
        .arg("/nomidl")
        .arg("/h")
        .arg("nul")
        .arg("/metadata_dir")
        .arg(&metadata_dir)
        .arg("/reference")
        .arg(format!("{metadata_dir}\\Windows.Foundation.winmd"))
        .arg("/winmd")
        .arg(".windows/winmd/component.winmd")
        .arg("src/component.idl");

    if !command.status()?.success() {
        panic!();
    }

    let files = metadata::reader::File::with_default(&[".windows/winmd/component.winmd"])?;
    write(
        "src/bindings.rs",
        bindgen::component("test_component", &files),
    )?;

    Ok(())
}
