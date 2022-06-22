use std::fs::*;
use std::process::*;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/component.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    std::fs::create_dir_all(".windows/winmd")?;

    Command::new("midlrt.exe").arg("/winrt").arg("/nomidl").arg("/h").arg("nul").arg("/metadata_dir").arg(&metadata_dir).arg("/reference").arg(format!("{}\\Windows.Foundation.winmd", metadata_dir)).arg("/winmd").arg(".windows/winmd/component.winmd").arg("src/component.idl").status()?;

    let files = vec![metadata::reader::File::new("../../libs/metadata/default/Windows.winmd").unwrap(), metadata::reader::File::new(".windows/winmd/component.winmd").unwrap()];
    write("src/bindings.rs", bindgen::component("test_component", &files))?;
    Command::new("rustfmt").arg("src/bindings.rs").status()?;

    Ok(())
}
