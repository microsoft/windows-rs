use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];
const MEMBER_RECORDS: [&str; 13] = [
    "DeviceIoControl",
    "QuerySecurity",
    "Request",
    "Survivor",
    "AlignedValue",
    "AlignedArray",
    "AlignedUnion",
    "PaddingName",
    "FloatGap",
    "ArrayContainer",
    "AlignedPointer",
    "PointerBefore",
    "AlignmentConflict",
];

fn golden(name: &str, output: &projection::Output) -> String {
    let rdl: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let path = test_clang::input_dir().join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, &rdl).unwrap();
    }
    assert_eq!(
        rdl,
        std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    rdl
}

#[test]
fn member_alignment_metadata() {
    member_alignment();
}

fn member_alignment() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("member_alignment", &MEMBER_RECORDS);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let wide = snapshot.target != Target::X86;
        for (name, size, offsets) in [
            (
                "DeviceIoControl",
                if wide { 32 } else { 16 },
                if wide {
                    vec![0, 8, 16, 24]
                } else {
                    vec![0, 4, 8, 12]
                },
            ),
            (
                "QuerySecurity",
                if wide { 16 } else { 8 },
                if wide { vec![0, 8] } else { vec![0, 4] },
            ),
        ] {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == name)
                .unwrap();
            let definition = record
                .observations
                .iter()
                .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                .unwrap();
            assert_eq!(
                definition.layout,
                Layout {
                    size,
                    alignment: if wide { 8 } else { 4 }
                }
            );
            assert_eq!(
                definition
                    .fields
                    .iter()
                    .map(|field| field.offset_bits / 8)
                    .collect::<Vec<_>>(),
                offsets
            );
            assert_eq!(
                definition.fields[1]
                    .attributes
                    .iter()
                    .any(|attribute| matches!(
                        attribute.kind,
                        AttributeKind::Aligned { bits: 64, .. }
                    )),
                wide
            );
        }
    }
    let result = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(result.output.records, MEMBER_RECORDS.len());
    assert!(result.omissions.is_empty(), "{:?}", result.omissions);
    let rdl = golden("member_alignment", &result.output);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-member-alignment.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join("target\\test-clang-member-alignment.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rewritten).unwrap(), rdl);
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(rewritten).unwrap();
}

#[test]
fn member_alignment_rejects_unqualified_abi() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture(
        "member_alignment_calls",
        &[
            "ByPointer",
            "ArrayParameter",
            "AliasArrayParameter",
            "ByValue",
            "ByReturn",
            "ByContainingArray",
            "AlignedCallback",
            "IAligned",
            "PackedMember",
            "PackedContainer",
            "ByPackedValue",
            "ByPackedReturn",
            "ByPackedPointer",
            "PackedCallback",
            "AlignedBitfield",
        ],
    );
    let imports = [
        "ByPointer",
        "ArrayParameter",
        "AliasArrayParameter",
        "ByValue",
        "ByReturn",
        "ByContainingArray",
        "ByPackedValue",
        "ByPackedReturn",
        "ByPackedPointer",
    ]
    .into_iter()
    .map(|name| {
        (
            name.into(),
            projection::Import {
                library: "fixture.dll".into(),
                name: None,
            },
        )
    })
    .collect();
    let result = projection::project_partial_with_imports(
        capture_partial(&[input], &TARGETS).unwrap(),
        "Fixture",
        &imports,
    )
    .unwrap();
    for name in [
        "ByValue",
        "ByReturn",
        "ByContainingArray",
        "AlignedCallback",
        "IAligned",
        "ByPackedValue",
        "ByPackedReturn",
        "PackedCallback",
    ] {
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains(name) && message.contains("calling-ABI contract")),
            "{name}: {:?}",
            result.omissions
        );
    }
    assert!(
        result
            .omissions
            .iter()
            .any(|message| message.contains("AlignedBitfield")),
        "{:?}",
        result.omissions
    );
    let rdl = golden("member_alignment_calls", &result.output);
    for name in [
        "ByPointer",
        "ArrayParameter",
        "AliasArrayParameter",
        "ByPackedPointer",
    ] {
        assert!(rdl.contains(&format!("fn {name}(")), "{name}: {rdl}");
    }
}

#[test]
fn member_alignment_requests_conflict_even_with_identical_layout() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::selected_fixture("member_alignment", &["AlignmentConflict"]),
        test_clang::selected_fixture("member_alignment", &["AlignmentConflict"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source.insert_str(0, "#define MEMBER_REQUEST 4\n");
    for input in &inputs {
        for snapshot in capture(std::slice::from_ref(input), &TARGETS).unwrap() {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == "AlignmentConflict")
                .unwrap();
            for definition in record
                .observations
                .iter()
                .filter_map(|id| snapshot.observations[id.0].definition.as_ref())
            {
                assert_eq!(
                    definition.layout,
                    Layout {
                        size: 16,
                        alignment: 16
                    }
                );
                assert_eq!(definition.fields[0].offset_bits, 0);
            }
        }
    }
    for _ in 0..2 {
        let error = capture_partial(&inputs, &TARGETS)
            .err()
            .unwrap()
            .to_string();
        assert!(
            error.contains("AlignmentConflict") && error.contains("differ"),
            "{error}"
        );
        inputs.reverse();
    }
}
