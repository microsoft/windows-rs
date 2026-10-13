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
    let error = merge(&captured).err().unwrap().to_string();
    assert!(
        error.contains("_GUID") && error.contains("UnexposedAttr"),
        "{error}"
    );
}
