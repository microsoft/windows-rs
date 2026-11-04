use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn import_annotation_precedence() {
    helpers::ensure_libclang();
    let input = Input {
        name: "import_library.h".into(),
        source: std::fs::read_to_string(test_clang::input_dir().join("fixtures/import_library.h"))
            .unwrap(),
        ..Default::default()
    };
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    for fallback in [false, true] {
        let mut imports = projection::Imports::default();
        if fallback {
            for snapshot in &snapshots {
                for value in &snapshot.values {
                    for observation in &value.observations {
                        let ValueKind::Function { mangling, .. } = &observation.kind else {
                            panic!()
                        };
                        imports.symbols.entry(snapshot.target).or_default().insert(
                            mangling.clone(),
                            projection::Import {
                                library: "scanned.dll".into(),
                                name: Some("ScannedEntry".into()),
                            },
                        );
                    }
                }
            }
        }
        let output = projection::project_with_imports(&merged, "Fixture", &imports).unwrap();
        assert_eq!(output.functions, 9);
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        let winmd = std::path::Path::new(env!("OUT_DIR")).join(format!("import-{fallback}.winmd"));
        windows_rdl::reader()
            .input_text(&text)
            .output(&winmd)
            .write()
            .unwrap();
        let index = windows_metadata::reader::Index::read(&winmd).unwrap();
        for method in index.expect("Fixture", "Apis").methods() {
            let import = method.impl_map().unwrap();
            assert_eq!(import.import_scope().name(), "actual.dll");
            let entry = match method.name() {
                "Named" => "NativeEntry",
                "Ordinal" | "Char16Ordinal" | "Enum16Ordinal" => "#42",
                "StringOrdinal" | "Char32Ordinal" | "Enum32Ordinal" => "#65535",
                name => {
                    if fallback {
                        "ScannedEntry"
                    } else {
                        name
                    }
                }
            };
            assert_eq!(import.import_name(), entry);
            assert_eq!(
                import
                    .flags()
                    .contains(windows_metadata::PInvokeAttributes::SupportsLastError),
                method.name() == "DllOnly"
            );
            assert_eq!(
                import
                    .flags()
                    .contains(windows_metadata::PInvokeAttributes::CallConvCdecl),
                method.name() == "DefaultConvention"
            );
        }
    }
    let mut imports = projection::Imports::default();
    let x86 = snapshots
        .iter()
        .find(|snapshot| snapshot.target == Target::X86)
        .unwrap();
    let function = x86
        .values
        .iter()
        .find(|value| value.name == "DefaultConvention")
        .unwrap();
    let ValueKind::Function {
        stdcall_candidate: Some(candidate),
        ..
    } = &function.observations[0].kind
    else {
        panic!()
    };
    imports
        .default_stdcall
        .entry(Target::X86)
        .or_default()
        .insert(
            candidate.mangling.clone(),
            projection::Import {
                library: "scanned.dll".into(),
                name: None,
            },
        );
    let output = projection::project_with_imports(&merged, "Fixture", &imports).unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("extern fn DefaultConvention("));
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(output.omissions.is_empty());
    assert_eq!(output.output.functions, 9);
}

#[test]
fn import_annotation_rust() {
    helpers::ensure_libclang();
    let input = Input {
        name: "import_library.h".into(),
        source: std::fs::read_to_string(test_clang::input_dir().join("fixtures/import_library.h"))
            .unwrap(),
        ..Default::default()
    };
    let output = projection::generate(&[input], "Fixture").unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let winmd = std::path::Path::new(env!("OUT_DIR")).join("import-rust.winmd");
    let rust = winmd.with_extension("rs");
    windows_rdl::reader()
        .input_text(&text)
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::Bindgen::new()
        .input(&winmd)
        .output(&rust)
        .filter("Fixture")
        .write();
    let generated = std::fs::read_to_string(&rust).unwrap();
    assert!(generated.contains("\"actual.dll\" \"system\" \"NativeEntry\""));
    assert!(!generated.contains("fn Ordinal("));
    assert!(!generated.contains("fn StringOrdinal("));
}

#[test]
fn import_annotation_function_alias() {
    helpers::ensure_libclang();
    let input = Input {
        name: "import_alias.h".into(),
        source: "extern \"C\" __attribute__((annotate(\"Win32Metadata.ImportLibrary\", \"actual.dll\"))) int Native();\n#define Public Native\n".into(),
        macros: vec!["Public".into()],
        ..Default::default()
    };
    let output = projection::generate(&[input], "Fixture").unwrap();
    assert_eq!(output.functions, 2);
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("#[library(\"actual.dll\", import = \"Native\")]"));
    assert!(text.contains("fn Public("));
}

#[test]
fn variant_order_survives_metadata_roundtrip() {
    helpers::ensure_libclang();
    let snapshots = capture(
        &[test_clang::selected_fixture("variant_order", &["Public"])],
        &TARGETS,
    )
    .unwrap();
    let mut imports = projection::Imports::default();
    for snapshot in &snapshots {
        let value = snapshot
            .values
            .iter()
            .find(|value| value.name == "Public")
            .unwrap();
        for observation in &value.observations {
            let ValueKind::Function { mangling, .. } = &observation.kind else {
                panic!("expected a function");
            };
            imports.symbols.entry(snapshot.target).or_default().insert(
                mangling.clone(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: Some(
                        if snapshot.target == Target::X86 {
                            "Native"
                        } else {
                            "Public"
                        }
                        .into(),
                    ),
                },
            );
        }
    }
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let actual = &output.partitions["variant_order"].rdl;
    let golden = test_clang::input_dir().join("variant_order.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, actual).unwrap();
    }
    assert_eq!(
        *actual,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-variant-order.winmd");
    let rdl = winmd.with_extension("rdl");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(actual)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(*actual, std::fs::read_to_string(&rdl).unwrap());
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(rdl).unwrap();
}

fn imports(snapshots: &[Snapshot]) -> projection::Imports {
    let mut imports = projection::Imports::default();
    for snapshot in snapshots
        .iter()
        .filter(|snapshot| snapshot.target != Target::X86)
    {
        for value in &snapshot.values {
            for observation in &value.observations {
                if let ValueKind::Function {
                    alias: None,
                    mangling,
                    ..
                } = &observation.kind
                {
                    imports.symbols.entry(snapshot.target).or_default().insert(
                        mangling.clone(),
                        projection::Import {
                            library: "fixture.dll".into(),
                            name: Some(value.name.clone()),
                        },
                    );
                }
            }
        }
    }
    imports
}

#[test]
fn unresolved_imports_are_target_local() {
    helpers::ensure_libclang();
    let mut input = test_clang::selected_fixture("target_imports", &["Available"]);
    input.macros.push("PublicAvailable".into());
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    assert!(captured.omissions.is_empty());
    let imports = imports(&captured.snapshots);
    let error =
        projection::project_with_imports(&merge(&captured.snapshots).unwrap(), "Fixture", &imports)
            .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("no configured DLL/export ownership"),
        "{error}"
    );
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(result.output.functions, 2);
    assert_eq!(result.omissions.len(), 2);
    assert!(
        result.omissions.iter().all(
            |line| line.contains("X86:") && line.contains("no configured DLL/export ownership")
        )
    );
    let actual: String = result
        .output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let golden = test_clang::input_dir().join("target_imports.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn all_unresolved_imports_remain_explicit() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("target_imports", &["Available"]);
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    let result = projection::project_partial(captured, "Fixture").unwrap();
    assert_eq!(result.output.functions, 0);
    assert_eq!(result.omissions.len(), 3);
    for target in TARGETS {
        assert!(result.omissions.iter().any(|line| {
            line.contains(&format!("{target:?}:"))
                && line.contains("no configured DLL/export ownership")
        }));
    }
}

#[test]
fn missing_imports_do_not_hide_contract_failures() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("target_imports", &["Unsupported"]);
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    assert!(captured.omissions.is_empty());
    let imports = imports(&captured.snapshots);
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(result.output.functions, 0);
    assert!(
        result
            .omissions
            .iter()
            .any(|line| line.contains("const field")),
        "{:?}",
        result.omissions
    );
}

#[test]
fn invalid_imports_remain_global_failures() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("target_imports", &["Available"]);
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    let mut imports = imports(&captured.snapshots);
    imports
        .symbols
        .get_mut(&Target::X64)
        .unwrap()
        .values_mut()
        .next()
        .unwrap()
        .library
        .clear();
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(result.output.functions, 0);
    assert!(
        result
            .omissions
            .iter()
            .any(|line| line.contains("empty DLL/export ownership"))
    );
}
