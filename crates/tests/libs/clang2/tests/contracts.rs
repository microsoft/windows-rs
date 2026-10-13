use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use windows_clang2::*;
use windows_metadata::{HasAttributes, ParamAttributes, Value, reader::*};

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
