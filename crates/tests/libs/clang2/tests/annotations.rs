use windows_clang2::*;
use windows_metadata::{Value, reader, reader::HasAttributes};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input() -> Input {
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "annotations.h".into();
    input.source = test_clang2::fixture("annotations").source;
    input.roots.clear();
    input
}

fn imports() -> projection::Imports {
    [
        "ReadBuffer",
        "ReadFixed",
        "ReadHook",
        "InspectResult",
        "OptionalResult",
        "Conditional",
        "ReadFile",
        "GetModuleFileNameW",
    ]
    .into_iter()
    .map(|name| {
        (
            name.into(),
            projection::Import {
                library: "example.dll".into(),
                name: None,
            },
        )
    })
    .collect()
}

fn attributes<'a>(value: impl HasAttributes<'a>) -> Vec<Vec<Value>> {
    value
        .attributes()
        .filter(|attribute| attribute.name() == "NativeSalAttribute")
        .map(|attribute| {
            attribute
                .value()
                .into_iter()
                .map(|(_, value)| value)
                .collect()
        })
        .collect()
}

fn annotation(name: &str, arguments: &str, expanded: &str) -> Vec<Value> {
    vec![
        Value::Utf8("2".into()),
        Value::Utf8(name.into()),
        Value::Utf8(arguments.into()),
        Value::Utf8(expanded.into()),
    ]
}

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn annotation_roundtrip() {
    helpers::ensure_libclang();
    let input = input();
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let alias = |name: &str| {
            &snapshot
                .aliases
                .iter()
                .find(|alias| alias.name == name)
                .unwrap()
                .observations[0]
        };
        assert_eq!(alias("AContract").canonical, alias("ZPlain").canonical);
        assert!(
            alias("AContract").parameters.as_ref().unwrap()[0]
                .attributes
                .iter()
                .any(|attribute| matches!(attribute.kind, AttributeKind::Sal(_)))
        );
        assert!(
            alias("ZPlain").parameters.as_ref().unwrap()[0]
                .attributes
                .is_empty()
        );
    }
    let merged = merge(&snapshots).unwrap();
    let output = projection::project_with_imports(&merged, "Fixture", &imports()).unwrap();
    let text: String = output.partitions.values().map(|p| p.rdl.as_str()).collect();
    assert!(!text.contains("windows_clang:sal:"));
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let expected = test_clang2::input_dir().join("annotations.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let winmd = root.join("target\\test-clang2-annotations.winmd");
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&again)
        .write()
        .unwrap();
    for path in [&winmd, &again] {
        for bit in [1, 2, 4] {
            let index =
                reader::Index::new_for_architecture(vec![reader::File::read(path).unwrap()], bit);
            assert_eq!(
                index
                    .expect("Fixture", "Status")
                    .find_attribute("SuccessConditionAttribute")
                    .unwrap()
                    .value(),
                vec![(
                    String::new(),
                    windows_rdl::emit::SuccessCondition::Nonnegative.to_value()
                )]
            );
            let apis = index.expect("Fixture", "Apis");
            let method = |name| {
                apis.methods()
                    .find(|method| {
                        method.name() == name
                            && (method.arches() == 0 || method.arches() & bit != 0)
                    })
                    .unwrap()
            };
            let read = method("ReadBuffer");
            assert!(attributes(read).is_empty());
            assert!(read.find_attribute("MustUseAttribute").is_some());
            let params: Vec<_> = read.params().collect();
            assert!(attributes(params[0]).is_empty());
            assert_eq!(
                test_clang2::buffer_extent(
                    params[0],
                    windows_metadata::BufferAccess::Read,
                    windows_metadata::BufferPhase::Success
                ),
                Some(windows_metadata::BufferExtent {
                    count_optional: true,
                    ..windows_metadata::BufferExtent::new(
                        windows_metadata::BufferAccess::Read,
                        windows_metadata::BufferPhase::Success,
                        windows_metadata::BufferUnits::Bytes,
                        windows_metadata::BufferCount::Dereference(2),
                    )
                })
            );
            assert!(params[0].is_optional());
            assert_eq!(params[0].direction(), reader::ParamDirection::Output);
            let fixed = method("ReadFixed").params().next().unwrap();
            assert_eq!(
                attributes(fixed),
                vec![annotation("_In_reads_", "16", "16")]
            );
            assert_eq!(fixed.direction(), reader::ParamDirection::Input);
            assert_eq!(
                attributes(method("ReadHook").params().next().unwrap()),
                vec![annotation("_In_reads_", "REVIEW_LIMIT", "16")]
            );
            assert!(
                !method("InspectResult")
                    .params()
                    .next()
                    .unwrap()
                    .is_optional()
            );
            assert!(
                method("OptionalResult")
                    .params()
                    .next()
                    .unwrap()
                    .is_optional()
            );
            let interface = index.expect("Fixture", "IAnnotated");
            let parameter = interface.methods().next().unwrap().params().next().unwrap();
            assert!(attributes(parameter).is_empty());
            assert!(parameter.find_attribute("ComOutPtrAttribute").is_some());
            assert_eq!(
                parameter
                    .find_attribute("NativeReturnedPointerAttribute")
                    .unwrap()
                    .value(),
                [
                    ("MayBeNull".into(), Value::Bool(false)),
                    ("NullOnFailure".into(), Value::Bool(true)),
                ]
            );
            let field = index
                .expect("Fixture", "BufferView")
                .fields()
                .find(|field| field.name() == "data")
                .unwrap();
            assert!(attributes(field).is_empty());
            assert_eq!(
                test_clang2::buffer_extent(
                    field,
                    windows_metadata::BufferAccess::Write,
                    windows_metadata::BufferPhase::Value
                ),
                Some(windows_metadata::BufferExtent::new(
                    windows_metadata::BufferAccess::Write,
                    windows_metadata::BufferPhase::Value,
                    windows_metadata::BufferUnits::Elements,
                    windows_metadata::BufferCount::Field("count".into())
                ))
            );
            let conditional = attributes(method("Conditional").params().next().unwrap());
            assert_eq!(conditional.len(), 1);
            assert_eq!(conditional[0][1], Value::Utf8("_When_".into()));
            assert_eq!(
                conditional[0][2],
                Value::Utf8("capacity > 0,_Out_writes_(capacity)".into())
            );
            assert_eq!(
                conditional[0][3],
                Value::Utf8("capacity > 0, _Out_writes_(capacity)".into())
            );
            for (name, expanded) in [
                (
                    "Scoped",
                    "capacity > 0, _At_(value, _Group_(_In_reads_(16) _Always_(_In_)))",
                ),
                (
                    "ScopedBuffer",
                    "value, index, capacity, _When_(index > 0, _Out_)",
                ),
            ] {
                let param = index
                    .expect("Fixture", name)
                    .methods()
                    .next()
                    .unwrap()
                    .params()
                    .next()
                    .unwrap();
                let values = attributes(param);
                assert_eq!(values.len(), 1);
                assert_eq!(values[0][3], Value::Utf8(expanded.into()));
                let plain = index
                    .expect("Fixture", "Unscoped")
                    .methods()
                    .next()
                    .unwrap()
                    .params()
                    .next()
                    .unwrap();
                assert_eq!(param.flags(), plain.flags());
                assert!(param.find_attribute("NativeInputAttribute").is_none());
                assert!(param.find_attribute("NativeArrayInfoAttribute").is_none());
            }
            assert_eq!(
                attributes(index.expect("Fixture", "Failure"))[0][3],
                Value::Utf8("_Unchanged_(*value)".into())
            );
            for name in ["AContract", "CopiedContract"] {
                let callback = index.expect("Fixture", name);
                let invoke = callback
                    .methods()
                    .find(|method| method.name() == "Invoke")
                    .unwrap();
                let params: Vec<_> = invoke.params().collect();
                assert_eq!(
                    params.iter().map(|param| param.name()).collect::<Vec<_>>(),
                    ["buffer", "capacity", "written"]
                );
                let contract = attributes(params[0]);
                assert_eq!(contract.len(), 1);
                assert_eq!(contract[0][1], Value::Utf8("_When_".into()));
                assert_eq!(
                    contract[0][2],
                    Value::Utf8("capacity > 0,_Out_writes_bytes_to_(capacity, *written)".into())
                );
                assert_eq!(params[1].direction(), reader::ParamDirection::Input);
                assert_eq!(params[2].direction(), reader::ParamDirection::Output);
            }
            let plain = index.expect("Fixture", "ZPlain");
            for (name, callback) in [
                ("ContractAddress", "AContract"),
                ("ResultAddress", "AResultContract"),
            ] {
                assert_eq!(
                    index.expect("Fixture", name).underlying_type(),
                    Some(windows_metadata::Type::PtrMut(
                        Box::new(windows_metadata::Type::class_named("Fixture", callback)),
                        1
                    ))
                );
            }
            assert!(
                plain
                    .methods()
                    .find(|method| method.name() == "Invoke")
                    .unwrap()
                    .params()
                    .all(|param| attributes(param).is_empty())
            );
            let result_callback = index
                .expect("Fixture", "ResultOnly")
                .fields()
                .next()
                .unwrap()
                .ty();
            assert!(matches!(result_callback, windows_metadata::Type::FnPtr(_)));
            let inherited = index.expect("Fixture", "CopiedResult");
            assert!(inherited.find_attribute("MustUseAttribute").is_some());
            assert!(inherited.find_attribute("NativeSalAttribute").is_none());
            let holder = index.expect("Fixture", "CallbackHolder");
            let field = |name| {
                holder
                    .fields()
                    .find(|field| field.name() == name)
                    .unwrap()
                    .ty()
            };
            assert_eq!(
                field("named"),
                windows_metadata::Type::class_named("Fixture", "AContract")
            );
            assert_eq!(
                field("inferred"),
                windows_metadata::Type::class_named("Fixture", "ZPlain")
            );
            let tagged = index.expect("Fixture", "Tagged");
            let sal: Vec<_> = tagged
                .attributes()
                .filter(|attribute| attribute.name() == "NativeSalAttribute")
                .map(|attribute| attribute.value())
                .collect();
            assert_eq!(sal.len(), 2);
            assert_eq!(
                sal.iter()
                    .filter(|values| values.iter().any(|(name, value)| {
                        name == "TypedefName" && *value == Value::Utf8("Tagged".into())
                    }))
                    .count(),
                1
            );
            assert!(
                index
                    .expect("Fixture", "Code")
                    .attributes()
                    .filter(|attribute| attribute.name() == "NativeSalAttribute")
                    .any(|attribute| attribute
                        .value()
                        .iter()
                        .any(|(name, value)| name == "TypedefName"
                            && *value == Value::Utf8("Code".into())))
            );
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
    let mut selected = self::input();
    selected.roots = vec!["AContract".into(), "OnlyInferred".into()];
    let selected = projection::generate(&[selected], "Fixture").unwrap();
    let text: String = selected
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("callback: extern"), "{text}");
    assert_eq!(
        projection::project_partial_with_imports(
            capture_partial(&[input], &TARGETS).unwrap(),
            "Fixture",
            &imports()
        )
        .unwrap()
        .output,
        output
    );
}

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn annotation_conflicts() {
    helpers::ensure_libclang();
    let mut inputs = [input(), input()];
    inputs[1].name = "second.h".into();
    inputs[1].source.insert_str(0, "#define REVIEW_LIMIT 17\n");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let error = merge(&snapshots).err().unwrap().to_string();
        assert!(
            !error.contains("same target, compiler, and arguments"),
            "{error}"
        );
        inputs.reverse();
    }
}

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn sdk_annotations() {
    helpers::ensure_libclang();
    let mut input = input();
    input.source = "#include <windows.h>\n#include <unknwn.h>\n".into();
    input.roots = vec![
        "ReadFile".into(),
        "GetModuleFileNameW".into(),
        "IClassFactory".into(),
    ];
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let read = snapshot
            .values
            .iter()
            .find(|value| value.name == "ReadFile")
            .unwrap();
        let ValueKind::Function { parameters, .. } = &read.observations[0].kind else {
            panic!()
        };
        let sal = parameters[1]
            .attributes
            .iter()
            .find_map(|attribute| {
                if let AttributeKind::Sal(value) = &attribute.kind
                    && value.name == "_Out_writes_bytes_to_opt_"
                {
                    Some(value)
                } else {
                    None
                }
            })
            .unwrap();
        assert_eq!(sal.arguments, "nNumberOfBytesToRead,*lpNumberOfBytesRead");
        assert_eq!(sal.expanded, sal.arguments);
    }
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
            .unwrap();
    assert_eq!(output.functions, 2);
    assert_eq!(output.interfaces, 2);
    let text: String = output.partitions.values().map(|p| p.rdl.as_str()).collect();
    assert!(!text.contains("_COM_Outptr_"));
    assert!(text.contains("#[out_ptr(null_on_failure)]"));
    assert!(text.contains("#[success(return >= 0)]"));
    assert!(
        text.contains("#[writable(entry, bytes(nNumberOfBytesToRead))]"),
        "{text}"
    );
    assert!(
        text.contains("#[writable(entry, nSize)] lpFilename:"),
        "{text}"
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let path = root.join("target\\test-clang2-sdk-annotations.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&path)
        .write()
        .unwrap();
    for bit in [1, 2, 4] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&path).unwrap()], bit);
        let method = index
            .expect("Fixture", "Apis")
            .methods()
            .find(|method| {
                method.name() == "ReadFile" && (method.arches() == 0 || method.arches() & bit != 0)
            })
            .unwrap();
        assert_eq!(
            test_clang2::buffer_extent(
                method.params().nth(1).unwrap(),
                windows_metadata::BufferAccess::Read,
                windows_metadata::BufferPhase::Success
            ),
            Some(windows_metadata::BufferExtent {
                count_optional: true,
                ..windows_metadata::BufferExtent::new(
                    windows_metadata::BufferAccess::Read,
                    windows_metadata::BufferPhase::Success,
                    windows_metadata::BufferUnits::Bytes,
                    windows_metadata::BufferCount::Dereference(3)
                )
            })
        );
        assert_eq!(
            method.params().nth(1).unwrap().buffer_relationship(),
            Some(reader::BufferRelationship::BytesParam(2))
        );
        let module_name = index
            .expect("Fixture", "Apis")
            .methods()
            .find(|method| {
                method.name() == "GetModuleFileNameW"
                    && (method.arches() == 0 || method.arches() & bit != 0)
            })
            .unwrap();
        assert_eq!(
            module_name.params().nth(1).unwrap().buffer_relationship(),
            Some(reader::BufferRelationship::ElementsParam(2))
        );
        let create = index
            .expect("Fixture", "IClassFactory")
            .methods()
            .find(|method| method.name() == "CreateInstance")
            .unwrap();
        let result = create.params().last().unwrap();
        assert!(!attributes(result).contains(&annotation("_COM_Outptr_", "", "")));
        assert!(result.find_attribute("ComOutPtrAttribute").is_some());
        assert_eq!(
            result
                .find_attribute("NativeReturnedPointerAttribute")
                .unwrap()
                .value(),
            [
                ("MayBeNull".into(), Value::Bool(false)),
                ("NullOnFailure".into(), Value::Bool(true)),
            ]
        );
        assert!(!result.is_optional());
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn alias_annotation_reconciliation() {
    helpers::ensure_libclang();
    let input = |contract| {
        let mut input = test_clang2::fixture("annotation_merge");
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
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert_eq!(text.matches("NativeSal(").count(), 1);
        assert!(text.contains("#[success(return >= 0)]"));
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
    let mut input = test_clang2::fixture("annotation_merge");
    input.arguments.push("-DCONTRACT=0".into());
    input.roots = vec!["UsesVoid".into()];
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
            .any(|value| value.contains("AnnotatedVoid"))
    );
    assert!(
        result
            .omissions
            .iter()
            .any(|value| value.contains("UsesVoid"))
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
        let mut input = test_clang2::fixture("annotation_transport");
        input
            .source
            .insert_str(0, &format!("#define PAYLOAD {payload:?}\n"));
        let error = capture(&[input], &[Target::X64]).unwrap_err().to_string();
        assert!(
            error.contains("invalid SAL annotation transport"),
            "{error}"
        );
    }

    let mut input = test_clang2::fixture("annotation_transport");
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
        let mut input = test_clang2::fixture("annotation_transport");
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
    let evidence = format!("{snapshots:?}");
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(format!("{snapshots:?}"), evidence);
    let text: String = output.partitions.values().map(|p| p.rdl.as_str()).collect();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let path = root.join("target\\test-clang2-nested-transport.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&path)
        .write()
        .unwrap();
    let index = reader::Index::read(&path).unwrap();
    assert_eq!(
        attributes(index.expect("Fixture", "Annotated")),
        vec![annotation("_When_", "n > 0,_In_reads_(LIMIT)", &expected)]
    );
    std::fs::remove_file(path).unwrap();

    for malformed in [
        native.replace("sal:2:", "sal:3:"),
        native.trim_end_matches(')').into(),
        native.replace("\\n", "\\t"),
    ] {
        let input = input(&malformed);
        let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
        assert!(projection::project(&merge(&snapshots).unwrap(), "Fixture").is_err());
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.records, 0);
        assert!(partial.omissions.iter().any(|error| error.contains("SAL")));
    }
    let mut inputs = [input(&native), input(&nested("OTHER_LIMIT"))];
    inputs[1].name = "other.h".into();
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        assert!(merge(&snapshots).is_err());
        assert!(capture_partial(&inputs, &TARGETS).is_err());
        inputs.reverse();
    }
}

#[test]
fn callback_contract_boundaries() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang2::selected_fixture("callback_contracts", &["Reader"]),
        test_clang2::selected_fixture("callback_contracts", &["Reader"]),
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
    let input = test_clang2::selected_fixture("callback_contracts", &["UsesNested"]);
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
