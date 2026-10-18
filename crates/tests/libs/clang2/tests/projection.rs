use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;
use windows_metadata::reader::HasAttributes;
use windows_metadata::{Type, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn value_inheritance() {
    inheritance_layouts(&[None]);
    let mut inputs = [
        selected_fixture("value_inheritance", &[]),
        selected_fixture("value_inheritance", &[]),
    ];
    let expected = projection::generate(&inputs[..1], "Fixture").unwrap();
    let mut partial = selected_fixture("value_inheritance", &[]);
    partial.source = format!(
        "struct Rejected {{ volatile int value; }};\n{}",
        partial.source
    );
    let partial = projection::generate_partial(&[partial], "Fixture").unwrap();
    assert!(!partial.omissions.is_empty());
    assert_eq!(partial.output, expected);
    inputs[1].name = "other.h".into();
    inputs[1].source = format!("struct Unrelated {{ int value; }};\n{}", inputs[1].source);
    let mut snapshots = capture(&inputs, &TARGETS).unwrap();
    let base = |snapshot: &Snapshot| {
        snapshot
            .records
            .iter()
            .position(|record| record.name == "Base")
            .unwrap()
    };
    assert_ne!(base(&snapshots[0]), base(&snapshots[1]));
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    inputs.reverse();
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap(), output);
    let derived = snapshots[1]
        .records
        .iter()
        .find(|record| record.name == "Derived")
        .unwrap()
        .observations[0];
    snapshots[1].observations[derived.0]
        .definition
        .as_mut()
        .unwrap()
        .bases[0]
        .offset_bits += 8;
    assert!(
        merge(&snapshots)
            .err()
            .unwrap()
            .to_string()
            .contains("base identities, order, or offsets")
    );
}

#[test]
#[ignore = "requires the x86, x64, and ARM64 MSVC Rust targets"]
fn value_inheritance_cross_targets() {
    inheritance_layouts(&TARGETS.map(Some));
}

fn inheritance_layouts(targets: &[Option<Target>]) {
    helpers::ensure_libclang();
    let snapshots = capture(&[selected_fixture("value_inheritance", &[])], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.records, 8);
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = input_dir().join("value_inheritance.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let name = if targets.len() == 1 {
        "value-inheritance"
    } else {
        "value-inheritance-cross-targets"
    };
    let winmd = root.join(format!("target\\test-clang2-{name}.winmd"));
    let rewritten = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(text, std::fs::read_to_string(&rewritten).unwrap());
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rewritten)
        .output(&again)
        .write()
        .unwrap();
    let bindings = winmd.with_extension("rs");
    windows_bindgen::builder()
        .input(&again)
        .filter("Fixture")
        .sys()
        .flat()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    for (snapshot, bit) in snapshots.iter().zip([1, 2, 4]) {
        let pointer_size = if bit == 1 { 4 } else { 8 };
        let arch = match snapshot.target {
            Target::X86 => "x86",
            Target::X64 => "x86_64",
            Target::Arm64 => "aarch64",
        };
        rust.push_str(&format!(
            "\n#[cfg(target_arch = \"{arch}\")] const _: () = {{\n"
        ));
        for path in [&winmd, &again] {
            let index =
                reader::Index::new_for_architecture(vec![reader::File::read(path).unwrap()], bit);
            for record in &snapshot.records {
                let native = snapshot.observations[record.observations[0].0]
                    .definition
                    .as_ref()
                    .unwrap();
                let (layout, offsets) = record_layout(&index, &record.name, pointer_size);
                assert_eq!(layout, native.layout, "{arch}: {}", record.name);
                let expected: Vec<_> = native
                    .bases
                    .iter()
                    .map(|base| base.offset_bits)
                    .chain(native.fields.iter().map(|field| field.offset_bits))
                    .collect();
                assert_eq!(offsets, expected, "{arch}: {}", record.name);
                let fields: Vec<_> = index.expect("Fixture", &record.name).fields().collect();
                for (i, field) in fields.iter().enumerate() {
                    assert_eq!(
                        field.find_attribute("NativeBaseAttribute").is_some(),
                        i < native.bases.len()
                    );
                    rust.push_str(&format!(
                        "assert!(core::mem::offset_of!({}, {}) == {});\n",
                        record.name,
                        field.name(),
                        expected[i] / 8
                    ));
                }
                rust.push_str(&format!("assert!(core::mem::size_of::<{}>() == {});\nassert!(core::mem::align_of::<{}>() == {});\n",
                    record.name, layout.size, record.name, layout.alignment));
            }
        }
        rust.push_str("};\n");
    }
    std::fs::write(&bindings, rust).unwrap();
    let compiled = winmd.with_extension("rmeta");
    for target in targets {
        let mut command = std::process::Command::new("rustc");
        command.args([
            "--crate-name",
            "test_clang2_inheritance",
            "--crate-type=lib",
            "--emit=metadata",
            "--edition=2024",
        ]);
        if let Some(target) = target {
            command.args(["--target", target.triple()]);
        }
        let result = command
            .arg(&bindings)
            .arg("-o")
            .arg(&compiled)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{target:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for path in [winmd, rewritten, again, bindings, compiled] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn projection_gate() {
    helpers::ensure_libclang();
    round_trip();
    partial_review();
    unions();
    union_families();
    packing();
}

fn partial_review() {
    let captured = capture_partial(&[selected_fixture("partial_kinds", &[])], &TARGETS).unwrap();
    for snapshot in &captured.snapshots {
        assert_eq!(
            snapshot
                .records
                .iter()
                .map(|record| record.name.as_str())
                .collect::<Vec<_>>(),
            ["Safe"]
        );
        assert!(snapshot.aliases.is_empty());
    }
    let mut inputs = [
        selected_fixture("partial_order_a", &[]),
        selected_fixture("partial_order_b", &[]),
    ];
    let first = capture_partial(&inputs, &TARGETS).unwrap_err().to_string();
    assert!(
        first.starts_with("field name, order, offset, bit width, or attributes differ:"),
        "{first}"
    );
    inputs.reverse();
    let second = capture_partial(&inputs, &[Target::Arm64, Target::X64, Target::X86])
        .unwrap_err()
        .to_string();
    assert_eq!(first, second);
    inputs[1]
        .source
        .push_str("\n#error fatal after other TUs succeed\n");
    assert!(
        capture_partial(&inputs, &TARGETS)
            .unwrap_err()
            .to_string()
            .contains("fatal after other TUs succeed")
    );
}

#[test]
fn partial_results() {
    helpers::ensure_libclang();
    let first = selected_fixture("partial", &[]);
    let mut second = selected_fixture("partial", &[]);
    second.name = "second.h".into();
    let mut inputs = [first, second];
    assert!(projection::generate(&inputs, "Fixture").is_err());
    let captured = capture_partial(&inputs, &TARGETS).unwrap();
    merge(&captured.snapshots).unwrap();
    for snapshot in &captured.snapshots {
        for name in [
            "Broken",
            "Dependent",
            "Transitive",
            "PlatformBad",
            "PlatformConsumer",
        ] {
            assert!(
                !snapshot.records.iter().any(|record| record.name == name),
                "{:?}: {name}",
                snapshot.target
            );
        }
        assert!(snapshot.records.iter().any(|record| record.name == "Good"));
    }
    let projected = projection::project_partial(captured, "Fixture").unwrap();
    let result = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert_eq!(projected, result);
    assert_eq!(
        (
            result.output.records,
            result.output.aliases,
            result.output.partitions.len()
        ),
        (14, 5, 2)
    );
    for stem in ["partial_types", "projection_types"] {
        assert_eq!(
            result.output.partitions[stem].rdl,
            std::fs::read_to_string(input_dir().join(format!("{stem}.rdl")))
                .unwrap()
                .replace("\r\n", "\n")
        );
    }
    for omitted in [
        "Broken",
        "Dependent",
        "Transitive",
        "PlatformConsumer",
        "ProjectionBad",
        "ProjectionConsumer",
        "Nothing",
        "unsupported_function",
    ] {
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains(omitted)),
            "{omitted}: {:?}",
            result.omissions
        );
    }
    inputs.reverse();
    assert_eq!(
        result,
        projection::generate_partial(&inputs, "Fixture").unwrap()
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-partial.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(
            result
                .output
                .partitions
                .values()
                .map(|partition| &partition.rdl),
        )
        .output(&winmd)
        .write()
        .unwrap();
    let index = reader::Index::read(&winmd).unwrap();
    assert_eq!(
        index
            .expect("Fixture", "VoidUser")
            .fields()
            .next()
            .unwrap()
            .ty(),
        Type::PtrMut(Box::new(Type::Void), 1)
    );
    std::fs::remove_file(winmd).unwrap();

    let mut broken = selected_fixture("partial", &[]);
    broken
        .source
        .push_str("\n#error intentional compiler failure\n");
    assert!(
        projection::generate_partial(&[broken], "Fixture")
            .unwrap_err()
            .to_string()
            .contains("intentional compiler failure")
    );
    let first = selected_fixture("projection_types", &[]);
    let mut second = selected_fixture("projection_types", &[]);
    second.name = "conflict.h".into();
    second.source = second.source.replace("long x;", "int x;");
    let error = projection::generate_partial(&[first, second], "Fixture")
        .unwrap_err()
        .to_string();
    assert!(
        error.starts_with("capture: native field types differ:"),
        "{error}"
    );
}

fn packing() {
    let mut inputs = [
        selected_fixture("packing_families", &["PublicPacking"]),
        selected_fixture("packing_families", &["PublicPacking"]),
    ];
    inputs[1].name = "second.h".into();
    inputs[1].source.insert_str(0, "#define PACK_TWO\n");
    let error = projection::generate(&inputs, "Fixture")
        .unwrap_err()
        .to_string();
    assert!(error.contains("incompatible native definitions"), "{error}");
    let snapshots = capture(&[selected_fixture("packing", &[])], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let rdl = &output.partitions["packing"].rdl;
    assert_eq!(output.records, 7);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-packing.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let index = reader::Index::read(&winmd).unwrap();
    for (name, packing) in [
        ("One", 1),
        ("Two", 2),
        ("Four", 4),
        ("Eight", 8),
        ("Sixteen", 16),
        ("PackedUnion", 2),
        ("Nested", 2),
    ] {
        assert_eq!(
            index
                .expect("Fixture", name)
                .class_layout()
                .unwrap()
                .packing_size(),
            packing
        );
        for snapshot in &snapshots {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == name)
                .unwrap();
            let native = snapshot.observations[record.observations[0].0]
                .definition
                .as_ref()
                .unwrap();
            let (layout, offsets) = record_layout(&index, name, 8);
            assert_eq!(layout, native.layout, "{name}");
            assert_eq!(
                offsets,
                native
                    .fields
                    .iter()
                    .map(|field| field.offset_bits)
                    .collect::<Vec<_>>()
            );
        }
    }
    let roundtrip = root.join("target\\test-clang2-packing.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&roundtrip).unwrap(), *rdl);
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(roundtrip).unwrap();
}

fn round_trip() {
    let first = selected_fixture("projection", &[]);
    let mut second = selected_fixture("projection", &[]);
    second.name = "second.h".into();
    let inputs = [first, second];
    let mut snapshots = capture(&inputs, &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(
        (output.records, output.aliases, output.partitions.len()),
        (4, 1, 1)
    );
    let rdl = &output.partitions["projection_types"].rdl;
    assert_eq!(
        rdl,
        &std::fs::read_to_string(input_dir().join("projection_types.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap()
    );
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\test-clang2-projection");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("fixture.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for (target, bit) in [(Target::X86, 1), (Target::X64, 2), (Target::Arm64, 4)] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        let pointer_size = if target == Target::X86 { 4 } else { 8 };
        assert_eq!(
            index.expect("Fixture", "Count").underlying_type(),
            Some(Type::U32)
        );
        let node_fields: Vec<_> = index
            .expect("Fixture", "Node")
            .fields()
            .map(|field| field.ty())
            .collect();
        assert_eq!(
            node_fields,
            [
                Type::value_named("Fixture", "Count"),
                Type::PtrConst(Box::new(Type::value_named("Fixture", "Node")), 1),
                Type::ArrayFixed(Box::new(Type::value_named("Fixture", "Point")), 2),
            ]
        );
        let variant = index
            .expect("Fixture", "Variant")
            .fields()
            .next()
            .unwrap()
            .ty();
        assert_eq!(
            variant,
            match target {
                Target::X86 => Type::I32,
                Target::X64 => Type::I64,
                Target::Arm64 => Type::PtrMut(Box::new(Type::Void), 1),
            }
        );
        for snapshot in snapshots
            .iter()
            .filter(|snapshot| snapshot.target == target)
        {
            for record in &snapshot.records {
                let public_name = record.name.trim_start_matches('_');
                for observation in &record.observations {
                    let Some(native) = &snapshot.observations[observation.0].definition else {
                        continue;
                    };
                    let (layout, offsets) = record_layout(&index, public_name, pointer_size);
                    assert_eq!(layout, native.layout, "{target:?} {public_name}");
                    assert_eq!(
                        offsets,
                        native
                            .fields
                            .iter()
                            .map(|field| field.offset_bits)
                            .collect::<Vec<_>>()
                    );
                }
            }
        }
    }
    let roundtrip = scratch.join("fixture.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&roundtrip).unwrap(), *rdl);
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(roundtrip).unwrap();
}

fn record_layout(index: &reader::Index, name: &str, pointer_size: u64) -> (Layout, Vec<u64>) {
    let definition = index.expect("Fixture", name);
    let explicit = definition
        .flags()
        .contains(windows_metadata::TypeAttributes::ExplicitLayout);
    let packing = definition
        .class_layout()
        .map_or(8, |layout| u64::from(layout.packing_size()));
    let mut offsets = Vec::new();
    let mut size = 0_u64;
    let mut alignment = 1;
    for field in definition.fields() {
        let layout = type_layout(index, &field.ty(), pointer_size);
        let field_alignment = layout.alignment.min(packing);
        let offset = if explicit {
            0
        } else {
            size.div_ceil(field_alignment) * field_alignment
        };
        offsets.push(offset * 8);
        size = size.max(offset + layout.size);
        alignment = alignment.max(field_alignment);
    }
    (
        Layout {
            size: size.div_ceil(alignment) * alignment,
            alignment,
        },
        offsets,
    )
}

fn type_layout(index: &reader::Index, ty: &Type, pointer_size: u64) -> Layout {
    let size = match ty {
        Type::I8 | Type::U8 => 1,
        Type::I32 | Type::U32 | Type::F32 => 4,
        Type::I64 | Type::U64 => 8,
        Type::PtrMut(..) | Type::PtrConst(..) => pointer_size,
        Type::ValueName(name) => {
            if let Some(underlying) = index.expect(&name.namespace, &name.name).underlying_type() {
                return type_layout(index, &underlying, pointer_size);
            }
            return record_layout(index, &name.name, pointer_size).0;
        }
        Type::ArrayFixed(element, count) => {
            let mut layout = type_layout(index, element, pointer_size);
            layout.size *= *count as u64;
            return layout;
        }
        _ => panic!("unexpected metadata type: {ty:?}"),
    };
    Layout {
        size,
        alignment: size,
    }
}

fn unions() {
    let mut inputs = [
        selected_fixture("unions", &[]),
        selected_fixture("unions", &[]),
    ];
    inputs[1].name = "second.h".into();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(
        (output.records, output.aliases, output.partitions.len()),
        (5, 3, 1)
    );
    let rdl = &output.partitions["union_types"].rdl;
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join("union_types.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    inputs.reverse();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    assert_eq!(
        output,
        projection::generate_partial(&inputs, "Fixture")
            .unwrap()
            .output
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-unions.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for snapshot in &snapshots {
        let (bit, pointer_size) = match snapshot.target {
            Target::X86 => (1, 4),
            Target::X64 => (2, 8),
            Target::Arm64 => (4, 8),
        };
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        for record in &snapshot.records {
            let name = record.name.trim_start_matches('_');
            let metadata = index.expect("Fixture", name);
            assert_eq!(
                metadata
                    .flags()
                    .contains(windows_metadata::TypeAttributes::ExplicitLayout),
                record.kind == RecordKind::Union
            );
            for id in &record.observations {
                let Some(native) = &snapshot.observations[id.0].definition else {
                    continue;
                };
                let (layout, offsets) = record_layout(&index, name, pointer_size);
                assert_eq!(layout, native.layout, "{:?} {name}", snapshot.target);
                assert_eq!(
                    offsets,
                    native
                        .fields
                        .iter()
                        .map(|field| field.offset_bits)
                        .collect::<Vec<_>>()
                );
            }
        }
        assert_eq!(
            record_layout(&index, "Value", pointer_size).0,
            Layout {
                size: if bit == 1 { 12 } else { 16 },
                alignment: pointer_size,
            }
        );
        assert_eq!(
            record_layout(&index, "PackedValue", pointer_size).0,
            Layout {
                size: 5,
                alignment: 1
            }
        );
    }
    let roundtrip = root.join("target\\test-clang2-unions.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&roundtrip).unwrap(), *rdl);
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(roundtrip).unwrap();

    let result =
        projection::generate_partial(&[selected_fixture("union_rejections", &[])], "Fixture")
            .unwrap();
    assert_eq!(result.output.records, 5);
    assert_eq!(
        result.output.partitions["union_rejections"].rdl,
        std::fs::read_to_string(input_dir().join("union_rejections.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    assert!(result.omissions.is_empty());

    let mut snapshots = capture(&[selected_fixture("unions", &[])], &TARGETS).unwrap();
    let snapshot = &mut snapshots[0];
    let record = snapshot
        .records
        .iter()
        .find(|record| record.name == "_Value")
        .unwrap();
    let definition = snapshot.observations[record.observations[0].0]
        .definition
        .as_mut()
        .unwrap();
    definition.fields[1].offset_bits = 32;
    assert!(
        projection::project(&merge(&snapshots).unwrap(), "Fixture")
            .unwrap_err()
            .to_string()
            .contains("cannot use metadata layout")
    );
}

fn union_families() {
    let mut inputs = [
        selected_fixture("union_family", &[]),
        selected_fixture("union_family", &[]),
    ];
    inputs[1].name = "second.h".into();
    inputs[1].source.insert_str(0, "#define TAG Second\n");
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.records.len(), 2);
    let output = projection::project(&merged, "Fixture").unwrap();
    assert_eq!(output.records, 1);
    assert_eq!(
        output.partitions["union_family_types"].rdl,
        std::fs::read_to_string(input_dir().join("union_family_types.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    inputs[1].source.insert_str(0, "#define KIND struct\n");
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    for pair in snapshots.as_chunks::<2>().0 {
        assert_eq!(
            pair[0].observations[0].definition,
            pair[1].observations[0].definition
        );
        assert_ne!(pair[0].records[0].kind, pair[1].records[0].kind);
    }
    let merged = merge(&snapshots).unwrap();
    assert!(
        projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string()
            .contains("incompatible native definitions")
    );
    assert!(
        projection::generate_partial(&inputs, "Fixture")
            .unwrap_err()
            .to_string()
            .contains("incompatible native definitions")
    );

    inputs[1].source = "#define FIELD_TYPE long\n#include \"union_family_types.h\"\n".into();
    assert!(merge(&capture(&inputs, &TARGETS).unwrap()).is_err());
}

#[test]
fn rejections() {
    helpers::ensure_libclang();
    let input = selected_fixture("projection_unsupported", &[]);
    let error = projection::generate(&[input], "Fixture")
        .unwrap_err()
        .to_string();
    assert!(error.starts_with("capture:"), "{error}");
    assert!(
        error.contains("projection_unsupported.h:2:5: FunctionDecl `unsupported_function`"),
        "{error}"
    );
    for target in TARGETS {
        assert!(error.contains(target.triple()), "{error}");
    }
    for (roots, expected) in [
        (vec!["Empty"], "empty record"),
        (vec!["ConstField"], "const field"),
        (vec!["VolatileField"], "volatile/restrict"),
        (vec!["FieldCollision"], "duplicate projected field"),
    ] {
        let input = selected_fixture("projection_rejections", &roots);
        let error = projection::generate(&[input], "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains(expected), "{roots:?}: {error}");
    }
    let output = projection::generate(
        &[selected_fixture(
            "projection_rejections",
            &["OpaquePointer"],
        )],
        "Fixture",
    )
    .unwrap();
    assert_eq!(output.records, 2);
    assert!(
        output.partitions["projection_rejections"]
            .rdl
            .contains("struct Incomplete {}")
    );
    let first = selected_fixture("projection_types", &["Node"]);
    let mut second = selected_fixture("projection_types", &["Node"]);
    second.name = "other.h".into();
    let output = projection::generate(&[first, second], "Fixture").unwrap();
    assert!(output.partitions["foundation"].rdl.contains("struct Node"));
    let input = selected_fixture("projection", &["Node"]);
    let snapshots = capture(&[input], &[Target::X86]).unwrap();
    assert!(projection::project(&merge(&snapshots).unwrap(), "Fixture").is_err());
}
