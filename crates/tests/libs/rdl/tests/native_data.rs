use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_data_{name}.winmd"))
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

fn data<'a>(index: &'a Index, namespace: &str, name: &str) -> Field<'a> {
    let Item::Data(field) = index.expect_item(namespace, name) else {
        panic!("expected imported data: {namespace}.{name}");
    };
    assert!(field.is_imported_data());
    assert_eq!(
        field.flags(),
        FieldAttributes::Public | FieldAttributes::Static
    );
    assert!(field.constant().is_none());
    field
}

fn check(path: &Path, data_namespace: &str, type_namespace: &str) {
    let index = Index::read(path).unwrap();
    index.validate_native_contracts(None).unwrap();
    let renamed = data(&index, data_namespace, "Renamed");
    assert_eq!(renamed.ty(), Type::value_named(type_namespace, "Alias"));
    assert_eq!(
        renamed.data_import().unwrap(),
        Some(NativeDataImport {
            dll: "fixture.dll".into(),
            target: NativeDataImportTarget::Name("DifferentExport".into()),
            kind: NativeDataImportKind::Data,
        })
    );
    assert_eq!(
        data(&index, data_namespace, "Readonly").ty(),
        Type::Const(Box::new(Type::Volatile(Box::new(Type::I32))))
    );
    assert_eq!(
        data(&index, data_namespace, "Readonly")
            .data_import()
            .unwrap()
            .unwrap()
            .kind,
        NativeDataImportKind::Data
    );
    let mutable = data(&index, data_namespace, "MutableConstant");
    assert_eq!(mutable.ty(), Type::U32);
    assert_eq!(
        mutable.data_import().unwrap().unwrap().kind,
        NativeDataImportKind::Constant
    );
    for (name, ordinal) in [("OrdinalZero", 0), ("OrdinalMax", 65535)] {
        assert_eq!(
            data(&index, data_namespace, name)
                .data_import()
                .unwrap()
                .unwrap()
                .target,
            NativeDataImportTarget::Ordinal(ordinal)
        );
    }
    assert_eq!(
        data(&index, data_namespace, "SpelledOrdinal")
            .data_import()
            .unwrap()
            .unwrap()
            .target,
        NativeDataImportTarget::Name("#17".into())
    );
    assert_eq!(
        data(&index, data_namespace, "Array")
            .data_import()
            .unwrap()
            .unwrap()
            .dll,
        "Fixture.DLL"
    );
    assert_eq!(
        data(&index, data_namespace, "Array").ty(),
        Type::ArrayFixed(Box::new(Type::U16), 4)
    );
    let pointer = data(&index, data_namespace, "Pointer");
    assert_eq!(
        pointer.ty(),
        Type::PtrConst(
            Box::new(Type::PtrMut(
                Box::new(Type::value_named(type_namespace, "Record")),
                1
            )),
            1
        )
    );
    let evidence = pointer
        .find_attribute("NativeAnnotationAttribute")
        .unwrap()
        .value();
    assert_eq!(evidence[2].1, Value::Utf8("__declspec(dllimport)".into()));
    assert_eq!(
        data(&index, data_namespace, "Forward").ty(),
        Type::value_named(type_namespace, "Tag")
    );
}

#[test]
fn reviewed_declarations_roundtrip_without_values_or_local_storage() {
    let reference = vocabulary("reviewed");
    let source = include_str!("../native_data.rdl");
    let input = output("reviewed");
    compile(source, &input, &reference);
    check(&input, "Test", "Test");
    let index = Index::read(&input).unwrap();
    let Item::Const(initialized) = index.expect_item("Test", "Initialized") else {
        panic!()
    };
    assert_eq!(initialized.constant().unwrap().value(), Value::U32(7));
    assert!(initialized.data_import().unwrap().is_none());
    let Item::Const(missing) = index.expect_item("Test", "MissingInitializer") else {
        panic!()
    };
    assert!(missing.flags().contains(FieldAttributes::Literal));
    assert!(missing.constant().is_none());
    assert!(missing.data_import().unwrap().is_none());
    let Item::Fn(function) = index.expect_item("Test", "Companion") else {
        panic!()
    };
    assert_eq!(function.impl_map().unwrap().import_name(), "NativeFunction");
    let rdl = input.with_extension("rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(&rdl).unwrap(),
        source.replace("\r\n", "\n")
    );
    let dir = Path::new(env!("OUT_DIR")).join("native_data_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();
    let roundtrip = dir.join(input.file_name().unwrap());
    compile(
        &std::fs::read_to_string(&rdl).unwrap(),
        &roundtrip,
        &reference,
    );
    assert_eq!(
        std::fs::read(&input).unwrap(),
        std::fs::read(&roundtrip).unwrap()
    );
    let merged = output("merged");
    merge().input(&input).output(&merged).merge().unwrap();
    check(&merged, "Test", "Test");
    let routed = output("routed");
    let mut remapper = merge::Remapper::new();
    remapper
        .input(&merged)
        .source("Test")
        .fallback("Types")
        .output(&routed);
    for name in [
        "Array",
        "Forward",
        "MutableConstant",
        "OrdinalMax",
        "OrdinalZero",
        "Pointer",
        "Readonly",
        "Renamed",
        "SpelledOrdinal",
    ] {
        remapper.route(name, "Data");
    }
    remapper.remap().unwrap();
    check(&routed, "Data", "Types");
    let routed_rdl = routed.with_extension("rdl");
    windows_rdl::writer()
        .input(&routed)
        .output(&routed_rdl)
        .write()
        .unwrap();
    let compiled_routed = output("routed_roundtrip");
    compile(
        &std::fs::read_to_string(routed_rdl).unwrap(),
        &compiled_routed,
        &reference,
    );
    check(&compiled_routed, "Data", "Types");
    let inventory =
        windows_rdl::inventory(Path::new(env!("CARGO_MANIFEST_DIR")).join("native_data.rdl"))
            .unwrap();
    let entry = inventory
        .iter()
        .find(|entry| entry.name == "Readonly")
        .unwrap();
    assert_eq!(entry.kind, "data");
    assert!(entry.shape.contains("Const"));
    assert!(entry.attributes.contains("kind = data"));
}

#[test]
fn malformed_declarations_reject_before_output_replacement() {
    let reference = vocabulary("invalid");
    for (item, diagnostic) in [
        ("static Value: u32;", "requires an import"),
        (
            "#[import(\"a.dll\", name=\"x\", kind=data)] static Value: u32 = 0;",
            "expected `;`",
        ),
        (
            "#[import(\"a.dll\", name=\"x\", kind=data)] static mut Value: u32;",
            "expected identifier",
        ),
        (
            "#[import(\"a.dll\", kind=data)] static Value: u32;",
            "exactly one",
        ),
        (
            "#[import(\"a.dll\", name=\"x\", ordinal=1, kind=data)] static Value: u32;",
            "exactly one",
        ),
        (
            "#[import(\"a.dll\", name=\"x\")] static Value: u32;",
            "kind is required",
        ),
        (
            "#[import(\"a.dll\", name=\"x\", kind=code)] static Value: u32;",
            "data or constant",
        ),
        (
            "#[import(\"a.dll\", name=\"\", kind=data)] static Value: u32;",
            "must not be empty",
        ),
        (
            "#[import(\"\", name=\"x\", kind=data)] static Value: u32;",
            "nonempty DLL",
        ),
        (
            "#[import(\"a.dll\", ordinal=65536, kind=data)] static Value: u32;",
            "fit u16",
        ),
        (
            "#[import(\"a.dll\", name=\"x\", name=\"y\", kind=data)] static Value: u32;",
            "duplicate",
        ),
        (
            "#[import(\"a.dll\", name=\"x\", kind=data)] #[import(\"b.dll\", name=\"y\", kind=data)] static Value: u32;",
            "duplicate",
        ),
        (
            "#[import(\"a.dll\", name=\"x\", kind=data)] static Value: void;",
            "nonvoid",
        ),
        (
            "type Alias = void; #[import(\"a.dll\", name=\"x\", kind=data)] static Value: Alias;",
            "nonvoid",
        ),
        (
            "#[import(\"a.dll\", name=\"x\", kind=data)] static Value: String;",
            "nonvoid",
        ),
    ] {
        let path = output("invalid");
        std::fs::write(&path, b"last good output").unwrap();
        let error = windows_rdl::reader()
            .reference(&reference)
            .input_text(&format!("#[win32] mod Test {{ {item} }}"))
            .output(&path)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{item}: {error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"last good output");
    }
    let path = output("winrt");
    std::fs::write(&path, b"last good output").unwrap();
    let error = windows_rdl::reader()
        .reference(&reference)
        .input_text(
            "#[winrt] mod Test { #[import(\"a.dll\", name=\"x\", kind=data)] static Value: u32; }",
        )
        .output(&path)
        .write()
        .unwrap_err();
    assert!(error.to_string().contains("not valid in WinRT"), "{error}");
    assert_eq!(std::fs::read(path).unwrap(), b"last good output");

    let path = output("winrt_target");
    std::fs::write(&path, b"last good output").unwrap();
    let error = windows_rdl::reader()
        .reference(&reference)
        .input_text(
            "#[winrt] mod Managed { struct Record { value: u32, } }
            #[win32] mod Test { type Alias = Managed::Record;
            #[import(\"a.dll\", name = \"x\", kind = data)] static Value: Alias; }",
        )
        .output(&path)
        .write()
        .unwrap_err();
    assert!(
        error.to_string().contains("type cannot be WinRT"),
        "{error}"
    );
    assert_eq!(std::fs::read(path).unwrap(), b"last good output");
}

#[test]
fn architecture_specific_routing_and_external_nominal_types_survive() {
    let reference = vocabulary("architecture");
    let external = output("external");
    compile(
        "#[win32] mod External { struct Record { value: i32, } }",
        &external,
        &reference,
    );
    let input = output("architecture");
    windows_rdl::reader().reference(&reference).reference(&external).input_text(
        "#[win32] mod Test {
        #[arch(X86)] #[import(\"x86.dll\", name=\"_Value\", kind=data)] static Value: External::Record;
        #[arch(X64)] #[import(\"x64.dll\", ordinal=19, kind=constant)] static Value: Const<External::Record>;
        }").output(&input).write().unwrap();
    let merged = output("architecture_merged");
    merge().input(&input).output(&merged).merge().unwrap();
    let remapped = output("architecture_remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .fallback("Routed")
        .output(&remapped)
        .remap()
        .unwrap();
    for (architecture, dll, target, ty) in [
        (
            1,
            "x86.dll",
            NativeDataImportTarget::Name("_Value".into()),
            Type::value_named("External", "Record"),
        ),
        (
            2,
            "x64.dll",
            NativeDataImportTarget::Ordinal(19),
            Type::Const(Box::new(Type::value_named("External", "Record"))),
        ),
    ] {
        let index = Index::new_for_architecture(vec![File::read(&remapped).unwrap()], architecture);
        let field = data(&index, "Routed", "Value");
        let contract = field.data_import().unwrap().unwrap();
        assert_eq!(contract.dll, dll);
        assert_eq!(contract.target, target);
        assert_eq!(field.ty(), ty);
    }
    let rdl = remapped.with_extension("rdl");
    windows_rdl::writer()
        .input(&remapped)
        .output(&rdl)
        .write()
        .unwrap();
    let rendered = std::fs::read_to_string(&rdl).unwrap();
    assert!(rendered.contains("#[arch(X86)]"));
    assert!(rendered.contains("#[arch(X64)]"));
    windows_rdl::reader()
        .reference(&reference)
        .reference(&external)
        .input(&rdl)
        .output(output("architecture_roundtrip"))
        .write()
        .unwrap();
}

#[test]
fn architecture_merge_compares_import_kind_and_target_without_inventing_values() {
    let reference = vocabulary("arch_merge");
    let x86 = output("arch_merge_x86");
    let x64 = output("arch_merge_x64");
    for (path, kind, target) in [
        (&x86, "data", "\"x86.dll\", name = \"_NativeValue\""),
        (&x64, "constant", "\"x64.dll\", ordinal = 41"),
    ] {
        compile(
            &format!(
                "#[win32] mod Test {{
                #[import(\"same.dll\", name = \"Shared\", kind = data)] static Shared: i32;
                #[import(\"same.dll\", name = \"Kind\", kind = {kind})] static Kind: i32;
                #[import({target}, kind = data)] static Route: i32;
                #[Windows::Win32::Metadata::NativeAnnotation(\"CXX\", 0, \"{kind}\")]
                #[import(\"same.dll\", name = \"Evidence\", kind = data)] static Evidence: i32;
                #[arch(X86)] #[import(\"same.dll\", name = \"OnlyX86\", kind = data)] static OnlyX86: i32;
                }}"
            ),
            path,
            &reference,
        );
    }
    let merged = output("arch_merge");
    merge()
        .arch_input(&x86, 1)
        .arch_input(&x64, 2)
        .output(&merged)
        .merge()
        .unwrap();
    let all = Index::read(&merged).unwrap();
    assert_eq!(data(&all, "Test", "Shared").arches(), 0);
    assert_eq!(all.get_item("Test", "Kind").count(), 2);
    assert_eq!(all.get_item("Test", "Route").count(), 2);
    assert_eq!(all.get_item("Test", "Evidence").count(), 2);
    assert_eq!(data(&all, "Test", "OnlyX86").arches(), 1);
    for (architecture, kind, dll, target) in [
        (
            1,
            NativeDataImportKind::Data,
            "x86.dll",
            NativeDataImportTarget::Name("_NativeValue".into()),
        ),
        (
            2,
            NativeDataImportKind::Constant,
            "x64.dll",
            NativeDataImportTarget::Ordinal(41),
        ),
    ] {
        let index = Index::new_for_architecture(vec![File::read(&merged).unwrap()], architecture);
        assert_eq!(
            data(&index, "Test", "Kind")
                .data_import()
                .unwrap()
                .unwrap()
                .kind,
            kind
        );
        let route = data(&index, "Test", "Route")
            .data_import()
            .unwrap()
            .unwrap();
        assert_eq!(route.dll, dll);
        assert_eq!(route.target, target);
    }
    let union = output("arch_merge_union");
    merge()
        .input(&x86)
        .input(&x64)
        .union_enums()
        .output(&union)
        .merge()
        .unwrap();
    let index = Index::read(&union).unwrap();
    assert_eq!(index.get_item("Test", "Shared").count(), 1);
    for name in ["Kind", "Route", "Evidence"] {
        assert_eq!(index.get_item("Test", name).count(), 2);
    }
}

#[test]
fn native_interface_and_callback_identities_remain_reference_types() {
    let reference = vocabulary("native_references");
    let input = output("native_references");
    compile(
        "#[win32] mod Test {
        #[no_guid] interface IObject {}
        extern \"C\" fn Callback(value: i32);
        #[import(\"a.dll\", name = \"Object\", kind = data)] static Object: IObject;
        #[import(\"a.dll\", name = \"Handler\", kind = data)] static Handler: Callback;
        #[import(\"a.dll\", name = \"VoidPointer\", kind = data)] static VoidPointer: *mut void;
        }",
        &input,
        &reference,
    );
    let index = Index::read(&input).unwrap();
    assert_eq!(
        data(&index, "Test", "Object").ty(),
        Type::class_named("Test", "IObject")
    );
    assert_eq!(
        data(&index, "Test", "Handler").ty(),
        Type::class_named("Test", "Callback")
    );
    assert_eq!(
        data(&index, "Test", "VoidPointer").ty(),
        Type::PtrMut(Box::new(Type::Void), 1)
    );
    let rdl = input.with_extension("rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    let roundtrip = output("native_references_roundtrip");
    compile(
        &std::fs::read_to_string(rdl).unwrap(),
        &roundtrip,
        &reference,
    );
    let index = Index::read(&roundtrip).unwrap();
    assert_eq!(
        data(&index, "Test", "Object").ty(),
        Type::class_named("Test", "IObject")
    );
    assert_eq!(
        data(&index, "Test", "Handler").ty(),
        Type::class_named("Test", "Callback")
    );
}
