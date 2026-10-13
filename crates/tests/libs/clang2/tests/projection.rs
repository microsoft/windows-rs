use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;
use windows_metadata::{Type, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn projection_gate() {
    helpers::ensure_libclang();
    round_trip();
    rejections();
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
                Type::U32,
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
    let packing = definition
        .class_layout()
        .map_or(8, |layout| u64::from(layout.packing_size()));
    let mut offsets = Vec::new();
    let mut size = 0_u64;
    let mut alignment = 1;
    for field in definition.fields() {
        let layout = type_layout(index, &field.ty(), pointer_size);
        let field_alignment = layout.alignment.min(packing);
        size = size.div_ceil(field_alignment) * field_alignment;
        offsets.push(size * 8);
        size += layout.size;
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
        Type::I32 | Type::U32 => 4,
        Type::I64 => 8,
        Type::PtrMut(..) | Type::PtrConst(..) => pointer_size,
        Type::ValueName(name) => return record_layout(index, &name.name, pointer_size).0,
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

fn rejections() {
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
        (vec!["OpaquePointer"], "is incomplete"),
        (vec!["ConstField"], "const field"),
        (vec!["VolatileField"], "volatile/restrict"),
        (vec!["FieldCollision"], "duplicate projected field"),
        (vec!["First", "Second"], "competing public typedefs"),
    ] {
        let input = selected_fixture("projection_rejections", &roots);
        let error = projection::generate(&[input], "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains(expected), "{roots:?}: {error}");
    }
    let first = selected_fixture("projection_types", &["Node"]);
    let mut second = selected_fixture("projection_types", &["Node"]);
    second.name = "other.h".into();
    let error = projection::generate(&[first, second], "Fixture")
        .unwrap_err()
        .to_string();
    assert!(error.contains("ambiguous header ownership"), "{error}");
    let input = selected_fixture("projection", &["Node"]);
    let snapshots = capture(&[input], &[Target::X86]).unwrap();
    assert!(projection::project(&merge(&snapshots).unwrap(), "Fixture").is_err());
}
