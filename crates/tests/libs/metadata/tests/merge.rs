/// Tests that two `.winmd` files can be merged into one via [`windows_metadata::merge`] and that
/// the resulting file round-trips correctly through [`windows_rdl::writer`].
///
/// The golden file `tests/merge_out.rdl` is regenerated in-place each run; CI checks that no diff
/// is produced, catching any regressions.
#[test]
fn test() {
    // Step 1: compile each .rdl input into its own .winmd.
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

    // Step 2: merge the two .winmd files into one.
    windows_metadata::merge()
        .input("tests/merge_a.winmd")
        .input("tests/merge_b.winmd")
        .output("tests/merged.winmd")
        .merge()
        .unwrap();

    // Step 3: write the merged .winmd back to RDL (golden file).
    windows_rdl::writer()
        .input("tests/merged.winmd")
        .output("tests/merge_out.rdl")
        .filter("Test")
        .write()
        .unwrap();

    // Step 4: verify the merged metadata has both types from the inputs.
    let index = windows_metadata::reader::TypeIndex::read("tests/merged.winmd").unwrap();

    let point = index.expect("Test", "Point");
    let fields: Vec<_> = point.fields().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "x");
    assert_eq!(fields[0].ty(), windows_metadata::Type::I32);
    assert_eq!(fields[1].name(), "y");
    assert_eq!(fields[1].ty(), windows_metadata::Type::I32);

    let color = index.expect("Test", "Color");
    assert_eq!(color.category(), windows_metadata::reader::TypeCategory::Enum);
    let fields: Vec<_> = color.fields().collect();
    // Enums in winmd have one `value__` field (underlying type) plus one field per variant.
    assert_eq!(fields.len(), 4); // value__ + Red + Green + Blue
}
