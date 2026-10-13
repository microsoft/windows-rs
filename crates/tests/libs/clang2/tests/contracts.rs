use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use windows_clang2::*;
use windows_metadata::{HasAttributes, ParamAttributes, Value, reader::*};

#[cfg(target_arch = "x86_64")]
mod support;

#[cfg(target_arch = "x86_64")]
fn native_compiler(test: &str) -> Option<Compiler> {
    if support::worker(test) {
        return None;
    }
    Some(
        Compiler::with_layout_bridge(
            Path::new(&std::env::var_os("WINDOWS_CLANG2_LAYOUT_NATIVE").unwrap())
                .join("windows_clang_layout.dll"),
        )
        .unwrap(),
    )
}

fn input(source: &str, target: Target, names: &[&str]) -> Input {
    let mut input = Input::new("contracts.cpp", source, target);
    input
        .required
        .extend(names.iter().map(|name| name.to_string()));
    let sdk = std::env::var_os("WINDOWS_CLANG2_SDK_INCLUDE").map_or_else(
        || PathBuf::from(std::env::var_os("USERPROFILE").unwrap())
            .join(".nuget\\packages\\microsoft.windows.sdk.cpp\\10.0.28000.2270\\c\\Include\\10.0.28000.0"),
        PathBuf::from,
    );
    assert!(
        sdk.join("shared\\sal.h").is_file(),
        "missing pinned SDK: {}",
        sdk.display()
    );
    input
        .arguments
        .push(format!("-I{}", sdk.join("shared").display()));
    input
}

fn imports(snapshot: &Snapshot) -> BTreeMap<String, Import> {
    snapshot
        .functions()
        .map(|(decl, function)| {
            (
                function.linker_name.clone(),
                Import {
                    dll: "fixture.dll".into(),
                    export: decl.qualified_name.clone(),
                },
            )
        })
        .collect()
}

#[test]
fn original_sdk_annotations_preserve_positional_contracts_in_winmd() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(
                include_str!("..\\input\\source_contracts.h"),
                target,
                &["Read", "Write", "Query", "Unnamed", "Unknown"],
            ))
            .unwrap();
        let rdl = snapshot
            .rdl_with_imports("Test", &imports(&snapshot))
            .unwrap();
        assert_eq!(
            rdl,
            include_str!("..\\input\\source_contracts.rdl").replace("\r\n", "\n")
        );
        let directory = Path::new(env!("OUT_DIR")).join(format!("contracts_{target:?}"));
        std::fs::create_dir_all(&directory).unwrap();
        let vocabulary = directory.join("vocabulary.winmd");
        windows_rdl::reader()
            .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
            .output(&vocabulary)
            .write()
            .unwrap();
        let binary = directory.join("contracts.winmd");
        windows_rdl::reader()
            .reference(&vocabulary)
            .input_text(&rdl)
            .output(&binary)
            .write()
            .unwrap();
        let text = directory.join("roundtrip.rdl");
        windows_rdl::writer()
            .input(&binary)
            .output(&text)
            .write()
            .unwrap();
        let roundtrip = directory.join("roundtrip.winmd");
        windows_rdl::reader()
            .reference(&vocabulary)
            .input(&text)
            .output(&roundtrip)
            .write()
            .unwrap();
        for path in [&binary, &roundtrip] {
            let index = Index::read(path).unwrap();
            index.validate_native_contracts(None).unwrap();
            let method = |name| {
                let Item::Fn(method) = index.expect_item("Test", name) else {
                    panic!("expected function");
                };
                method
            };
            for (name, expected) in [
                (
                    "Read",
                    &[
                        ParamDirection::Output,
                        ParamDirection::Input,
                        ParamDirection::Output,
                    ][..],
                ),
                (
                    "Write",
                    &[
                        ParamDirection::Input,
                        ParamDirection::Input,
                        ParamDirection::Output,
                    ][..],
                ),
                (
                    "Query",
                    &[ParamDirection::Input, ParamDirection::Output][..],
                ),
                (
                    "Unnamed",
                    &[ParamDirection::Input, ParamDirection::Input][..],
                ),
                ("Unknown", &[ParamDirection::Unspecified][..]),
            ] {
                let parameters = method(name).params_by_sequence(expected.len()).unwrap();
                assert_eq!(
                    parameters
                        .params()
                        .iter()
                        .map(|param| param.unwrap().direction())
                        .collect::<Vec<_>>(),
                    expected
                );
            }
            let read = method("Read").params_by_sequence(3).unwrap();
            assert!(
                read.params()[2]
                    .unwrap()
                    .flags()
                    .contains(ParamAttributes::Optional)
            );
            let extents = read.params()[0]
                .unwrap()
                .attributes()
                .filter(|attribute| attribute.name() == "NativeBufferExtentAttribute")
                .map(|attribute| {
                    attribute
                        .value()
                        .into_iter()
                        .map(|(_, value)| value)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            assert_eq!(
                extents,
                [
                    vec![
                        Value::Utf8("capacity".into()),
                        Value::Utf8("bytes".into()),
                        Value::I32(1),
                        Value::Bool(false)
                    ],
                    vec![
                        Value::Utf8("valid".into()),
                        Value::Utf8("bytes".into()),
                        Value::I32(2),
                        Value::Bool(true)
                    ],
                ]
            );
            let bindings = read.params()[0]
                .unwrap()
                .attributes()
                .filter(|attribute| attribute.name() == "NativeAnnotationBindingAttribute")
                .map(|attribute| {
                    attribute
                        .value()
                        .into_iter()
                        .map(|(_, value)| value)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            assert_eq!(
                bindings,
                [
                    vec![
                        Value::Utf8("SAL".into()),
                        Value::I32(0),
                        Value::Utf8("capacity".into()),
                        Value::I32(1)
                    ],
                    vec![
                        Value::Utf8("SAL".into()),
                        Value::I32(0),
                        Value::Utf8("written".into()),
                        Value::I32(2)
                    ],
                ]
            );
            let query = method("Query").params_by_sequence(2).unwrap();
            assert!(
                query.params()[1]
                    .unwrap()
                    .attributes()
                    .any(|attribute| attribute.name() == "ComOutPtrAttribute")
            );
        }
    }
}

#[test]
fn conflicting_or_unsupported_consumed_contracts_reject() {
    let compiler = Compiler::new().unwrap();
    for declaration in [
        "void Rejected(/* [out] */ _In_ int* value);",
        "void Rejected(_In_ _Out_ int* value);",
        "void Rejected(/* [size_is(length)] */ int* value, int length);",
        "void Rejected(_In_reads_bytes_(length + 1) void* value, unsigned length);",
        "void Rejected(_In_reads_bytes_(missing) void* value);",
        "void Rejected(_In_reads_bytes_(length) void* value, double length);",
        "void Rejected(_Out_writes_bytes_to_(length, *written) void* value, unsigned length, unsigned** written);",
        "void Rejected(_In_reads_bytes_(length) int value, unsigned length);",
        "void Rejected(_In_z_ const char* value);",
        "#define INPUT _In_\nvoid Rejected(INPUT int* value);",
        "#define INPUT NEXT\n#define NEXT _In_\nvoid Rejected(INPUT int* value);",
    ] {
        let source = format!("#include <sal.h>\n{declaration}");
        let error = compiler
            .capture(&input(&source, Target::X64, &["Rejected"]))
            .unwrap_err();
        assert_eq!(error.stage, Stage::Capture, "{declaration}: {error}");
    }
}

#[test]
fn redeclarations_compare_contracts_and_not_parameter_spellings() {
    let compiler = Compiler::new().unwrap();
    let source = "#include <sal.h>
        void Write(_In_reads_bytes_(count) const void* data, _In_ unsigned count);
        void Write(_In_reads_bytes_(length) const void* buffer, _In_ unsigned length);";
    let snapshot = compiler
        .capture(&input(source, Target::X64, &["Write"]))
        .unwrap();
    let contracts = snapshot
        .parameters()
        .map(|(_, parameter)| &parameter.contract)
        .collect::<Vec<_>>();
    assert_eq!(contracts[0].extents, contracts[2].extents);
    let mut bindings = [
        contracts[0].annotations[0].bindings.clone(),
        contracts[2].annotations[0].bindings.clone(),
    ];
    bindings.sort();
    assert_eq!(
        bindings,
        [vec![("count".into(), 1)], vec![("length".into(), 1)]]
    );
    let source = "#include <sal.h>
        void Read(_In_ int* data);
        void Read(_Out_ int* data);";
    assert!(
        compiler
            .capture(&input(source, Target::X64, &["Read"]))
            .is_err()
    );
}

#[test]
#[cfg(target_arch = "x86_64")]
fn explicit_source_contexts_match_reviewed_owners_and_winmd() {
    let Some(compiler) =
        native_compiler("explicit_source_contexts_match_reviewed_owners_and_winmd")
    else {
        return;
    };
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(
                include_str!("..\\input\\source_ownership.h"),
                target,
                &["Root", "Other", "Status", "Plain"],
            ))
            .unwrap();
        let rdl = snapshot
            .rdl_with_imports("Test", &imports(&snapshot))
            .unwrap();
        assert_eq!(
            rdl,
            include_str!("..\\input\\source_ownership.rdl").replace("\r\n", "\n")
        );
        let directory = Path::new(env!("OUT_DIR")).join(format!("ownership_{target:?}"));
        std::fs::create_dir_all(&directory).unwrap();
        let vocabulary = directory.join("vocabulary.winmd");
        windows_rdl::reader()
            .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
            .output(&vocabulary)
            .write()
            .unwrap();
        let binary = directory.join("ownership.winmd");
        windows_rdl::reader()
            .reference(&vocabulary)
            .input_text(&rdl)
            .output(&binary)
            .write()
            .unwrap();
        let index = Index::read(&binary).unwrap();
        index.validate_native_contracts(None).unwrap();
        for (name, payload) in [
            ("Root", "/* interface Root */\n/* [object][local] */"),
            ("Other", "/* interface Other */\n/* [object] */"),
        ] {
            let owner = index.expect("Test", name);
            assert_eq!(
                annotation_values(owner.attributes()),
                [vec![
                    Value::Utf8("MIDL-section".into()),
                    Value::I32(0),
                    Value::Utf8(payload.into()),
                ]]
            );
            assert!(
                owner
                    .methods()
                    .all(|method| annotation_values(method.attributes()).is_empty())
            );
        }
        let Item::Fn(status) = index.expect_item("Test", "Status") else {
            panic!();
        };
        assert_eq!(
            annotation_values(status.attributes()),
            [vec![
                Value::Utf8("SAL-return".into()),
                Value::I32(0),
                Value::Utf8("_Check_return_".into()),
            ]]
        );
        let Item::Fn(plain) = index.expect_item("Test", "Plain") else {
            panic!();
        };
        assert!(annotation_values(plain.attributes()).is_empty());
    }
}

#[cfg(target_arch = "x86_64")]
fn annotation_values<'a>(attributes: impl Iterator<Item = Attribute<'a>>) -> Vec<Vec<Value>> {
    attributes
        .filter(|attribute| attribute.name() == "NativeAnnotationAttribute")
        .map(|attribute| {
            attribute
                .value()
                .into_iter()
                .map(|(_, value)| value)
                .collect()
        })
        .collect()
}

#[test]
#[cfg(target_arch = "x86_64")]
fn wrappers_follow_definitions_active_at_each_original_invocation() {
    let Some(compiler) =
        native_compiler("wrappers_follow_definitions_active_at_each_original_invocation")
    else {
        return;
    };
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let source = include_str!("..\\input\\macro_ownership.h");
        let snapshot = compiler
            .capture(&input(
                source,
                target,
                &["Before", "After", "Fill", "Plain"],
            ))
            .unwrap();
        for (name, direction, leaf, definition_line) in [
            ("Before", Direction::In, "_In_", 4),
            ("After", Direction::Out, "_Out_", 7),
        ] {
            let parameter = snapshot
                .parameters()
                .find(|(owner, _)| owner.qualified_name == name)
                .unwrap()
                .1;
            assert_eq!(parameter.contract.direction, Some(direction));
            assert_eq!(parameter.ordinal, 0);
            let annotation = &parameter.contract.annotations[0];
            assert_eq!(annotation.payload, "INPUT");
            assert!(annotation.bindings.is_empty());
            let resolution = annotation.resolution.as_ref().unwrap();
            assert_eq!(resolution.payload, leaf);
            assert_eq!(
                resolution
                    .definitions
                    .iter()
                    .map(|definition| definition.name.as_str())
                    .collect::<Vec<_>>(),
                ["INPUT", "LEAF", leaf]
            );
            assert_eq!(resolution.definitions[0].tokens, ["LEAF"]);
            assert_eq!(resolution.definitions[0].location.line, 3);
            assert_eq!(resolution.definitions[1].tokens, [leaf]);
            assert_eq!(resolution.definitions[1].location.line, definition_line);
            assert!(
                resolution
                    .definitions
                    .iter()
                    .all(|definition| !definition.function_like && !definition.variadic)
            );
        }
        let fill = snapshot
            .parameters()
            .find(|(owner, parameter)| owner.qualified_name == "Fill" && parameter.ordinal == 0)
            .unwrap()
            .1;
        let annotation = &fill.contract.annotations[0];
        assert_eq!(annotation.payload, "BUFFER ( capacity , * written )");
        assert_eq!(
            annotation.bindings,
            [("capacity".into(), 1), ("written".into(), 2)]
        );
        let resolution = annotation.resolution.as_ref().unwrap();
        assert_eq!(
            resolution.payload,
            "_Out_writes_bytes_to_ ( capacity , * written )"
        );
        assert_eq!(
            resolution
                .definitions
                .iter()
                .map(|definition| definition.name.as_str())
                .collect::<Vec<_>>(),
            ["BUFFER", "_Out_writes_bytes_to_"]
        );
        assert!(!resolution.definitions[0].function_like);
        assert!(resolution.definitions[1].function_like);
        assert_eq!(resolution.definitions[1].parameters, ["size", "count"]);
        assert_eq!(
            fill.contract.extents,
            [
                BufferExtent {
                    phase: "capacity".into(),
                    units: "bytes".into(),
                    parameter: 1,
                    dereference: false
                },
                BufferExtent {
                    phase: "valid".into(),
                    units: "bytes".into(),
                    parameter: 2,
                    dereference: true
                },
            ]
        );
        let plain = snapshot
            .parameters()
            .find(|(owner, _)| owner.qualified_name == "Plain")
            .unwrap()
            .1;
        assert!(plain.contract.annotations.is_empty());
        let directory = Path::new(env!("OUT_DIR")).join(format!("macro_ownership_{target:?}"));
        std::fs::create_dir_all(&directory).unwrap();
        let vocabulary = directory.join("vocabulary.winmd");
        windows_rdl::reader()
            .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
            .output(&vocabulary)
            .write()
            .unwrap();
        let binary = directory.join("ownership.winmd");
        let rdl = snapshot
            .rdl_with_imports("Test", &imports(&snapshot))
            .unwrap();
        windows_rdl::reader()
            .reference(&vocabulary)
            .input_text(&rdl)
            .output(&binary)
            .write()
            .unwrap();
        let index = Index::read(&binary).unwrap();
        index.validate_native_contracts(None).unwrap();
        let Item::Fn(method) = index.expect_item("Test", "Fill") else {
            panic!();
        };
        let parameters = method.params_by_sequence(3).unwrap();
        let values = annotation_values(parameters.params()[0].unwrap().attributes());
        assert_eq!(
            values[0],
            [
                Value::Utf8("SAL".into()),
                Value::I32(0),
                Value::Utf8(annotation.payload.clone())
            ]
        );
        assert_eq!(
            values[1],
            [
                Value::Utf8("SAL-expansion".into()),
                Value::I32(0),
                Value::Utf8(resolution.payload.clone())
            ]
        );
        assert_eq!(values.len(), 4);
        assert_eq!(values[2][0], Value::Utf8("SAL-definition-0".into()));
        assert_eq!(values[3][1], Value::I32(1));
    }
}

#[test]
#[cfg(target_arch = "x86_64")]
fn consumed_source_context_ambiguity_and_transformations_reject() {
    let Some(compiler) =
        native_compiler("consumed_source_context_ambiguity_and_transformations_reject")
    else {
        return;
    };
    let proof = "struct __declspec(uuid(\"00112233-4455-6677-8899-aabbccddeeff\")) Proof { virtual void __stdcall Method() = 0; };";
    for (source, required, diagnostic) in [
        (
            format!("/* interface Wrong */\n/* [object] */\n{proof}"),
            "Proof",
            "mismatched",
        ),
        (
            format!(
                "/* interface Proof */\n/* [object] */\nnamespace a {{ {proof} }} namespace b {{ {proof} }}"
            ),
            "a::Proof",
            "one compiler interface",
        ),
        (
            format!(
                "/* interface Proof */\n/* [object] */\nnamespace a {{ {proof} }}\n/* interface Proof */\n/* [local] */\nnamespace b {{ {proof} }}"
            ),
            "a::Proof",
            "duplicate",
        ),
        (
            format!("/* interface Proof */\n/* [object */\n{proof}"),
            "Proof",
            "malformed",
        ),
        (format!("/* [object */\n{proof}"), "Proof", "malformed"),
        (
            "#include <sal.h>\n#define INPUT(x) _In_ x\nvoid Proof(INPUT(int*) value);".into(),
            "Proof",
            "transformation",
        ),
        (
            "#include <sal.h>\n#define INPUT _In_ _Out_\nvoid Proof(INPUT int* value);".into(),
            "Proof",
            "transformation",
        ),
        (
            "#include <sal.h>\n_Out_ int Proof();".into(),
            "Proof",
            "return SAL",
        ),
    ] {
        let error = compiler
            .capture(&input(&source, Target::X64, &[required]))
            .unwrap_err();
        assert_eq!(error.stage, Stage::Capture, "{source}: {error}");
        assert!(error.message.contains(diagnostic), "{source}: {error}");
    }
}

#[test]
#[cfg(target_arch = "x86_64")]
fn context_prefixes_do_not_cross_declarations_files_or_inactive_branches() {
    let Some(compiler) =
        native_compiler("context_prefixes_do_not_cross_declarations_files_or_inactive_branches")
    else {
        return;
    };
    for source in [
        "#include <sal.h>\n_Check_return_ int Previous();\nvoid Proof();",
        "#include <sal.h>\n#if 0\n_Check_return_\n#endif\nvoid Proof();",
        "#include <sal.h>\n#if _SAL_VERSION\nvoid Proof();\n#endif",
        "#include <sal.h>\n#if \\\n_SAL_VERSION\nvoid Proof();\n#endif",
        "#include <sal.h>\n#define INPUT _In_\n#undef INPUT\n#define INPUT\nvoid Proof(INPUT int* value);",
        "#include <sal.h>\n#define INPUT NEXT\n#define NEXT INPUT\nvoid Proof(INPUT* value);",
    ] {
        // The cyclic macro leaves an unknown type and must fail as a source error.
        if source.contains("#define NEXT INPUT") {
            assert_eq!(
                compiler
                    .capture(&input(source, Target::X64, &["Proof"]))
                    .unwrap_err()
                    .stage,
                Stage::Source
            );
            continue;
        }
        let snapshot = compiler
            .capture(&input(source, Target::X64, &["Proof"]))
            .unwrap();
        assert!(
            snapshot
                .functions()
                .all(|(_, function)| function.annotations.is_empty())
        );
        assert!(
            snapshot
                .parameters()
                .all(|(_, parameter)| parameter.contract.annotations.is_empty())
        );
    }
    let directory = Path::new(env!("OUT_DIR")).join("ownership_cross_file");
    std::fs::create_dir_all(&directory).unwrap();
    let prefix = directory.join("prefix.h");
    std::fs::write(&prefix, "/* [object][local] */\n").unwrap();
    let source = format!(
        "#include {:?}\nstruct __declspec(uuid(\"00112233-4455-6677-8899-aabbccddeeff\")) Proof {{ virtual void __stdcall Method() = 0; }};",
        prefix.to_string_lossy().replace('\\', "/")
    );
    let snapshot = compiler
        .capture(&input(&source, Target::X64, &["Proof"]))
        .unwrap();
    assert!(
        snapshot
            .interfaces()
            .all(|(owner, _)| owner.annotations.is_empty())
    );
    let repeated = directory.join("repeated.h");
    std::fs::write(&repeated, "void Proof(INPUT int* value);\n").unwrap();
    let include = format!(
        "#include {:?}\n",
        repeated.to_string_lossy().replace('\\', "/")
    );
    let source = format!(
        "#include <sal.h>\n#define INPUT LEAF\n#define LEAF _In_\n{include}\
        #undef LEAF\n#define LEAF _Out_\n{include}"
    );
    let mut repeated_input = input(&source, Target::X64, &["Proof"]);
    repeated_input.roots.push(repeated);
    let error = compiler.capture(&repeated_input).unwrap_err();
    assert_eq!(error.stage, Stage::Capture);
    assert!(error.message.contains("ambiguous consumed"), "{error}");
}
