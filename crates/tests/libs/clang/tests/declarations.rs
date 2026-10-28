use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn declaration_fidelity() {
    helpers::ensure_libclang();
    let first = selected_fixture("declarations", &[]);
    let mut second = selected_fixture("declarations", &[]);
    second.name = "second.h".into();
    let mut snapshots = capture(&[first, second], &TARGETS).unwrap();
    let source = std::fs::read_to_string(input_dir().join("declaration_types.h")).unwrap();
    for snapshot in &snapshots {
        for (name, invocation) in [
            ("Handle__", "DECLARE_TAG(Handle)"),
            ("OtherHandle__", "WRAP_TAG(OtherHandle)"),
        ] {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == name)
                .unwrap();
            let observation = record
                .observations
                .iter()
                .map(|id| &snapshot.observations[id.0])
                .find(|observation| observation.definition.is_some())
                .unwrap();
            assert!(
                snapshot.files[observation.location.file.0].ends_with("declaration_types.h"),
                "{name}: {:?}",
                observation.location
            );
            assert_eq!(
                observation.location.offset as usize,
                source.find(invocation).unwrap()
            );
            assert_eq!(observation.location.column, 1);
            let definition = observation.definition.as_ref().unwrap();
            assert_eq!(
                definition.layout,
                Layout {
                    size: 4,
                    alignment: 4
                }
            );
            assert!(
                snapshot.files[definition.fields[0].location.file.0]
                    .ends_with("declaration_types.h")
            );
        }
        let alias = |name| {
            snapshot
                .aliases
                .iter()
                .find(|alias| alias.name == name)
                .unwrap()
                .observations[0]
                .canonical
        };
        let TypeKind::Pointer(record) = snapshot.types[alias("PRecord").0].kind else {
            panic!("expected pointer")
        };
        let TypeKind::Record(record) = snapshot.types[record.0].kind else {
            panic!("expected record")
        };
        assert_eq!(snapshot.records[record.0].name, "_Record");
        assert_eq!(
            snapshot.types[alias("PPRecord").0].layout.unwrap().size,
            if snapshot.target == Target::X86 { 4 } else { 8 }
        );
        assert!(matches!(
            snapshot.types[alias("RecordArray").0].kind,
            TypeKind::Array {
                length: Some(2),
                ..
            }
        ));
        assert!(matches!(
            snapshot.types[alias("PointerArray").0].kind,
            TypeKind::Array {
                length: Some(3),
                ..
            }
        ));
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(
        (output.records, output.aliases, output.partitions.len()),
        (3, 6, 1)
    );
    let rdl = &output.partitions["declaration_types"].rdl;
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join("declaration_types.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap()
    );

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-declarations.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let index = windows_metadata::reader::Index::read(&winmd).unwrap();
    assert_eq!(
        index.expect("Fixture", "PRecord").underlying_type(),
        Some(windows_metadata::Type::PtrMut(
            Box::new(windows_metadata::Type::value_named("Fixture", "Record")),
            1
        ))
    );
    std::fs::remove_file(winmd).unwrap();

    let error = capture(
        &[selected_fixture("declaration_invalid", &["BrokenPointer"])],
        &TARGETS,
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("unsupported type `unsigned int &`"),
        "{error}"
    );
    using_aliases();
    record_alias_identity();
}

fn using_aliases() {
    let first = selected_fixture("using", &[]);
    let mut second = selected_fixture("using", &[]);
    second.name = "typedef.h".into();
    second.source.insert_str(0, "#define TYPEDEF_SYNTAX\n");
    let mut snapshots = capture(&[first, second], &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!((merged.records.len(), merged.aliases.len()), (1, 3));
    let output = projection::project(&merged, "Fixture").unwrap();
    assert_eq!(
        output.partitions["using_types"].rdl,
        std::fs::read_to_string(input_dir().join("using_types.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    drop(merged);
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap()
    );
    let selected = capture(&[selected_fixture("using", &["NumberPointer"])], &TARGETS).unwrap();
    assert!(
        selected
            .iter()
            .all(|snapshot| snapshot.aliases.len() == 2 && snapshot.records.is_empty())
    );
    let snapshots = capture(
        &[
            selected_fixture("using", &[]),
            selected_fixture("using_conflict", &[]),
        ],
        &TARGETS,
    )
    .unwrap();
    assert!(projection::project(&merge(&snapshots).unwrap(), "Fixture").is_err());
}

fn record_alias_identity() {
    let inputs = [
        selected_fixture("record_alias_a", &[]),
        selected_fixture("record_alias_b", &[]),
    ];
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    assert_eq!(snapshots.len(), TARGETS.len() * 2);
    for pair in snapshots.as_chunks::<2>().0 {
        assert!(merge(&pair[..1]).is_ok());
        assert!(merge(&pair[1..]).is_ok());
        assert_ne!(pair[0].records[0].usr, pair[1].records[0].usr);
        let definition = |snapshot: &Snapshot| {
            let definition = snapshot.observations[0].definition.as_ref().unwrap();
            (
                definition.layout,
                definition
                    .fields
                    .iter()
                    .map(|field| (field.name.clone(), field.offset_bits))
                    .collect::<Vec<_>>(),
            )
        };
        assert_eq!(definition(&pair[0]), definition(&pair[1]));
    }
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.records.len(), 2);
    let output = projection::project(&merged, "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (1, 1));
    for stem in ["record_alias_types", "record_alias_b"] {
        assert_eq!(
            output.partitions[stem].rdl,
            std::fs::read_to_string(input_dir().join(format!("{stem}.rdl")))
                .unwrap()
                .replace("\r\n", "\n")
        );
    }
    assert_eq!(
        output,
        projection::project_partial(capture_partial(&inputs, &TARGETS).unwrap(), "Fixture")
            .unwrap()
            .output
    );
    let mut snapshots = snapshots;
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap()
    );
}
