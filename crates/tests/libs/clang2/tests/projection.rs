use windows_clang2::{Input, Plan, ProjectionOptions, ReferenceKind, TypeReference, capture};
use windows_metadata::{
    Type,
    reader::{Index, Item},
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
        assert_eq!(
            constant.constant().unwrap().value(),
            windows_metadata::Value::I32(42)
        );
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

#[test]
fn real_iunknown_projects_through_a_checked_external_binding() {
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
    let snapshot = capture(
        [Input::new(
            "wrapper.hpp",
            "#include <unknwnbase.h>\nextern \"C\" void Use(IUnknown* input, IUnknown** output);",
        )],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-include",
            sal.to_str().unwrap(),
            "-isystem",
            shared.to_str().unwrap(),
            "-isystem",
            um.to_str().unwrap(),
        ],
        &["Use"],
    )
    .unwrap();
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
            windows_metadata::reader::ParamDirection::Output
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
    assert!(base.extends().is_none());
    let base_methods: Vec<_> = base.methods().map(|method| method.name()).collect();
    assert_eq!(base_methods, ["Query", "Count"]);

    let Item::Type(derived) = index.expect_item("Test", "IDerived") else {
        panic!()
    };
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

    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [Type::class_named("Test", "IDerived")]
    );
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
