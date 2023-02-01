use std::process::Command;
use windows_metadata::reader::*;

#[test]
fn basic() {
    // For visual inspection: ildasm.exe %temp%\test_riddle.winmd
    let output = std::env::temp_dir().join("test_riddle.winmd").to_string_lossy().into_owned();

    let mut command = Command::new("cargo.exe");
    command.arg("install").arg("--path").arg("../../tools/riddle");

    if !command.status().unwrap().success() {
        panic!("Failed to install riddle");
    }

    let mut command = Command::new("riddle.exe");
    command.arg("-input").arg("src/basic.rs").arg("-output").arg(&output);

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }

    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

    let root = reader.tree("Root", &[]).expect("Root namespace not found");
    assert_eq!(root.namespace, "Root");
    assert_eq!(root.nested.len(), 1);
    let nested = &root.nested["Nested"];
    assert_eq!(nested.namespace, "Root.Nested");
    assert_eq!(nested.nested.len(), 0);

    let types: Vec<TypeDef> = reader.namespace_types("Root").collect();
    assert_eq!(types.len(), 1);

    assert_eq!(reader.type_def_namespace(types[0]), "Root");
    assert_eq!(reader.type_def_name(types[0]), "IRoot");
    assert_eq!(reader.type_def_kind(types[0]), TypeKind::Interface);

    let types: Vec<TypeDef> = reader.namespace_types("Root.Nested").collect();
    assert_eq!(types.len(), 1);

    assert_eq!(reader.type_def_namespace(types[0]), "Root.Nested");
    assert_eq!(reader.type_def_name(types[0]), "INested");
    assert_eq!(reader.type_def_kind(types[0]), TypeKind::Interface);
}
