use test_clang::selected_fixture;
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn selection_and_variants() {
    helpers::ensure_libclang();
    let a = selected_fixture("selection", &["Root", "Root"]);
    let mut b = selected_fixture("selection", &["Root"]);
    b.name = "second.h".into();
    let mut snapshots = capture(&[a, b], &TARGETS).unwrap();
    for snapshot in &snapshots {
        assert_eq!(snapshot.roots.len(), 1);
        let DeclarationId::Record(root) = snapshot.roots[0] else {
            panic!("expected record root")
        };
        assert_eq!(snapshot.records[root.0].name, "Root");
        let mut names: Vec<_> = snapshot
            .records
            .iter()
            .map(|record| record.name.as_str())
            .collect();
        names.sort_unstable();
        let expected = match snapshot.target {
            Target::X86 => vec!["Leaf", "Opaque", "Required", "Root", "X86Only"],
            Target::X64 => vec!["Leaf", "Opaque", "Required", "Root"],
            Target::Arm64 => vec!["ArmOnly", "Leaf", "Opaque", "Required", "Root"],
        };
        assert_eq!(names, expected);
        let required = snapshot
            .records
            .iter()
            .find(|record| record.name == "Required")
            .unwrap();
        assert_eq!(required.observations.len(), 3);
        assert!(
            snapshot
                .files
                .iter()
                .any(|file| file.ends_with("dependencies.h"))
        );
    }
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.records.len(), 6);
    assert_eq!(merged.targets.len(), 3);
    for record in &merged.records {
        let targets: Vec<_> = record.groups.keys().copied().collect();
        match record.name.as_str() {
            "X86Only" => assert_eq!(targets, [Target::X86]),
            "ArmOnly" => assert_eq!(targets, [Target::Arm64]),
            _ => assert_eq!(targets, TARGETS),
        }
        for (&target, &id) in &record.groups {
            let resolution = &merged.targets[&target];
            let group = &resolution.groups[id];
            assert_eq!(group.records.len(), 2);
            if record.name == "Opaque" {
                assert!(resolution.definition(group).is_none());
            }
            if record.name == "Root" {
                let definition = resolution.definition(group).unwrap();
                assert_eq!(definition.fields.len(), 4);
                assert_eq!(
                    definition.layout,
                    if target == Target::X86 {
                        Layout {
                            size: 24,
                            alignment: 4,
                        }
                    } else {
                        Layout {
                            size: 40,
                            alignment: 8,
                        }
                    }
                );
                let reference = group.definition.unwrap();
                let snapshot = resolution.snapshots[reference.snapshot];
                let arch = &snapshot.types[definition.fields[3].canonical.0].kind;
                match target {
                    Target::X86 => assert!(matches!(arch, TypeKind::Pointer(_))),
                    Target::X64 => assert_eq!(*arch, TypeKind::Scalar(Scalar::Long)),
                    Target::Arm64 => assert!(matches!(arch, TypeKind::Record(_))),
                }
            }
        }
    }
    let expected = summary(&merged);
    snapshots.reverse();
    assert_eq!(summary(&merge(&snapshots).unwrap()), expected);
    snapshots.rotate_left(1);
    assert_eq!(summary(&merge(&snapshots).unwrap()), expected);
    errors();
    selective_scaling();
    multi_tu_scaling();
}

fn summary(merged: &Merged<'_>) -> Vec<String> {
    merged
        .records
        .iter()
        .map(|record| {
            let variants: Vec<_> = record
                .groups
                .iter()
                .map(|(target, id)| {
                    let resolution = &merged.targets[target];
                    let definition = resolution.definition(&resolution.groups[*id]);
                    (
                        target,
                        definition.map(|value| {
                            (
                                value.layout,
                                value
                                    .fields
                                    .iter()
                                    .map(|field| (&field.name, field.offset_bits))
                                    .collect::<Vec<_>>(),
                            )
                        }),
                    )
                })
                .collect();
            format!("{} {variants:?}", record.name)
        })
        .collect()
}

fn errors() {
    for (fixture, root, message) in [(
        "selection",
        "Missing",
        "requested declaration `Missing` not found",
    )] {
        let error = capture(&[selected_fixture(fixture, &[root])], &TARGETS)
            .unwrap_err()
            .to_string();
        assert!(error.contains(message), "{error}");
        for target in TARGETS {
            assert!(error.contains(target.triple()), "{error}");
        }
        let snapshots = capture(
            &[selected_fixture("reachable_bitfield", &["Broken"])],
            &TARGETS,
        )
        .unwrap();
        assert_eq!(merge(&snapshots).unwrap().records.len(), 2);
    }
    let mut bad_arguments = selected_fixture("selection", &["Root"]);
    bad_arguments.arguments.push("invalid\0argument".into());
    assert!(capture(&[bad_arguments], &TARGETS).is_err());
    let mut broken = selected_fixture("selection", &["Root"]);
    broken.source.push_str("\nstruct Unselected { int value };");
    assert!(capture(&[broken], &TARGETS).is_err());

    let a = selected_fixture("selection", &["Root"]);
    let mut b = selected_fixture("selection", &["Root"]);
    b.name = "conflict.h".into();
    b.source = b.source.replace("long arch;", "int arch;");
    let snapshots = capture(&[a, b], &TARGETS).unwrap();
    let error = merge(&snapshots).err().unwrap().to_string();
    assert!(
        error.contains(Target::X64.triple()) && error.contains("native field types differ"),
        "{error}"
    );

    let a = selected_fixture("selection", &["Root"]);
    let mut b = selected_fixture("selection", &["Root"]);
    b.name = "configuration.h".into();
    b.arguments.push("-DOTHER_CONFIGURATION".into());
    let snapshots = capture(&[a, b], &TARGETS).unwrap();
    assert!(merge(&snapshots).is_err());
    assert!(merge(&[]).is_err());
}

fn multi_tu_scaling() {
    let count = 32;
    let [template, _] = test_clang::shared_graph(count);
    for tus in [2, 8, 32] {
        let inputs: Vec<_> = (0..tus)
            .map(|id| Input {
                name: format!("graph_{id}.h"),
                source: template.source.clone(),
                roots: vec![format!("R{}", count - 1)],
                ..Default::default()
            })
            .collect();
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        assert_eq!(merged.records.len(), count);
        for resolution in merged.targets.values() {
            assert_eq!(resolution.work.definition_pairs, count * (tus - 1));
            assert!(resolution.work.type_pairs <= (2 * count + 1) * (tus - 1));
        }
        for snapshot in &snapshots {
            assert_eq!(snapshot.roots.len(), 1);
            assert_eq!(snapshot.records.len(), count);
        }
    }
}

fn selective_scaling() {
    let mut previous = None;
    for count in [64, 512, 4096] {
        let mut source = "struct Root { int value; };\n".to_string();
        for id in 0..count {
            source.push_str(&format!("struct Unused{id} {{ unsigned flag : 1; }};\n"));
        }
        let snapshots = capture(
            &[Input {
                name: "wide.h".into(),
                source,
                roots: vec!["Root".into()],
                ..Default::default()
            }],
            &TARGETS,
        )
        .unwrap();
        for snapshot in &snapshots {
            let stats = snapshot.statistics();
            assert_eq!(
                (stats.records, stats.observations, stats.fields, stats.types),
                (1, 1, 1, 1)
            );
        }
        let bytes: usize = snapshots
            .iter()
            .map(|snapshot| snapshot.statistics().owned_bytes)
            .sum();
        if let Some(previous) = previous {
            assert_eq!(bytes, previous);
        }
        previous = Some(bytes);
    }
}
