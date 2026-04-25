#[test]
fn test() {
    windows_rdl::reader()
        .input("tests/merge_a.rdl")
        .output("tests/merge_a.winmd")
        .write()
        .unwrap();

    windows_rdl::reader()
        .input("tests/merge_b.rdl")
        .output("tests/merge_b.winmd")
        .write()
        .unwrap();

    windows_metadata::merge()
        .input("tests/merge_a.winmd")
        .input("tests/merge_b.winmd")
        .output("tests/merged.winmd")
        .merge()
        .unwrap();

    windows_rdl::writer()
        .input("tests/merged.winmd")
        .output("tests/merge_out.rdl")
        .filter("Test")
        .write()
        .unwrap();

    let index = windows_metadata::reader::TypeIndex::read("tests/merged.winmd").unwrap();

    let point = index.expect("Test", "Point");
    let fields: Vec<_> = point.fields().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "x");
    assert_eq!(fields[0].ty(), windows_metadata::Type::I32);
    assert_eq!(fields[1].name(), "y");
    assert_eq!(fields[1].ty(), windows_metadata::Type::I32);

    let color = index.expect("Test", "Color");
    assert_eq!(
        color.category(),
        windows_metadata::reader::TypeCategory::Enum
    );
    let fields: Vec<_> = color.fields().collect();
    assert_eq!(fields.len(), 4);
}
