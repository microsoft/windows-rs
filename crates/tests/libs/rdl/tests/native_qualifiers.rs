use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_qualifiers_{name}.winmd"))
}

fn assert_metadata(path: &Path, namespace: &str) {
    let index = Index::read(path).unwrap();
    let record = Type::value_named(namespace, "Record");
    let input = Type::Const(Box::new(Type::PtrMut(
        Box::new(Type::Volatile(Box::new(record.clone()))),
        1,
    )));
    let result = Type::Volatile(Box::new(Type::PtrConst(Box::new(record.clone()), 1)));
    let fields = index.expect(namespace, "Pointers").fields();
    assert_eq!(
        fields.map(|field| field.ty()).collect::<Vec<_>>(),
        [
            Type::Const(Box::new(Type::PtrMut(Box::new(Type::I32), 1))),
            Type::Volatile(Box::new(Type::I32)),
            Type::PtrConst(
                Box::new(Type::Const(Box::new(Type::PtrMut(
                    Box::new(result.clone()),
                    1,
                )))),
                1,
            ),
            Type::ArrayFixed(Box::new(Type::Volatile(Box::new(record.clone()))), 2),
            Type::Const(Box::new(Type::ArrayFixed(Box::new(record.clone()), 2))),
            Type::RefMut(Box::new(Type::Volatile(Box::new(record.clone())))),
            Type::value_named(namespace, "TopConst"),
            Type::Const(Box::new(Type::Volatile(Box::new(record.clone())))),
            Type::Volatile(Box::new(Type::Const(Box::new(record.clone())))),
        ]
    );
    for (owner, name, count) in [
        ("Callback", "Invoke", 1),
        ("Apis", "Export", 2),
        ("IQualified", "Exchange", 2),
    ] {
        let method = index
            .expect(namespace, owner)
            .methods()
            .find(|method| method.name() == name)
            .unwrap();
        let signature = method.signature(&[]);
        assert_eq!(signature.return_type, result);
        assert_eq!(signature.types[0], input);
        let params = method.params_by_sequence(count).unwrap();
        assert_eq!(
            params.params()[0].unwrap().direction(),
            ParamDirection::Input
        );
        if count == 2 {
            assert_eq!(
                signature.types[1],
                Type::PtrMut(Box::new(Type::Const(Box::new(record.clone()))), 1),
            );
            assert_eq!(
                params.params()[1].unwrap().direction(),
                ParamDirection::Output
            );
        }
    }
    for (name, target) in [
        ("TopConst", Type::Const(Box::new(record.clone()))),
        (
            "TopVolatile",
            Type::Volatile(Box::new(Type::PtrMut(Box::new(record), 1))),
        ),
    ] {
        let alias = index.expect(namespace, name);
        assert!(alias.has_attribute("NativeTypedefAttribute"));
        assert_eq!(alias.fields().next().unwrap().ty(), target);
    }
}

#[test]
fn reviewed_native_qualifiers_roundtrip_and_remap_at_the_same_type_nodes() {
    let fixture = include_str!("../native_qualifiers.rdl");
    let input = output("fixture");
    windows_rdl::reader()
        .input_text(fixture)
        .output(&input)
        .write()
        .unwrap();
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
    let dir = Path::new(env!("OUT_DIR")).join("native_qualifiers_roundtrip");
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
fn qualified_values_do_not_bypass_opaque_storage_validation() {
    for ty in [
        "Const<Tag>",
        "Volatile<Tag>",
        "Const<Volatile<Tag>>",
        "*mut [Volatile<Tag>; 2]",
        "&Const<[Tag; 0]>",
    ] {
        let path = output("opaque_rejection");
        std::fs::write(&path, b"last good output").unwrap();
        let error = windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{ #[opaque] struct Tag {{}} struct Holder {{ value: {ty}, }} }}"
            ))
            .output(&path)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains("requires storage"), "{error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"last good output");
    }
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Test {
                #[opaque] struct Tag {}
                type Alias = Volatile<Tag>;
                struct Holder { value: Const<*mut Alias>, }
            }",
        )
        .output(output("opaque_pointer"))
        .write()
        .unwrap();
}

#[test]
fn native_qualifier_arity_and_winrt_uses_reject_before_output_replacement() {
    for (source, diagnostic) in [
        (
            "#[win32] mod Test { type Alias = Const<i32, u32>; }",
            "one type argument",
        ),
        (
            "#[win32] mod Test { type Alias = Volatile<3>; }",
            "one type argument",
        ),
        (
            "#[winrt] mod Test { struct Record { value: Const<u32>, } }",
            "not valid in WinRT",
        ),
        (
            "#[winrt] mod Test { interface IRecord { fn Read(&self, value: *mut Volatile<u32>); } }",
            "not valid in WinRT",
        ),
    ] {
        let path = output("invalid");
        std::fs::write(&path, b"last good output").unwrap();
        let error = windows_rdl::reader()
            .input_text(source)
            .output(&path)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"last good output");
    }
}

#[test]
fn native_qualifier_vocabulary_and_qualified_constants_compile() {
    let path = output("vocabulary");
    windows_rdl::reader()
        .input_text(include_str!("../../../../../metadata/metadata.rdl"))
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    for name in ["NativeConstAttribute", "NativeVolatileAttribute"] {
        let marker = index.expect("Windows.Win32.Metadata", name);
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
    }
    let path = output("constant");
    windows_rdl::reader()
        .input_text("#[win32] mod Test { const Count: Const<usize> = 1u64; }")
        .output(&path)
        .write()
        .unwrap();
    let rdl = path.with_extension("rdl");
    windows_rdl::writer()
        .input(&path)
        .output(&rdl)
        .write()
        .unwrap();
    assert!(
        std::fs::read_to_string(rdl)
            .unwrap()
            .contains("Const<usize> = 1u64;")
    );
}
