use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{HasAttributes, Type, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn primary_names() {
    helpers::ensure_libclang();
    definitions();
    ambiguities();
}

fn definitions() {
    let mut inputs = [
        selected_fixture("primary", &[]),
        selected_fixture("primary", &[]),
    ];
    for input in &mut inputs {
        input.source = "#include \"primary.h\"".into();
    }
    inputs[1].name = "other.h".into();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    for snapshot in &snapshots {
        for alias in &snapshot.aliases {
            for observation in &alias.observations {
                assert_eq!(
                    observation.defines_tag,
                    matches!(alias.name.as_str(), "Zed" | "State"),
                    "{}",
                    alias.name
                );
            }
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    for stem in ["primary", "primary_types"] {
        assert_eq!(
            output.partitions[stem].rdl,
            std::fs::read_to_string(input_dir().join(format!("{stem}.rdl")))
                .unwrap()
                .replace("\r\n", "\n")
        );
    }
    inputs.reverse();
    inputs[0].name = "renamed.h".into();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-primary.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|partition| &partition.rdl))
        .output(&winmd)
        .write()
        .unwrap();
    for bit in [1, 2, 4] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        for (alias, primary) in [
            ("Aardvark", "Zed"),
            ("AConsumer", "Zed"),
            ("Before", "Zed"),
            ("Alias", "Standalone"),
            ("AState", "State"),
        ] {
            assert_eq!(
                index.expect("Fixture", alias).underlying_type(),
                Some(Type::value_named("Fixture", primary))
            );
            assert!(
                !index
                    .expect("Fixture", primary)
                    .attributes()
                    .any(|attribute| attribute.name() == "NativeTypedefAttribute")
            );
        }
    }
    std::fs::remove_file(winmd).unwrap();
}

fn ambiguities() {
    for prefix in [
        "#define NAMES Tag, Other\n",
        "#define NAMES Other, Tag\n",
        "#define ENUM\n#define NAMES Tag, Other\n",
    ] {
        let mut input = selected_fixture("primary_ambiguous", &[]);
        input.source = format!("{prefix}#include \"primary_ambiguous.h\"");
        let output = projection::generate(&[input], "Fixture").unwrap();
        assert_eq!(output.aliases, 1);
        assert!(
            output.partitions["primary_ambiguous"]
                .rdl
                .contains("type Other = Tag;")
        );
    }
    for prefix in ["", "#define ENUM\n", "#define NAMES Second, First\n"] {
        let mut input = selected_fixture("primary_ambiguous", &[]);
        input.source = format!("{prefix}#include \"primary_ambiguous.h\"");
        let output = projection::generate(&[input], "Fixture").unwrap();
        let rdl = &output.partitions["primary_ambiguous"].rdl;
        assert!(
            rdl.contains("type First = Tag;") && rdl.contains("type Second = Tag;"),
            "{rdl}"
        );
    }
    let mut inputs = [
        selected_fixture("primary_family", &[]),
        selected_fixture("primary_family", &[]),
    ];
    inputs[0].source = "#include \"primary_family.h\"".into();
    inputs[1].name = "other.h".into();
    inputs[1].source = "#define TAG Two\n#include \"primary_family.h\"".into();
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap().records, 1);
    for input in &mut inputs {
        input.source.insert_str(0, "#define OTHER\n");
    }
    let error = projection::generate_partial(&inputs, "Fixture")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("ambiguous primary name")
            && error.contains("Bridge")
            && error.contains("OtherBridge"),
        "{error}"
    );
}
