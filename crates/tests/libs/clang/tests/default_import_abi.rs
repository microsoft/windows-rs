use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn imports(name: &str, symbol: &str) -> projection::Imports {
    let import = projection::Import {
        library: "fixture.dll".into(),
        name: None,
    };
    let mut result = projection::Imports::default();
    for target in [Target::X64, Target::Arm64] {
        result
            .symbols
            .entry(target)
            .or_default()
            .insert(name.into(), import.clone());
    }
    result
        .default_stdcall
        .entry(Target::X86)
        .or_default()
        .insert(symbol.into(), import);
    result
}

#[test]
fn checked_import_convention() {
    helpers::ensure_libclang();
    let snapshots = capture(
        &[selected_fixture("default_import_abi", &["FetchImport"])],
        &TARGETS,
    )
    .unwrap();
    for snapshot in &snapshots {
        let observation = &snapshot.values[0].observations[0];
        let ValueKind::Function {
            convention_origin,
            stdcall_candidate,
            ..
        } = &observation.kind
        else {
            panic!()
        };
        assert_eq!(*convention_origin, CallingConventionOrigin::Default);
        let TypeKind::Function(original) = &snapshot.types[observation.canonical.0].kind else {
            panic!()
        };
        assert_eq!(original.convention, CallingConvention::C);
        assert_eq!(stdcall_candidate.is_some(), snapshot.target == Target::X86);
        if let Some(candidate) = stdcall_candidate {
            assert_eq!(candidate.mangling, "_FetchImport@8");
            let TypeKind::Function(candidate) = &snapshot.types[candidate.canonical.0].kind else {
                panic!()
            };
            assert_eq!(candidate.convention, CallingConvention::Stdcall);
            assert_eq!(candidate.parameters, original.parameters);
            assert_eq!(candidate.result, original.result);
        }
    }
    let output = projection::project_with_imports(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &imports("FetchImport", "_FetchImport@8"),
    )
    .unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let path = input_dir().join("default_import_abi.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    let partial = projection::project_partial_with_imports(
        capture_partial(
            &[selected_fixture("default_import_abi", &["FetchImport"])],
            &TARGETS,
        )
        .unwrap(),
        "Fixture",
        &imports("FetchImport", "_FetchImport@8"),
    )
    .unwrap();
    assert!(partial.omissions.is_empty());
    assert_eq!(
        text,
        partial
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect::<String>()
    );
    assert!(
        projection::project_with_imports(
            &merge(&snapshots).unwrap(),
            "Fixture",
            &imports("FetchImport", "_FetchImport@12")
        )
        .unwrap_err()
        .to_string()
        .contains("no configured DLL/export ownership")
    );
}

#[test]
fn explicit_convention_is_not_replaced() {
    helpers::ensure_libclang();
    let input = selected_fixture("default_import_abi", &["FixedImport"]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let ValueKind::Function {
            convention_origin,
            stdcall_candidate,
            ..
        } = &snapshot.values[0].observations[0].kind
        else {
            panic!()
        };
        assert_eq!(*convention_origin, CallingConventionOrigin::Explicit);
        assert!(stdcall_candidate.is_none());
    }
    let error = projection::project_with_imports(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &imports("FixedImport", "_FixedImport@4"),
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("no configured DLL/export ownership"),
        "{error}"
    );
}

#[test]
fn every_observation_must_allow_resolution() {
    helpers::ensure_libclang();
    let reject = |inputs: &[Input]| {
        let captured = capture_partial(inputs, &TARGETS).unwrap();
        let output = projection::project_partial_with_imports(
            captured,
            "Fixture",
            &imports("FetchImport", "_FetchImport@8"),
        )
        .unwrap();
        assert_eq!(output.output.functions, 1);
        assert!(output.omissions.iter().all(|line| line.contains("X86:")));
        assert!(
            output
                .omissions
                .iter()
                .any(|line| line.contains("no configured DLL/export ownership"))
        );
        let text: String = output
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(text.contains("extern \"C\" fn FetchImport("));
        assert!(!text.contains("extern fn FetchImport("));
    };
    let mut inputs = [
        selected_fixture("default_import_abi", &["FetchImport"]),
        selected_fixture("default_import_abi", &["FetchImport"]),
    ];
    inputs[0].source = "#include \"default_import_abi.h\"".into();
    inputs[1].name = "explicit.h".into();
    inputs[1].source =
        "#define IMPORT_CONVENTION __cdecl\n#include \"default_import_abi.h\"".into();
    for _ in 0..2 {
        reject(&inputs);
        inputs.reverse();
    }
    for source in [
        "#include \"default_import_abi.h\"\n\
         extern \"C\" int __cdecl FetchImport(ImportRequest* request, ImportCallback callback);",
        "#define IMPORT_CONVENTION __cdecl\n#include \"default_import_abi.h\"\n\
         extern \"C\" int FetchImport(ImportRequest* request, ImportCallback callback);",
    ] {
        let mut input = selected_fixture("default_import_abi", &["FetchImport"]);
        input.source = source.into();
        reject(&[input]);
    }
    inputs[1].source =
        "#define IMPORT_CONVENTION __stdcall\n#include \"default_import_abi.h\"".into();
    assert!(capture_partial(&inputs, &TARGETS).is_err());
}

#[test]
fn native_and_explicit_imports_take_precedence() {
    helpers::ensure_libclang();
    let snapshots = capture(
        &[selected_fixture("default_import_abi", &["FetchImport"])],
        &TARGETS,
    )
    .unwrap();
    let resolution = merge(&snapshots).unwrap();
    let mut imports = imports("FetchImport", "_FetchImport@8");
    imports.symbols.entry(Target::X86).or_default().insert(
        "_FetchImport".into(),
        projection::Import {
            library: "native.dll".into(),
            name: None,
        },
    );
    for expected in ["native.dll", "override.dll"] {
        if expected == "override.dll" {
            imports.functions.insert(
                "FetchImport".into(),
                projection::Import {
                    library: expected.into(),
                    name: None,
                },
            );
        }
        let output = projection::project_with_imports(&resolution, "Fixture", &imports).unwrap();
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(text.contains(&format!("#[library(\"{expected}\")]")));
        assert!(text.contains("extern \"C\" fn FetchImport("));
    }
}

#[test]
fn canonical_publication_preserves_other_contracts() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("canonical_abi", &[]),
        selected_fixture("canonical_abi", &["Observed", "ObservedCallback"]),
    ];
    inputs[0].name = "first.h".into();
    inputs[0].source = "#include \"canonical_abi.h\"".into();
    inputs[1].name = "second.h".into();
    inputs[1].source = "#define EXPLICIT_WIDE\n#include \"canonical_abi.h\"".into();
    let mut previous = None;
    for _ in 0..2 {
        let mut snapshots = capture(&inputs, &TARGETS).unwrap();
        let mut imports = projection::Imports::default();
        for snapshot in &snapshots {
            for value in &snapshot.values {
                for observation in &value.observations {
                    let ValueKind::Function { mangling, .. } = &observation.kind else {
                        panic!()
                    };
                    imports.symbols.entry(snapshot.target).or_default().insert(
                        mangling.clone(),
                        projection::Import {
                            library: if value.name == "LibraryVariant"
                                && snapshot.target == Target::X86
                            {
                                "narrow.dll"
                            } else {
                                "fixture.dll"
                            }
                            .into(),
                            name: (value.name == "ExportVariant" && snapshot.target == Target::X86)
                                .then(|| "#17".into()),
                        },
                    );
                }
            }
        }
        for _ in 0..2 {
            let output =
                projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports)
                    .unwrap();
            if let Some(previous) = &previous {
                assert_eq!(&output, previous);
            } else {
                let text = &output.partitions["canonical_abi"].rdl;
                let path = input_dir().join("canonical_abi.rdl");
                if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
                    std::fs::write(&path, text).unwrap();
                }
                assert_eq!(
                    text,
                    &std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
                );
                let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
                let winmd = root.join("target\\test-clang-canonical-abi.winmd");
                let rdl = winmd.with_extension("rdl");
                windows_rdl::reader()
                    .input(root.join("metadata\\metadata.rdl"))
                    .input_text(text)
                    .output(&winmd)
                    .write()
                    .unwrap();
                windows_rdl::writer()
                    .input(&winmd)
                    .filter("Fixture")
                    .output(&rdl)
                    .write()
                    .unwrap();
                assert_eq!(text, &std::fs::read_to_string(&rdl).unwrap());
                std::fs::remove_file(winmd).unwrap();
                std::fs::remove_file(rdl).unwrap();
            }
            previous = Some(output);
            snapshots.reverse();
        }
        inputs.reverse();
    }
}
