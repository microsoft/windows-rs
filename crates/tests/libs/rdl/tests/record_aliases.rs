use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("record_aliases_{name}.winmd"))
}

fn compile(source: &str, output: &Path) {
    windows_rdl::reader()
        .input_text(source)
        .output(output)
        .write()
        .unwrap();
}

fn assert_alias(index: &Index, namespace: &str, name: &str, target: Type) {
    let alias = index.expect(namespace, name);
    let marker = alias
        .attributes()
        .find(|attr| {
            attr.namespace() == "Windows.Win32.Metadata" && attr.name() == "NativeTypedefAttribute"
        })
        .unwrap();
    assert!(marker.value().is_empty());
    assert_eq!(alias.category(), TypeCategory::Struct);
    assert!(!alias.is_opaque());
    assert!(alias.class_layout().is_none());
    assert_eq!(alias.fields().len(), 1);
    let field = alias.fields().next().unwrap();
    assert_eq!(field.name(), "Value");
    assert_eq!(field.flags(), FieldAttributes::Public);
    assert_eq!(field.ty(), target);
}

fn assert_metadata(path: &Path, routed: bool) {
    let index = Index::read(path).unwrap();
    let (records, aliases, other, consumers) = if routed {
        (
            "Native.Records",
            "Native.Aliases",
            "Native.Other",
            "Native.Consumers",
        )
    } else {
        ("Test", "Test", "Test", "Test")
    };
    assert_eq!(index.types().count(), 15);
    let record = Type::value_named(records, "Record");
    let alias = |name| Type::value_named(aliases, name);
    for (name, target) in [
        ("AliasA", record.clone()),
        ("AliasB", record),
        ("AliasChain", alias("AliasA")),
        ("ConstPointer", Type::PtrConst(Box::new(alias("AliasB")), 1)),
        ("Pointer", Type::PtrMut(Box::new(alias("AliasA")), 1)),
        ("LookAlias", Type::value_named(other, "Lookalike")),
        ("UnionAlias", Type::value_named(records, "VALUE")),
    ] {
        assert_alias(&index, aliases, name, target);
    }
    let record = index.expect(records, "Record");
    let lookalike = index.expect(other, "Lookalike");
    for definition in [record, lookalike] {
        assert!(!definition.has_attribute("NativeTypedefAttribute"));
        assert!(!definition.is_opaque());
        assert_eq!(definition.class_layout().unwrap().packing_size(), 1);
        assert_eq!(
            definition
                .fields()
                .map(|field| (field.name(), field.ty()))
                .collect::<Vec<_>>(),
            [("key", Type::U16), ("value", Type::U32)],
        );
    }
    assert_ne!(record.full_name(), lookalike.full_name());
    let ordinary = index.expect(consumers, "Ordinary");
    assert!(!ordinary.has_attribute("NativeTypedefAttribute"));
    assert_eq!(ordinary.fields().len(), 1);
    assert_eq!(ordinary.fields().next().unwrap().name(), "Value");
    assert_eq!(
        ordinary.fields().next().unwrap().ty(),
        Type::value_named(records, "Record")
    );
    let union = index.expect(records, "VALUE");
    assert!(union.flags().contains(TypeAttributes::ExplicitLayout));
    assert!(!union.has_attribute("NativeTypedefAttribute"));
    assert_eq!(
        union
            .fields()
            .map(|field| (field.name(), field.ty()))
            .collect::<Vec<_>>(),
        [("integer", Type::U32), ("float", Type::F32)],
    );
    assert_eq!(
        index
            .expect(consumers, "Holder")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>(),
        [
            alias("AliasA"),
            alias("AliasB"),
            alias("AliasChain"),
            Type::ArrayFixed(Box::new(alias("AliasA")), 2),
            Type::PtrConst(Box::new(Type::PtrMut(Box::new(alias("AliasB")), 1)), 1),
            alias("Pointer"),
            Type::RefConst(Box::new(alias("AliasA"))),
            Type::RefMut(Box::new(alias("AliasB"))),
            alias("LookAlias"),
            alias("UnionAlias"),
        ],
    );
    for (owner, name, types, result) in [
        (
            "Callback",
            "Invoke",
            vec![alias("AliasA"), Type::PtrMut(Box::new(alias("AliasB")), 1)],
            alias("AliasChain"),
        ),
        (
            "Apis",
            "Export",
            vec![alias("AliasA"), Type::RefMut(Box::new(alias("AliasChain")))],
            alias("AliasB"),
        ),
        (
            "IPairs",
            "Exchange",
            vec![
                alias("AliasChain"),
                Type::PtrMut(Box::new(alias("LookAlias")), 1),
            ],
            alias("UnionAlias"),
        ),
    ] {
        let method = index
            .expect(consumers, owner)
            .methods()
            .find(|method| method.name() == name)
            .unwrap();
        let signature = method.signature(&[]);
        assert_eq!(signature.types, types);
        assert_eq!(signature.return_type, result);
        let params = method.params_by_sequence(2).unwrap();
        assert_eq!(
            params.params()[0].unwrap().direction(),
            ParamDirection::Input
        );
        assert_eq!(
            params.params()[1].unwrap().direction(),
            ParamDirection::Output
        );
    }
}

#[test]
fn reviewed_complete_record_aliases_roundtrip_without_copying_or_merging_records() {
    let fixture = include_str!("../record_aliases.rdl");
    let input = output("fixture");
    compile(fixture, &input);
    assert_metadata(&input, false);
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
    let dir = Path::new(env!("OUT_DIR")).join("record_aliases_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();
    let roundtrip = dir.join(input.file_name().unwrap());
    windows_rdl::reader()
        .input(&rdl)
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_metadata(&roundtrip, false);
    assert_eq!(
        std::fs::read(&input).unwrap(),
        std::fs::read(&roundtrip).unwrap()
    );
    let merged = output("merged");
    merge().input(&input).output(&merged).merge().unwrap();
    assert_metadata(&merged, false);
    let remapped = output("remapped");
    let mut remapper = merge::Remapper::new();
    remapper
        .input(&merged)
        .source("Test")
        .fallback("Native.Consumers");
    for name in [
        "AliasA",
        "AliasB",
        "AliasChain",
        "ConstPointer",
        "Pointer",
        "LookAlias",
        "UnionAlias",
    ] {
        remapper.route(name, "Native.Aliases");
    }
    remapper
        .route("Record", "Native.Records")
        .route("VALUE", "Native.Records")
        .route("Lookalike", "Native.Other")
        .output(&remapped)
        .remap()
        .unwrap();
    assert_metadata(&remapped, true);
}

#[test]
fn external_record_aliases_retain_targets_and_reference_scope() {
    let reference = output("reference");
    compile(
        "#[win32] mod Other { struct Record { value: u32 } type Existing = Record; }",
        &reference,
    );
    let input = output("external");
    windows_rdl::reader()
        .reference(&reference)
        .input_text(
            "#[win32] mod Test { type First = Other::Record; type Second = Other::Existing; \
         struct Holder { first: First, second: Second } }",
        )
        .output(&input)
        .write()
        .unwrap();
    let index = Index::read(&input).unwrap();
    assert_alias(
        &index,
        "Test",
        "First",
        Type::value_named("Other", "Record"),
    );
    assert_alias(
        &index,
        "Test",
        "Second",
        Type::value_named("Other", "Existing"),
    );
    assert_eq!(index.types().count(), 3);
    for name in ["First", "Second"] {
        let field = index.expect("Test", name).fields().next().unwrap();
        let mut blob = field.blob(2);
        assert_eq!(blob.read_u8(), 0x06);
        assert_eq!(blob.read_u8(), 0x11);
        let TypeDefOrRef::TypeRef(target) = blob.decode() else {
            panic!("expected TypeRef");
        };
        let ResolutionScope::AssemblyRef(assembly) = target.scope() else {
            panic!("expected external assembly scope");
        };
        assert_eq!(
            assembly.pos(),
            1,
            "external reference follows the System assembly"
        );
    }
    let rdl = input.with_extension("rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    let dir = Path::new(env!("OUT_DIR")).join("record_aliases_external_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();
    let roundtrip = dir.join(input.file_name().unwrap());
    windows_rdl::reader()
        .reference(&reference)
        .input(&rdl)
        .output(&roundtrip)
        .write()
        .unwrap();
    let index = Index::read(&roundtrip).unwrap();
    assert_alias(
        &index,
        "Test",
        "First",
        Type::value_named("Other", "Record"),
    );
    assert_alias(
        &index,
        "Test",
        "Second",
        Type::value_named("Other", "Existing"),
    );
    assert_eq!(
        std::fs::read(input).unwrap(),
        std::fs::read(roundtrip).unwrap()
    );
}

#[test]
fn malformed_typedef_payloads_are_rejected_before_clearing_outputs() {
    for count in [0, 2] {
        let mut file = writer::File::new("malformed_alias");
        let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
        let alias = file.TypeDef(
            "Test",
            "Alias",
            base,
            TypeAttributes::Public | TypeAttributes::SequentialLayout,
        );
        for position in 0..count {
            file.Field(
                &format!("Value{position}"),
                &Type::I32,
                FieldAttributes::Public,
            );
        }
        let marker = file.TypeRef("Windows.Win32.Metadata", "NativeTypedefAttribute");
        let ctor = file.MemberRef(
            ".ctor",
            &Signature {
                flags: MethodCallAttributes::HASTHIS,
                return_type: Type::Void,
                types: vec![],
            },
            writer::MemberRefParent::TypeRef(marker),
        );
        file.Attribute(
            writer::HasAttribute::TypeDef(alias),
            writer::AttributeType::MemberRef(ctor),
            &[],
        );
        let input = output(&format!("malformed_{count}"));
        std::fs::write(&input, file.into_stream()).unwrap();
        let expected = if count == 0 {
            "typedef `Alias` has no field"
        } else {
            "typedef `Alias` must have one target field"
        };
        for mode in ["file", "split", "partition"] {
            let dir = Path::new(env!("OUT_DIR")).join(format!("record_aliases_bad_{count}_{mode}"));
            std::fs::create_dir_all(&dir).unwrap();
            let sentinel = dir.join("existing.rdl");
            std::fs::write(&sentinel, b"last-good-output").unwrap();
            let mut writer = windows_rdl::writer();
            writer.input(&input);
            match mode {
                "file" => {
                    writer.output(&sentinel);
                }
                "split" => {
                    writer.output(&dir).split();
                }
                "partition" => {
                    writer.output(&dir).partition(
                        [(String::from("Alias"), String::from("alias"))]
                            .into_iter()
                            .collect(),
                    );
                }
                _ => unreachable!(),
            }
            let error = writer.write().unwrap_err();
            assert!(error.message.contains(expected), "{mode}: {error}");
            assert_eq!(std::fs::read(&sentinel).unwrap(), b"last-good-output");
            assert_eq!(std::fs::read_dir(dir).unwrap().count(), 1);
        }
    }
}

#[test]
fn neutral_aliases_resolve_architecture_specific_record_definitions() {
    let x86 = output("x86");
    let x64 = output("x64");
    for (path, ty) in [(&x86, "u32"), (&x64, "u64")] {
        compile(
            &format!(
                "#[win32] mod Test {{ struct Record {{ value: {ty} }} \
            type AliasA = Record; type AliasB = Record; type Chain = AliasA; }}"
            ),
            path,
        );
    }
    let merged = output("arch_merged");
    merge()
        .arch_input(&x86, 1)
        .arch_input(&x64, 2)
        .output(&merged)
        .merge()
        .unwrap();
    for (bits, ty) in [(1, Type::U32), (2, Type::U64)] {
        let index = Index::new_for_architecture(vec![File::read(&merged).unwrap()], bits);
        let record = index.expect("Test", "Record");
        assert_eq!(record.arches(), bits);
        assert_eq!(record.fields().next().unwrap().ty(), ty);
        for name in ["AliasA", "AliasB"] {
            assert_alias(&index, "Test", name, Type::value_named("Test", "Record"));
            assert_eq!(index.expect("Test", name).arches(), 0);
        }
        assert_alias(&index, "Test", "Chain", Type::value_named("Test", "AliasA"));
    }
}
