use windows_metadata::*;

#[test]
fn name() {
    let writer = writer::File::new("test");

    let bytes = writer.into_stream();
    std::fs::write("tests/assembly.winmd", bytes).unwrap();

    let reader = reader::File::read("tests/assembly.winmd").unwrap();
    assert_eq!(reader.assembly_name(), Some("test"));

    let reader = reader::File::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap();
    assert_eq!(reader.assembly_name(), Some("Windows.Win32.winmd"));
}
