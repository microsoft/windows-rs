use test_clang2::selected_fixture;
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn fidelity_gate() {
    helpers::ensure_libclang();
    aliases();
    attributes();
}

fn aliases() {
    let a = selected_fixture("aliases", &["Node", "Handle"]);
    let mut b = selected_fixture("aliases", &["Node", "Handle"]);
    b.name = "second.h".into();
    b.source = b
        .source
        .replace("OtherCount value", "int value")
        .replace("ConstCount* constant", "const int* constant");
    let snapshots = capture(&[a, b], &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.records.len(), 1);
    assert!(merged.aliases.iter().any(|alias| alias.name == "Handle"));
    for snapshot in &snapshots {
        assert!(snapshot.aliases.iter().any(|alias| alias.name == "Node"));
        let node = snapshot
            .records
            .iter()
            .find(|record| record.name == "Node")
            .unwrap();
        let definition = node
            .observations
            .iter()
            .find_map(|id| snapshot.observations[id.0].definition.as_ref())
            .unwrap();
        if snapshot.input == "aliases.h" {
            assert!(matches!(
                snapshot.types[definition.fields[0].written.0].kind,
                TypeKind::Alias(_)
            ));
        }
        assert_eq!(
            snapshot.types[definition.fields[0].canonical.0].kind,
            TypeKind::Scalar(Scalar::Int)
        );
    }
    let a = selected_fixture("aliases", &["Count"]);
    let mut equivalent = selected_fixture("aliases", &["Count"]);
    equivalent.name = "renamed.h".into();
    let snapshots = capture(&[a, equivalent], &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.aliases.len(), 1);
    for resolution in merged.targets.values() {
        assert_eq!(resolution.aliases[0].observations.len(), 4);
    }
    let a = selected_fixture("aliases", &["Count"]);
    let mut b = selected_fixture("aliases", &["Count"]);
    b.name = "bad.h".into();
    b.source = b
        .source
        .replace("typedef int Count;", "typedef long Count;");
    let snapshots = capture(&[a, b], &TARGETS).unwrap();
    assert!(
        merge(&snapshots).is_err(),
        "{:?}",
        snapshots
            .iter()
            .map(|snapshot| snapshot
                .aliases
                .iter()
                .map(|alias| (&alias.name, &alias.usr))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );

    let snapshots = capture(
        &[selected_fixture("reachable_alias", &["Broken"])],
        &TARGETS,
    )
    .unwrap();
    assert!(merge(&snapshots).is_ok());
}

fn attributes() {
    let snapshots = capture(&[selected_fixture("attributes", &["Packed"])], &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    for resolution in merged.targets.values() {
        let definition = resolution.definition(&resolution.groups[0]).unwrap();
        assert_eq!(
            definition.layout,
            Layout {
                size: 5,
                alignment: 1
            }
        );
        assert_eq!(definition.fields[1].offset_bits, 8);
    }
    for root in ["Aligned", "FieldAligned", "PragmaOne", "PragmaTwo"] {
        let snapshots = capture(&[selected_fixture("attributes", &[root])], &TARGETS).unwrap();
        for snapshot in &snapshots {
            let observation = snapshot
                .observations
                .iter()
                .find(|value| value.definition.is_some())
                .unwrap();
            let definition = observation.definition.as_ref().unwrap();
            match root {
                "Aligned" => assert_eq!(
                    definition.layout,
                    Layout {
                        size: 16,
                        alignment: 16
                    }
                ),
                "FieldAligned" => {
                    assert_eq!(
                        definition.layout,
                        Layout {
                            size: 32,
                            alignment: 16
                        }
                    );
                    assert_eq!(definition.fields[1].offset_bits, 128);
                }
                _ => assert_eq!(
                    definition.layout,
                    Layout {
                        size: 1,
                        alignment: 1
                    }
                ),
            }
            assert!(
                observation
                    .attributes
                    .iter()
                    .chain(definition.fields.iter().flat_map(|field| &field.attributes))
                    .any(|attribute| matches!(attribute.kind, AttributeKind::Unresolved(_)))
            );
            if root.starts_with("Pragma") {
                assert!(observation.attributes.iter().all(
                    |attribute| attribute.spelling.is_empty()
                        && attribute.tokens.is_empty()
                        && attribute.location.is_none()
                ));
            }
        }
        let error = merge(&snapshots).err().unwrap().to_string();
        assert!(error.contains("cannot verify"), "{error}");
    }
}
