use windows_clang2::*;

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn sdk_header() {
    helpers::ensure_libclang();
    let a = test_clang2::sdk_fixture();
    let mut b = test_clang2::sdk_fixture();
    b.name = "second.h".into();
    let snapshots = capture(&[a, b], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.records.len(), 2);
    for snapshot in &snapshots {
        assert_eq!(snapshot.records.len(), 2);
        assert_eq!(snapshot.roots.len(), 1);
        let DeclarationId::Record(root) = snapshot.roots[0] else {
            panic!("expected record root")
        };
        assert_eq!(snapshot.records[root.0].name, "ListHolder");
        assert!(snapshot.files.iter().any(|file| file.ends_with("winnt.h")));
    }
    for record in &merged.records {
        assert_eq!(record.groups.len(), 3);
        for (&target, &id) in &record.groups {
            let resolution = &merged.targets[&target];
            let group = &resolution.groups[id];
            assert_eq!(group.records.len(), 2);
            let definition = resolution.definition(group).unwrap();
            if record.name == "_LIST_ENTRY" {
                assert_eq!(
                    definition.layout,
                    Layout {
                        size: if target == Target::X86 { 8 } else { 16 },
                        alignment: if target == Target::X86 { 4 } else { 8 }
                    }
                );
                assert_eq!(
                    definition
                        .fields
                        .iter()
                        .map(|field| field.offset_bits)
                        .collect::<Vec<_>>(),
                    [0, if target == Target::X86 { 32 } else { 64 }]
                );
            } else {
                assert_eq!(record.name, "ListHolder");
                assert_eq!(
                    definition.layout,
                    if target == Target::X86 {
                        Layout {
                            size: 12,
                            alignment: 4,
                        }
                    } else {
                        Layout {
                            size: 24,
                            alignment: 8,
                        }
                    }
                );
            }
        }
    }
    let mut attributed = test_clang2::sdk_fixture();
    attributed.source = test_clang2::fixture("sdk_attribute").source;
    attributed.name = "sdk_attribute.h".into();
    attributed.roots = vec!["GuidHolder".into()];
    let captured = capture(&[attributed], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let merged = merge(&captured).unwrap();
    assert_eq!(merged.records.len(), 2);
    for snapshot in &captured {
        let guid = snapshot
            .records
            .iter()
            .find(|record| record.name == "_GUID")
            .unwrap();
        let observation = &snapshot.observations[guid.observations[0].0];
        assert!(
            observation
                .attributes
                .iter()
                .any(|attribute| { attribute.kind == AttributeKind::DefaultTypeVisibility })
        );
        assert_eq!(
            observation.definition.as_ref().unwrap().layout,
            Layout {
                size: 16,
                alignment: 4
            }
        );
    }
    let output = projection::project(&merged, "Fixture").unwrap();
    assert_eq!(output.records, 2);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-sdk-guid.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|partition| &partition.rdl))
        .output(&winmd)
        .write()
        .unwrap();
    for bit in [1, 2, 4] {
        use windows_metadata::{Type, reader};
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        assert_eq!(
            index
                .expect("Fixture", "GUID")
                .fields()
                .map(|field| field.ty())
                .collect::<Vec<_>>(),
            [
                Type::U32,
                Type::U16,
                Type::U16,
                Type::ArrayFixed(Box::new(Type::U8), 8)
            ]
        );
        assert_eq!(
            index
                .expect("Fixture", "GuidHolder")
                .fields()
                .next()
                .unwrap()
                .ty(),
            Type::value_named("Fixture", "GUID")
        );
        assert!(index.get("Fixture", "_GUID").next().is_none());
    }
    std::fs::remove_file(winmd).unwrap();
}
