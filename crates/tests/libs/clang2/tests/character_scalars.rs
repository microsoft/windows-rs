use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;

#[test]
fn character_scalars() {
    helpers::ensure_libclang();
    let input = selected_fixture("character_scalars", &[]);
    let snapshots = capture(
        std::slice::from_ref(&input),
        &[Target::X86, Target::X64, Target::Arm64],
    )
    .unwrap();
    for snapshot in &snapshots {
        for (scalar, size) in [(Scalar::Char16, 2), (Scalar::Char32, 4)] {
            assert!(
                snapshot
                    .types
                    .iter()
                    .any(|ty| ty.kind == TypeKind::Scalar(scalar.clone())
                        && ty.layout
                            == Some(Layout {
                                size,
                                alignment: size
                            }))
            );
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!((output.records, output.enums, output.aliases), (4, 2, 6));
    let expected = std::fs::read_to_string(input_dir().join("character_scalars.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["character_scalars"].rdl, expected);
    assert_eq!(
        projection::generate_partial(&[input], "Fixture")
            .unwrap()
            .output,
        output
    );

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-character-scalars.winmd");
    let rewritten = winmd.with_extension("rdl");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&expected)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rewritten).unwrap(), expected);
    for path in [winmd, rewritten] {
        std::fs::remove_file(path).unwrap();
    }

    for (name, original, replacement) in [
        ("Compare16", "char16_t unit", "unsigned short unit"),
        ("Compare16", "char16_t unit", "wchar_t unit"),
        ("Compare32", "char32_t unit", "unsigned int unit"),
    ] {
        let first = selected_fixture("character_scalars", &[name]);
        let mut second = selected_fixture("character_scalars", &[name]);
        second.name = "different.h".into();
        second.source = std::fs::read_to_string(input_dir().join("character_scalars.h"))
            .unwrap()
            .replace(original, replacement);
        let error = projection::generate(&[first, second], "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("native field types differ"), "{error}");
    }
}
