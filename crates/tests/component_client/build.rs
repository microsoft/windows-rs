use std::fs::*;
use std::io::prelude::*;
use std::process::*;

fn main() -> std::io::Result<()> {
    create_dir_all(".windows/winmd")?;
    copy(
        "../component/.windows/winmd/component.winmd",
        ".windows/winmd/component.winmd",
    )?;

    std::fs::remove_file("src/bindings.rs").ok();
    let mut bindings = File::create("src/bindings.rs")?;

    // TODO: this needs to be simpler
    let files = vec![
        metadata::reader::File::new("../../libs/metadata/default/Windows.winmd").unwrap(),
        metadata::reader::File::new(".windows/winmd/component.winmd").unwrap(),
    ];
    let reader = &metadata::reader::Reader::new(&files);
    let tree = reader
        .tree("test_component", &[])
        .expect("`test_component` namespace not found");
    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.component = true;
    bindings.write_all(bindgen::namespace(&gen, &tree).as_bytes())?;

    drop(bindings);

    Command::new("rustfmt").arg("src/bindings.rs").status()?;
    Ok(())
}
