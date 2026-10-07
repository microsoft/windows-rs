use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("overloads_{name}"))
}

fn compile(source: &str, name: &str, reference: bool) -> PathBuf {
    let path = output(name);
    let mut reader = windows_rdl::reader();
    reader.input_text(source).output(&path);
    if reference {
        reader.reference_default();
    }
    reader.write().unwrap();
    path
}

fn attributes<'a>(item: impl HasAttributes<'a>) -> Vec<(String, String, Vec<(String, Value)>)> {
    let mut attributes: Vec<_> = item
        .attributes()
        .map(|attr| {
            (
                attr.namespace().to_string(),
                attr.name().to_string(),
                attr.value(),
            )
        })
        .collect();
    attributes.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
    attributes
}

fn assert_interface(left: TypeDef, right: TypeDef) {
    assert_eq!(attributes(left), attributes(right));
    let generics: Vec<_> = left
        .generic_params()
        .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
        .collect();
    assert_eq!(left.methods().len(), right.methods().len());
    for (left, right) in left.methods().zip(right.methods()) {
        assert_eq!(left.name(), right.name());
        assert_eq!(left.flags(), right.flags());
        let a = left.signature(&generics);
        let b = right.signature(&generics);
        assert_eq!(a.flags, b.flags);
        assert_eq!(a.types, b.types);
        assert_eq!(a.return_type, b.return_type);
        assert_eq!(attributes(left), attributes(right));
        assert_eq!(left.params().len(), right.params().len());
        for (left, right) in left.params().zip(right.params()) {
            assert_eq!(left.name(), right.name());
            assert_eq!(left.sequence(), right.sequence());
            assert_eq!(left.flags(), right.flags());
            assert_eq!(attributes(left), attributes(right));
        }
    }
}

#[test]
fn raw_and_intrinsic_overloads_are_equivalent() {
    let source = include_str!("../input/interface_overloads.rdl");
    let path = compile(source, "intrinsic.winmd", false);
    let index = Index::read(&path).unwrap();
    let interface = index.expect("Test", "IReader");
    let expected = [
        ("Read", None),
        ("Read", Some("ReadWithOptions")),
        ("Read", Some("Read3")),
        ("Bind", Some("Bind")),
        ("Convert", Some("FromInt")),
        ("Convert", Some("FromString")),
        ("type", Some("match")),
    ];
    for ((logical, abi), method) in expected.iter().zip(interface.methods()) {
        assert_eq!(method.name(), *logical);
        let overload = method.find_attribute("OverloadAttribute");
        if let Some(abi) = abi {
            assert_eq!(
                overload.unwrap().value(),
                vec![(String::new(), Value::Utf8(abi.to_string()))]
            );
        } else {
            assert!(overload.is_none());
        }
    }
    let raw_path = compile(include_str!("overloads_raw.rdl"), "raw.winmd", true);
    let raw_index = Index::read(&raw_path).unwrap();
    assert_interface(interface, raw_index.expect("Test", "IReader"));
    let canonical = output("canonical.rdl");
    windows_rdl::writer()
        .input(&raw_path)
        .output(&canonical)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(canonical).unwrap(),
        windows_rdl::formatter::format(source)
    );
}

#[test]
fn invalid_intrinsic_attributes_are_rejected() {
    for (name, source, message) in [
        ("missing", "#[overload]", "single method name"),
        ("empty", "#[overload()]", "single method name"),
        ("string", "#[overload(\"Read\")]", "single method name"),
        ("multiple", "#[overload(Read, Other)]", "single method name"),
        (
            "duplicate",
            "#[overload(Read)] #[overload(Read)]",
            "duplicate `overload`",
        ),
        (
            "special",
            "#[overload(Read)] #[special]",
            "cannot be combined with `special`",
        ),
        (
            "conflict",
            "#[overload(Read)] #[Windows::Foundation::Metadata::Overload(\"Other\")]",
            "cannot be combined with `OverloadAttribute`",
        ),
        (
            "imported_conflict",
            "#[overload(Read)] #[Overload(\"Other\")]",
            "cannot be combined with `OverloadAttribute`",
        ),
    ] {
        let source = format!(
            "use Windows::Foundation::Metadata::*; #[winrt] mod Test {{ interface I {{ {source} fn ReadWithOptions(&self); }} }}"
        );
        let err = windows_rdl::reader()
            .input_text(&source)
            .reference_default()
            .output(output(&format!("invalid_{name}.winmd")))
            .write()
            .unwrap_err();
        assert!(err.to_string().contains(message), "{err}");
    }
    for (name, source) in [
        (
            "win32",
            "#[win32] mod Test { interface I { #[overload(Read)] fn ReadWithOptions(&self); } }",
        ),
        (
            "property",
            "#[winrt] mod Test { interface I { #[overload(Read)] Value: u32; } }",
        ),
        (
            "event",
            "#[winrt] mod Test { interface I { #[overload(Read)] event Changed: Handler; } }",
        ),
        (
            "module",
            "#[overload(Read)] #[winrt] mod Test { interface I {} }",
        ),
        (
            "nested_module",
            "#[winrt] mod Test { #[overload(Read)] mod Child {} }",
        ),
        (
            "interface",
            "#[winrt] mod Test { #[overload(Read)] interface I {} }",
        ),
        (
            "parameter",
            "#[winrt] mod Test { interface I { fn Read(&self, #[overload(Read)] value: u32); } }",
        ),
        (
            "return",
            "#[winrt] mod Test { interface I { fn Read(&self) -> #[overload(Read)] u32; } }",
        ),
    ] {
        let err = windows_rdl::reader()
            .input_text(source)
            .output(output(&format!("invalid_{name}.winmd")))
            .write()
            .unwrap_err();
        assert!(
            err.to_string().contains("only supported on WinRT methods"),
            "{err}"
        );
    }
}

#[test]
fn custom_attributes_and_default_overload_are_preserved() {
    let source = r#"
#[winrt]
mod Test {
    attribute OverloadAttribute { fn(value: String); }
    interface I {
        #[Overload("vendor annotation")]
        #[overload(Convert)]
        fn FromInt(&self, value: i32);
        #[Windows::Foundation::Metadata::DefaultOverload]
        #[overload(Convert)]
        fn FromString(&self, value: String);
    }
}
"#;
    let path = compile(source, "custom.winmd", true);
    let before = Index::read(&path).unwrap();
    let rdl = output("custom.rdl");
    windows_rdl::writer()
        .input(&path)
        .output(&rdl)
        .write()
        .unwrap();
    let text = std::fs::read_to_string(rdl).unwrap();
    assert!(text.contains("#[Overload(\"vendor annotation\")]"));
    assert!(text.contains("#[Windows::Foundation::Metadata::DefaultOverload]"));
    let after = Index::read(compile(&text, "custom_roundtrip.winmd", true)).unwrap();
    assert_interface(before.expect("Test", "I"), after.expect("Test", "I"));
}

fn metadata_fixture(winrt: bool, special: bool, namespace: &str, values: &[Value]) -> Vec<u8> {
    let mut file = writer::File::new("overloads");
    let mut flags = TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract;
    if winrt {
        flags |= TypeAttributes::WindowsRuntime;
    }
    file.TypeDef("Test", "I", writer::TypeDefOrRef::default(), flags);
    let mut flags =
        MethodAttributes::Public | MethodAttributes::Virtual | MethodAttributes::Abstract;
    if special {
        flags |= MethodAttributes::SpecialName;
    }
    let method = file.MethodDef(
        "Read",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            ..Default::default()
        },
        flags,
        Default::default(),
    );
    for value in values {
        let parent = writer::MemberRefParent::TypeRef(file.TypeRef(namespace, "OverloadAttribute"));
        let ctor = file.MemberRef(
            ".ctor",
            &Signature {
                flags: MethodCallAttributes::HASTHIS,
                types: vec![value.ty()],
                ..Default::default()
            },
            parent,
        );
        file.Attribute(
            writer::HasAttribute::MethodDef(method),
            writer::AttributeType::MemberRef(ctor),
            &[(String::new(), value.clone())],
        );
    }
    file.into_stream()
}

#[test]
fn writer_checks_standard_attribute_shape() {
    for (name, values, message) in [
        (
            "wrong_type",
            vec![Value::I32(1)],
            "invalid OverloadAttribute",
        ),
        (
            "empty_name",
            vec![Value::Utf8(String::new())],
            "invalid OverloadAttribute",
        ),
        (
            "duplicate",
            vec![Value::Utf8("A".into()), Value::Utf8("B".into())],
            "duplicate OverloadAttribute",
        ),
    ] {
        let bytes = metadata_fixture(true, false, "Windows.Foundation.Metadata", &values);
        let err = windows_rdl::writer()
            .input_bytes(&bytes)
            .output(output(&format!("{name}.rdl")))
            .write()
            .unwrap_err();
        assert!(err.to_string().contains(message), "{err}");
    }
}

#[test]
fn writer_keeps_raw_attributes_outside_intrinsic_domain() {
    for (name, winrt, special, namespace, abi) in [
        (
            "win32",
            false,
            false,
            "Windows.Foundation.Metadata",
            "ReadUnique",
        ),
        (
            "special",
            true,
            true,
            "Windows.Foundation.Metadata",
            "ReadUnique",
        ),
        ("custom_namespace", true, false, "Vendor", "ReadUnique"),
        (
            "unrepresentable",
            true,
            false,
            "Windows.Foundation.Metadata",
            "self",
        ),
    ] {
        let bytes = metadata_fixture(winrt, special, namespace, &[Value::Utf8(abi.to_string())]);
        let rdl = output(&format!("{name}.rdl"));
        windows_rdl::writer()
            .input_bytes(&bytes)
            .output(&rdl)
            .write()
            .unwrap();
        let text = std::fs::read_to_string(rdl).unwrap();
        assert!(!text.contains("#[overload("));
        assert!(text.contains(&format!("Overload(\"{abi}\")")));
        assert!(text.contains("fn Read("));
    }
}

#[test]
fn sdk_overloads_preserve_metadata() {
    let original = Index::read(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../libs/default/Windows.winmd"),
    )
    .unwrap();
    for (namespace, name) in [
        ("Windows.AI.MachineLearning", "ILearningModelBinding"),
        ("Windows.AI.Actions.Hosting", "IActionCatalog2"),
    ] {
        let rdl = output(&format!("{name}.rdl"));
        windows_rdl::writer()
            .input_default()
            .filter(&format!("{namespace}.{name}"))
            .output(&rdl)
            .write()
            .unwrap();
        let source = std::fs::read_to_string(&rdl).unwrap();
        assert!(source.contains("#[overload("));
        let index = Index::read(compile(&source, &format!("{name}.winmd"), true)).unwrap();
        assert_interface(
            original.expect(namespace, name),
            index.expect(namespace, name),
        );
    }
}
