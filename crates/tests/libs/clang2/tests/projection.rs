use windows_clang2::{
    Input, Plan, ProjectionOptions, ReferenceKind, Snapshot, TypeReference, capture,
};
use windows_metadata::{
    Type, Value,
    reader::{HasAttributes, Index, Item, ParamDirection, TypeDef},
};

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];

fn options() -> ProjectionOptions {
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    for (name, kind) in [
        ("Data", ReferenceKind::Value),
        ("IUnknown", ReferenceKind::Interface),
    ] {
        options.references.insert(
            name.into(),
            TypeReference {
                namespace: "External".into(),
                name: name.into(),
                kind,
            },
        );
    }
    options
}

fn compile(name: &str, plan: &Plan) -> Index {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join(name);
    std::fs::create_dir_all(&scratch).unwrap();
    let reference = scratch.join("external.winmd");
    windows_rdl::reader()
        .input_text(include_str!("../input/external.rdl"))
        .output(&reference)
        .write()
        .unwrap();
    let rdl = plan.rdl();
    let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("expected")
        .join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        std::fs::write(&expected, &rdl).unwrap();
    }
    assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
    let output = scratch.join("test.winmd");
    windows_rdl::reader()
        .input_text(&rdl)
        .reference(reference)
        .output(&output)
        .write()
        .unwrap();
    Index::read(output).unwrap()
}

#[test]
fn external_references_do_not_hide_type_root_conflicts() {
    let source = include_str!("../input/dependencies.h");
    for root in ["Packet", "Use", "Callback", "ITEM"] {
        let snapshot = capture(
            [
                Input::new("a.hpp", format!("#define VALUE int\n{source}")),
                Input::new("b.hpp", format!("#define VALUE float\n{source}")),
            ],
            ARGS,
            &[root],
        )
        .unwrap();
        assert!(snapshot.resolve().is_err());
    }
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            format!("#define VALUE int\n{source}"),
        )],
        ARGS,
        &["Packet", "Use"],
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    let plan = resolved.project(&options()).unwrap();
    let index = compile("referenced_record", &plan);
    let Item::Type(packet) = index.expect_item("Test", "Packet") else {
        panic!()
    };
    assert_eq!(
        packet.fields().next().unwrap().ty(),
        Type::PtrMut(Box::new(Type::value_named("External", "Data")), 1)
    );
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [Type::PtrMut(
            Box::new(Type::value_named("Test", "Packet")),
            1
        )]
    );
}

#[test]
fn indirect_interface_constants_are_omitted_after_native_resolution() {
    let source = include_str!("../input/interface_constants.h");
    for indirect in [true, false] {
        let source = if indirect {
            source.to_string()
        } else {
            source.replace("typedef Raw LPUNKNOWN;", "typedef IUnknown* LPUNKNOWN;")
        };
        let snapshot = capture(
            [Input::new("api.hpp", source)],
            ARGS,
            &["UNKNOWN_NULL", "INTEGER_CONSTANT", "Use"],
        )
        .unwrap();
        if indirect {
            assert!(snapshot.dump().contains("] Raw "));
            assert!(snapshot.dump().contains("] LPUNKNOWN "));
        }
        let resolved = snapshot.resolve().unwrap();
        let plan = resolved.project(&options()).unwrap();
        assert_eq!(
            plan.omitted().get("UNKNOWN_NULL").unwrap(),
            "interface values are not metadata constants"
        );
        let index = compile("interface_constants", &plan);
        let Item::Fn(function) = index.expect_item("Test", "Use") else {
            panic!()
        };
        assert_eq!(
            function.signature(&[]).types,
            [
                Type::class_named("External", "IUnknown"),
                Type::PtrMut(Box::new(Type::class_named("External", "IUnknown")), 1),
            ]
        );
        let Item::Const(constant) = index.expect_item("Test", "INTEGER_CONSTANT") else {
            panic!()
        };
        assert_eq!(constant.constant().unwrap().value(), Value::I32(42));
    }
}

#[test]
fn mismatched_record_layout_is_not_emitted() {
    let snapshot = capture(
        [Input::new(
            "pack.hpp",
            "#pragma pack(push, 1)\nstruct Packet { char flag; int value; };\n#pragma pack(pop)",
        )],
        ARGS,
        &["Packet"],
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    assert!(
        resolved
            .project(&options())
            .unwrap_err()
            .to_string()
            .contains("packing")
    );
}

#[test]
fn incompatible_interface_observations_are_not_hidden_by_binding() {
    let snapshot = capture([
        Input::new("a.hpp", "struct IUnknown { virtual void Method(int value) = 0; }; extern \"C\" void Use(IUnknown* value);"),
        Input::new("b.hpp", "struct IUnknown { virtual void Method(float value) = 0; }; extern \"C\" void Use(IUnknown* value);"),
    ], ARGS, &["Use"]).unwrap();
    assert!(snapshot.resolve().is_err());
}

fn sdk_capture(target: &str, source: &str, roots: &[&str]) -> Snapshot {
    use std::path::PathBuf;
    let tools = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("tools");
    let version = helpers::read_str_const(
        tools.join("win32").join("src").join("main.rs"),
        "SDK_VERSION",
    );
    let (marketing, _) = version.rsplit_once('.').unwrap();
    let include = helpers::nuget_package("microsoft.windows.sdk.cpp", &version)
        .join("c")
        .join("Include")
        .join(format!("{marketing}.0"));
    let sal = tools.join("win32").join("src").join("sal.h");
    let shared = include.join("shared");
    let um = include.join("um");
    capture(
        [Input::new("wrapper.hpp", source)],
        &[
            "-x",
            "c++",
            target,
            // specstrings.h redefines COM SAL macros; install the capture shim after it.
            "-include",
            "specstrings.h",
            "-include",
            sal.to_str().unwrap(),
            "-isystem",
            shared.to_str().unwrap(),
            "-isystem",
            um.to_str().unwrap(),
        ],
        roots,
    )
    .unwrap()
}

#[test]
fn real_iunknown_projects_through_a_checked_external_binding() {
    let snapshot = sdk_capture(
        "--target=x86_64-pc-windows-msvc",
        "#include <unknwnbase.h>\nextern \"C\" void Use(IUnknown* input, IUnknown** output);",
        &["Use"],
    );
    let resolved = snapshot.resolve().unwrap();
    assert!(snapshot.dump().contains("name: \"QueryInterface\""));
    let index = compile("real_iunknown", &resolved.project(&options()).unwrap());
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [
            Type::class_named("External", "IUnknown"),
            Type::PtrMut(Box::new(Type::class_named("External", "IUnknown")), 1),
        ]
    );
}

#[test]
fn real_interfaces_project_locally() {
    for (expected, target) in [
        ("sdk_interfaces", "--target=x86_64-pc-windows-msvc"),
        ("sdk_interfaces_x86", "--target=i686-pc-windows-msvc"),
        ("sdk_interfaces", "--target=aarch64-pc-windows-msvc"),
    ] {
        let snapshot = sdk_capture(target, include_str!("../input/sdk_interfaces.h"), &["Use"]);
        let mut options = ProjectionOptions::new("Test");
        options.library = Some("test.dll".into());
        options.references.insert(
            "_GUID".into(),
            TypeReference {
                namespace: "External".into(),
                name: "GUID".into(),
                kind: ReferenceKind::Value,
            },
        );
        let resolved = snapshot.resolve().unwrap();
        let plan = resolved.project(&options).unwrap();
        let index = compile(expected, &plan);
        let Item::Type(unknown) = index.expect_item("Test", "IUnknown") else {
            panic!()
        };
        assert_guid(unknown, 0, 0x46);
        assert_eq!(
            unknown
                .methods()
                .map(|method| method.name())
                .collect::<Vec<_>>(),
            ["QueryInterface", "AddRef", "Release"]
        );
        let query = unknown.methods().next().unwrap();
        assert_eq!(query.signature(&[]).return_type, Type::I32);
        assert_eq!(
            query.signature(&[]).types,
            [
                Type::PtrConst(Box::new(Type::value_named("External", "GUID")), 1),
                Type::PtrMut(Box::new(Type::Void), 2),
            ]
        );
        let parameters = query.params_by_sequence(2).unwrap();
        let output = parameters.params()[1].unwrap();
        assert_eq!(output.direction(), ParamDirection::Output);
        assert!(output.has_attribute("ComOutPtrAttribute"));
        assert!(!output.is_optional());
        for method in unknown.methods().skip(1) {
            assert_eq!(method.signature(&[]).return_type, Type::U32);
            assert!(method.signature(&[]).types.is_empty());
        }

        let Item::Type(factory) = index.expect_item("Test", "IClassFactory") else {
            panic!()
        };
        assert_guid(factory, 1, 0x46);
        assert_eq!(
            factory.interface_impls().next().unwrap().interface(&[]),
            Type::class_named("Test", "IUnknown")
        );
        assert_eq!(
            factory
                .methods()
                .map(|method| method.name())
                .collect::<Vec<_>>(),
            ["CreateInstance", "LockServer"]
        );
        let create = factory.methods().next().unwrap();
        assert_eq!(
            create.signature(&[]).types,
            [
                Type::class_named("Test", "IUnknown"),
                Type::PtrConst(Box::new(Type::value_named("External", "GUID")), 1),
                Type::PtrMut(Box::new(Type::Void), 2),
            ]
        );
        let parameters = create.params_by_sequence(3).unwrap();
        assert_eq!(
            parameters.params()[0].unwrap().direction(),
            ParamDirection::Input
        );
        assert!(parameters.params()[0].unwrap().is_optional());
        assert_eq!(
            parameters.params()[1].unwrap().direction(),
            ParamDirection::Input
        );
        assert_eq!(
            parameters.params()[2].unwrap().direction(),
            ParamDirection::Output
        );
        assert!(
            parameters.params()[2]
                .unwrap()
                .has_attribute("ComOutPtrAttribute")
        );
        assert_eq!(create.signature(&[]).return_type, Type::I32);
        let lock = factory.methods().nth(1).unwrap();
        assert_eq!(lock.signature(&[]).types, [Type::I32]);
        assert_eq!(lock.signature(&[]).return_type, Type::I32);

        let Item::Fn(function) = index.expect_item("Test", "Use") else {
            panic!()
        };
        assert_eq!(
            function.signature(&[]).types,
            [
                Type::class_named("Test", "IUnknown"),
                Type::PtrMut(Box::new(Type::class_named("Test", "IUnknown")), 1),
                Type::class_named("Test", "IClassFactory"),
            ]
        );
    }
}

fn assert_guid(ty: TypeDef<'_>, first: u32, last: u8) {
    let values: Vec<_> = ty
        .find_attribute("GuidAttribute")
        .unwrap()
        .value()
        .into_iter()
        .map(|(_, value)| value)
        .collect();
    assert_eq!(
        values,
        [
            Value::U32(first),
            Value::U16(0),
            Value::U16(0),
            Value::U8(0xc0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(last),
        ]
    );
}

#[test]
fn projection_rejects_unknown_layout_and_annotations() {
    for (source, root, reason) in [
        (
            "struct Data { int value; }; extern \"C\" void Use(Data value);",
            "Use",
            "by-value external",
        ),
        (
            "struct Data { int value; }; extern \"C\" Data Use();",
            "Use",
            "by-value external",
        ),
        ("struct Packet {};", "Packet", "empty record"),
        (
            "extern \"C\" void Use(int n, int* __attribute__((annotate(\"_Out_writes_(n)\"))) data);",
            "Use",
            "annotation projection",
        ),
    ] {
        let snapshot = capture([Input::new("api.hpp", source)], ARGS, &[root]).unwrap();
        assert!(
            snapshot
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap_err()
                .to_string()
                .contains(reason)
        );
    }
}

#[test]
fn projection_consumes_merged_annotation_evidence() {
    for reversed in [false, true] {
        let mut declarations = [
            "extern \"C\" void Fill(int* buffer);",
            "extern \"C\" void Fill(int* __attribute__((annotate(\"_Out_\"))) data);",
        ];
        if reversed {
            declarations.reverse();
        }
        let snapshot = capture(
            [Input::new("api.hpp", declarations.join("\n"))],
            ARGS,
            &["Fill"],
        )
        .unwrap();
        let index = compile(
            "annotations",
            &snapshot.resolve().unwrap().project(&options()).unwrap(),
        );
        let Item::Fn(function) = index.expect_item("Test", "Fill") else {
            panic!()
        };
        let parameters = function.params_by_sequence(1).unwrap();
        assert_eq!(
            parameters.params()[0].unwrap().direction(),
            ParamDirection::Output
        );
    }
}

#[test]
fn plan_uses_checked_completion_and_owns_its_rendering_data() {
    let plan = {
        let snapshot = capture(
            [
                Input::new("a.hpp", "struct Data; struct Packet { Data* value; };"),
                Input::new("b.hpp", "struct Data { int value; };"),
            ],
            ARGS,
            &["Packet"],
        )
        .unwrap();
        snapshot
            .resolve()
            .unwrap()
            .project(&ProjectionOptions::new("Test"))
            .unwrap()
    };
    let index = compile("completion", &plan);
    let Item::Type(data) = index.expect_item("Test", "Data") else {
        panic!()
    };
    assert_eq!(data.fields().next().unwrap().ty(), Type::I32);
}

#[test]
fn local_interfaces_project_real_com_metadata() {
    let source = include_str!("../input/interface.h");
    let snapshot = capture([Input::new("api.hpp", source)], ARGS, &["Use"]).unwrap();
    let resolved = snapshot.resolve().unwrap();
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    let plan = resolved.project(&options).unwrap();
    let index = compile("local_interface", &plan);

    let Item::Type(base) = index.expect_item("Test", "IBase") else {
        panic!()
    };
    assert_guid(base, 0, 0x46);
    assert!(base.extends().is_none());
    let base_methods: Vec<_> = base.methods().map(|method| method.name()).collect();
    assert_eq!(base_methods, ["Query", "Count"]);

    let Item::Type(derived) = index.expect_item("Test", "IDerived") else {
        panic!()
    };
    assert_guid(derived, 0, 0x47);
    assert_eq!(
        derived.interface_impls().next().unwrap().interface(&[]),
        Type::class_named("Test", "IBase")
    );
    let derived_methods: Vec<_> = derived.methods().map(|method| method.name()).collect();
    assert_eq!(derived_methods, ["Notify"]);
    let notify = derived.methods().next().unwrap();
    assert_eq!(
        notify.signature(&[]).types,
        [Type::I32, Type::PtrMut(Box::new(Type::I32), 1)]
    );
    let parameters = notify.params_by_sequence(2).unwrap();
    assert_eq!(
        parameters.params()[0].unwrap().direction(),
        ParamDirection::Input
    );
    assert_eq!(
        parameters.params()[1].unwrap().direction(),
        ParamDirection::Output
    );

    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [Type::class_named("Test", "IDerived")]
    );
}

#[test]
fn com_output_annotations_preserve_pointer_contracts() {
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            include_str!("../input/com_parameters.h"),
        )],
        ARGS,
        &["Create", "IFactory"],
    )
    .unwrap();
    let index = compile(
        "com_parameters",
        &snapshot.resolve().unwrap().project(&options()).unwrap(),
    );
    let Item::Fn(function) = index.expect_item("Test", "Create") else {
        panic!()
    };
    let Item::Type(interface) = index.expect_item("Test", "IFactory") else {
        panic!()
    };
    for method in [function, interface.methods().next().unwrap()] {
        assert_eq!(
            method.signature(&[]).types,
            [
                Type::PtrMut(Box::new(Type::Void), 2),
                Type::PtrMut(Box::new(Type::class_named("External", "IUnknown")), 1),
                Type::class_named("External", "IUnknown"),
            ]
        );
        let parameters = method.params_by_sequence(3).unwrap();
        for (index, parameter) in parameters.params()[..2].iter().enumerate() {
            let parameter = parameter.unwrap();
            assert_eq!(parameter.direction(), ParamDirection::Output);
            assert_eq!(parameter.has_attribute("ComOutPtrAttribute"), index == 0);
            assert!(!parameter.is_optional());
        }
        let optional = parameters.params()[2].unwrap();
        assert_eq!(optional.direction(), ParamDirection::Input);
        assert!(optional.is_optional());
    }
}

#[test]
fn com_output_annotations_reject_incompatible_types() {
    for ty in [
        "int",
        "int*",
        "int**",
        "void*",
        "void***",
        "const void**",
        "IUnknown*",
    ] {
        let snapshot = capture(
            [Input::new(
                "api.hpp",
                format!(
                    "#define OUTPUT_TYPE {ty}\n{}",
                    include_str!("../input/invalid_com_output.h")
                ),
            )],
            ARGS,
            &["Create"],
        )
        .unwrap();
        assert!(
            snapshot
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap_err()
                .to_string()
                .contains("_COM_Outptr_ requires"),
            "{ty}"
        );
    }
}

#[test]
fn interface_methods_require_the_com_calling_convention() {
    for (target, convention) in [
        ("i686", "__cdecl"),
        ("i686", "__thiscall"),
        ("i686", "__fastcall"),
        ("x86_64", "__vectorcall"),
    ] {
        let target = format!("--target={target}-pc-windows-msvc");
        let snapshot = capture(
            [Input::new(
                "api.hpp",
                include_str!("../input/interface.h").replace("__stdcall", convention),
            )],
            &["-x", "c++", &target],
            &["Use"],
        )
        .unwrap();
        assert!(
            snapshot
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap_err()
                .to_string()
                .contains("calling convention incompatible with COM"),
            "{target}: {convention}"
        );
    }
}

#[test]
fn numeric_and_pointer_macro_values_roundtrip() {
    let snapshot = capture(
        [Input::new(
            "values.hpp",
            include_str!("../input/constants.h"),
        )],
        ARGS,
        &[
            "NEGATIVE",
            "UNSIGNED",
            "FLOAT",
            "DOUBLE",
            "POINTER",
            "BOOLEAN",
            "WHOLE",
            "NEGATIVE_ZERO",
        ],
    )
    .unwrap();
    let index = compile(
        "constants",
        &snapshot.resolve().unwrap().project(&options()).unwrap(),
    );
    use windows_metadata::Value;
    for (name, value) in [
        ("NEGATIVE", Value::I32(-42)),
        ("UNSIGNED", Value::U32(u32::MAX)),
        ("FLOAT", Value::F32(1.25)),
        ("WHOLE", Value::F32(2.0)),
        ("DOUBLE", Value::F64(2.5)),
        ("BOOLEAN", Value::Bool(true)),
    ] {
        let Item::Const(field) = index.expect_item("Test", name) else {
            panic!()
        };
        assert_eq!(field.constant().unwrap().value(), value);
    }
    let Item::Const(field) = index.expect_item("Test", "POINTER") else {
        panic!()
    };
    assert_eq!(field.ty(), Type::PtrMut(Box::new(Type::I32), 1));
    assert_eq!(field.constant().unwrap().value(), Value::I32(0));
    let Item::Const(field) = index.expect_item("Test", "NEGATIVE_ZERO") else {
        panic!()
    };
    let Value::F64(value) = field.constant().unwrap().value() else {
        panic!()
    };
    assert_eq!(value.to_bits(), (-0.0_f64).to_bits());
}
