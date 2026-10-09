use windows_clang2::{Input, ProjectionOptions, ReferenceKind, TypeReference, capture};
use windows_metadata::reader::Index;

const SOURCE: &str = include_str!("../input/packed.h");
const ROOTS: &[&str] = &[
    "Packed1",
    "PackedChoice",
    "PackedAnonymous",
    "Packed2",
    "Packed4",
    "PackedContainer",
    "NaturalChoice",
    "PackedField",
    "PackedGap",
];

#[test]
fn packed_storage_preserves_canonical_metadata_across_targets_and_tus() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", SOURCE)];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(inputs, &["-x", "c++", &target], ROOTS).unwrap();
            let rdl = snapshot
                .resolve()
                .unwrap()
                .project(&ProjectionOptions::new("Test"))
                .unwrap()
                .rdl();
            let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("expected")
                .join("packed.rdl");
            if std::env::var_os("UPDATE_EXPECT").is_some() {
                std::fs::write(&expected, &rdl).unwrap();
            }
            assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("packed-{arch}-{reversed}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .output(&output)
                .write()
                .unwrap();
            let roundtrip = output.with_extension("rdl");
            windows_rdl::writer()
                .input(&output)
                .filter("Test")
                .output(&roundtrip)
                .write()
                .unwrap();
            let reencoded = output.with_extension("roundtrip.winmd");
            windows_rdl::reader()
                .input(&roundtrip)
                .output(&reencoded)
                .write()
                .unwrap();
            let before = Index::read(output).unwrap();
            let after = Index::read(reencoded).unwrap();
            for name in ROOTS {
                let before_root = before.expect("Test", name);
                let after_root = after.expect("Test", name);
                let before_nested = before.nested_recursive(before_root);
                let after_nested = after.nested_recursive(after_root);
                assert_eq!(before_nested.len(), after_nested.len());
                for (before, after) in std::iter::once(before_root)
                    .chain(before_nested)
                    .zip(std::iter::once(after_root).chain(after_nested))
                {
                    assert_eq!(before.name(), after.name());
                    assert_eq!(before.flags(), after.flags());
                    assert_eq!(
                        before
                            .class_layout()
                            .map(|layout| (layout.packing_size(), layout.class_size())),
                        after
                            .class_layout()
                            .map(|layout| (layout.packing_size(), layout.class_size()))
                    );
                    let fields = |ty: windows_metadata::reader::TypeDef| {
                        ty.fields()
                            .map(|field| (field.name().to_owned(), field.flags(), field.ty()))
                            .collect::<Vec<_>>()
                    };
                    assert_eq!(fields(before), fields(after));
                }
            }
        }
    }
}

#[test]
fn packed_storage_does_not_enable_unproven_by_value_calls() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        for declaration in [
            "extern \"C\" void Use(Packed1 value);",
            "extern \"C\" Packed1 Use();",
            "extern \"C\" void Use(PackedContainer value);",
            "typedef void (*Callback)(Packed1 value); extern \"C\" void Use(Callback callback);",
        ] {
            let source = format!("{SOURCE}\n{declaration}");
            let snapshot = capture(
                [Input::new("packed.hpp", source)],
                &["-x", "c++", &target],
                &["Use"],
            )
            .unwrap();
            let mut options = ProjectionOptions::new("Test");
            options.library = Some("test.dll".into());
            let error = snapshot
                .resolve()
                .unwrap()
                .project(&options)
                .unwrap_err()
                .to_string();
            assert!(error.contains("adjusted record layouts"), "{error}");
        }
    }
}

#[test]
fn packed_storage_rejects_transitively_forced_alignment() {
    let source = r#"
struct __attribute__((aligned(16))) Aligned { int value; };
struct Nested { Aligned value; };
struct __attribute__((packed)) Packed { char tag; Nested values[2]; };
"#;
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-w64-windows-gnu");
        let snapshot = capture(
            [Input::new("packed.hpp", source)],
            &["-x", "c++", &target],
            &["Packed"],
        )
        .unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&ProjectionOptions::new("Test"))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("forced or unproven external alignment"),
            "{error}"
        );
    }
}

#[test]
fn packed_storage_does_not_assume_external_alignment_policy() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let snapshot = capture(
            [Input::new("packed.hpp", SOURCE)],
            &["-x", "c++", &target],
            &["PackedField"],
        )
        .unwrap();
        let mut options = ProjectionOptions::new("Test");
        options.references.insert(
            "NaturalChoice".into(),
            TypeReference {
                namespace: "External".into(),
                name: "Choice".into(),
                kind: ReferenceKind::Value,
            },
        );
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&options)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("forced or unproven external alignment"),
            "{error}"
        );
    }
}
