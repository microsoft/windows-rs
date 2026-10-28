use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::HasAttributes;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn publication_correspondence() {
    helpers::ensure_libclang();
    spelling();
    recursive();
    conflicts();
    architecture();
}

fn golden(output: &projection::Output, stem: &str) {
    assert_eq!(
        output.partitions[stem].rdl,
        std::fs::read_to_string(input_dir().join(format!("{stem}.rdl")))
            .unwrap()
            .replace("\r\n", "\n")
    );
}

fn spelling() {
    let chained = projection::generate(&[selected_fixture("publication", &[])], "Fixture").unwrap();
    let mut direct = selected_fixture("publication", &[]);
    direct.source.insert_str(0, "#define DIRECT_SECOND\n");
    assert_eq!(chained, projection::generate(&[direct], "Fixture").unwrap());
    golden(&chained, "publication_aliases");
    let mut inputs = [
        selected_fixture("publication", &[]),
        selected_fixture("publication", &[]),
    ];
    inputs[1].name = "second.h".into();
    inputs[1].source.insert_str(0, "#define DIRECT_SECOND\n");
    assert_eq!(chained, projection::generate(&inputs, "Fixture").unwrap());
    inputs.reverse();
    assert_eq!(chained, projection::generate(&inputs, "Fixture").unwrap());
}

fn recursive() {
    let mut inputs = [
        selected_fixture("publication_node_a", &[]),
        selected_fixture("publication_node_b", &[]),
    ];
    for (input, header) in inputs
        .iter_mut()
        .zip(["publication_node_a", "publication_node_b"])
    {
        input.source = format!("#include \"{header}.h\"\n");
    }
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.records.len(), 2);
    let output = projection::project(&merged, "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (1, 2));
    golden(&output, "publication_node_types");
    golden(&output, "publication_node_b");
    inputs.reverse();
    inputs[0].name = "renamed-first.h".into();
    inputs[1].name = "renamed-second.h".into();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
}

fn conflicts() {
    for (first, second) in [
        ("", "#define FIELD_TYPE long\n"),
        (
            "#define FIELD_TYPE int*\n",
            "#define FIELD_TYPE const int*\n",
        ),
        ("#define FIELD_TYPE Left*\n", "#define FIELD_TYPE Right*\n"),
        (
            "#define NO_NEXT\n#define FIELD_TYPE char\n",
            "#define NO_NEXT\n#define FIELD_TYPE char\n#define PACKING __attribute__((packed))\n",
        ),
        (
            "#define FIELD_ARRAY [1]\n",
            "#if defined(_M_X64)\n#define FIELD_ARRAY [2]\n#else\n#define FIELD_ARRAY [1]\n#endif\n",
        ),
        ("#define POINTER_ALIAS\n", "#define POINTER_ALIAS\n"),
        ("", "#if defined(_M_X64)\n#define FIELD_TYPE long\n#endif\n"),
    ] {
        let mut inputs = [
            selected_fixture("publication_conflict_entry", &[]),
            selected_fixture("publication_conflict_entry", &[]),
        ];
        inputs[0].source.insert_str(0, first);
        inputs[1].name = "other.h".into();
        inputs[1]
            .source
            .insert_str(0, &format!("#define TAG Second\n{second}"));
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        for resolution in merged.targets.values() {
            let definitions: Vec<_> = ["First", "Second"]
                .iter()
                .map(|name| {
                    let group = resolution
                        .groups
                        .iter()
                        .find(|group| resolution.name(group) == *name)
                        .unwrap();
                    resolution.definition(group).unwrap()
                })
                .collect();
            assert_eq!(definitions[0].layout, definitions[1].layout);
        }
        let error = projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("incompatible native"),
            "{first:?} {second:?}: {error}"
        );
        if second.contains("_M_X64") {
            assert!(error.contains("X64"), "{error}");
        }
        assert!(
            projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string()
                .contains("incompatible native")
        );
    }
}

fn architecture() {
    let output =
        projection::generate(&[selected_fixture("publication_arch", &[])], "Fixture").unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-publication.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|partition| &partition.rdl))
        .output(&winmd)
        .write()
        .unwrap();
    for bit in [1, 2, 4] {
        let index = windows_metadata::reader::Index::new_for_architecture(
            vec![windows_metadata::reader::File::read(&winmd).unwrap()],
            bit,
        );
        assert_eq!(index.get("Fixture", "Zeta").count(), 1);
        let zeta = index.expect("Fixture", "Zeta");
        assert_eq!(
            zeta.attributes()
                .any(|attribute| attribute.name() == "NativeTypedefAttribute"),
            bit == 1,
        );
        if bit == 1 {
            assert_eq!(
                zeta.underlying_type(),
                Some(windows_metadata::Type::value_named("Fixture", "Alpha"))
            );
        } else {
            assert_eq!(zeta.fields().next().unwrap().name(), "value");
            assert_eq!(
                zeta.fields().next().unwrap().ty(),
                windows_metadata::Type::I32
            );
        }
    }
    let rdl = root.join("target\\test-clang-publication.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(&rdl).unwrap(),
        output.partitions["publication_arch_types"].rdl
    );
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(rdl).unwrap();
}
