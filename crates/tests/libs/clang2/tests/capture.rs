use windows_clang2::*;

use test_clang2::shared_graph;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input(name: &str, source: &str) -> Input {
    Input {
        name: name.into(),
        source: source.into(),
        ..Default::default()
    }
}

fn fixture() -> Input {
    test_clang2::fixture("record")
}

fn check_record(snapshot: &Snapshot) {
    let x86 = snapshot.target == Target::X86;
    assert_eq!(snapshot.records.len(), 1);
    assert_eq!(snapshot.observations.len(), 1);
    assert_eq!(snapshot.files, ["record.h"]);
    let record = &snapshot.records[0];
    assert_eq!(record.name, "Record");
    assert!(!record.usr.is_empty());
    assert_eq!(record.observations, [ObservationId(0)]);
    let observation = &snapshot.observations[0];
    assert_eq!(observation.record, RecordId(0));
    assert_eq!(observation.location.line, 1);
    let definition = observation.definition.as_ref().unwrap();
    assert_eq!(
        definition.layout,
        Layout {
            size: if x86 { 16 } else { 24 },
            alignment: if x86 { 4 } else { 8 }
        }
    );
    assert_eq!(
        definition
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect::<Vec<_>>(),
        ["count", "native_long", "next", "bytes"]
    );
    assert_eq!(
        definition
            .fields
            .iter()
            .map(|field| field.offset_bits)
            .collect::<Vec<_>>(),
        if x86 {
            [0, 32, 64, 96]
        } else {
            [0, 32, 64, 128]
        }
    );
    for (i, field) in definition.fields.iter().enumerate() {
        assert_eq!(field.location.line as usize, i + 2);
        for id in [field.written, field.canonical] {
            let ty = &snapshot.types[id.0];
            match i {
                0 | 1 => {
                    assert_eq!(
                        ty.kind,
                        TypeKind::Scalar(if i == 0 { Scalar::Int } else { Scalar::Long })
                    );
                    assert_eq!(
                        ty.layout,
                        Some(Layout {
                            size: 4,
                            alignment: 4
                        })
                    );
                }
                2 => {
                    let TypeKind::Pointer(pointee) = ty.kind else {
                        panic!("expected pointer")
                    };
                    assert!(!ty.qualifiers.constant);
                    assert_eq!(ty.layout.unwrap().size, if x86 { 4 } else { 8 });
                    assert_eq!(
                        snapshot.types[pointee.0].kind,
                        TypeKind::Record(RecordId(0))
                    );
                    assert!(snapshot.types[pointee.0].qualifiers.constant);
                }
                3 => {
                    let TypeKind::Array { element, length } = ty.kind else {
                        panic!("expected array")
                    };
                    assert_eq!(length, Some(3));
                    assert_eq!(
                        snapshot.types[element.0].kind,
                        TypeKind::Scalar(Scalar::UnsignedChar)
                    );
                }
                _ => unreachable!(),
            }
        }
    }
    assert_ne!(definition.fields[0].written, definition.fields[1].written);
}

#[test]
fn native_slice() {
    helpers::ensure_libclang();
    let snapshots = capture(&[fixture()], &TARGETS).unwrap();
    assert_eq!(snapshots.len(), TARGETS.len());
    let snapshots = std::thread::spawn(move || {
        for (snapshot, target) in snapshots.iter().zip(TARGETS) {
            assert_eq!(snapshot.target, target);
            assert_eq!(snapshot.input, "record.h");
            assert!(!snapshot.compiler.is_empty());
            assert!(snapshot.diagnostics.is_empty());
            check_record(snapshot);
        }
        snapshots
    })
    .join()
    .unwrap();
    for (i, target) in TARGETS.into_iter().enumerate() {
        assert_eq!(
            snapshots[i],
            capture(&[fixture()], &[target]).unwrap().remove(0)
        );
    }
    std::thread::scope(|scope| {
        let first = scope.spawn(|| capture(&[fixture()], &TARGETS).unwrap());
        let second = scope.spawn(|| capture(&[fixture()], &TARGETS).unwrap());
        assert_eq!(snapshots, first.join().unwrap());
        assert_eq!(snapshots, second.join().unwrap());
    });
    let broken = input("broken.h", "struct Record {\n int value\n};");
    let error = capture(&[broken], &TARGETS).unwrap_err().to_string();
    assert!(error.contains("broken.h:2:"), "{error}");
    for target in TARGETS {
        assert!(error.contains(target.triple()), "{error}");
    }
    let bitfields = capture(
        &[input(
            "bitfield.h",
            "struct Record { unsigned flags : 3; };",
        )],
        &[Target::X64],
    )
    .unwrap();
    assert_eq!(
        bitfields[0].observations[0]
            .definition
            .as_ref()
            .unwrap()
            .fields[0]
            .bit_width,
        Some(3)
    );
    assert_eq!(snapshots, capture(&[fixture()], &TARGETS).unwrap());
    multi_tu();
    scaling();
}

fn summary(resolution: &Resolution<'_>) -> Vec<String> {
    fn ty(snapshot: &Snapshot, id: TypeId) -> String {
        let value = &snapshot.types[id.0];
        let kind = match &value.kind {
            TypeKind::Void => "void".into(),
            TypeKind::Scalar(scalar) => format!("{scalar:?}"),
            TypeKind::Pointer(target) => format!("*{}", ty(snapshot, *target)),
            TypeKind::LValueReference(target) => format!("&{}", ty(snapshot, *target)),
            TypeKind::Array { element, length } => {
                format!("[{};{length:?}]", ty(snapshot, *element))
            }
            TypeKind::Record(record) => snapshot.records[record.0].name.clone(),
            TypeKind::Alias(alias) => snapshot.aliases[alias.0].name.clone(),
            TypeKind::Enum(id) => snapshot.enums[id.0].name.clone(),
            TypeKind::Function(function) => format!(
                "{:?}({:?})->{}",
                function.convention,
                function
                    .parameters
                    .iter()
                    .map(|id| ty(snapshot, *id))
                    .collect::<Vec<_>>(),
                ty(snapshot, function.result)
            ),
            TypeKind::Attributed {
                convention,
                modified,
                equivalent,
            } => format!(
                "{convention:?} {} => {}",
                ty(snapshot, *modified),
                ty(snapshot, *equivalent)
            ),
        };
        format!("{kind}{:?}", value.qualifiers)
    }
    resolution
        .groups
        .iter()
        .map(|group| {
            let name = resolution.name(group);
            let Some(reference) = group.definition else {
                return format!("{name}: incomplete");
            };
            let snapshot = &resolution.snapshots[reference.snapshot];
            let definition = resolution.definition(group).unwrap();
            format!(
                "{name} {:?} {:?}",
                definition.layout,
                definition
                    .fields
                    .iter()
                    .map(|field| {
                        (
                            field.name.as_str(),
                            field.offset_bits,
                            ty(snapshot, field.written),
                            ty(snapshot, field.canonical),
                        )
                    })
                    .collect::<Vec<_>>()
            )
        })
        .collect()
}

fn multi_tu() {
    let a = include_str!("../input/shared_a.h");
    let b = include_str!("../input/shared_b.h");
    for target in TARGETS {
        let snapshots = capture(&[input("a.h", a), input("b.h", b)], &[target]).unwrap();
        let resolved = resolve(&snapshots).unwrap();
        assert_eq!(resolved.groups.len(), 2);
        assert_eq!(resolved.work.definition_pairs, 2);
        for snapshot in &snapshots {
            assert_eq!(snapshot.records.len(), 2);
            assert_eq!(snapshot.observations.len(), 5);
            assert_eq!(snapshot.files.len(), 1);
        }
        let expected = summary(&resolved);
        for inputs in [
            [input("b.h", b), input("a.h", a)],
            [input("z.h", a), input("a.h", b)],
            [
                input("a.h", &a.replace("struct Node;\n", "")),
                input("b.h", b),
            ],
        ] {
            assert_eq!(
                expected,
                summary(&resolve(&capture(&inputs, &[target]).unwrap()).unwrap())
            );
        }
        let incompatible = capture(
            &[
                input("a.h", a),
                input("conflict.h", include_str!("../input/shared_conflict.h")),
            ],
            &[target],
        )
        .unwrap();
        let error = resolve(&incompatible).err().unwrap().to_string();
        assert!(
            error.contains("native field types differ")
                && error.contains("conflict.h")
                && error.contains("a.h"),
            "{error}"
        );
        let completion = capture(
            &[
                input("use.h", include_str!("../input/completion_use.h")),
                input(
                    "definition.h",
                    include_str!("../input/completion_definition.h"),
                ),
            ],
            &[target],
        )
        .unwrap();
        let resolved = resolve(&completion).unwrap();
        let record = completion[0]
            .records
            .iter()
            .position(|record| record.name == "Node")
            .unwrap();
        let group = resolved.group(RecordRef {
            snapshot: 0,
            record: RecordId(record),
        });
        assert!(resolved.definition(group).is_some());
        assert!(
            completion[0].records[record]
                .observations
                .iter()
                .all(|id| { completion[0].observations[id.0].definition.is_none() })
        );
        assert_eq!(resolved.work.definition_pairs, 1);

        // A conflict in a dependent definition must fail even when Wrapper's pointer shape matches.
        let bad =
            include_str!("../input/completion_definition.h").replace("int value", "long value");
        let conflict = capture(
            &[
                input("use.h", include_str!("../input/completion_use.h")),
                input(
                    "definition.h",
                    include_str!("../input/completion_definition.h"),
                ),
                input("bad.h", &bad),
            ],
            &[target],
        )
        .unwrap();
        assert!(resolve(&conflict).is_err());
        let opaque = capture(
            &[
                input("a.h", "struct A; struct B; struct Wrapper { A* value; };"),
                input("b.h", "struct A; struct B; struct Wrapper { B* value; };"),
            ],
            &[target],
        )
        .unwrap();
        assert!(resolve(&opaque).is_err());
    }
    let different_targets = capture(&[fixture()], &TARGETS).unwrap();
    assert!(resolve(&different_targets).is_err());
}

fn scaling() {
    let mut previous = None;
    for count in [64, 128, 256] {
        let snapshots = capture(&shared_graph(count), &[Target::X64]).unwrap();
        let resolved = resolve(&snapshots).unwrap();
        assert_eq!(resolved.groups.len(), count);
        assert_eq!(resolved.work.definition_pairs, count);
        assert!(
            resolved.work.type_pairs <= 3 * count + 1,
            "{:?}",
            resolved.work
        );
        for snapshot in &snapshots {
            let stats = snapshot.statistics();
            assert_eq!(stats.records, count);
            assert_eq!(stats.observations, 2 * count);
            assert_eq!(stats.fields, 3 * count - 1);
            assert_eq!(stats.types, 2 * count + 1);
            assert_eq!(snapshot.files.len(), 1);
        }
        if let Some(work) = previous {
            assert!(resolved.work.type_pairs <= 2 * work + 1);
        }
        previous = Some(resolved.work.type_pairs);
    }
}
