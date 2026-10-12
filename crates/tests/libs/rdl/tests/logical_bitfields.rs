use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("logical_bitfields_{name}.winmd"))
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

fn assert_metadata(path: &Path, records: &str, values: &str) {
    let index = Index::read(path).unwrap();
    index.validate_native_contracts(None).unwrap();
    let packet = index.expect(records, "Packet");
    let fields: Vec<_> = packet.fields().collect();
    assert_eq!(
        fields
            .iter()
            .map(|field| (field.name(), field.ty()))
            .collect::<Vec<_>>(),
        [
            ("prefix", Type::U8),
            ("bits", Type::U32),
            ("legacy", Type::U16),
            ("suffix", Type::U16)
        ]
    );
    let members = fields[1].bitfields().unwrap();
    assert_eq!(
        members
            .iter()
            .map(|member| (
                member.name.as_str(),
                member.offset,
                member.width,
                member.logical_type.clone()
            ))
            .collect::<Vec<_>>(),
        [
            ("signed", 0, 3, Some(Type::I8)),
            (
                "state",
                5,
                3,
                Some(Type::Const(Box::new(Type::Volatile(Box::new(
                    Type::value_named(values, "Status")
                )))))
            ),
            ("flag", 8, 1, Some(Type::Bool)),
            (
                "alias",
                9,
                7,
                Some(Type::Volatile(Box::new(Type::Const(Box::new(
                    Type::value_named(values, "SignedAlias")
                )))))
            ),
        ]
    );
    assert_eq!(
        members[2].annotations,
        [BitfieldAnnotation {
            source: "CXX".to_string(),
            ordinal: 0,
            payload: "[[deprecated(\"flag\")]]".to_string(),
        }]
    );
    assert!(members.iter().all(|member| member.name != "_"));
    let legacy = fields[2].bitfields().unwrap();
    assert_eq!(
        legacy
            .iter()
            .map(|member| (member.name.as_str(), member.offset, member.width))
            .collect::<Vec<_>>(),
        [("first", 0, 4), ("last", 6, 2)]
    );
    assert!(legacy.iter().all(|member| member.logical_type.is_none()));
}

#[test]
fn reviewed_logical_members_preserve_type_evidence_without_adding_storage_fields() {
    let reference = vocabulary("reviewed");
    let source = include_str!("../logical_bitfields.rdl");
    let input = output("fixture");
    compile(source, &input, &reference);
    assert_metadata(&input, "Test", "Test");
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
    let dir = Path::new(env!("OUT_DIR")).join("logical_bitfields_roundtrip");
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
    assert_metadata(&roundtrip, "Test", "Test");
    let merged = output("merged");
    merge().input(&input).output(&merged).merge().unwrap();
    assert_metadata(&merged, "Test", "Test");
    let remapped = output("remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .route("Packet", "Records")
        .fallback("Values")
        .output(&remapped)
        .remap()
        .unwrap();
    assert_metadata(&remapped, "Records", "Values");
    let inventory =
        windows_rdl::inventory(Path::new(env!("CARGO_MANIFEST_DIR")).join("logical_bitfields.rdl"))
            .unwrap();
    assert!(format!("{inventory:?}").contains("Const"));
    assert!(format!("{inventory:?}").contains("deprecated"));
}

#[test]
fn invalid_logical_members_reject_before_output_replacement() {
    let reference = vocabulary("invalid");
    for (block, diagnostic) in [
        ("value: i32: 0", "nonzero"),
        ("value: i32: 33", "exceeds backing storage"),
        ("value: *mut i32: 3", "integral or enum"),
        ("value: f32: 3", "integral or enum"),
        ("value: Record: 3", "integral or enum"),
        ("value: Pointer: 3", "integral or enum"),
        ("value: i8: 3, value: i16: 4", "duplicate bit-field member"),
        ("_: i32: 3", "padding cannot"),
        (
            "#[Windows::Win32::Metadata::NativeAnnotation(\"SAL\", 0, \"payload\")] _: 2",
            "padding cannot",
        ),
        ("value: 4294967295, other: 2", "overflows"),
    ] {
        let path = output("invalid");
        std::fs::write(&path, b"last good output").unwrap();
        let error = windows_rdl::reader()
            .reference(&reference)
            .input_text(&format!(
                "#[win32] mod Test {{ struct Record {{}} type Pointer = *mut i32;
                struct Packet {{ bits: u32 {{ {block} }}, }} }}"
            ))
            .output(&path)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{block}: {error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"last good output");
    }
}

#[test]
fn external_enum_identity_and_each_integral_leaf_remain_logical_types() {
    let reference = vocabulary("leaves");
    let external = output("external");
    compile(
        "#[win32] mod Other { #[repr(u8)] enum Flags { One = 1, } }",
        &external,
        &reference,
    );
    let input = output("external_alias");
    windows_rdl::reader()
        .reference(&external)
        .reference(&reference)
        .input_text("#[win32] mod Test { struct Packet { bits: u8 { flag: Other::Flags: 1, }, } }")
        .output(&input)
        .write()
        .unwrap();
    let index = Index::read(&input).unwrap();
    assert_eq!(
        index
            .expect("Test", "Packet")
            .fields()
            .next()
            .unwrap()
            .bitfields()
            .unwrap()[0]
            .logical_type,
        Some(Type::value_named("Other", "Flags"))
    );
    for (position, (spelling, ty)) in [
        ("bool", Type::Bool),
        ("Char16", Type::Char),
        ("i8", Type::I8),
        ("u8", Type::U8),
        ("i16", Type::I16),
        ("u16", Type::U16),
        ("i32", Type::I32),
        ("u32", Type::U32),
        ("i64", Type::I64),
        ("u64", Type::U64),
        ("isize", Type::ISize),
        ("usize", Type::USize),
    ]
    .into_iter()
    .enumerate()
    {
        let path = output(&format!("leaf_{position}"));
        compile(
            &format!(
                "#[win32] mod Test {{ struct Packet {{ bits: u64 {{ value: {spelling}: 1, }}, }} }}"
            ),
            &path,
            &reference,
        );
        let index = Index::read(&path).unwrap();
        assert_eq!(
            index
                .expect("Test", "Packet")
                .fields()
                .next()
                .unwrap()
                .bitfields()
                .unwrap()[0]
                .logical_type,
            Some(ty)
        );
    }
}

#[test]
fn backing_alias_capacity_uses_the_owners_architecture() {
    let reference = vocabulary("architecture");
    let aliases = "#[arch(X86)] type Storage = u8; #[arch(X64)] type Storage = u16;";
    let path = output("architecture");
    compile(
        &format!(
            "#[win32] mod Test {{ {aliases}
                #[arch(X86)] struct Packet {{ bits: Storage {{ value: i8: 8, }}, }}
                #[arch(X64)] struct Packet {{ bits: Storage {{ value: i8: 12, }}, }} }}"
        ),
        &path,
        &reference,
    );
    let index = Index::read(&path).unwrap();
    index.validate_native_contracts(None).unwrap();
    let merged = output("architecture_merged");
    merge().input(&path).output(&merged).merge().unwrap();
    let remapped = output("architecture_remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .fallback("Routed")
        .output(&remapped)
        .remap()
        .unwrap();
    Index::read(&remapped)
        .unwrap()
        .validate_native_contracts(None)
        .unwrap();

    for owner in ["#[arch(X86)]", ""] {
        std::fs::write(&path, b"last good output").unwrap();
        let error = windows_rdl::reader()
            .reference(&reference)
            .input_text(&format!(
                "#[win32] mod Test {{ {aliases}
                    {owner} struct Packet {{ bits: Storage {{ value: i8: 12, }}, }} }}"
            ))
            .output(&path)
            .write()
            .unwrap_err();
        assert!(
            error.to_string().contains("exceeds backing storage"),
            "{error}"
        );
        assert_eq!(std::fs::read(&path).unwrap(), b"last good output");
    }
}
