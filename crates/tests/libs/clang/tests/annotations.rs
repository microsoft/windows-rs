use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn function_metadata_redeclarations() {
    helpers::ensure_libclang();
    let source = include_str!("../input/fixtures/function_metadata_redeclarations.h");
    let mut inputs = ["first.h", "second.h"].map(|name| Input {
        name: name.into(),
        source: source.into(),
        ..Default::default()
    });
    inputs[1].source.insert_str(0, "#define REVERSE\n");
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    for snapshot in &snapshots {
        let observations = &snapshot.values[0].observations;
        assert_eq!(observations.len(), 3);
        for observation in observations {
            assert!(
                observation
                    .attributes
                    .iter()
                    .any(|attribute| { matches!(attribute.kind, AttributeKind::PreserveResult) })
            );
            assert!(
                observation.attributes.iter().any(|attribute| {
                    matches!(attribute.kind, AttributeKind::AssociatedEnum(_))
                })
            );
        }
    }
    let expected = projection::generate(&inputs, "Fixture").unwrap();
    inputs.reverse();
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap(), expected);

    for declarations in [
        "_P _P int F(); int F();",
        "_I(\"first.dll\") int F(); _I(\"second.dll\") int F();",
    ] {
        let input = Input {
            name: "invalid_redeclarations.h".into(),
            source: format!(
                "#define _P __attribute__((annotate(\"Win32Metadata.PreserveResult\")))\n\
                 #define _I(name) __attribute__((annotate(\"Win32Metadata.ImportLibrary\", name)))\n\
                 extern \"C\" {{ {declarations} }}"
            ),
            ..Default::default()
        };
        assert!(
            capture(&[input], &TARGETS)
                .and_then(|snapshots| merge(&snapshots).map(|_| ()))
                .is_err(),
            "{declarations}",
        );
    }
}

#[test]
fn preserve_result_validation() {
    helpers::ensure_libclang();
    for (declaration, message) in [
        ("_P _P int F();", "duplicate PreserveResult"),
        ("typedef _P int Scalar;", "requires a function"),
        ("typedef _P int (**Callback)();", "requires a function"),
        ("struct _P Record {};", "requires a function"),
        ("struct Record { _P int field; };", "requires a function"),
        ("int F(_P int parameter);", "requires a function"),
        (
            "__attribute__((annotate(\"Win32Metadata.PreserveResult\", 1))) int F();",
            "takes no arguments",
        ),
    ] {
        let input = Input {
            name: "preserve_result_invalid.h".into(),
            source: format!(
                "#define _P __attribute__((annotate(\"Win32Metadata.PreserveResult\")))\n{declaration}"
            ),
            ..Default::default()
        };
        let error = capture(std::slice::from_ref(&input), &TARGETS).unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert!(
            partial
                .omissions
                .iter()
                .any(|error| error.contains(message)),
            "{:?}",
            partial.omissions
        );
    }
}

#[test]
fn preserve_result_reconciliation() {
    helpers::ensure_libclang();
    for declaration in [
        "extern \"C\" _P int F();",
        "typedef _P int (*Callback)();",
        "struct Record { _P virtual int __stdcall F() = 0; };",
    ] {
        let mut inputs = ["annotated.h", "plain.h"].map(|name| Input {
            name: name.into(),
            source: format!(
                "#define _P __attribute__((annotate(\"Win32Metadata.PreserveResult\")))\n{}",
                if name == "annotated.h" {
                    declaration.to_string()
                } else {
                    declaration.replace("_P ", "")
                }
            ),
            ..Default::default()
        });
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            assert!(
                merge(&snapshots)
                    .and_then(|merged| projection::project(&merged, "Fixture"))
                    .is_err(),
                "{declaration}"
            );
            assert!(
                projection::generate_partial(&inputs, "Fixture").is_err(),
                "{declaration}"
            );
            inputs.reverse();
        }
    }
}

#[test]
fn import_annotation_validation() {
    helpers::ensure_libclang();
    for arguments in [
        "",
        ", 1",
        ", L\"a.dll\"",
        ", u8\"a.dll\"",
        ", \"a\\0.dll\"",
        ", \"\"",
        ", \"a/b.dll\"",
        ", \"a\\\\b.dll\"",
        ", \"C:example.dll\"",
        ", \"a.dll\", \"\"",
        ", \"a.dll\", -1",
        ", \"a.dll\", 0",
        ", \"a.dll\", 65536",
        ", \"a.dll\", static_cast<char16_t>(0)",
        ", \"a.dll\", static_cast<char32_t>(65536)",
        ", \"a.dll\", true",
        ", \"a.dll\", 1.0",
        ", \"a.dll\", \"#0\"",
        ", \"a.dll\", \"#65536\"",
        ", \"a.dll\", \"#+1\"",
        ", \"a.dll\", \"Entry\\n\"",
        ", \"a.dll\", \"Entry\\0Suffix\"",
        ", \"a.dll\", \"Entry\", 1",
    ] {
        let input = Input {
            name: "invalid_import.h".into(),
            source: format!(
                "extern \"C\" __attribute__((annotate(\"Win32Metadata.ImportLibrary\"{arguments}))) int F();"
            ),
            ..Default::default()
        };
        let error = capture(std::slice::from_ref(&input), &TARGETS).unwrap_err();
        assert!(error.to_string().contains("ImportLibrary"), "{error}");
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        assert!(output.output.partitions.is_empty(), "{arguments}");
        assert!(
            output
                .omissions
                .iter()
                .any(|error| error.contains("ImportLibrary"))
        );
    }
    for declaration in [
        "_I _I int F();",
        "int F(_I int value);",
        "typedef _I int Callback();",
        "struct S { _I virtual int F() = 0; };",
    ] {
        let input = Input {
            name: "invalid_owner.h".into(),
            source: format!(
                "#define _I __attribute__((annotate(\"Win32Metadata.ImportLibrary\", \"a.dll\")))\nextern \"C\" {{ {declaration} }}"
            ),
            ..Default::default()
        };
        assert!(capture(&[input], &TARGETS).is_err(), "{declaration}");
    }
}

#[test]
fn import_annotation_reconciliation() {
    helpers::ensure_libclang();
    let input = |name: &str, annotation: &str| Input {
        name: name.into(),
        source: format!("extern \"C\" {annotation} int F();"),
        ..Default::default()
    };
    let first = "__attribute__((annotate(\"Win32Metadata.ImportLibrary\", \"a.dll\", \"Entry\")))";
    for second in [
        "",
        "__attribute__((annotate(\"Win32Metadata.ImportLibrary\", \"b.dll\", \"Entry\")))",
        "__attribute__((annotate(\"Win32Metadata.ImportLibrary\", \"a.dll\", \"Other\")))",
    ] {
        let mut inputs = [input("a.h", first), input("b.h", second)];
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            assert!(merge(&snapshots).is_err());
            assert!(projection::generate_partial(&inputs, "Fixture").is_err());
            inputs.reverse();
        }
    }
}

#[test]
fn last_error_validation() {
    helpers::ensure_libclang();
    for (source, message) in [
        (
            "extern \"C\" __attribute__((annotate(\"Win32Metadata.SetLastError\", 1))) int F();",
            "takes no arguments",
        ),
        (
            "extern \"C\" __attribute__((annotate(\"Win32Metadata.SetLastError(1)\"))) int F();",
            "Win32Metadata.SetLastError(1)",
        ),
        (
            "extern \"C\" __attribute__((annotate(\"Win32Metadata.SetLastError\"), annotate(\"Win32Metadata.SetLastError\"))) int F();",
            "duplicate SetLastError",
        ),
        (
            "struct __attribute__((annotate(\"Win32Metadata.SetLastError\"))) Record {};",
            "requires a free function",
        ),
        (
            "typedef __attribute__((annotate(\"Win32Metadata.SetLastError\"))) int (*Callback)();",
            "requires a free function",
        ),
        (
            "struct Record { __attribute__((annotate(\"Win32Metadata.SetLastError\"))) virtual int F() = 0; };",
            "requires a free function",
        ),
        (
            "extern \"C\" int F(__attribute__((annotate(\"Win32Metadata.SetLastError\"))) int value);",
            "requires a free function",
        ),
    ] {
        let input = Input {
            name: "last_error.h".into(),
            source: source.into(),
            ..Default::default()
        };
        let error = capture(std::slice::from_ref(&input), &TARGETS)
            .and_then(|snapshots| merge(&snapshots).map(|_| ()))
            .unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert!(partial.output.partitions.is_empty());
        assert!(
            partial
                .omissions
                .iter()
                .any(|error| error.contains(message)),
            "{:?}",
            partial.omissions
        );
    }
}

#[test]
fn last_error_reconciliation() {
    helpers::ensure_libclang();
    let source =
        std::fs::read_to_string(test_clang::input_dir().join("fixtures/last_error.h")).unwrap();
    let mut inputs = [
        Input {
            name: "annotated.h".into(),
            source: source.clone(),
            ..Default::default()
        },
        Input {
            name: "unannotated.h".into(),
            source: source.replace("_Win32_SetLastError_ int", "int"),
            ..Default::default()
        },
    ];
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        assert!(merge(&snapshots).is_err());
        assert!(projection::generate_partial(&inputs, "Fixture").is_err());
        inputs.reverse();
    }
}

#[test]
fn last_error_metadata_and_rust() {
    helpers::ensure_libclang();
    let source =
        std::fs::read_to_string(test_clang::input_dir().join("fixtures/last_error.h")).unwrap();
    let imports = ["CaptureError", "Plain"]
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
    let mut bindings = None;
    for enabled in [false, true] {
        let input = Input {
            name: "last_error.h".into(),
            source: if enabled {
                source.clone()
            } else {
                source.replace("_Win32_SetLastError_ int", "int")
            },
            ..Default::default()
        };
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let output =
            projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports)
                .unwrap();
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        let winmd =
            std::path::Path::new(env!("OUT_DIR")).join(format!("last-error-{enabled}.winmd"));
        let rust = winmd.with_extension("rs");
        windows_rdl::reader()
            .input_text(&text)
            .output(&winmd)
            .write()
            .unwrap();
        let index = windows_metadata::reader::Index::read(&winmd).unwrap();
        for method in index.expect("Fixture", "Apis").methods() {
            assert_eq!(
                method
                    .impl_map()
                    .unwrap()
                    .flags()
                    .contains(windows_metadata::PInvokeAttributes::SupportsLastError),
                enabled && method.name() == "CaptureError",
            );
        }
        windows_bindgen::Bindgen::new()
            .input(&winmd)
            .output(&rust)
            .filter("Fixture")
            .write();
        let generated = std::fs::read_to_string(&rust).unwrap();
        if let Some(bindings) = &bindings {
            assert_eq!(&generated, bindings);
        } else {
            bindings = Some(generated);
        }
    }
}

#[test]
fn alias_annotation_reconciliation() {
    helpers::ensure_libclang();
    let input = |contract| {
        let mut input = test_clang::fixture("annotation_merge");
        input.name = format!("contract{contract}.h");
        input
            .source
            .insert_str(0, &format!("#define CONTRACT {contract}\n"));
        input.roots = vec!["Status".into()];
        input
    };
    let mut inputs: Vec<_> = [0, 1, 2, 3].into_iter().map(input).collect();
    let mut expected = None;
    for _ in 0..inputs.len() {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        if let Some(expected) = &expected {
            assert_eq!(&output, expected);
        } else {
            expected = Some(output);
        }
        inputs.rotate_left(1);
    }
    for contracts in [[0, 1, 2], [0, 1, 4]] {
        let mut inputs = contracts.map(input);
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            let error = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap_err();
            assert!(
                error.to_string().contains("no observed SAL annotation set"),
                "{error}"
            );
            assert!(
                projection::project_partial(capture_partial(&inputs, &TARGETS).unwrap(), "Fixture")
                    .is_err()
            );
            inputs.reverse();
        }
    }
}

#[test]
fn expanded_annotation_owner_is_not_discarded() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("annotation_merge");
    input.arguments.push("-DCONTRACT=0".into());
    input.roots = vec!["UsesVoid".into()];
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("value: *mut AnnotatedVoid,"));

    input.roots = vec!["UsesConstVoid".into()];
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    assert!(projection::project(&merge(&snapshots).unwrap(), "Fixture").is_err());
    let result =
        projection::project_partial(capture_partial(&[input], &TARGETS).unwrap(), "Fixture")
            .unwrap();
    assert!(result.output.partitions.is_empty());
    assert!(
        result
            .omissions
            .iter()
            .any(|value| value.contains("AnnotatedConstVoid"))
    );
    assert!(
        result
            .omissions
            .iter()
            .any(|value| value.contains("UsesConstVoid"))
    );
}

#[test]
fn invalid_annotation_transport() {
    helpers::ensure_libclang();
    for payload in [
        "windows_clang:sal:3:_In_\n()\n()",
        "windows_clang:sal:2:1Invalid\n()\n()",
        "windows_clang:sal:2:_In_\n()\n()\nextra",
        "windows_clang:sal:2:_In_\n()\nmissing-parentheses",
    ] {
        let mut input = test_clang::fixture("annotation_transport");
        input
            .source
            .insert_str(0, &format!("#define PAYLOAD {payload:?}\n"));
        let error = capture(&[input], &[Target::X64]).unwrap_err().to_string();
        assert!(
            error.contains("invalid SAL annotation transport"),
            "{error}"
        );
    }

    let mut input = test_clang::fixture("annotation_transport");
    input
        .source
        .insert_str(0, "#define PAYLOAD \"unrecognized annotation\"\n");
    let snapshots = capture(&[input], &[Target::X64]).unwrap();
    assert!(merge(&snapshots).is_err());
}

#[test]
fn nested_annotation_transport() {
    helpers::ensure_libclang();
    let nested = |arguments: &str| {
        let payload = format!("windows_clang:sal:2:_In_reads_\n({arguments})\n(16)");
        format!("__attribute__((annotate({payload:?})))")
    };
    let native = nested("LIMIT");
    let input = |expanded: &str| {
        let payload =
            format!("windows_clang:sal:2:_When_\n(n > 0,_In_reads_(LIMIT))\n({expanded})");
        let mut input = test_clang::fixture("annotation_transport");
        input
            .source
            .insert_str(0, &format!("#define PAYLOAD {payload:?}\n"));
        input
    };
    let ordinary_literal = format!("{native:?}");
    let raw_literal = format!("u8R\"scope({native})scope\"");
    let expanded = format!("1'000, {ordinary_literal}, {raw_literal}, {native}");
    let expected = format!("1'000, {ordinary_literal}, {raw_literal}, _In_reads_(16)");
    let snapshots = capture(&[input(&expanded)], &TARGETS).unwrap();
    assert!(format!("{snapshots:?}").contains(&format!("{expected:?}")));

    for malformed in [
        native.replace("sal:2:", "sal:3:"),
        native.trim_end_matches(')').into(),
        native.replace("\\n", "\\t"),
    ] {
        let input = input(&malformed);
        let error = capture(std::slice::from_ref(&input), &TARGETS)
            .unwrap_err()
            .to_string();
        assert!(error.contains("SAL"), "{error}");
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.records, 0);
        assert!(partial.omissions.iter().any(|error| error.contains("SAL")));
    }
    let mut inputs = [input(&native), input(&nested("OTHER_LIMIT"))];
    inputs[1].name = "other.h".into();
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        inputs.reverse();
    }
}

#[test]
fn callback_contract_boundaries() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::selected_fixture("callback_contracts", &["Reader"]),
        test_clang::selected_fixture("callback_contracts", &["Reader"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1]
        .source
        .insert_str(0, "#define DIRECTION \"_Out_\"\n");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        let error = projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("conflicting public names or contracts"),
            "{error}"
        );
        assert!(projection::generate_partial(&inputs, "Fixture").is_err());
        inputs.reverse();
    }
    let input = test_clang::selected_fixture("callback_contracts", &["UsesNested"]);
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(output.output.partitions.is_empty());
    assert!(
        output
            .omissions
            .iter()
            .any(|value| value.contains("nested callback contract"))
    );
    assert!(
        output
            .omissions
            .iter()
            .any(|value| value.contains("UsesNested"))
    );
}
