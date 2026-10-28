use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{Type, reader};

#[test]
fn shared() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("shared_owner_a", &[]),
        selected_fixture("shared_owner_b", &[]),
        selected_fixture("shared_consumer", &[]),
    ];
    for input in &mut inputs {
        input.source = format!("#include \"{}\"", input.name);
    }
    let output = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!((output.records, output.enums, output.aliases), (2, 1, 3));
    assert_eq!(output.partitions.len(), 2);
    for stem in ["shared", "shared_consumer"] {
        let expected = input_dir().join(format!("{stem}.rdl"));
        if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
            std::fs::write(&expected, &output.partitions[stem].rdl).unwrap();
        }
        assert_eq!(
            output.partitions[stem].rdl,
            std::fs::read_to_string(expected)
                .unwrap()
                .replace("\r\n", "\n")
        );
    }
    let owners: Vec<_> = output.partitions["shared"]
        .headers
        .iter()
        .map(|header| {
            std::path::Path::new(header)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        })
        .collect();
    assert_eq!(owners, ["shared_owner_a.h", "shared_owner_b.h"]);
    assert_eq!(output.partitions["shared_consumer"].headers.len(), 1);
    assert_eq!(
        projection::generate_partial(&inputs, "Fixture")
            .unwrap()
            .output,
        output
    );
    inputs.reverse();
    for (index, input) in inputs.iter_mut().enumerate() {
        input.name = format!("renamed_{index}.h");
    }
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap(), output);

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-shared.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|partition| &partition.rdl))
        .output(&winmd)
        .write()
        .unwrap();
    for arch in [1, 2, 4] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], arch);
        for alias in ["Count", "PlatformCount"] {
            assert_eq!(
                index.expect("Fixture", alias).underlying_type(),
                Some(Type::U32)
            );
        }
        assert_eq!(
            index
                .expect("Fixture", "SharedRecord")
                .fields()
                .next()
                .unwrap()
                .ty(),
            Type::value_named("Fixture", "Count")
        );
    }
    std::fs::remove_file(winmd).unwrap();

    let mut conflict = selected_fixture("shared_owner_b", &[]);
    conflict.source = "#define CONFLICT\n#include \"shared_owner_b.h\"".into();
    let error = projection::generate_partial(
        &[selected_fixture("shared_owner_a", &[]), conflict],
        "Fixture",
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("native field types differ"), "{error}");
    for names in [("a\\same.h", "b\\same.h"), ("shared.h", "other.h")] {
        let inputs = [
            Input {
                name: names.0.into(),
                source: "struct First { int value; };".into(),
                ..Default::default()
            },
            Input {
                name: names.1.into(),
                source: "struct Second { int value; };".into(),
                ..Default::default()
            },
        ];
        let error = projection::generate(&inputs, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("collide"), "{error}");
    }
}

#[test]
fn one_header_in_multiple_translation_units() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("shared_owner_a", &[]),
        selected_fixture("shared_owner_a", &[]),
    ];
    for (index, input) in inputs.iter_mut().enumerate() {
        input.name = format!("consumer_{index}.h");
        input.source = "#include \"shared_owner_a.h\"".into();
    }
    let output = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!(output.partitions.len(), 1);
    assert_eq!(output.partitions["shared_owner_a"].headers.len(), 1);
}
