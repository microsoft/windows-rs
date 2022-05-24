use std::fs::*;
use std::io::prelude::*;
use std::process::*;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/component.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    std::fs::create_dir_all(".windows/winmd")?;

    Command::new("midlrt.exe").arg("/winrt").arg("/nomidl").arg("/h").arg("nul").arg("/metadata_dir").arg(&metadata_dir).arg("/reference").arg(format!("{}\\Windows.Foundation.winmd", metadata_dir)).arg("/winmd").arg(".windows/winmd/component.winmd").arg("src/component.idl").status()?;

    std::fs::remove_file("src/bindings.rs").ok();
    let mut bindings = File::create("src/bindings.rs")?;

    // TODO: this needs to be simpler
    let files = vec![metadata::reader::File::new("../../libs/metadata/default/Windows.winmd").unwrap(), metadata::reader::File::new(".windows/winmd/component.winmd").unwrap()];
    let reader = &metadata::reader::Reader::new(&files);
    let tree = reader.tree("test_nightly_component").expect("`test_nightly_component` namespace not found");
    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.component = true;
    bindings.write_all(bindgen::namespace(&gen, &tree).as_bytes())?;
    bindings.write_all(bindgen::namespace_impl(&gen, &tree).as_bytes())?;
    
    drop(bindings);
    Command::new("rustfmt").arg("src/bindings.rs").status()?;

    Ok(())
}
