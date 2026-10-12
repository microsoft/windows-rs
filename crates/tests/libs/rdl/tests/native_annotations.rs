use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_annotations_{name}.winmd"))
}

fn vocabulary(name: &str) -> PathBuf {
    let path = output(&format!("vocabulary_{name}"));
    windows_rdl::reader()
        .input_text(include_str!("../../../../../metadata/metadata.rdl"))
        .output(&path)
        .write()
        .unwrap();
    path
}

fn compile(source: &str, path: &Path, reference: &Path) {
    windows_rdl::reader()
        .reference(reference)
        .input_text(source)
        .output(path)
        .write()
        .unwrap();
}

fn annotations<'a>(owner: impl HasAttributes<'a>, name: &str) -> Vec<Vec<Value>> {
    let mut result: Vec<_> = owner
        .attributes()
        .filter(|attr| attr.namespace() == "Windows.Win32.Metadata" && attr.name() == name)
        .map(|attr| attr.value().into_iter().map(|(_, value)| value).collect())
        .collect();
    result.sort_by_key(|values| format!("{values:?}"));
    result
}

fn evidence(source: &str, ordinal: i32, payload: &str) -> Vec<Value> {
    vec![
        Value::Utf8(source.to_string()),
        Value::I32(ordinal),
        Value::Utf8(payload.to_string()),
    ]
}

fn binding(source: &str, ordinal: i32, position: i32) -> Vec<Value> {
    vec![
        Value::Utf8(source.to_string()),
        Value::I32(ordinal),
        Value::Utf8("count".to_string()),
        Value::I32(position),
    ]
}

fn assert_metadata(path: &Path, namespace: &str, renamed: bool) {
    let index = Index::read(path).unwrap();
    index.validate_native_contracts(None).unwrap();
    let alias = index.expect(namespace, "Alias");
    assert_eq!(
        alias.fields().next().unwrap().ty(),
        Type::Const(Box::new(Type::PtrMut(
            Box::new(Type::value_named(namespace, "Record")),
            1,
        )))
    );
    assert_eq!(
        annotations(alias, "NativeAnnotationAttribute"),
        [evidence("CXX", 0, "__declspec(annotate(\"alias\"))")]
    );
    let field = index.expect(namespace, "Record").fields().next().unwrap();
    assert_eq!(
        annotations(field, "NativeAnnotationAttribute"),
        [evidence("SAL", 0, "_Field_size_(count)")]
    );
    for (owner, name, direction, count) in [
        ("Callback", "Invoke", ParamDirection::Unspecified, 2),
        ("Apis", "Export", ParamDirection::Unspecified, 3),
        ("IContracts", "Exchange", ParamDirection::InputOutput, 2),
    ] {
        let method = index
            .expect(namespace, owner)
            .methods()
            .find(|method| method.name() == name)
            .unwrap();
        let params = method.params_by_sequence(count).unwrap();
        let first = params.params()[0].unwrap();
        assert_eq!(first.direction(), direction);
        assert_eq!(
            params.params()[1].unwrap().name(),
            if renamed { "renamed_length" } else { "length" }
        );
        assert_eq!(
            params.params()[1].unwrap().direction(),
            ParamDirection::Input
        );
        if name == "Export" {
            assert_eq!(
                method.signature(&[]).types[0],
                Type::PtrMut(
                    Box::new(Type::Volatile(Box::new(Type::value_named(
                        namespace, "Record"
                    )))),
                    1,
                ),
            );
            assert!(first.is_optional());
            assert_eq!(
                annotations(first, "NativeAnnotationAttribute"),
                [
                    evidence("MIDL", 0, "[out, size_is(count)]"),
                    evidence("SAL", 10, "_When_(count > 0, _In_)"),
                    evidence("SAL", 2, "_In_reads_(count)"),
                ]
            );
            assert_eq!(
                annotations(first, "NativeAnnotationBindingAttribute"),
                [
                    binding("MIDL", 0, 1),
                    binding("SAL", 10, 1),
                    binding("SAL", 2, 1)
                ]
            );
            assert_eq!(
                annotations(method, "NativeAnnotationAttribute"),
                [evidence("SAL", 0, "_Success_(return != 0)")]
            );
            assert_eq!(
                params.params()[2].unwrap().direction(),
                ParamDirection::Output
            );
        } else {
            let source = if name == "Invoke" { "SAL" } else { "MIDL" };
            let payload = if name == "Invoke" {
                "_In_reads_(count)"
            } else {
                "[in, size_is(count)]"
            };
            assert_eq!(
                annotations(first, "NativeAnnotationAttribute"),
                [evidence(source, 0, payload)]
            );
            assert_eq!(
                annotations(first, "NativeAnnotationBindingAttribute"),
                [binding(source, 0, 1)]
            );
        }
        if name != "Exchange" {
            assert_eq!(
                annotations(params.return_param().unwrap(), "NativeAnnotationAttribute"),
                [evidence("SAL", 0, "_Ret_maybenull_")]
            );
        }
    }
    let method = index
        .expect(namespace, "Apis")
        .methods()
        .find(|method| method.name() == "VoidResult")
        .unwrap();
    assert_eq!(method.signature(&[]).return_type, Type::Void);
    let params = method.params_by_sequence(1).unwrap();
    let result = params.return_param().unwrap();
    assert_eq!(result.parent(), method);
    assert_eq!(
        annotations(result, "NativeAnnotationBindingAttribute"),
        [binding("SAL", 0, 0)]
    );
    assert_eq!(
        annotations(result, "NativeAnnotationAttribute"),
        [evidence("SAL", 0, "_Post_satisfies_(count == 0)")]
    );
}

#[test]
fn reviewed_evidence_and_bindings_roundtrip_without_lowering_or_name_inference() {
    let reference = vocabulary("reviewed");
    let fixture = include_str!("../native_annotations.rdl");
    let input = output("fixture");
    compile(fixture, &input, &reference);
    assert_metadata(&input, "Test", false);
    let rdl = input.with_extension("rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(&rdl).unwrap(),
        fixture.replace("\r\n", "\n")
    );
    let dir = Path::new(env!("OUT_DIR")).join("native_annotations_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();
    let roundtrip = dir.join(input.file_name().unwrap());
    compile(
        &std::fs::read_to_string(&rdl).unwrap(),
        &roundtrip,
        &reference,
    );
    assert_metadata(&roundtrip, "Test", false);
    assert_eq!(
        std::fs::read(&input).unwrap(),
        std::fs::read(&roundtrip).unwrap()
    );
    let renamed = output("renamed");
    compile(
        &fixture.replace("length:", "renamed_length:"),
        &renamed,
        &reference,
    );
    assert_metadata(&renamed, "Test", true);
    let merged = output("merged");
    merge().input(&input).output(&merged).merge().unwrap();
    assert_metadata(&merged, "Test", false);
    let remapped = output("remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .fallback("Routed")
        .output(&remapped)
        .remap()
        .unwrap();
    assert_metadata(&remapped, "Routed", false);
}

#[test]
fn direction_defaults_and_explicit_unspecified_are_distinct_for_each_pointer_kind() {
    let reference = vocabulary("direction");
    for (position, ty) in ["i32", "*mut i32", "*const i32", "Const<*mut i32>"]
        .into_iter()
        .enumerate()
    {
        let input = output(&format!("direction_{position}"));
        compile(
            &format!(
                "#[win32] mod Test {{ extern \"C\" fn Callback(
                    inferred: {ty},
                    #[unspecified] unknown: {ty},
                    #[in] input: {ty},
                    #[out] output: {ty},
                    #[in] #[out] both: {ty}
                ); }}"
            ),
            &input,
            &reference,
        );
        let check = |path: &Path| {
            let index = Index::read(path).unwrap();
            let method = index.expect("Test", "Callback").methods().next().unwrap();
            let params = method.params_by_sequence(5).unwrap();
            assert_eq!(
                params
                    .params()
                    .iter()
                    .map(|param| param.unwrap().direction())
                    .collect::<Vec<_>>(),
                [
                    if position == 1 || position == 3 {
                        ParamDirection::Output
                    } else {
                        ParamDirection::Input
                    },
                    ParamDirection::Unspecified,
                    ParamDirection::Input,
                    ParamDirection::Output,
                    ParamDirection::InputOutput,
                ]
            );
        };
        check(&input);
        let rdl = input.with_extension("rdl");
        windows_rdl::writer()
            .input(&input)
            .output(&rdl)
            .write()
            .unwrap();
        let roundtrip = output(&format!("direction_roundtrip_{position}"));
        compile(
            &std::fs::read_to_string(rdl).unwrap(),
            &roundtrip,
            &reference,
        );
        check(&roundtrip);
    }
}

#[test]
fn invalid_contracts_reject_without_replacing_output() {
    let reference = vocabulary("invalid");
    for (body, diagnostic) in [
        (
            "extern \"C\" fn Callback(#[unspecified] #[in] value: *mut i32);",
            "conflicts",
        ),
        (
            "extern \"C\" fn Callback(#[unspecified(1)] value: *mut i32);",
            "does not accept arguments",
        ),
        (
            "#[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", -1, \"_In_\")]
             type Alias = i32;",
            "invalid native annotation arguments",
        ),
        (
            "#[Windows::Win32::Metadata::NativeAnnotation(\"\", 0, \"_In_\")]
             type Alias = i32;",
            "invalid native annotation arguments",
        ),
        (
            "#[Windows::Win32::Metadata::NativeAnnotationBinding(\"SAL\", 0, \"count\", 0)]
             type Alias = i32;",
            "no matching evidence",
        ),
        (
            "#[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"payload\")]
             #[Windows::Win32::Metadata::NativeAnnotationBinding(\"SAL\", 0, \"count\", 0)]
             type Alias = i32;",
            "requires a callable owner",
        ),
        (
            "extern \"C\" fn Callback(
             #[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"_In_reads_(count)\")]
             #[Windows::Win32::Metadata::NativeAnnotationBinding(\"SAL\", 0, \"count\", 1)]
             value: *mut i32);",
            "out of range",
        ),
        (
            "extern \"C\" fn Callback(
             #[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"_In_reads_(count)\")]
             #[Windows::Win32::Metadata::NativeAnnotationBinding(\"MIDL\", 0, \"count\", 0)]
             value: *mut i32);",
            "no matching evidence",
        ),
        (
            "extern \"C\" fn Callback(
             #[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"_In_\")]
             #[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"_Out_\")]
             value: *mut i32);",
            "duplicate native annotation",
        ),
        (
            "extern \"C\" fn Callback(
             #[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"_In_reads_(count)\")]
             value: *mut i32,
             #[Windows::Win32::Metadata::NativeAnnotationBinding(\"SAL\", 0, \"count\", 0)]
             count: usize);",
            "no matching evidence",
        ),
    ] {
        let path = output("invalid");
        std::fs::write(&path, b"last good output").unwrap();
        let error = windows_rdl::reader()
            .reference(&reference)
            .input_text(&format!("#[win32] mod Test {{ {body} }}"))
            .output(&path)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"last good output");
    }
    for body in [
        "struct Record {
         #[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"payload\")]
         value: i32, }",
        "interface IContract { fn Read(&self,
         #[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"_In_\")]
         value: i32); }",
    ] {
        let path = output("invalid_winrt");
        std::fs::write(&path, b"last good output").unwrap();
        let error = windows_rdl::reader()
            .reference(&reference)
            .input_text(&format!("#[winrt] mod Test {{ {body} }}"))
            .output(&path)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains("not valid in WinRT"), "{error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"last good output");
    }
}

#[test]
fn raw_payload_literals_and_architecture_alias_annotations_are_preserved() {
    let reference = vocabulary("literal");
    let source = r##"
        #[win32] mod Test {
            #[arch(X86)]
            #[Windows::Win32::Metadata::NativeAnnotation("SAL", 0, "literal #[in] //!\n\"quoted\"")]
            type Alias = *mut i32;
        }
    "##;
    let input = output("literal");
    compile(source, &input, &reference);
    let rdl = input.with_extension("rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    let text = std::fs::read_to_string(&rdl).unwrap();
    assert_eq!(text.matches("#[arch(X86)]").count(), 1);
    let roundtrip = output("literal_roundtrip");
    compile(&text, &roundtrip, &reference);
    for path in [&input, &roundtrip] {
        let index = Index::read(path).unwrap();
        let alias = index.expect("Test", "Alias");
        assert_eq!(alias.arches(), 1);
        assert_eq!(
            annotations(alias, "NativeAnnotationAttribute"),
            [evidence("SAL", 0, "literal #[in] //!\n\"quoted\"")]
        );
    }
}
