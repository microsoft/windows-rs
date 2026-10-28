use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

#[test]
fn nested_record_metadata() {
    nested_records_case();
}

fn nested_records_case() {
    helpers::ensure_libclang();
    let input = selected_fixture("nested_records", &[]);
    let snapshots = capture(
        std::slice::from_ref(&input),
        &[Target::X86, Target::X64, Target::Arm64],
    )
    .unwrap();
    for snapshot in &snapshots {
        let nested = snapshot
            .records
            .iter()
            .position(|record| record.name == "Nested")
            .unwrap();
        let definition = snapshot.records[nested]
            .observations
            .iter()
            .find_map(|id| snapshot.observations[id.0].definition.as_ref())
            .unwrap();
        assert_eq!(
            definition
                .fields
                .iter()
                .map(|field| field.offset_bits)
                .collect::<Vec<_>>(),
            [0, 32, 64]
        );
        assert_eq!(definition.fields[1].name, "");
        let TypeKind::Record(id) = snapshot.types[definition.fields[1].canonical.0].kind else {
            panic!()
        };
        assert_eq!(
            snapshot.records[id.0].owner,
            Some(RecordOwner {
                record: RecordId(nested),
                field: Some(1)
            })
        );
        assert_eq!(snapshot.records[id.0].name, "");
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (8, 1));
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(
            input_dir().join("nested_records.rdl"),
            &output.partitions["nested_records"].rdl,
        )
        .unwrap();
    }
    let expected = std::fs::read_to_string(input_dir().join("nested_records.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["nested_records"].rdl, expected);
    assert_eq!(
        projection::generate_partial(std::slice::from_ref(&input), "Fixture")
            .unwrap()
            .output,
        output
    );

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-nested-records.winmd");
    let rdl = winmd.with_extension("rdl");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&expected)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rdl).unwrap(), expected);
    let index = windows_metadata::reader::Index::read(&winmd).unwrap();
    let named = index
        .nested(index.expect("Fixture", "NamedShadow"))
        .next()
        .unwrap();
    assert_eq!(named.name(), "NamedShadow_0");
    assert_eq!(named.anonymous_index().unwrap(), None);
    let shared = index.expect("Fixture", "Shared");
    assert_eq!(index.nested(shared).count(), 1);
    assert_eq!(
        index
            .nested(shared)
            .next()
            .unwrap()
            .anonymous_index()
            .unwrap(),
        Some(0)
    );
    let fields: Vec<_> = shared.fields().map(|field| field.ty()).collect();
    assert_eq!(
        fields,
        vec![windows_metadata::Type::value_named("Fixture", "Shared/Shared_0"); 2]
    );
    assert_eq!(
        index
            .nested(index.expect("Fixture", "Scoped"))
            .next()
            .unwrap()
            .name(),
        "Item"
    );
    for path in [winmd, rdl] {
        std::fs::remove_file(path).unwrap();
    }

    let mut other = selected_fixture("nested_records", &[]);
    other.name = "other.h".into();
    other.source = format!("\n\n{}", other.source);
    let mut inputs = [input, other];
    let forward = projection::generate(&inputs, "Fixture").unwrap();
    inputs.reverse();
    let reverse = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!(forward, reverse);
    assert_eq!(forward.partitions["shared"].rdl, expected);
}

#[test]
fn owned_public_families_and_conflicts() {
    helpers::ensure_libclang();
    let input = |name: &str, tag: &str, scalar: &str| Input {
        name: name.into(),
        source: format!("typedef struct {tag} {{ struct {{ {scalar} value; }} nested; }} Shared;"),
        ..Default::default()
    };
    let mut inputs = [input("a.h", "A", "int"), input("b.h", "B", "int")];
    let output = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (1, 0));
    inputs.reverse();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    let error = projection::generate(
        &[
            input("a.h", "A", "int"),
            input("bad.h", "A", "unsigned int"),
        ],
        "Fixture",
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("native field types differ"),
        "{error}"
    );
    let error = projection::generate(
        &[
            input("a.h", "A", "int"),
            input("bad.h", "B", "unsigned int"),
        ],
        "Fixture",
    )
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("incompatible native definitions"),
        "{error}"
    );
}

#[test]
fn nested_architecture_identity() {
    helpers::ensure_libclang();
    let mut snapshots = Vec::new();
    for (position, target) in [Target::X86, Target::X64, Target::Arm64]
        .into_iter()
        .enumerate()
    {
        let mut input = selected_fixture("nested_records", &[]);
        input.source = format!("{}{}", "\n".repeat(position * 3), input.source);
        snapshots.extend(capture(&[input], &[target]).unwrap());
    }
    let usrs: std::collections::HashSet<_> = snapshots
        .iter()
        .map(|snapshot| {
            &snapshot
                .records
                .iter()
                .find(|record| {
                    record
                        .owner
                        .is_some_and(|owner| snapshot.records[owner.record.0].name == "Shared")
                })
                .unwrap()
                .usr
        })
        .collect();
    assert_eq!(usrs.len(), 3);
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.records.len(), snapshots[0].records.len());
    assert!(merged.records.iter().all(|record| record.groups.len() == 3));
    assert!(
        merged
            .records
            .iter()
            .any(|record| record.name == "Shared::<field 0>")
    );
}

#[test]
fn nested_failures_prune_only_their_owners() {
    helpers::ensure_libclang();
    let input = Input {
        name: "pruning.h".into(),
        source: "struct Bad { struct Item { int value = 1; }; Item value; }; \
                 struct BadUser { Bad* value; }; \
                 struct Good { struct Item { int value; }; Item value; };"
            .into(),
        ..Default::default()
    };
    let result = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(result.output.records, 1);
    assert!(
        result.output.partitions["pruning"]
            .rdl
            .contains("struct Good")
    );
    assert!(
        result
            .omissions
            .iter()
            .any(|message| message.contains("field initializers"))
    );
    assert!(
        result
            .omissions
            .iter()
            .any(|message| message.contains("BadUser"))
    );
    assert!(
        !result
            .omissions
            .iter()
            .any(|message| message.contains("Good"))
    );
}

#[test]
fn nested_completion_across_translation_units() {
    helpers::ensure_libclang();
    let declaration = "struct Outer { struct Inner; Inner* value; };";
    let mut inputs = [
        Input {
            name: "a.h".into(),
            source: declaration.into(),
            ..Default::default()
        },
        Input {
            name: "b.h".into(),
            source: format!("{declaration} struct Outer::Inner {{ int value; }};"),
            ..Default::default()
        },
    ];
    let output = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!(output.records, 1);
    assert!(
        output.partitions["shared"]
            .rdl
            .contains("value: *mut Outer::Inner")
    );
    inputs.reverse();
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap(), output);
}

#[test]
fn nested_written_aliases_must_agree() {
    helpers::ensure_libclang();
    let source = "typedef int First; typedef int Second; \
                  struct Outer { struct Inner { First value; }; Inner value; };";
    let inputs = [
        Input {
            name: "a.h".into(),
            source: source.into(),
            ..Default::default()
        },
        Input {
            name: "b.h".into(),
            source: source.replace("First value", "Second value"),
            ..Default::default()
        },
    ];
    assert!(projection::generate(&inputs, "Fixture").is_err());
    assert!(projection::generate_partial(&inputs, "Fixture").is_err());
}
