use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{Type as MetadataType, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn native_sized_aliases() {
    roundtrip();
}

#[test]
fn native_sized_aliases_preserve_agreement() {
    helpers::ensure_libclang();
    for (old, new, diagnostic) in [
        ("unsigned int Word", "unsigned long Word", "differ"),
        (
            "DEFAULT(return >= 0)",
            "DEFAULT(return > 0)",
            "no observed SAL annotation set contains every contract",
        ),
    ] {
        let mut inputs = [
            selected_fixture("native_sized_aliases", &[]),
            selected_fixture("native_sized_aliases", &[]),
        ];
        inputs[1].name = "other.h".into();
        inputs[1].source = inputs[1].source.replace(old, new);
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string();
            assert!(error.contains(diagnostic), "{error}");
            inputs.reverse();
        }
    }
}

fn roundtrip() {
    helpers::ensure_libclang();
    let input = selected_fixture("native_sized_aliases", &[]);
    let mut snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let alias = snapshot
            .aliases
            .iter()
            .find(|alias| alias.name == "Word")
            .unwrap();
        let ty = &snapshot.types[alias.observations[0].canonical.0];
        assert_eq!(
            ty.kind,
            TypeKind::Scalar(if snapshot.target == Target::X86 {
                Scalar::UnsignedInt
            } else {
                Scalar::UnsignedLongLong
            })
        );
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap()
    );
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    let actual = &output.partitions["native_sized_aliases"].rdl;
    let expected = input_dir().join("native_sized_aliases.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, actual).unwrap();
    }
    assert_eq!(
        *actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\test-clang-native-sized");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    let rdl = scratch.join("roundtrip.rdl");
    let again = scratch.join("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(actual)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&again)
        .write()
        .unwrap();
    for path in [&winmd, &again] {
        for bit in [1, 2, 4] {
            let index =
                reader::Index::new_for_architecture(vec![reader::File::read(path).unwrap()], bit);
            for (name, ty) in [
                ("Word", MetadataType::USize),
                ("SignedWord", MetadataType::ISize),
                ("Copy", MetadataType::USize),
                ("Fixed32", MetadataType::U32),
                ("Fixed64", MetadataType::U64),
                (
                    "MixedSignedness",
                    if bit == 1 {
                        MetadataType::I32
                    } else {
                        MetadataType::U64
                    },
                ),
                (
                    "WrongArm64",
                    if bit == 2 {
                        MetadataType::U64
                    } else {
                        MetadataType::U32
                    },
                ),
                (
                    "Floating",
                    if bit == 1 {
                        MetadataType::F32
                    } else {
                        MetadataType::F64
                    },
                ),
            ] {
                assert_eq!(
                    index.expect("Fixture", name).underlying_type(),
                    Some(ty),
                    "{name}/{bit}"
                );
            }
            if bit == 1 {
                assert_eq!(
                    index.expect("Fixture", "Only32").underlying_type(),
                    Some(MetadataType::U32)
                );
            } else {
                assert_eq!(
                    index.expect("Fixture", "Only64").underlying_type(),
                    Some(MetadataType::U64)
                );
            }
            if bit == 2 {
                assert_eq!(
                    index.expect("Fixture", "OnlyX64").underlying_type(),
                    Some(MetadataType::U64)
                );
            }
        }
    }
    std::fs::remove_dir_all(scratch).unwrap();
}
