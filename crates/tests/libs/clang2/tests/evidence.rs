use test_clang2::selected_fixture;
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn macro_probe_provenance() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("macro_provenance", &["CHOICE"]);
    input.macros = vec!["CHOICE".into()];
    for snapshot in capture(std::slice::from_ref(&input), &TARGETS).unwrap() {
        let value = snapshot
            .values
            .iter()
            .find(|value| value.name == "CHOICE")
            .unwrap();
        let observation = &value.observations[0];
        assert_eq!(observation.location.line, 7);
        assert!(matches!(
            observation.kind,
            ValueKind::Constant {
                value: Some(ConstantValue::Integer(EnumValue::Signed(2))),
                origin: ConstantOrigin::Macro,
                ..
            }
        ));
    }
    input.roots = vec!["__windows_clang_macro_0".into()];
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in snapshots {
        let record = snapshot
            .records
            .iter()
            .find(|record| record.name == "__windows_clang_macro_0")
            .unwrap();
        assert_eq!(record.kind, RecordKind::Struct);
        assert_eq!(snapshot.values[0].name, "CHOICE");
    }
    input.arguments.push("-DSTALE=1".into());
    for error in [
        capture(std::slice::from_ref(&input), &TARGETS).unwrap_err(),
        capture_partial(&[input], &TARGETS).unwrap_err(),
    ] {
        assert!(
            error
                .to_string()
                .contains("no expansion at its evaluation probe"),
            "{error}"
        );
    }
}

#[test]
fn compact_attribute_evidence() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("attribute_evidence", &["Imported"]);
    let small = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    input.source = input.source.replace(
        "struct Unrelated",
        &format!("{}struct Unrelated", "// unrelated source\n".repeat(10_000)),
    );
    let large = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for (small, large) in small.iter().zip(&large) {
        assert_eq!(
            small.statistics().owned_bytes,
            large.statistics().owned_bytes
        );
        let attribute = &large.values[0].observations[0].attributes[0];
        assert_eq!(attribute.kind, AttributeKind::DllImport);
        assert_eq!(attribute.location.as_ref().unwrap().line, 10_006);
    }
    input.roots = vec!["Exported".into()];
    let exported = capture_partial(std::slice::from_ref(&input), &TARGETS).unwrap();
    assert!(exported.omissions.is_empty());
    for snapshot in exported.snapshots {
        assert_eq!(
            snapshot.values[0].observations[0].attributes[0].kind,
            AttributeKind::DllExport
        );
    }
    input.roots = vec!["Unsupported".into()];
    let partial = capture_partial(&[input], &TARGETS).unwrap();
    assert_eq!(partial.omissions.len(), 1);
    assert!(partial.omissions[0].contains("NoInline"));
    assert!(partial.omissions[0].len() < 256, "{}", partial.omissions[0]);
}
