use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

// Object-like identifier aliases rename members and parameters only in their written source;
// imports keep the compiler's linkage name.
#[test]
fn written_names() {
    helpers::ensure_libclang();
    let snapshots = capture(
        &[selected_fixture("written_names", &[])],
        &[Target::X86, Target::X64, Target::Arm64],
    )
    .unwrap();
    let imports = std::iter::once((
        "DrawTextW".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    ))
    .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let text = &output.partitions["written_names"].rdl;
    let golden = input_dir().join("written_names.rdl");
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
