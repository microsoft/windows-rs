use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn imports() -> projection::Imports {
    [
        "EchoPlain",
        "UseAssigned",
        "PointToAssigned",
        "ReturnAssigned",
        "ReturnAlias",
        "ReturnContainer",
        "ReturnDeleted",
        "SetCallback",
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
    .collect()
}

#[test]
fn special_members_preserve_parameter_and_return_distinctions() {
    helpers::ensure_libclang();
    let captured = capture_partial(&[test_clang::fixture("record_abi")], &TARGETS).unwrap();
    assert!(captured.omissions.is_empty(), "{:?}", captured.omissions);
    for snapshot in &captured.snapshots {
        let record = snapshot
            .records
            .iter()
            .find(|record| record.name == "Assigned")
            .unwrap();
        let definition = snapshot.observations[record.observations[0].0]
            .definition
            .as_ref()
            .unwrap();
        assert!(definition.semantics.can_pass_in_registers);
        assert!(definition.semantics.non_trivial_copy_assignment);
    }
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports()).unwrap();
    assert_eq!(result.output.functions, 3);
    assert!(
        result
            .omissions
            .iter()
            .any(|error| error.contains("C++ special members")),
        "{:?}",
        result.omissions
    );
    for name in [
        "ReturnAssigned",
        "ReturnAlias",
        "ReturnContainer",
        "ReturnDeleted",
        "ReturnCallback",
        "SetCallback",
    ] {
        assert!(
            result.omissions.iter().any(|error| error.contains(name)),
            "{name}: {:?}",
            result.omissions
        );
    }
    let actual = &result.output.partitions["record_abi"].rdl;
    let path = test_clang::input_dir().join("record_abi.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, actual).unwrap();
    }
    assert_eq!(
        actual,
        &std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
}

#[test]
fn native_agreement_checks_special_members() {
    helpers::ensure_libclang();
    let mut plain = test_clang::fixture("record_abi");
    plain.name = "plain_record.h".into();
    plain.source.insert_str(0, "#define PLAIN_ASSIGNMENT\n");
    let mut inputs = [plain, test_clang::fixture("record_abi")];
    for _ in 0..2 {
        let error = capture_partial(&inputs, &TARGETS).unwrap_err().to_string();
        assert!(error.contains("special-member semantics differ"), "{error}");
        inputs.reverse();
    }
}

#[test]
fn pointer_data_and_opaque_handles() {
    helpers::ensure_libclang();
    let imports: projection::Imports = [
        "ReadPoints",
        "UseConstructed",
        "UseHandles",
        "ReturnPoint",
        "ReturnConstructed",
        "CopyPoint",
        "ReturnHidden",
        "CopyEmpty",
        "CopyResource",
        "UseInvalidStorage",
        "UseInvalidArray",
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
    let captured =
        capture_partial(&[test_clang::fixture("cpp_pointer_records")], &TARGETS).unwrap();
    assert!(captured.omissions.is_empty(), "{:?}", captured.omissions);
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(result.output.functions, 3, "{:?}", result.omissions);
    for name in [
        "ReturnPoint",
        "ReturnConstructed",
        "CopyPoint",
        "ReturnHidden",
        "CopyEmpty",
        "CopyResource",
        "UseInvalidStorage",
        "UseInvalidArray",
    ] {
        assert!(
            result.omissions.iter().any(|error| error.contains(name)),
            "{name}: {:?}",
            result.omissions
        );
    }
    let actual = &result.output.partitions["cpp_pointer_records"].rdl;
    let path = test_clang::input_dir().join("cpp_pointer_records.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, actual).unwrap();
    }
    assert_eq!(
        actual,
        &std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    let winmd = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\cpp-pointer-records.winmd");
    windows_rdl::reader()
        .input_text(actual)
        .output(&winmd)
        .write()
        .unwrap();
    let index = windows_metadata::reader::Index::read(&winmd).unwrap();
    for name in [
        "Empty",
        "EmptyStruct",
        "DerivedEmpty",
        "Forward",
        "Hidden",
        "Resource",
    ] {
        let record = index.expect("Fixture", name);
        assert_eq!(record.fields().count(), 0);
        assert!(record.class_layout().is_none());
        assert_eq!(record.methods().count(), 0);
    }
    assert_eq!(index.expect("Fixture", "Point").fields().count(), 2);
    assert_eq!(index.expect("Fixture", "Constructed").fields().count(), 1);
    std::fs::remove_file(winmd).unwrap();
}

#[test]
fn opaque_records_still_check_native_storage() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::selected_fixture("cpp_pointer_records", &["Hidden"]),
        test_clang::selected_fixture("cpp_pointer_records", &["Hidden"]),
    ];
    inputs[1].name = "protected.h".into();
    inputs[1]
        .source
        .insert_str(0, "#define PROTECTED_STORAGE\n");
    for _ in 0..2 {
        let error = capture_partial(&inputs, &TARGETS).unwrap_err().to_string();
        assert!(
            error.contains("field") && error.contains("differ"),
            "{error}"
        );
        inputs.reverse();
    }
}
