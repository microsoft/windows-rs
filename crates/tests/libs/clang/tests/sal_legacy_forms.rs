use windows_clang::*;

#[test]
fn sal_legacy_forms() {
    helpers::ensure_libclang();
    let output =
        projection::generate(&[test_clang::fixture("sal_legacy_forms")], "Fixture").unwrap();
    let text = &output.partitions["sal_legacy_forms"].rdl;
    let golden = test_clang::input_dir().join("sal_legacy_forms.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, text).unwrap();
    }
    assert_eq!(
        *text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
}
