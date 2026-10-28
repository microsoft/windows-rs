use windows_clang::*;

#[test]
fn callback_interop_rdl() {
    let output = fixture();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = test_clang::input_dir().join("callback_interop.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn callback_convention_conflicts() {
    helpers::ensure_libclang();
    let imports = [(
        "ConventionFunction".into(),
        projection::Import {
            library: "example.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    for name in [
        "ConventionCallback",
        "ConventionRecord",
        "ConventionFunction",
    ] {
        let mut inputs = [
            test_clang::selected_fixture("callback_convention_conflicts", &[name]),
            test_clang::selected_fixture("callback_convention_conflicts", &[name]),
        ];
        inputs[1].name = "other.h".into();
        inputs[1].source.insert_str(0, "#define OTHER\n");
        for _ in 0..2 {
            let snapshots = capture(&inputs, &[Target::X86, Target::X64, Target::Arm64]).unwrap();
            let merged = merge(&snapshots).unwrap();
            let error = projection::project_with_imports(&merged, "Fixture", &imports).unwrap_err();
            assert!(error.to_string().contains(name), "{error}");
            inputs.reverse();
        }
    }
}

fn fixture() -> projection::Output {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture(
        "callback_interop",
        &[
            "SystemCallbacks",
            "CdeclCallbacks",
            "CallbackSelection",
            "NamedSystem",
            "NamedCdecl",
            "NestedSystem",
            "NestedCdecl",
        ],
    );
    let mut snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &snapshots {
        for (name, expected) in [
            ("SystemCallbacks", CallingConvention::Stdcall),
            ("CdeclCallbacks", CallingConvention::C),
        ] {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == name)
                .unwrap();
            let definition = snapshot.observations[record.observations[0].0]
                .definition
                .as_ref()
                .unwrap();
            for field in definition.fields.iter().take(2) {
                assert_eq!(
                    written_convention(snapshot, field.written),
                    expected,
                    "{:?} {name}.{}",
                    snapshot.target,
                    field.name
                );
            }
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap()
    );
    output
}

fn written_convention(snapshot: &Snapshot, mut id: TypeId) -> CallingConvention {
    loop {
        match &snapshot.types[id.0].kind {
            TypeKind::Alias(alias) => id = snapshot.aliases[alias.0].observations[0].underlying,
            TypeKind::Pointer(inner) => id = *inner,
            TypeKind::Attributed { convention, .. } => return *convention,
            TypeKind::Function(function) => return function.convention,
            other => panic!("{other:?}"),
        }
    }
}
