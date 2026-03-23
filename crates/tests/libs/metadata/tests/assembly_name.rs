use windows_metadata::*;

#[test]
fn name() {
    let writer = writer::File::new("TestName");

    let bytes = writer.into_stream();
    std::fs::write("tests/assembly_name.winmd", bytes).unwrap();

    let reader = reader::File::read("tests/assembly_name.winmd").unwrap();
    assert_eq!(reader.assembly_name(), Some("TestName"));

    let reader = reader::File::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap();
    assert_eq!(reader.assembly_name(), Some("Windows.Win32.winmd"));
}
