use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("opaque_{name}.winmd"))
}

fn compile(source: &str, path: &Path) {
    windows_rdl::reader()
        .input_text(source)
        .output(path)
        .write()
        .unwrap();
}

fn assert_metadata(path: &Path, namespace: &str) {
    let index = Index::read(path).unwrap();
    index.validate_opaque_types(None).unwrap();
    for (name, union) in [("_OPAQUE", false), ("_UNION", true)] {
        let opaque = index.expect(namespace, name);
        assert!(opaque.is_opaque());
        assert_eq!(opaque.category(), TypeCategory::Struct);
        assert_eq!(
            opaque.flags().contains(TypeAttributes::ExplicitLayout),
            union
        );
        assert!(opaque.fields().next().is_none());
        assert!(opaque.class_layout().is_none());
        assert!(
            index
                .validate_opaque_storage(&Type::value_named(namespace, name))
                .is_err()
        );
    }
    let empty = index.expect(namespace, "Empty");
    assert!(!empty.is_opaque());
    index
        .validate_opaque_storage(&Type::value_named(namespace, "Empty"))
        .unwrap();
    for (name, target) in [("Alias", "_OPAQUE"), ("RecordAlias", "Alias")] {
        let alias = index.expect(namespace, name);
        assert!(alias.has_attribute("NativeTypedefAttribute"));
        assert_eq!(
            alias.fields().next().unwrap().ty(),
            Type::value_named(namespace, target)
        );
        assert!(
            index
                .validate_opaque_storage(&Type::value_named(namespace, name))
                .is_err()
        );
    }
    for name in ["PointerAlias", "PointerAlias2"] {
        index
            .validate_opaque_storage(&Type::value_named(namespace, name))
            .unwrap();
    }
    let expected = [
        Type::PtrMut(Box::new(Type::value_named(namespace, "_OPAQUE")), 1),
        Type::PtrConst(
            Box::new(Type::PtrMut(
                Box::new(Type::value_named(namespace, "_OPAQUE")),
                1,
            )),
            1,
        ),
        Type::ArrayFixed(
            Box::new(Type::PtrMut(
                Box::new(Type::value_named(namespace, "RecordAlias")),
                1,
            )),
            3,
        ),
        Type::RefConst(Box::new(Type::value_named(namespace, "Alias"))),
        Type::RefMut(Box::new(Type::value_named(namespace, "_OPAQUE"))),
        Type::PtrConst(Box::new(Type::value_named(namespace, "_UNION")), 1),
        Type::value_named(namespace, "Empty"),
    ];
    assert_eq!(
        index
            .expect(namespace, "Pointers")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>(),
        expected,
    );
    for (owner, method_name, types, result, directions) in [
        (
            "Callback",
            "Invoke",
            vec![Type::PtrConst(
                Box::new(Type::value_named(namespace, "Alias")),
                1,
            )],
            Type::PtrMut(Box::new(Type::value_named(namespace, "_OPAQUE")), 1),
            vec![ParamDirection::Input],
        ),
        (
            "Apis",
            "Export",
            vec![
                Type::value_named(namespace, "PointerAlias"),
                Type::PtrMut(
                    Box::new(Type::PtrConst(
                        Box::new(Type::value_named(namespace, "RecordAlias")),
                        1,
                    )),
                    1,
                ),
            ],
            Type::value_named(namespace, "PointerAlias2"),
            vec![ParamDirection::Input, ParamDirection::Output],
        ),
        (
            "IPointers",
            "Acquire",
            vec![Type::PtrMut(
                Box::new(Type::value_named(namespace, "PointerAlias")),
                1,
            )],
            Type::PtrMut(Box::new(Type::value_named(namespace, "_OPAQUE")), 1),
            vec![ParamDirection::Output],
        ),
    ] {
        let method = index
            .expect(namespace, owner)
            .methods()
            .find(|method| method.name() == method_name)
            .unwrap();
        let signature = method.signature(&[]);
        assert_eq!(signature.return_type, result);
        assert_eq!(signature.types, types);
        let params = method.params_by_sequence(types.len()).unwrap();
        assert_eq!(
            params
                .params()
                .iter()
                .map(|param| param.unwrap().direction())
                .collect::<Vec<_>>(),
            directions,
        );
        for ty in signature.types.iter().chain([&signature.return_type]) {
            index.validate_opaque_storage(ty).unwrap();
        }
    }
}

#[test]
fn reviewed_opaque_nominal_types_roundtrip_and_survive_merge_and_remap() {
    let fixture = include_str!("../opaque.rdl");
    let input = output("fixture");
    compile(fixture, &input);
    assert_metadata(&input, "Test");
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
    let dir = Path::new(env!("OUT_DIR")).join("opaque_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();
    let roundtrip = dir.join(input.file_name().unwrap());
    windows_rdl::reader()
        .input(&rdl)
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_metadata(&roundtrip, "Test");
    assert_eq!(
        std::fs::read(&input).unwrap(),
        std::fs::read(&roundtrip).unwrap()
    );
    let merged = output("merged");
    merge().input(&input).output(&merged).merge().unwrap();
    assert_metadata(&merged, "Test");
    let remapped = output("remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .fallback("Routed")
        .output(&remapped)
        .remap()
        .unwrap();
    assert_metadata(&remapped, "Routed");
}

#[test]
fn storage_requiring_uses_reject_opaque_records_before_replacing_output() {
    for (name, item) in [
        ("field", "struct Holder { value: Tag }"),
        ("union_field", "union Holder { value: Tag }"),
        ("array", "struct Holder { value: [Tag; 2] }"),
        ("zero_array", "struct Holder { value: [Tag; 0] }"),
        ("pointer_array", "struct Holder { value: *mut [Tag; 2] }"),
        ("reference_array", "struct Holder { value: &mut [Tag; 2] }"),
        ("slice", "struct Holder { value: &[Tag] }"),
        (
            "alias_field",
            "type A = Tag; type B = A; struct Holder { value: B }",
        ),
        ("alias_array", "type A = [Tag; 2];"),
        (
            "input",
            "#[library(\"test.dll\")] extern \"C\" fn Use(value: Tag);",
        ),
        (
            "output",
            "#[library(\"test.dll\")] extern \"C\" fn Use() -> Tag;",
        ),
        ("callback", "extern \"C\" fn Callback(value: Tag);"),
        ("callback_output", "extern \"C\" fn Callback() -> Tag;"),
        (
            "interface",
            "#[no_guid] interface I { fn Use(&self, value: Tag); }",
        ),
        (
            "interface_output",
            "#[no_guid] interface I { fn Use(&self) -> Tag; }",
        ),
    ] {
        let path = output(name);
        std::fs::write(&path, b"last-good-output").unwrap();
        let source = format!("#[win32] mod Test {{ #[opaque] struct Tag {{}} {item} }}");
        let error = windows_rdl::reader()
            .input_text(&source)
            .output(&path)
            .write()
            .unwrap_err();
        assert!(
            error
                .message
                .contains("requires storage for opaque type `Test.Tag`"),
            "{name}: {error}"
        );
        assert_eq!(std::fs::read(path).unwrap(), b"last-good-output");
    }
}

#[test]
fn opaque_declarations_cannot_claim_fields_or_layout() {
    for (name, source) in [
        (
            "members",
            "#[win32] mod Test { #[opaque] struct Tag { value: i32 } }",
        ),
        (
            "union_members",
            "#[win32] mod Test { #[opaque] union Tag { value: i32 } }",
        ),
        (
            "packing",
            "#[win32] mod Test { #[opaque] #[packed(1)] struct Tag {} }",
        ),
        (
            "alignment",
            "#[win32] mod Test { #[opaque] #[align(8)] struct Tag {} }",
        ),
        ("winrt", "#[winrt] mod Test { #[opaque] struct Tag {} }"),
        (
            "arguments",
            "#[win32] mod Test { #[opaque(1)] struct Tag {} }",
        ),
        (
            "enum",
            "#[win32] mod Test { #[opaque] #[repr(i32)] enum Tag { A = 0 } }",
        ),
        ("alias", "#[win32] mod Test { #[opaque] type Tag = i32; }"),
        ("module", "#[opaque] mod Test { struct Tag {} }"),
        (
            "field_attribute",
            "#[win32] mod Test { struct Tag { #[opaque] value: i32 } }",
        ),
        (
            "parameter_attribute",
            "#[win32] mod Test { extern \"C\" fn F(#[opaque] value: i32); }",
        ),
        ("pending_forward", "#[win32] mod Test { struct Tag; }"),
        (
            "base",
            "#[win32] mod Test { #[opaque] struct Tag {} #[no_guid] interface I : Tag {} }",
        ),
        (
            "conflict",
            "#[win32] mod Test { #[opaque] struct Tag {} struct Tag {} }",
        ),
    ] {
        let path = output(name);
        let error = windows_rdl::reader()
            .input_text(source)
            .output(&path)
            .write()
            .unwrap_err();
        let expected = match name {
            "members" | "union_members" | "packing" | "alignment" | "winrt" => {
                "must be a native record without"
            }
            "arguments" => "`opaque` does not accept arguments",
            "pending_forward" => "expected curly braces",
            "base" => "interface base must resolve to an interface type",
            "conflict" => "has conflicting opaque and complete definitions",
            _ => "`opaque` is only valid on native records",
        };
        assert!(error.message.contains(expected), "{name}: {error}");
    }
}

#[test]
fn opaque_vocabulary_and_qualified_spelling_preserve_the_same_contract() {
    let source = "use Windows::Win32::Metadata::*;\n\
        #[win32] mod Test { #[Windows::Win32::Metadata::NativeOpaque] struct Tag {} }";
    let path = output("vocabulary");
    windows_rdl::reader()
        .input_text(include_str!("../../../../../metadata/metadata.rdl"))
        .input_text(source)
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    assert!(index.expect("Test", "Tag").is_opaque());
    let marker = index.expect("Windows.Win32.Metadata", "NativeOpaqueAttribute");
    assert_eq!(marker.category(), TypeCategory::Attribute);
    assert!(marker.fields().next().is_none());
    assert!(
        marker
            .methods()
            .next()
            .unwrap()
            .signature(&[])
            .types
            .is_empty()
    );
    let source = "use Windows::Win32::Metadata::*;\n\
        #[win32] mod Test { #[NativeOpaque] type Alias = i32; }";
    let error = windows_rdl::reader()
        .input_text(include_str!("../../../../../metadata/metadata.rdl"))
        .input_text(source)
        .output(&path)
        .write()
        .unwrap_err();
    assert!(
        error
            .message
            .contains("`opaque` is only valid on native records")
    );
    assert!(Index::read(path).unwrap().expect("Test", "Tag").is_opaque());
}

#[test]
fn external_opaque_aliases_keep_pointer_identity_and_reject_values() {
    let reference = output("reference");
    compile(
        "#[win32] mod Other { #[opaque] struct Tag {} type Alias = Tag; type Pointer = *mut Alias; }",
        &reference,
    );
    for (name, ty, valid) in [
        ("external_pointer", "*mut Other::Alias", true),
        ("external_pointer_alias", "Other::Pointer", true),
        ("external_value", "Other::Alias", false),
        ("external_array", "*mut [Other::Alias; 2]", false),
    ] {
        let source =
            format!("#[win32] mod Test {{ type Local = {ty}; struct Holder {{ value: Local }} }}");
        let path = output(name);
        let result = windows_rdl::reader()
            .input_text(&source)
            .reference(&reference)
            .output(&path)
            .write();
        if valid {
            result.unwrap();
            let index = Index::read(path).unwrap();
            assert!(
                index
                    .expect("Test", "Local")
                    .has_attribute("NativeTypedefAttribute")
            );
        } else {
            let error = result.unwrap_err();
            assert!(
                error
                    .message
                    .contains("requires storage for opaque type `Other.Tag`"),
                "{error}"
            );
        }
    }
}

#[test]
fn architecture_merge_keeps_opaque_and_empty_records_distinct() {
    let x86 = output("arch_x86");
    let x64 = output("arch_x64");
    compile("#[win32] mod Test { #[opaque] struct Tag {} }", &x86);
    compile("#[win32] mod Test { struct Tag {} }", &x64);
    let merged = output("arch_merged");
    merge()
        .arch_input(&x86, 1)
        .arch_input(&x64, 2)
        .output(&merged)
        .merge()
        .unwrap();
    for (bits, opaque) in [(1, true), (2, false)] {
        let index = Index::new_for_architecture(vec![File::read(&merged).unwrap()], bits);
        let tag = index.expect("Test", "Tag");
        assert_eq!(tag.arches(), bits);
        assert_eq!(tag.is_opaque(), opaque);
        assert_eq!(
            index
                .validate_opaque_storage(&Type::value_named("Test", "Tag"))
                .is_err(),
            opaque
        );
    }
}
