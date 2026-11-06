use windows_metadata::*;

mod boundaries {
    use super::*;

    fn u16_at(bytes: &[u8], offset: usize) -> u16 {
        u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap())
    }

    fn u32_at(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    }

    fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    struct Fixture {
        bytes: Vec<u8>,
        nt: usize,
        section: usize,
        clr: usize,
        metadata: usize,
    }

    impl Fixture {
        fn new() -> Self {
            let mut file = writer::File::new("bounds");
            let base = file.TypeRef("System", "ValueType");
            file.TypeDef(
                "Test",
                "Record",
                writer::TypeDefOrRef::TypeRef(base),
                TypeAttributes::Public | TypeAttributes::SequentialLayout,
            );
            file.Field("Value", &Type::U32, FieldAttributes::Public);
            let bytes = file.into_stream();
            let nt = u32_at(&bytes, 0x3c) as usize;
            let section = nt + 24 + u16_at(&bytes, nt + 20) as usize;
            let clr = u32_at(&bytes, section + 20) as usize;
            let metadata = clr + (u32_at(&bytes, clr + 8) - u32_at(&bytes, section + 12)) as usize;
            Self {
                bytes,
                nt,
                section,
                clr,
                metadata,
            }
        }

        fn stream(&self, name: &[u8]) -> usize {
            let version_len = u32_at(&self.bytes, self.metadata + 12) as usize;
            let mut offset = self.metadata + 20 + version_len;
            for _ in 0..u16_at(&self.bytes, offset - 2) {
                let start = offset + 8;
                let len = self.bytes[start..].iter().position(|b| *b == 0).unwrap();
                if &self.bytes[start..start + len] == name {
                    return offset;
                }
                offset = (start + len + 4) & !3;
            }
            panic!("missing fixture stream");
        }
    }

    fn check_record(bytes: Vec<u8>) {
        let index = reader::Index::new(vec![reader::File::new(bytes).unwrap()]);
        let record = index.expect("Test", "Record");
        let field = record.fields().next().unwrap();
        assert_eq!(field.name(), "Value");
        assert_eq!(field.ty(), Type::U32);
    }

    #[test]
    fn truncated_container() {
        let fixture = Fixture::new();
        let end = fixture.metadata + u32_at(&fixture.bytes, fixture.clr + 12) as usize;
        check_record(fixture.bytes[..end].to_vec());
        for end in [0, 63, fixture.section + 39, fixture.clr + 71, end - 1] {
            assert!(
                reader::File::new(fixture.bytes[..end].to_vec()).is_none(),
                "accepted a file truncated at {end}"
            );
        }
    }

    #[test]
    fn declared_header_bounds() {
        let mut fixture = Fixture::new();
        fixture.bytes[fixture.nt + 20..fixture.nt + 22].copy_from_slice(&0u16.to_le_bytes());
        assert!(reader::File::new(fixture.bytes).is_none());
    }

    #[test]
    fn section_and_directory_bounds() {
        let mut fixture = Fixture::new();
        put_u32(&mut fixture.bytes, fixture.section + 16, 71);
        assert!(reader::File::new(fixture.bytes).is_none());

        let mut fixture = Fixture::new();
        put_u32(&mut fixture.bytes, fixture.section + 8, u32::MAX);
        assert!(reader::File::new(fixture.bytes).is_none());

        let mut fixture = Fixture::new();
        put_u32(&mut fixture.bytes, fixture.nt + 24 + 212, u32::MAX);
        assert!(reader::File::new(fixture.bytes).is_none());
    }

    #[test]
    fn declared_stream_bounds() {
        for name in [b"#~".as_slice(), b"#Strings", b"#Blob", b"#GUID"] {
            let mut fixture = Fixture::new();
            let stream = fixture.stream(name);
            put_u32(&mut fixture.bytes, stream + 4, u32::MAX);
            assert!(reader::File::new(fixture.bytes).is_none());
        }
        let mut fixture = Fixture::new();
        let stream = fixture.stream(b"#Strings");
        put_u32(&mut fixture.bytes, stream, u32::MAX);
        assert!(reader::File::new(fixture.bytes).is_none());
    }

    #[test]
    fn table_rows_must_fit_stream() {
        let mut fixture = Fixture::new();
        let stream = fixture.stream(b"#~");
        put_u32(&mut fixture.bytes, stream + 4, 24);
        assert!(reader::File::new(fixture.bytes).is_none());

        let mut fixture = Fixture::new();
        let stream = fixture.stream(b"#~");
        let tables = fixture.metadata + u32_at(&fixture.bytes, stream) as usize;
        put_u32(&mut fixture.bytes, tables + 24, u32::MAX);
        assert!(reader::File::new(fixture.bytes).is_none());
    }

    #[test]
    fn heap_access_stays_in_stream() {
        for name in [b"#Strings".as_slice(), b"#Blob"] {
            let mut fixture = Fixture::new();
            let stream = fixture.stream(name);
            put_u32(&mut fixture.bytes, stream + 4, 1);
            let result = std::panic::catch_unwind(|| check_record(fixture.bytes));
            assert!(result.is_err(), "read beyond {name:?}");
        }
    }

    #[test]
    fn oversized_metadata_version() {
        let mut fixture = Fixture::new();
        put_u32(&mut fixture.bytes, fixture.metadata + 12, u32::MAX);
        assert!(reader::File::new(fixture.bytes).is_none());
    }

    #[test]
    fn pe64_headers() {
        let mut fixture = Fixture::new();
        let optional = fixture.nt + 24;
        let section = fixture.bytes[fixture.section..fixture.section + 40].to_vec();
        let clr_rva = u32_at(&fixture.bytes, optional + 208);
        fixture.bytes[optional..optional + 240].fill(0);
        fixture.bytes[optional..optional + 2].copy_from_slice(&0x20bu16.to_le_bytes());
        put_u32(&mut fixture.bytes, optional + 108, 16);
        put_u32(&mut fixture.bytes, optional + 224, clr_rva);
        put_u32(&mut fixture.bytes, optional + 228, 72);
        fixture.bytes[fixture.nt + 20..fixture.nt + 22].copy_from_slice(&240u16.to_le_bytes());
        fixture.bytes[optional + 240..optional + 280].copy_from_slice(&section);
        check_record(fixture.bytes);
    }
}

#[test]
fn in_memory_roundtrip() {
    let mut file = writer::File::new("test");
    let value_type = file.TypeRef("System", "ValueType");
    file.TypeDef(
        "Test",
        "Record",
        writer::TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public | TypeAttributes::SequentialLayout,
    );
    file.Field("Value", &Type::U32, FieldAttributes::Public);
    file.Field(
        "Values",
        &Type::ArrayFixed(Box::new(Type::U16), 3),
        FieldAttributes::Public,
    );

    let index = reader::Index::new(vec![reader::File::new(file.into_stream()).unwrap()]);
    let record = index.expect("Test", "Record");
    let fields: Vec<_> = record.fields().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "Value");
    assert_eq!(fields[0].ty(), Type::U32);
    assert_eq!(fields[1].name(), "Values");
    assert_eq!(fields[1].ty(), Type::ArrayFixed(Box::new(Type::U16), 3));
}

#[test]
fn type_index() {
    let index = reader::Index::new(vec![
        reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
    ]);

    let def = index.expect("Windows.Foundation", "Point");
    assert_eq!(def.namespace(), "Windows.Foundation");
    assert_eq!(def.name(), "Point");

    let extends = def.extends().unwrap();
    assert_eq!(extends.namespace(), "System");
    assert_eq!(extends.name(), "ValueType");

    let fields: Vec<_> = def.fields().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "X");
    assert_eq!(fields[1].name(), "Y");
    assert_eq!(fields[0].ty(), Type::F32);
    assert_eq!(fields[1].ty(), Type::F32);
}

#[test]
fn item_index() {
    let index = reader::Index::new(vec![
        reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
        reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
    ]);

    let reader::Item::Type(ty) = index.expect_item("Windows.Foundation", "Point") else {
        panic!()
    };
    assert_eq!(ty.namespace(), "Windows.Foundation");
    assert_eq!(ty.name(), "Point");

    let reader::Item::Fn(function) = index.expect_item("Windows.Win32", "ReadFileEx") else {
        panic!()
    };
    assert_eq!(function.name(), "ReadFileEx");

    let reader::Item::Const(constant) = index.expect_item("Windows.Win32", "CONTROL_C_EXIT") else {
        panic!()
    };
    assert_eq!(constant.name(), "CONTROL_C_EXIT");
}

#[test]
fn array() {
    let index = reader::Index::new(vec![
        reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
    ]);
    let def = index
        .types()
        .find(|def| def.name() == "SID_IDENTIFIER_AUTHORITY")
        .unwrap();

    let field = def.fields().find(|field| field.name() == "Value").unwrap();

    assert_eq!(
        field.ty(),
        Type::ArrayFixed(Box::new(Type::value_named("Windows.Win32", "BYTE")), 6)
    );
    assert_eq!(
        index.expect("Windows.Win32", "BYTE").underlying_type(),
        Some(Type::U8)
    );
}

#[test]
fn nested() {
    let index = reader::Index::new(vec![
        reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
    ]);

    let def = index
        .types()
        .find(|def| def.name() == "D3D10_BUFFER_RTV")
        .unwrap();

    let fields: Vec<reader::Field> = def.fields().collect();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name(), "Anonymous");
    assert_eq!(fields[1].name(), "Anonymous2");

    assert_eq!(
        fields[0].ty(),
        Type::value_named("Windows.Win32", "D3D10_BUFFER_RTV/D3D10_BUFFER_RTV_0")
    );
    assert_eq!(
        fields[1].ty(),
        Type::value_named("Windows.Win32", "D3D10_BUFFER_RTV/D3D10_BUFFER_RTV_1")
    );

    let types: Vec<reader::TypeDef> = index.nested(def).collect();
    assert_eq!(types.len(), 2);

    assert_eq!(types[0].namespace(), "");
    assert_eq!(types[1].namespace(), "");

    assert_eq!(types[0].name(), "D3D10_BUFFER_RTV_0");
    assert_eq!(types[1].name(), "D3D10_BUFFER_RTV_1");

    let fields: Vec<reader::Field> = types[0].fields().collect();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name(), "FirstElement");
    assert_eq!(fields[1].name(), "ElementOffset");

    let fields: Vec<reader::Field> = types[1].fields().collect();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name(), "NumElements");
    assert_eq!(fields[1].name(), "ElementWidth");
}
