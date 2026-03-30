#[test]
fn name() {
    // This has no input so it just writes and empty .winmd file.
    windows_rdl::reader()
        .output("tests/AssemblyName.winmd")
        .write()
        .unwrap();

    // This just tests that the Assembly table has derived the assembly table from the RDL reader's output file name.
    let reader = windows_metadata::reader::File::read("tests/AssemblyName.winmd").unwrap();
    assert_eq!(reader.assembly_name(), Some("AssemblyName.winmd"));
}
