use std::path::Path;
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn native_reference_metadata() {
    generate("native-reference-metadata");
}

fn generate(name: &str) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
    helpers::ensure_libclang();
    let input = test_clang::fixture("native_references");
    let imports = projection::Imports::from_iter(
        [
            "ApplyReference",
            "InvokeReference",
            "InvokeAnonymous",
            "ReferenceObject",
            "InvokeObject",
        ]
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "reference.dll".into(),
                    name: None,
                },
            )
        }),
    );
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output = projection::project_with_options(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &projection::Options {
            imports: Some(&imports),
            ..Default::default()
        },
    )
    .unwrap();
    let rdl = &output.partitions["native_references"].rdl;
    let golden = test_clang::input_dir().join("native_references.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, rdl).unwrap();
    }
    assert_eq!(
        *rdl,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target").join(name);
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = scratch.join("roundtrip.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(rewritten).unwrap(), *rdl);
    (root, scratch, winmd)
}
