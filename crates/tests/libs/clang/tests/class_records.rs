use test_clang::selected_fixture;
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input(spelling: &str) -> Input {
    let mut input = selected_fixture("class_records", &["Inherited", "Value"]);
    input.name = format!("{spelling}.h");
    input.source = format!("#define RECORD {spelling}\n#include \"class_records.h\"\n");
    input
}

#[test]
fn class_and_struct_contracts_match() {
    helpers::ensure_libclang();
    let mut expected = None;
    for (spelling, kind) in [("struct", RecordKind::Struct), ("class", RecordKind::Class)] {
        let snapshots = capture(&[input(spelling)], &TARGETS).unwrap();
        for snapshot in &snapshots {
            for record in &snapshot.records {
                assert_eq!(record.kind, kind);
                let definition = snapshot.observations[record.observations[0].0]
                    .definition
                    .as_ref()
                    .unwrap();
                let size = if record.name == "Value" || snapshot.target == Target::X86 {
                    4
                } else {
                    8
                };
                assert_eq!(
                    definition.layout,
                    Layout {
                        size,
                        alignment: size
                    }
                );
                let slots: Vec<_> = definition
                    .methods
                    .iter()
                    .map(|method| method.slot)
                    .collect();
                match record.name.as_str() {
                    "Base" => assert_eq!(slots, [0]),
                    "Derived" => assert_eq!(slots, [1]),
                    "Inherited" | "Value" => assert!(slots.is_empty()),
                    _ => panic!(),
                }
            }
        }
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        if let Some(expected) = &expected {
            assert_eq!(&output, expected);
        }
        expected = Some(output);
    }
    let expected = expected.unwrap();
    assert_eq!((expected.interfaces, expected.records), (3, 1));
    let mut inputs = [input("class"), input("struct")];
    for _ in 0..2 {
        assert_eq!(projection::generate(&inputs, "Fixture").unwrap(), expected);
        inputs.reverse();
    }
    let text = &expected.partitions["class_records"].rdl;
    let path = test_clang::input_dir().join("class_records.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, text).unwrap();
    }
    assert_eq!(
        text,
        &std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-class-records.winmd");
    let rdl = winmd.with_extension("rdl");
    let mut source = text.clone();
    for _ in 0..2 {
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(&source)
            .output(&winmd)
            .write()
            .unwrap();
        let index = windows_metadata::reader::Index::read(&winmd).unwrap();
        for (name, count) in [("Base", 1), ("Derived", 1), ("Inherited", 0)] {
            assert_eq!(index.expect("Fixture", name).methods().count(), count);
        }
        windows_rdl::writer()
            .input(&winmd)
            .filter("Fixture")
            .output(&rdl)
            .write()
            .unwrap();
        source = std::fs::read_to_string(&rdl).unwrap();
        assert_eq!(&source, text);
    }
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(rdl).unwrap();
    inputs[0].source.insert_str(0, "#define VALUE_TYPE float\n");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        assert!(merge(&snapshots).is_err());
        assert!(capture_partial(&inputs, &TARGETS).is_err());
        inputs.reverse();
    }
}

#[test]
fn unsupported_class_contracts() {
    helpers::ensure_libclang();
    for (name, message) in [
        ("PrivateMethod", "only public virtual"),
        ("PrivateBase", "only public non-virtual bases"),
        ("WithData", "requires one vptr"),
        ("Destroyed", "CXXDestructor"),
    ] {
        let input = selected_fixture("class_records", &[name]);
        let error = projection::generate(&[input], "Fixture").unwrap_err();
        assert!(error.to_string().contains(message), "{name}: {error}");
        let input = selected_fixture("class_records", &[name]);
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert!(
            partial
                .omissions
                .iter()
                .any(|error| error.contains(message)),
            "{name}: {:?}",
            partial.omissions
        );
        assert!(
            !partial
                .output
                .partitions
                .values()
                .any(|part| part.rdl.contains(name)),
            "{name}"
        );
    }
}
