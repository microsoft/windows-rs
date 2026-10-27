use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn mixed_layout() {
    helpers::ensure_libclang();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let input = test_clang::selected_fixture("mixed_layout", &[]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let rdl: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let golden = test_clang::input_dir().join("mixed_layout.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, &rdl).unwrap();
    }
    assert_eq!(
        rdl,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let stem = root.join("target\\test-clang-mixed_layout");
    let winmd = stem.with_extension("winmd");
    let rewritten = stem.with_extension("rdl");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(rdl, std::fs::read_to_string(&rewritten).unwrap());
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rewritten)
        .output(&winmd)
        .write()
        .unwrap();
    for path in [winmd, rewritten] {
        std::fs::remove_file(path).unwrap();
    }
}
