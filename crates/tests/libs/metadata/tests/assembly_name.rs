use windows_metadata::*;

#[test]
fn file() {
    let writer = writer::File::new("TestName");

    let bytes = writer.into_stream();
    std::fs::write("tests/assembly_name.winmd", bytes).unwrap();

    let reader = reader::File::read("tests/assembly_name.winmd").unwrap();
    assert_eq!(reader.assembly_name(), Some("TestName"));

    let reader = reader::File::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap();
    assert_eq!(reader.assembly_name(), Some("Windows.Win32.winmd"));
}

#[test]
fn index() {
    let index = reader::TypeIndex::new(vec![
        reader::File::read("../../../libs/bindgen/default/Windows.winmd").unwrap(),
        reader::File::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap(),
        reader::File::read("../../../libs/bindgen/default/Windows.Wdk.winmd").unwrap(),
    ]);

    assert_eq!(
        index.assembly_name("Windows.Foundation.Metadata", "ActivatableAttribute"),
        Some("Windows")
    );
    assert_eq!(
        index.assembly_name("Windows.Win32.Foundation.Metadata", "AgileAttribute"),
        Some("Windows.Win32.winmd")
    );
    assert_eq!(
        index.assembly_name("Windows.Wdk.Foundation", "DRIVER_OBJECT"),
        Some("Windows.Wdk.winmd")
    );
    assert_eq!(
        index.assembly_name("Windows.Win32.Foundation.Metadata", "NotAttribute"),
        None
    );
}
